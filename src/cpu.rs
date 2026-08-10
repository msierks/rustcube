#[allow(dead_code)]
pub(crate) mod disassembler;
mod float;
pub(crate) mod instruction;
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
//const EXCEPTION_DSI: u32 = 0x4;
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
        self.msr = 0x0000_2030.into();

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
        self.write::<u32>(bus, 0x8000_0800, OP_RFI); // FPU unavailable
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

    fn check_exceptions(&mut self) {
        if self.state.exceptions & EXCEPTION_SYSTEM_RESET != 0 {
            if self.msr.ip() {
                self.cia = 0x100 | 0xFFF0_0000
            } else {
                self.cia = 0x100
            }

            self.state.exceptions &= !EXCEPTION_SYSTEM_RESET;

            info!("EXCEPTION_SYSTEM_RESET");
        } else if self.state.exceptions & EXCEPTION_PROGRAM != 0 {
            let srr0 = self.program_exception_srr0;
            let cause_bits = self.program_exception_srr1;
            self.program_exception_srr1 = 0;

            self.spr[SPR_SRR0] = srr0;
            self.spr[SPR_SRR1] = (self.msr.0 & 0x87C0_FFFF) | cause_bits;
            self.msr.set_le(self.msr.ile());

            self.msr.0 &= !0x04_EF36;
            if self.msr.ip() {
                self.cia = 0x700 | 0xFFF0_0000;
            } else {
                self.cia = 0x700;
            }

            self.nia = self.cia;

            self.state.exceptions &= !EXCEPTION_PROGRAM;

            info!("EXCEPTION_PROGRAM");
        } else if self.state.exceptions & EXCEPTION_SYSTEM_CALL != 0 {
            self.spr[SPR_SRR0] = self.nia;
            self.spr[SPR_SRR1] = self.msr.0 & 0x87C0_FFFF;
            self.msr.set_le(self.msr.le());
            self.msr.0 &= !0x04_EF36;

            if self.msr.ip() {
                self.cia = 0xC00 | 0xFFF0_0000;
            } else {
                self.cia = 0xC00;
            }

            self.nia = self.cia;

            self.state.exceptions &= !EXCEPTION_SYSTEM_CALL;

            info!("EXCEPTION_SYSTEM_CALL (PC={:#x})", self.cia);
        } else if self.state.exceptions & EXCEPTION_FPU_UNAVAILABLE != 0 {
            self.spr[SPR_SRR0] = self.nia;
            self.spr[SPR_SRR1] = self.msr.0 & 0x87C0_FFFF;
            self.msr.set_le(self.msr.le());

            self.msr.0 &= !0x04_EF36;
            if self.msr.ip() {
                self.cia = 0x800 | 0xFFF0_0000;
            } else {
                self.cia = 0x800;
            }

            self.nia = self.cia;

            self.state.exceptions &= !EXCEPTION_FPU_UNAVAILABLE;

            info!("EXCEPTION_FPU_UNAVAILABLE");
        } else if self.state.exceptions & EXCEPTION_EXTERNAL_INT != 0 {
            if !self.msr.ee() {
                return;
            }

            self.spr[SPR_SRR0] = self.nia;
            self.spr[SPR_SRR1] = self.msr.0 & 0x87C0_FFFF;
            self.msr.set_le(self.msr.le());
            self.msr.0 &= !0x04_EF36;

            if self.msr.ip() {
                self.cia = 0x500 | 0xFFF0_0000;
            } else {
                self.cia = 0x500;
            }

            self.nia = self.cia;

            self.state.exceptions &= !EXCEPTION_EXTERNAL_INT;

            info!("EXCEPTION_EXTERNAL_INT");
        } else if self.state.exceptions & EXCEPTION_PERFORMANCE_MONITOR != 0 {
            unimplemented!("EXCEPTION_PERFORMANCE_MONITOR");
        } else if self.state.exceptions & EXCEPTION_DECREMENTER != 0 {
            unimplemented!("EXCEPTION_DECREMENTER");
        } else if self.state.exceptions & EXCEPTION_THERMAL_MANAGEMENT != 0 {
            unimplemented!("EXCEPTION_THERMAL_MANAGEMENT");
        }
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

    pub fn translate_data_address(&mut self, ea: u32, memory: &mut Memory) -> u32 {
        if self.msr.dr() {
            self.dmmu
                .translate_address(mmu::EffectiveAddress(ea), self.msr, memory)
                .unwrap_or_else(|| {
                    panic!("DSI: unmapped instr ea={ea:#010x} pc={:#010x}", self.cia)
                })
        } else {
            // real addressing mode
            ea
        }
    }

    pub fn read<T>(&mut self, bus: &mut Bus, ea: u32) -> T
    where
        Mmio: ReadWrite<T>,
        Memory: ReadWrite<T>,
        Bootrom: ReadWrite<T>,
    {
        let addr = self.translate_data_address(ea, &mut bus.memory);

        bus.read(&mut self.state, addr)
    }

    pub fn write<T>(&mut self, bus: &mut Bus, ea: u32, val: T)
    where
        Mmio: ReadWrite<T>,
        Memory: ReadWrite<T>,
        Bootrom: ReadWrite<T>,
    {
        let addr = self.translate_data_address(ea, &mut bus.memory);

        bus.write(&mut self.state, addr, val);
    }

    pub fn write_bytes(&mut self, bus: &mut Bus, ea: u32, data: &[u8]) {
        let addr = self.translate_data_address(ea, &mut bus.memory);

        bus.write_bytes(&mut self.state, addr, data);
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
        self.state.timers.tick(cycles);
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
