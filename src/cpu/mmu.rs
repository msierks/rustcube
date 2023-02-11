use super::MachineStateRegister;

#[derive(Default, Clone, Copy, Debug)]
pub struct Bat {
    bepi: u16, // Block Effect Page Index
    bl: u16,   // Block-length Mask
    vs: bool,  // Supervisor state valid bit -- allows root access
    vp: bool,  // Problem state valid bit -- allows user access
    brpn: u16, // Block Real Page Number
    wimg: u8,  // Storage Access Controls
    pp: u8,    // Protection bits for Bat Ares (00 No Access, 01 Read Only, 10 Read/Write)
}

#[derive(Debug, Default)]
pub struct Mmu {
    pub dbat: [Bat; 4],
    pub ibat: [Bat; 4],
}

impl Mmu {
    pub fn write_ibatu(&mut self, index: usize, value: u32) {
        let bat = &mut self.ibat[index];

        // FixMe: validate BAT value
        // MSRIR | MSRDR = 1
        // (Vs & ~MSRPR) | (Vp & MSRPR) = 1

        bat.bepi = ((value >> 17) & 0x7FFF) as u16;
        bat.bl = ((value >> 2) & 0x7FF) as u16;
        bat.vs = ((value >> 1) & 1) != 0;
        bat.vp = (value & 1) != 0;
    }

    pub fn write_dbatu(&mut self, index: usize, value: u32) {
        let bat = &mut self.dbat[index];

        // FixMe: validate BAT value
        // MSRIR | MSRDR = 1
        // (Vs & ~MSRPR) | (Vp & MSRPR) = 1

        bat.bepi = ((value >> 17) & 0x7FFF) as u16;
        bat.bl = ((value >> 2) & 0x7FF) as u16;
        bat.vs = ((value >> 1) & 1) != 0;
        bat.vp = (value & 1) != 0;
    }

    pub fn write_ibatl(&mut self, index: usize, value: u32) {
        let bat = &mut self.ibat[index];

        // FixMe: validate BAT value

        bat.brpn = (value >> 17 & 0x7FFF) as u16;
        bat.wimg = (value >> 3 & 0x1F) as u8;
        bat.pp = (value & 3) as u8;
    }

    pub fn write_dbatl(&mut self, index: usize, value: u32) {
        let bat = &mut self.dbat[index];

        // FixMe: validate BAT value

        bat.brpn = (value >> 17 & 0x7FFF) as u16;
        bat.wimg = (value >> 3 & 0x1F) as u8;
        bat.pp = (value & 3) as u8;
    }
}

pub fn translate_address(bats: &[Bat; 4], msr: MachineStateRegister, ea: u32) -> u32 {
    // Block Address Translation
    for bat in bats {
        let ea_15 = (ea >> 17) as u16;
        let ea_bepi = (ea_15 & 0x7800) ^ ((ea_15 & 0x7FF) & (!bat.bl));

        if ea_bepi == bat.bepi && ((!msr.pr() && bat.vs) || (msr.pr() && bat.vp)) {
            let upper = u32::from(bat.brpn ^ ((ea_15 & 0x7FF) & bat.bl));
            let lower = ea & 0x1FFFF;

            return (upper << 17) ^ lower;
        }
    }

    // Segment Address Translation
    unimplemented!("MMU page/segment address translation {:#x} {:}", ea, ea);
}
