use super::registers::MachineStateRegister;
use crate::hw::memory::Memory;

const BAT_PAGE_SHIFT: u32 = 17; // 128 KiB
const BAT_PAGE_COUNT: usize = 1 << (32 - BAT_PAGE_SHIFT);
const BAT_PAGE_MASK: u32 = (1 << BAT_PAGE_SHIFT) - 1;
const BAT_MAPPED_SUPER: u32 = 1 << 0;
const BAT_MAPPED_USER: u32 = 1 << 1;
const TLB_SIZE: usize = 128;
const TLB_WAYS: usize = 2;

#[derive(Debug)]
pub struct Mmu {
    bat: [Bat; 4],
    bat_table: Box<[u32]>,
    pub sr: [SegmentRegister; 16],
    pub sdr1: SDR1,
    tlb: [TlbEntry; TLB_SIZE / TLB_WAYS],
}

impl Default for Mmu {
    fn default() -> Self {
        Mmu {
            bat: Default::default(),
            bat_table: vec![0u32; BAT_PAGE_COUNT].into_boxed_slice(),
            sr: Default::default(),
            sdr1: Default::default(),
            tlb: [Default::default(); TLB_SIZE / TLB_WAYS],
        }
    }
}

impl Mmu {
    pub fn rebuild_bat_table(&mut self) {
        self.bat_table.fill(0);

        for bat in &self.bat {
            let mut flags = 0;
            if bat.vs_vp_valid[0] {
                flags |= BAT_MAPPED_SUPER;
            }
            if bat.vs_vp_valid[1] {
                flags |= BAT_MAPPED_USER;
            }
            if flags == 0 {
                continue;
            }

            let start = (bat.bepi & !bat.bl) >> BAT_PAGE_SHIFT;
            let pages = (bat.bl >> BAT_PAGE_SHIFT) + 1;
            for i in 0..pages {
                let idx = (start + i) as usize;
                let add = flags & !(self.bat_table[idx] & (BAT_MAPPED_SUPER | BAT_MAPPED_USER));
                if add == 0 {
                    continue;
                }

                let ea = (start + i) << BAT_PAGE_SHIFT;
                let pa = ((ea & bat.bl) | (bat.brpn & !bat.bl)) & !BAT_PAGE_MASK;
                if self.bat_table[idx] & (BAT_MAPPED_SUPER | BAT_MAPPED_USER) == 0 {
                    self.bat_table[(start + i) as usize] = pa | flags;
                } else if (self.bat_table[idx] & !BAT_PAGE_MASK) == pa {
                    self.bat_table[(start + i) as usize] |= add;
                }
            }
        }
    }

    pub fn write_batu(&mut self, index: usize, value: u32) {
        let bat = &mut self.bat[index];

        bat.bepi = value & 0xFFFE_0000;
        bat.bl = (value << 15) & 0xFFE_0000;
        bat.vs_vp_valid[0] = ((value >> 1) & 1) != 0; // Supervisor mode
        bat.vs_vp_valid[1] = (value & 1) != 0; // User mode

        self.rebuild_bat_table();
    }

    pub fn write_batl(&mut self, index: usize, value: u32) {
        let bat = &mut self.bat[index];

        bat.brpn = value & 0xFFFE_0000;
        bat.wimg = ((value >> 3) & 0xF) as u8;
        bat.pp = (value & 0x3) as u8;

        self.rebuild_bat_table();
    }

    pub fn translate_address(
        &mut self,
        ea: EffectiveAddress,
        msr: MachineStateRegister,
        memory: &mut Memory,
    ) -> Option<u32> {
        let entry = self.bat_table[(ea.0 >> BAT_PAGE_SHIFT) as usize];
        let mapped = if msr.pr() {
            BAT_MAPPED_USER
        } else {
            BAT_MAPPED_SUPER
        };
        if entry & mapped != 0 {
            return Some((entry & !BAT_PAGE_MASK) | (ea.0 & BAT_PAGE_MASK));
        }

        //if let Some(paddr) = self.translate_block_address(ea, msr) {
        //    return Some(paddr);
        //}

        if let Some(paddr) = self.translate_page_address(ea, msr, memory) {
            return Some(paddr);
        }

        None
    }

    #[allow(dead_code)]
    fn translate_block_address(
        &self,
        ea: EffectiveAddress,
        msr: MachineStateRegister,
    ) -> Option<u32> {
        let ea = ea.0;
        for bat in &self.bat {
            if !bat.vs_vp_valid[msr.pr() as usize] {
                continue;
            }

            // Valid BAT match: (EA & ~BL) == (BEPI & ~BL) in the BEPI/BL bit positions.
            if (ea & !bat.bl & 0xFFFE_0000) != (bat.bepi & !bat.bl) {
                continue;
            }

            // PA = (EA & BL) | (BRPN & ~BL), plus the 128KiB page offset.
            let pa = ((ea & bat.bl) | (bat.brpn & !bat.bl)) & 0xFFFE_0000 | (ea & 0x1_FFFF);
            return Some(pa);
        }

        None
    }

    fn translate_page_address(
        &mut self,
        ea: EffectiveAddress,
        _msr: MachineStateRegister,
        memory: &mut Memory,
    ) -> Option<u32> {
        let sr = self.sr[ea.sr() as usize];
        let vsid = sr.vsid();

        if let Some(pa) = self.lookup_tlb(ea, vsid) {
            return Some(pa);
        }

        if sr.t() {
            panic!(
                "MMU: Direct-Store segment not supported. This needs to result in a DSI or ISI Exception."
            );
        }

        let offset = ea.offset();
        let page_index = ea.page_index();
        let api = ea.api();

        let mut hash = (vsid & 0x7_FFFF) ^ page_index; // Hash Value 1

        let mut pte_lo_needle = PageTableEntryLo(0);
        pte_lo_needle.set_vsid(vsid);
        pte_lo_needle.set_api(api);
        pte_lo_needle.set_v(true);

        for _ in 0..2 {
            let mut pteg_address = calculate_pteg_addr(self.sdr1.0, hash);

            for _ in 0..8 {
                let pte_lo = PageTableEntryLo(memory.read_u32(pteg_address));

                if pte_lo == pte_lo_needle {
                    let pte_hi = PageTableEntryHi(memory.read_u32(pteg_address + 4));

                    // TODO: Access bits
                    // Check segment register - ksp and kpp
                    //let key = (sr.kp() & msr.pr()) | (sr.ks() & !msr.pr());
                    //
                    self.update_tlb(ea, pte_lo, pte_hi);

                    return Some((pte_hi.rpn() << 12) | offset);
                }

                pteg_address += 8;
            }

            hash = !hash; // Hash Value 2
            pte_lo_needle.set_h(true);
        }

        None
    }

    fn lookup_tlb(&self, ea: EffectiveAddress, vsid: u32) -> Option<u32> {
        let tag = ea.tag();
        let i = ea.tlb_index() as usize;
        let tlbe = &self.tlb[i];

        // Compare EA tag and VSID against each TLB way
        if tlbe.tag[0] == tag && tlbe.ptel[0].vsid() == vsid && tlbe.ptel[0].v() {
            return Some((tlbe.pteh[0].rpn() << 12) | ea.offset());
        }
        if tlbe.tag[1] == tag && tlbe.ptel[1].vsid() == vsid && tlbe.ptel[1].v() {
            return Some((tlbe.pteh[1].rpn() << 12) | ea.offset());
        }

        None
    }

    fn update_tlb(
        &mut self,
        ea: EffectiveAddress,
        pte_lo: PageTableEntryLo,
        pte_hi: PageTableEntryHi,
    ) {
        // FIXME: When is 2nd way utilized ?
        let i = ea.tlb_index() as usize;
        let tlbe = &mut self.tlb[i];
        tlbe.tag[0] = ea.tag();
        tlbe.ptel[0] = pte_lo;
        tlbe.pteh[0] = pte_hi;
    }

    pub fn invalidate_tlb_entry(&mut self, ea: u32) {
        let i = EffectiveAddress(ea).tlb_index() as usize;
        let tlbe = &mut self.tlb[i];
        for way in 0..TLB_WAYS {
            tlbe.ptel[way].set_v(false);
        }
    }
}

/// PTEG physical address from SDR1 and the 19-bit hash value.
fn calculate_pteg_addr(sdr1: u32, hash: u32) -> u32 {
    let sdr1 = SDR1(sdr1);
    (sdr1.htaborg() << 16) | (((hash & (sdr1.htabmask() << 10)) | (hash & 0x3FF)) << 6)
}

#[derive(Default, Clone, Copy, Debug)]
pub struct Bat {
    /// Block Effect Page Index
    bepi: u32,
    /// Block-length Mask
    bl: u32,
    /// Block Rreal Page Number
    brpn: u32,
    /// Storage Access Controls Bits (W - Write through, I - Caching-inhibited, M - Memory Coherency, G -
    /// Guarded memory)
    /// Note: In Real Addressing Mode assume the following WIMG. (Data: 0b0011, Instruction 0b0001)
    wimg: u8,
    /// Protection bits for Bat Areas (00 - No Access, x1 - Read Only, 10 - Read/Write)
    pp: u8,
    /// Valid Supervisor and User bits
    vs_vp_valid: [bool; 2],
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    pub struct SegmentRegister(u32);
    impl Debug;
    /// Format - Always 0 (Gecko and Broadway don't support direct-store segments)
    t, _ : 31;
    /// Supervisor-state protection key
    ks, _ : 30;
    /// User-state protection key
    kp, _ : 29;

    // Format 0
    /// No-execute permission
    n, _ : 28;
    /// Virtual segment ID
    vsid, _ : 24, 0;
}

bitfield! {
    /// Page Table Format
    #[derive(Copy, Clone, Default)]
    pub struct SDR1(u32);
    impl Debug;
    /// The high-order 16 bits of the 32-bit phsyical address of the page table
    htaborg, _ : 31, 16;
    /// Mask for the page table
    htabmask, _ : 9, 0;
}

bitfield! {
    #[derive(Copy, Clone)]
    pub struct EffectiveAddress(u32);
    impl Debug;
    sr, _ : 31, 28;
    /// Abbreviated page index
    api, _ : 27, 22;
    page_index, _ : 27, 12;
    offset, _ : 11, 0;
    tag, _ : 27, 18;
    /// TBL Index
    tlb_index, _ : 17, 12;
}

bitfield! {
    #[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
    struct PageTableEntryLo(u32);
    /// Entry valid (v=1) or invalid (v=0)
    v, set_v : 31;
    /// Virtual segment ID
    vsid, set_vsid : 30, 7;
    /// hash function identifier
    h, set_h : 6;
    /// Abbreviated page index
    api, set_api : 5, 0;
}

bitfield! {
    #[derive(Copy, Clone, Default, Debug)]
    struct PageTableEntryHi(u32);
    /// Physical page number (real page number)
    rpn, set_rpn : 31, 12;
    /// Referenced bit
    r, set_r : 8;
    /// Changed bit
    c, set_c : 7;
    /// Memory/cache control bits
    wimg, set_wimg : 6, 3;
    /// Page protection bits for block
    pp, set_pp : 1, 0;
}

#[derive(Debug, Default, Copy, Clone)]
pub struct TlbEntry {
    tag: [u32; TLB_WAYS],
    ptel: [PageTableEntryLo; TLB_WAYS],
    pteh: [PageTableEntryHi; TLB_WAYS],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_address_translation() {
        let msr: MachineStateRegister = 0x0.into();
        let mut mmu = Mmu::default();

        mmu.write_batu(0, 0x8000_1FFF);
        mmu.write_batl(0, 0x0000_0002);
        mmu.write_batu(1, 0xC000_1FFF);
        mmu.write_batl(1, 0x0000_002A);
        mmu.write_batu(3, 0xFFF0_001F);
        mmu.write_batl(3, 0xFFF0_0001);

        let test_data: [(u32, Option<u32>); 19] = [
            (0xFFF0_0100, Some(0xFFF0_0100)),
            (0x8000_0000, Some(0x0000_0000)),
            (0xC000_0000, Some(0x0000_0000)),
            (0x8130_0000, Some(0x0130_0000)),
            (0xC124_4588, Some(0x0124_4588)),
            (0xCC00_0000, Some(0x0C00_0000)),
            (0xCC00_1000, Some(0x0C00_1000)),
            (0xCC00_2000, Some(0x0C00_2000)),
            (0xCC00_3000, Some(0x0C00_3000)),
            (0xCC00_4000, Some(0x0C00_4000)),
            (0xCC00_5000, Some(0x0C00_5000)),
            (0xCC00_6000, Some(0x0C00_6000)),
            (0xCC00_6400, Some(0x0C00_6400)),
            (0xCC00_6800, Some(0x0C00_6800)),
            (0xCC00_6C00, Some(0x0C00_6C00)),
            (0xCC00_8000, Some(0x0C00_8000)),
            (0x0000_0000, None),
            (0x0000_00FF, None),
            (0xDC00_0000, None),
        ];
        for (ea, expect) in test_data {
            let pa = mmu.translate_block_address(EffectiveAddress(ea), msr);
            assert_eq!(expect, pa,);
        }
    }

    // Note: GC is always in supervisor mode and all bat allow supervisor access
    #[test]
    fn test_block_address_translation_privilege_modes() {
        let msr_super: MachineStateRegister = 0x0.into();
        let msr_user: MachineStateRegister = 0x4000.into();
        let mut mmu = Mmu::default();

        mmu.write_batu(0, 0x8000_1FFE); // Supervisor valid only
        mmu.write_batl(0, 0x0000_0002);
        mmu.write_batu(1, 0xC000_1FFD); // User valid only
        mmu.write_batl(1, 0x0000_002A);
        mmu.write_batu(3, 0xFFF0_001F); // Both valid
        mmu.write_batl(3, 0xFFF0_0001);

        let test_data: [(u32, Option<u32>, MachineStateRegister); 6] = [
            (0x8000_0000, Some(0x0000_0000), msr_super),
            (0xC000_0000, None, msr_super),
            (0xFFF0_0000, Some(0xFFF0_0000), msr_super),
            (0x8000_0000, None, msr_user),
            (0xC000_0000, Some(0x0000_0000), msr_user),
            (0xFFF0_0000, Some(0xFFF0_0000), msr_user),
        ];

        for (ea, expect, msr) in test_data {
            let pa = mmu.translate_block_address(EffectiveAddress(ea), msr);
            assert_eq!(expect, pa,);
        }
    }

    #[test]
    fn test_bat_table_privilege_modes() {
        let msr_super: MachineStateRegister = 0x0.into();
        let msr_user: MachineStateRegister = 0x4000.into();
        let mut mmu = Mmu::default();
        let mut memory = Memory::default();

        mmu.write_batu(0, 0x8000_1FFE); // Supervisor valid only
        mmu.write_batl(0, 0x0000_0002);
        mmu.write_batu(1, 0xC000_1FFD); // User valid only
        mmu.write_batl(1, 0x0000_002A);
        mmu.write_batu(3, 0xFFF0_001F); // Both valid
        mmu.write_batl(3, 0xFFF0_0001);

        let test_data: [(u32, Option<u32>, MachineStateRegister); 6] = [
            (0x8000_0000, Some(0x0000_0000), msr_super),
            (0xC000_0000, None, msr_super),
            (0xFFF0_0000, Some(0xFFF0_0000), msr_super),
            (0x8000_0000, None, msr_user),
            (0xC000_0000, Some(0x0000_0000), msr_user),
            (0xFFF0_0000, Some(0xFFF0_0000), msr_user),
        ];

        for (ea, expect, msr) in test_data {
            let pa = mmu.translate_address(EffectiveAddress(ea), msr, &mut memory);
            assert_eq!(expect, pa);
        }
    }

    #[test]
    fn test_page_address_translation() {
        let msr: MachineStateRegister = 0x0.into();
        let mut mmu = Mmu::default();
        let mut memory = Memory::default();

        let mem_size: u32 = 0x180_0000;

        // Calculate page table size
        let start_addr = 0xA000_0000;
        let end_addr = 0xA080_0000;
        let mut pt_size = (end_addr - start_addr) / 128;
        if pt_size <= 0x1_0000 {
            pt_size = 0x1_0000;
        }

        // Configure SDR1
        let pt_location = mem_size - pt_size;
        let htaborg = 0xFFFF_0000 & pt_location;
        let mut htabmask = 0xFFFF;
        while htaborg & (htabmask << 16) != 0 && htabmask > 0 {
            htabmask >>= 1;
        }
        let sdr1 = htaborg | htabmask;
        mmu.sdr1 = SDR1(sdr1);

        // Configure the Segment register
        let mut i = (start_addr >> 28) & 0xF;
        let j = (end_addr >> 28) & 0xF;

        loop {
            for x in 0..16 {
                if j == x {
                    mmu.sr[x as usize] = SegmentRegister(i);
                    break;
                }
            }

            i += 1;

            if i >= j {
                break;
            }
        }

        // Clear page tables: -- though we can assume they are cleared in this case
        let mut count = pt_size / 4;
        let mut addr = pt_location;
        while count > 0 {
            memory.write_u32(addr, 0);
            addr += 4;
            count -= 1;
        }

        // Segment Register Selection and Loop Setup
        for ea_addr in (start_addr..end_addr).step_by(0x1000) {
            let sr_index = (ea_addr >> 28) & 0xF;
            let sr = mmu.sr[sr_index as usize];
            let vsid = sr.vsid() << 7;

            let api = (ea_addr >> 22) & 0x3F;
            let ptl = vsid | api | 0x8000_0000;

            let rpn = ea_addr & 0xF_FFFF;
            let wimg = 0x0 << 3; // TODO: implement and test this properly
            let ptu = rpn | wimg | 0x302; // R=C=1, PP=10

            let mut hash = calculate_hash_value_1(ea_addr, sr.0);
            let mut pte_addr = calculate_pteg_addr(sdr1, hash);

            // Search for an empty pte location
            'outer: for _ in 0..2 {
                for _ in 0..8 {
                    let pte_tmp = memory.read_u32(pte_addr);

                    if pte_tmp & 0x8000_0000 == 0 {
                        break 'outer;
                    };

                    pte_addr += 8;
                }

                hash = !hash; // try again with hash value 2
                pte_addr = calculate_pteg_addr(sdr1, hash);
            }
            // Loading the upper and lower words in PTE
            memory.write_u32(pte_addr, ptl);
            memory.write_u32(pte_addr + 4, ptu);
        }

        // Now we can do a address translation lookup, finally.
        for (ea, expect) in [
            (0xA000_0000, Some(0x0000_0000)),
            (0xA000_008F, Some(0x0000_008F)),
            (0xA000_1500, Some(0x0000_1500)),
            (0xA007_FFFF, Some(0x0007_FFFF)),
            (0xA000_0000, Some(0x0000_0000)), // Expect a TLB Hit
            (0xA007_FFFF, Some(0x0007_FFFF)), // Expect a TLB Hit
        ] {
            let pa = mmu.translate_page_address(EffectiveAddress(ea), msr, &mut memory);
            assert_eq!(expect, pa,);
        }

        fn calculate_hash_value_1(ea: u32, sr: u32) -> u32 {
            (sr & 0x7_FFFF) ^ ((ea >> 12) & 0xFFFF)
        }
    }

    #[test]
    fn test_calculate_pteg_addr() {
        // HTABMASK=0: only hash[9:0] select the PTEG within HTABORG.
        assert_eq!(calculate_pteg_addr(0x017F_0000, 0x0), 0x017F_0000);
        assert_eq!(calculate_pteg_addr(0x017F_0000, 0x00A), 0x017F_0280);
        assert_eq!(calculate_pteg_addr(0x017F_0000, 0x3FF), 0x017F_FFC0);
        // High hash bits must not leak when mask is 0.
        assert_eq!(calculate_pteg_addr(0x017F_0000, 0x0400), 0x017F_0000);
        assert_eq!(calculate_pteg_addr(0x017F_0000, 0x7_FFFF), 0x017F_FFC0);

        // HTABMASK selects hash[18:10] into the page-table index.
        assert_eq!(calculate_pteg_addr(0x0000_00FF, 0x0000), 0x0000_0000);
        assert_eq!(calculate_pteg_addr(0x0000_00FF, 0x03FF), 0x0000_FFC0);
        assert_eq!(calculate_pteg_addr(0x0000_00FF, 0x0400), 0x0001_0000);
        assert_eq!(calculate_pteg_addr(0x0000_00FF, 0x07FF), 0x0001_FFC0);
        assert_eq!(calculate_pteg_addr(0x0000_00FF, 0x1400), 0x0005_0000);
        assert_eq!(calculate_pteg_addr(0x0000_00FF, 0x7_FFFF), 0x00FF_FFC0);

        assert_eq!(calculate_pteg_addr(0x0100_000F, 0x0400), 0x0101_0000);
        assert_eq!(calculate_pteg_addr(0x0040_0001, 0x0400), 0x0041_0000);
        assert_eq!(calculate_pteg_addr(0x0040_0001, 0x1400), 0x0041_0000);

        // PTEG is always 64-byte aligned.
        for &(sdr1, hash) in &[
            (0x017F_0000, 0x7_FFFF),
            (0x0000_00FF, 0x7_FFFF),
            (0x0100_000F, 0x12345),
        ] {
            assert_eq!(calculate_pteg_addr(sdr1, hash) & 0x3F, 0);
        }
    }
}
