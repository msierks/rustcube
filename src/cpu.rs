#[allow(dead_code)]
pub(crate) mod disassembler;
mod float;
pub(crate) mod instruction;
pub(crate) mod l1_cache;
pub(crate) mod mmu;
mod op_branch;
mod op_condition;
mod op_float;
mod op_integer;
mod op_load_store;
mod op_system;
mod opcodes;
mod optable;
pub(crate) mod registers;
pub(crate) mod timers;
pub(crate) mod utils;

use std::cmp::Ordering;

use self::{
    instruction::Instruction,
    l1_cache::L1Cache,
    mmu::{EffectiveAddress, Mmu, SegmentRegister},
    optable::*,
    registers::*,
    timers::{Timers, BUS_CLOCK, CPU_CLOCK},
};
use crate::{
    bus::{Bus, ReadWrite},
    hw::{
        bootrom::Bootrom,
        memory::{Memory, MEMORY_SIZE},
        mmio::Mmio,
    },
};

pub(crate) const NUM_FPR: usize = 32;
pub(crate) const NUM_GPR: usize = 32;
pub(crate) const NUM_SPR: usize = 1023;
const NUM_SR: usize = 16;

const EXCEPTION_SYSTEM_RESET: u32 = 0x1;
//const EXCEPTION_MACHINE_CHECK: u32 = 0x2;
const EXCEPTION_DSI: u32 = 0x4;
//const EXCEPTION_ISI: u32 = 0x8;
const EXCEPTION_EXTERNAL_INT: u32 = 0x10;
//const EXCEPTION_ALIGNMENT: u32 = 0x20;
const EXCEPTION_PROGRAM: u32 = 0x40;
const EXCEPTION_FPU_UNAVAILABLE: u32 = 0x80;
const EXCEPTION_DECREMENTER: u32 = 0x100;
const EXCEPTION_SYSTEM_CALL: u32 = 0x200;
//const EXCEPTION_TRACE: u32 = 0x400;
//const EXCEPTION_FPU_ASSIST: u32 = 0x800;
const EXCEPTION_PERFORMANCE_MONITOR: u32 = 0x1000; // Gekko Only
const _EXCEPTION_IABR: u32 = 0x2000; // Gekko Only
const EXCEPTION_THERMAL_MANAGEMENT: u32 = 0x4000; // Gekko Only

const OP_RFI: u32 = 0x4C00_0064;
const PROCESSOR_VERSION: u32 = 0x0008_3214;

/// DSISR bit 1: page fault / translation not found.
const DSISR_PAGE_FAULT: u32 = 0x4000_0000;
/// DSISR bit 4: blocked by a page or DBAT PP bits
const _DSISR_PROTECTION: u32 = 0x0800_0000;
/// DSISR bit 5: lwarx/stwcx to write-through
const _DSISR_BAD_ACCESS: u32 = 0x0400_0000;
/// DSISR bit 6: Set for stores, clear for loads.
const DSISR_STORE: u32 = 0x0200_0000;
/// DSISR bit 9: Data address breakpoint match
const _DSISR_DABR: u32 = 0x0040_00000;
/// DSISR bit 11: eciwx and ecowx and EAR[E] = 0
const _DSISR_ECIWX_ECOWX: u32 = 0x0010_0000;

pub(crate) struct Cpu {
    /// Current Instruction Address
    pub(crate) cia: u32,
    /// Next Instruction Address
    nia: u32,
    /// General-Purpose Registers
    pub(crate) gpr: [u32; NUM_GPR],
    /// Floating-Point Registers
    fpr: [Fpr; NUM_FPR],
    /// Special-Purpose Registers
    pub(crate) spr: [u32; NUM_SPR],
    /// Condition Register
    cr: ConditionRegister,
    /// Floating-Point Status and Control Register
    fpscr: FloatingPointStatusControlRegister,
    /// Integer Exception Register
    xer: Xer,
    /// Machine State Register
    pub(crate) msr: MachineStateRegister,
    /// Segment Registers
    sr: [u32; NUM_SR],
    /// Hardware Implementation-Dependent Register 1
    hid2: HardwareImplementationDependentRegister2,
    /// Pending program-exception SRR1 reason bits (11–14).
    program_exception_srr1: u32,
    /// Effective address of the instruction that caused the pending program exception.
    program_exception_srr0: u32,
    /// Instruction Memory Management Unit (IMMU)
    immu: Mmu,
    /// Data Memory Management Unit (DMMU)
    dmmu: Mmu,
    /// Cpu State
    pub(crate) state: CpuState,
}

impl Default for Cpu {
    fn default() -> Self {
        let mut spr = [0; NUM_SPR];

        spr[SPR_PVR] = PROCESSOR_VERSION;

        let mut cpu = Cpu {
            cia: 0,
            nia: 0,
            gpr: Default::default(),
            fpr: Default::default(),
            spr,
            cr: Default::default(),
            fpscr: Default::default(),
            xer: Default::default(),
            msr: 0x40.into(),
            sr: [0; NUM_SR],
            hid2: Default::default(),
            program_exception_srr1: 0,
            program_exception_srr0: 0,
            immu: Default::default(),
            dmmu: Default::default(),
            state: Default::default(),
        };

        cpu.check_exceptions();

        cpu
    }
}

impl Cpu {
    pub fn emulate_bs2(&mut self, bus: &mut Bus) {
        self.msr = 0x0000_2032.into(); // FP | IR | DR | RI

        for i in 0..16 {
            self.sr[i] = 0x8000_0000;
            self.immu.sr[i] = SegmentRegister(0x8000_0000);
            self.dmmu.sr[i] = SegmentRegister(0x8000_0000);
        }

        self.spr[SPR_IBAT0U] = 0x8000_1FFF;
        self.spr[SPR_IBAT0L] = 0x0000_0002;
        self.spr[SPR_IBAT3U] = 0xFFF0_001F;
        self.spr[SPR_IBAT3L] = 0xFFF0_0001;
        self.spr[SPR_DBAT0U] = 0x8000_1FFF;
        self.spr[SPR_DBAT0L] = 0x0000_0002;
        self.spr[SPR_DBAT1U] = 0xC000_1FFF;
        self.spr[SPR_DBAT1L] = 0x0000_002A;
        self.spr[SPR_DBAT3U] = 0xFFF0_001F;
        self.spr[SPR_DBAT3L] = 0xFFF0_0001;

        self.immu.write_batu(0, 0x8000_1FFF);
        self.immu.write_batl(0, 0x0000_0002);
        self.immu.write_batu(3, 0xFFF0_001F);
        self.immu.write_batl(3, 0xFFF0_0001);
        self.dmmu.write_batu(0, 0x8000_1FFF);
        self.dmmu.write_batl(0, 0x0000_0002);
        self.dmmu.write_batu(1, 0xC000_1FFF);
        self.dmmu.write_batl(1, 0x0000_002A);
        self.dmmu.write_batu(3, 0xFFF0_001F);
        self.dmmu.write_batl(3, 0xFFF0_0001);

        self.gpr[1] = 0x8156_6550;
        self.gpr[2] = 0x8146_5CC0;
        self.gpr[13] = 0x8146_5320;

        // Magic Word (0x0D15_EA5E - Normal Boot, 0xE520_7C22 - booted from jtag)
        self.write::<u32>(bus, 0x8000_0020, 0x0D15_EA5E);
        // Version
        self.write::<u32>(bus, 0x8000_0024, 0x0000_0001);
        // Physical Memory Size
        self.write::<u32>(bus, 0x8000_0028, MEMORY_SIZE);
        // Simulated memory size (set by Wii BS2; some homebrew reads this)
        self.write::<u32>(bus, 0x8000_00F0, MEMORY_SIZE);
        // Console Type - Latest Devkit; some titles break on retail ID.
        self.write::<u32>(bus, 0x8000_002C, 0x1000_0006);
        // ArenaLo / ArenaHi
        self.write::<u32>(bus, 0x8000_0030, 0x0000_0000);
        self.write::<u32>(bus, 0x8000_0034, 0x817F_E8C0);
        // Fake VI Init of the IPL (0 - NTSC, 1 - PAL, 2 - debug, 3 - debug pal, 4 - mpal, 5 - pal 60)
        self.write::<u32>(bus, 0x8000_00CC, 0);
        // ARAM size: 16MB (BS2)
        self.write::<u32>(bus, 0x8000_00D0, 0x0100_0000);
        // Bus / CPU clock
        self.write::<u32>(bus, 0x8000_00F8, BUS_CLOCK as u32);
        self.write::<u32>(bus, 0x8000_00FC, CPU_CLOCK as u32);

        // Exception handlers
        self.write::<u32>(bus, 0x8000_0300, OP_RFI); // DSI
        self.write::<u32>(bus, 0x8000_0500, OP_RFI); // External interrupt
        self.write::<u32>(bus, 0x8000_0800, OP_RFI); // FPU unavailable
        self.write::<u32>(bus, 0x8000_0900, OP_RFI); // Decrementer
        self.write::<u32>(bus, 0x8000_0C00, OP_RFI); // System call
    }

    pub fn step(&mut self, bus: &mut Bus) {
        let addr = self.translate_instr_address(self.cia, &mut bus.memory);

        let instr = Instruction(bus.read::<u32>(&mut self.state, addr));

        self.nia = self.cia.wrapping_add(4);

        if instr.0 != 0 {
            OPTABLE[instr.opcd()](self, instr, bus);
        } else {
            unimplemented!();
        }

        self.cia = self.nia;

        if self.state.exceptions != 0 {
            self.check_exceptions();
            self.cia = self.nia;
        }
    }

    /// Record a program exception to be taken after the current instruction.
    fn generate_program_exception(&mut self, cause: ProgramException) {
        self.state.exceptions |= EXCEPTION_PROGRAM;
        if self.program_exception_srr1 == 0 {
            self.program_exception_srr0 = self.cia;
        }
        self.program_exception_srr1 |= cause.srr1_bits();
    }

    /// Record a DSI for an unmapped/failed data translation.
    fn generate_dsi_exception(&mut self, ea: u32, store: bool) {
        if self.state.exceptions & EXCEPTION_DSI != 0 {
            return;
        }

        self.spr[SPR_DAR] = ea;
        self.spr[SPR_DSISR] = DSISR_PAGE_FAULT | if store { DSISR_STORE } else { 0 };
        self.state.exceptions |= EXCEPTION_DSI;
    }

    fn check_exceptions(&mut self) {
        if self.state.exceptions & EXCEPTION_SYSTEM_RESET != 0 {
            self.cia = self.exception_vector(0x100);
            self.state.exceptions &= !EXCEPTION_SYSTEM_RESET;
            info!("EXCEPTION_SYSTEM_RESET");
        } else if self.state.exceptions & EXCEPTION_PROGRAM != 0 {
            let srr0 = self.program_exception_srr0;
            let cause_bits = self.program_exception_srr1;
            self.program_exception_srr1 = 0;
            self.take_exception(0x700, srr0, cause_bits, EXCEPTION_PROGRAM);
            info!("EXCEPTION_PROGRAM");
        } else if self.state.exceptions & EXCEPTION_SYSTEM_CALL != 0 {
            self.take_exception(0xC00, self.nia, 0, EXCEPTION_SYSTEM_CALL);
            info!("EXCEPTION_SYSTEM_CALL (PC={:#x})", self.cia);
        } else if self.state.exceptions & EXCEPTION_FPU_UNAVAILABLE != 0 {
            self.take_exception(0x800, self.nia, 0, EXCEPTION_FPU_UNAVAILABLE);
            info!("EXCEPTION_FPU_UNAVAILABLE");
        } else if self.state.exceptions & EXCEPTION_EXTERNAL_INT != 0 {
            if !self.take_ee_exception(0x500, EXCEPTION_EXTERNAL_INT) {
                return;
            }
            info!("EXCEPTION_EXTERNAL_INT");
        } else if self.state.exceptions & EXCEPTION_PERFORMANCE_MONITOR != 0 {
            unimplemented!("EXCEPTION_PERFORMANCE_MONITOR");
        } else if self.state.exceptions & EXCEPTION_DECREMENTER != 0 {
            if !self.take_ee_exception(0x900, EXCEPTION_DECREMENTER) {
                return;
            }
            info!("EXCEPTION_DECREMENTER -> {:#x}", self.cia);
        } else if self.state.exceptions & EXCEPTION_THERMAL_MANAGEMENT != 0 {
            unimplemented!("EXCEPTION_THERMAL_MANAGEMENT");
        }
    }

    fn take_exception(&mut self, vector: u32, srr0: u32, srr1_extra: u32, clear: u32) {
        self.spr[SPR_SRR0] = srr0;
        self.spr[SPR_SRR1] = (self.msr.0 & 0x87C0_FFFF) | srr1_extra;
        self.msr.set_le(self.msr.ile());
        self.msr.0 &= !0x04_EF36;
        self.cia = self.exception_vector(vector);
        self.nia = self.cia;
        self.state.exceptions &= !clear;
    }

    fn take_ee_exception(&mut self, vector: u32, clear: u32) -> bool {
        if !self.msr.ee() {
            return false;
        }
        self.take_exception(vector, self.nia, 0, clear);
        true
    }

    pub fn translate_instr_address(&mut self, ea: u32, memory: &mut Memory) -> u32 {
        if self.msr.ir() {
            self.immu
                .translate_address(EffectiveAddress(ea), self.msr, memory)
                .unwrap_or_else(|| {
                    panic!("ISI: unmapped instr ea={ea:#010x} pc={:#010x}", self.cia)
                })
        } else {
            // real addressing mode
            ea
        }
    }

    pub fn translate_data_address(
        &mut self,
        ea: u32,
        memory: &mut Memory,
        store: bool,
    ) -> Option<u32> {
        if self.msr.dr() {
            match self
                .dmmu
                .translate_address(mmu::EffectiveAddress(ea), self.msr, memory)
            {
                Some(pa) => Some(pa),
                None => {
                    self.generate_dsi_exception(ea, store);
                    None
                }
            }
        } else {
            // real addressing mode
            Some(ea)
        }
    }

    pub fn read<T>(&mut self, bus: &mut Bus, ea: u32) -> Option<T>
    where
        Mmio: ReadWrite<T>,
        Memory: ReadWrite<T>,
        L1Cache: ReadWrite<T>,
        Bootrom: ReadWrite<T>,
    {
        self.translate_data_address(ea, &mut bus.memory, false)
            .map(|addr| bus.read(&mut self.state, addr))
    }

    pub fn write<T>(&mut self, bus: &mut Bus, ea: u32, val: T) -> bool
    where
        Mmio: ReadWrite<T>,
        Memory: ReadWrite<T>,
        L1Cache: ReadWrite<T>,
    {
        match self.translate_data_address(ea, &mut bus.memory, true) {
            Some(addr) => {
                bus.write(&mut self.state, addr, val);
                true
            }
            None => false,
        }
    }

    pub fn write_bytes(&mut self, bus: &mut Bus, ea: u32, data: &[u8]) -> bool {
        match self.translate_data_address(ea, &mut bus.memory, true) {
            Some(addr) => {
                bus.write_bytes(&mut self.state, addr, data);
                true
            }
            None => false,
        }
    }

    fn set_xer_so(&mut self, value: bool) {
        self.xer.set_overflow(value);
        self.xer.set_summary_overflow(value);
    }

    fn update_cr0(&mut self, r: u32) {
        let value = r as i32;

        let mut flags = match value.cmp(&0) {
            Ordering::Less => 0x8,    // LT
            Ordering::Greater => 0x4, // GT
            Ordering::Equal => 0x2,   // EQ
        };

        flags |= self.xer.summary_overflow() as u32;

        self.cr.set_field(0, flags);
    }

    fn update_cr1(&mut self) {
        // FX, FEX, VX, OX
        let flags = (self.fpscr.0 & 0xF000_0000) >> 28;

        self.cr.set_field(1, flags);
    }

    fn ensure_fp(&mut self) -> bool {
        if !self.msr.fp() {
            self.state.exceptions |= EXCEPTION_FPU_UNAVAILABLE;
            return false;
        }
        true
    }

    fn ensure_ps(&mut self) -> bool {
        if !self.hid2.pse() {
            self.state.exceptions |= EXCEPTION_PROGRAM;
            return false;
        }
        self.ensure_fp()
    }

    pub fn tick(&mut self, cycles: u32) {
        if self.state.timers.tick_decrementer(cycles) {
            self.state.timers.set_decrementer(0xFFFF_FFFF);
            self.spr[SPR_DEC] = 0xFFFF_FFFF;
            self.state.exceptions |= EXCEPTION_DECREMENTER;
        }
        self.state.timers.tick(cycles);
    }

    fn exception_vector(&self, vector: u32) -> u32 {
        if self.msr.ip() {
            vector | 0xFFF0_0000
        } else {
            vector
        }
    }
}

pub struct CpuState {
    exceptions: u32,
    pub(crate) timers: Timers,
}

impl Default for CpuState {
    fn default() -> Self {
        CpuState {
            exceptions: EXCEPTION_SYSTEM_RESET,
            timers: Default::default(),
        }
    }
}

impl CpuState {
    pub(crate) fn external_interrupt(&mut self, enable: bool) {
        if enable {
            self.exceptions |= EXCEPTION_EXTERNAL_INT;
        } else {
            self.exceptions &= !EXCEPTION_EXTERNAL_INT;
        }
    }
}
