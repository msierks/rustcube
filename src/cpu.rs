#[allow(dead_code)]
pub mod disassembler;
mod float;
pub mod instruction;
pub mod mmu;
mod ops;
mod optable;
pub mod spr;
pub mod util;

use self::instruction::Instruction;
use self::optable::*;
use self::spr::*;
use self::util::*;
use super::Context;
use mmu::{translate_address, Mmu};
use std::cmp::Ordering;

pub const NUM_FPR: usize = 32;
pub const NUM_GPR: usize = 32;
pub const NUM_SPR: usize = 1023;
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

const PROCESSOR_VERSION: u32 = 0x0008_3214;

pub struct Cpu {
    /// Current Instruction Address
    cia: u32,
    /// Next Instruction Address
    nia: u32,
    /// General-Purpose Registers
    gpr: [u32; NUM_GPR],
    /// Floating-Point Registers
    fpr: [Fpr; NUM_FPR],
    /// Special-Purpose Registers
    spr: [u32; NUM_SPR],
    /// Condition Register
    cr: ConditionRegister,
    /// Floating-Point Status and Control Register
    fpscr: FloatingPointStatusControlRegister,
    /// Integer Exception Register
    xer: Xer,
    /// Machine State Register
    pub msr: MachineStateRegister,
    /// Segment Registers
    sr: [u32; NUM_SR],
    /// Hardware Implementation-Dependent Register 1
    hid2: Hid2,
    /// Excceptions
    exceptions: u32,
    /// Memory Management Unit
    mmu: Mmu,
    /// Primary Opcode Table
    optable: [fn(&mut Context, Instruction); OPTABLE_SIZE],
    /// SubOpcode 4 Table
    optable4: [fn(&mut Context, Instruction); OPTABLE4_SIZE],
    /// SubOpcode 19 Table
    optable19: [fn(&mut Context, Instruction); OPTABLE19_SIZE],
    /// SubOpcode 31 Table
    optable31: [fn(&mut Context, Instruction); OPTABLE31_SIZE],
    /// SubOpcode 59 Table
    optable59: [fn(&mut Context, Instruction); OPTABLE59_SIZE],
    /// SubOpcode 63 Table
    optable63: [fn(&mut Context, Instruction); OPTABLE63_SIZE],
}

impl Default for Cpu {
    fn default() -> Self {
        let mut optable = [ILLEGAL_OP.1; OPTABLE_SIZE];
        let mut optable4 = [ILLEGAL_OP.1; OPTABLE4_SIZE];
        let mut optable19 = [ILLEGAL_OP.1; OPTABLE19_SIZE];
        let mut optable31 = [ILLEGAL_OP.1; OPTABLE31_SIZE];
        let mut optable59 = [ILLEGAL_OP.1; OPTABLE59_SIZE];
        let mut optable63 = [ILLEGAL_OP.1; OPTABLE63_SIZE];

        for op in OPCODE_TABLE.iter() {
            optable[op.0 as usize] = op.2;
        }

        for op in OPCODE4X_TABLE.iter() {
            optable4[op.0 as usize] = op.2;
        }

        for n in 0..32 {
            let fill = n << 5;
            for op in OPCODE4A_TABLE.iter() {
                let xo_x = (op.0 as usize) | fill;
                optable4[xo_x] = op.2;
            }
        }

        for n in 0..16 {
            let fill = n << 6;
            for op in OPCODE4AA_TABLE.iter() {
                let xo_x = (op.0 as usize) | fill;
                optable4[xo_x] = op.2;
            }
        }

        for op in OPCODE19_TABLE.iter() {
            optable19[op.0 as usize] = op.2;
        }

        for op in OPCODE31_TABLE.iter() {
            optable31[op.0 as usize] = op.2;
        }

        for op in OPCODE59_TABLE.iter() {
            optable59[op.0 as usize] = op.2;
        }

        for op in OPCODE63X_TABLE.iter() {
            optable63[op.0 as usize] = op.2;
        }

        for n in 0..32 {
            let fill = n << 5;
            for op in OPCODE63A_TABLE.iter() {
                let xo_x = (op.0 as usize) | fill;
                optable63[xo_x] = op.2;
            }
        }

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
            exceptions: EXCEPTION_SYSTEM_RESET,
            mmu: Default::default(),
            optable,
            optable4,
            optable19,
            optable31,
            optable59,
            optable63,
        };

        cpu.check_exceptions();

        cpu
    }
}

impl Cpu {
    pub fn emulate_bs2(&mut self) {
        self.msr = 0x0000_2030.into();

        // FIXME: populate SPR's accoprdingly

        for i in 0..16 {
            self.sr[i] = 0x8000_0000;
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
        self.mmu.write_ibatu(0, 0x8000_1fff);
        self.mmu.write_ibatl(0, 0x0000_0002);
        self.mmu.write_ibatu(3, 0xfff0_001f);
        self.mmu.write_ibatl(3, 0xfff0_0001);
        self.mmu.write_dbatu(0, 0x8000_1fff);
        self.mmu.write_dbatl(0, 0x0000_0002);
        self.mmu.write_dbatu(1, 0xc000_1fff);
        self.mmu.write_dbatl(1, 0x0000_002a);
        self.mmu.write_dbatu(3, 0xfff0_001f);
        self.mmu.write_dbatl(3, 0xfff0_0001);

        // dolwin boot???
        self.gpr[1] = 0x816F_FFFC;
        self.gpr[13] = 0x8110_0000;
    }

    pub fn external_interrupt(&mut self, enable: bool) {
        if enable {
            self.exceptions |= EXCEPTION_EXTERNAL_INT;
        } else {
            self.exceptions &= !EXCEPTION_EXTERNAL_INT;
        }
    }

    fn check_exceptions(&mut self) {
        if self.exceptions & EXCEPTION_SYSTEM_RESET != 0 {
            if self.msr.ip() {
                self.cia = 0x100 | 0xFFF0_0000
            } else {
                self.cia = 0x100
            }

            self.exceptions &= !EXCEPTION_SYSTEM_RESET;

            info!("EXCEPTION_SYSTEM_RESET");
        } else if self.exceptions & EXCEPTION_PROGRAM != 0 {
            self.spr[SPR_SRR0] = self.nia;
            self.spr[SPR_SRR1] = self.msr.0 & 0x87C0_FFFF;
            self.msr.set_le(self.msr.le());

            self.msr.0 &= !0x04_EF36;
            if self.msr.ip() {
                self.cia = 0x700 | 0xFFF0_0000;
            } else {
                self.cia = 0x700;
            }

            self.nia = self.cia;

            self.exceptions &= !EXCEPTION_PROGRAM;

            info!("EXCEPTION_PROGRAM");
        } else if self.exceptions & EXCEPTION_SYSTEM_CALL != 0 {
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

            self.exceptions &= !EXCEPTION_SYSTEM_CALL;

            info!("EXCEPTION_SYSTEM_CALL (PC={:#x})", self.cia);
        } else if self.exceptions & EXCEPTION_FPU_UNAVAILABLE != 0 {
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

            self.exceptions &= !EXCEPTION_FPU_UNAVAILABLE;

            info!("EXCEPTION_FPU_UNAVAILABLE");
        } else if self.exceptions & EXCEPTION_EXTERNAL_INT != 0 {
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

            self.exceptions &= !EXCEPTION_EXTERNAL_INT;

            info!("EXCEPTION_EXTERNAL_INT");
        } else if self.exceptions & EXCEPTION_PERFORMANCE_MONITOR != 0 {
            unimplemented!("EXCEPTION_PERFORMANCE_MONITOR");
        } else if self.exceptions & EXCEPTION_DECREMENTER != 0 {
            unimplemented!("EXCEPTION_DECREMENTER");
        } else if self.exceptions & EXCEPTION_THERMAL_MANAGEMENT != 0 {
            unimplemented!("EXCEPTION_THERMAL_MANAGEMENT");
        }
    }

    pub fn translate_instr_address(&self, ea: u32) -> u32 {
        if self.msr.ir() {
            translate_address(&self.mmu.ibat, self.msr, ea)
        } else {
            // real addressing mode
            ea
        }
    }

    pub fn translate_data_address(&self, ea: u32) -> u32 {
        if self.msr.dr() {
            translate_address(&self.mmu.dbat, self.msr, ea)
        } else {
            // real addressing mode
            ea
        }
    }

    pub fn set_pc(&mut self, pc: u32) {
        self.cia = pc;
    }

    pub fn pc(&self) -> u32 {
        self.cia
    }

    pub fn gpr(&self) -> &[u32; NUM_GPR] {
        &self.gpr
    }

    pub fn mut_gpr(&mut self) -> &mut [u32; NUM_GPR] {
        &mut self.gpr
    }

    pub fn fpr(&self) -> &[Fpr; NUM_FPR] {
        &self.fpr
    }

    pub fn spr(&self) -> &[u32; NUM_SPR] {
        &self.spr
    }

    pub fn mut_spr(&mut self) -> &mut [u32; NUM_SPR] {
        &mut self.spr
    }

    pub fn lr(&self) -> u32 {
        self.spr[SPR_LR]
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
}

pub fn step(ctx: &mut Context) {
    let addr = ctx.cpu.translate_instr_address(ctx.cpu.cia);

    let instr = Instruction(ctx.read_instruction(addr));

    ctx.cpu.nia = ctx.cpu.cia.wrapping_add(4);

    if instr.0 != 0 {
        ctx.cpu.optable[instr.opcd()](ctx, instr);
    } else {
        unimplemented!();
    }

    if ctx.cpu.exceptions != 0 {
        ctx.cpu.check_exceptions();
    }

    ctx.cpu.cia = ctx.cpu.nia;
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    struct FloatingPointStatusControlRegister(u32);
    impl Debug;
    u32;
    rn, _ : 1, 0;            // Floating-point rounding control
    ni, _ : 2;               // Floating-point non-IEEE mode
    xe, _ : 3;               // Floating-point inexact exception enable
    ze, _ : 4;               // IEEE floating-point zero divide exception enable
    ue, _ : 5;               // IEEE floating-point underflow exception enable
    oe, _ : 6;               // IEEE floating-point overflow exception enable
    ve, _ : 7;               // Floating-point invalid operation exception enable
    vxcvi, _ : 8;            // Floating-point invalid operation exception for invalid integer convert
    vxsqrt, set_vxsqrt : 9;  // Floating-point invalid operation exception for invalid square root
    vxsoft, _ : 10;          // Floating-point invalid operation exceptions for woftware request
    fprf, set_fprf : 16, 12; // Floating-point result flags
    fpcc, set_fpcc : 15, 12; // Floating-point condition code
    fi, _ : 17;              // Floating-point fraction inexact
    fr, _ : 18;              // Floating-point fraction round
    vxvc, set_vxvc : 19;     // Floating-point invalid operation exception for invalid compare
    vximz, set_vximz : 20;   // Floating-point invalid operation exception for (inf) * 0
    vxzdz, set_vxzdz : 21;   // Floating-point invalid operation exception for 0 / 0
    vxidi, set_vxidi : 22;   // Floating-point invalid operation exception for (inf) / (inf)
    vxisi, _ : 23;           // Floating-point invalid operation exception for (inf) - (inf)
    vxsnan, set_vxsnan : 24; // Floating-point invalid operation exception for SNaN
    xx, _ : 25;              // Floating-point inexact exception
    zx, set_zx : 26;         // Floating-point zero divide exception
    ux, _ : 27;              // Floating-point underflow exception
    ox, _ : 28;              // Floating-point overflow exception
    vx, _ : 29;              // Floating-point invalid operation exception summary
    fex, _ : 30;             // Floating-point enabled exception summary
    fx, _ : 31;              // Floating-point exception summary
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    struct Gqr(u32);
    impl Debug;
    u32;
    st, _ : 2, 0;
    ss, _ : 13, 8;
    lt, _ : 18, 16;
    ls, _ : 29, 24;
}

bitfield! {
    #[derive(Copy, Clone)]
    pub struct MachineStateRegister(u32);
    impl Debug;
    pub le, set_le : 0;    // Little-endian mode enable
    pub ri, _ : 1;         // System reset of machine check exception is recoverable
    pub pm, _ : 2;         // Performance monitor marked mode
    pub dr, _ : 4;         // Data address trranslation
    pub ir, _ : 5;         // Instruction address translation
    pub ip, _ : 6;         // Exception prefix
    pub fe1, _ : 8;        // IEEE floating-point exception mode 1
    pub be, _ : 9;         // Branch trace enable
    pub se, _ : 10;        // Single-step strace enable
    pub fe0, _ : 11;       // IEEE floating-point exception mode 0
    pub me, _ : 12;        // Machine check enable
    pub fp, _ : 13;        // Floating-point available
    pub pr, _ : 14;        // Privilege level
    pub ee, _ : 15;        // External interrupt enable
    pub ile, _ : 16;       // Exception little-endian mode
    pub pow, set_pow : 18; // Power management enable
}

impl From<u32> for MachineStateRegister {
    fn from(v: u32) -> Self {
        MachineStateRegister(v)
    }
}

impl From<MachineStateRegister> for u32 {
    fn from(v: MachineStateRegister) -> Self {
        v.0
    }
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    pub struct Xer(u32);
    impl Debug;
    pub byte_count, _ : 6, 0;
    pub carry, set_carry : 29;
    pub overflow, set_overflow : 30;
    pub summary_overflow, set_summary_overflow : 31;
}

impl From<u32> for Xer {
    fn from(v: u32) -> Self {
        Xer(v)
    }
}

impl From<Xer> for u32 {
    fn from(s: Xer) -> u32 {
        s.0
    }
}

#[derive(Default, Debug)]
pub struct ConditionRegister(u32);

impl ConditionRegister {
    pub fn as_u32(&self) -> u32 {
        self.0
    }

    pub fn set(&mut self, value: u32) {
        self.0 = value;
    }

    pub fn set_field(&mut self, field: usize, value: u32) {
        self.0 = (self.0 & (!(0xF0000000 >> (field * 4)))) | (value << ((7 - field) * 4));
    }

    pub fn get_bit(&self, bit: usize) -> u8 {
        ((self.0 >> (31 - bit)) & 1) as u8
    }

    pub fn set_bit(&mut self, bit: usize, value: u8) {
        self.0 = ((value as u32) << (31 - bit)) | (self.0 & !(0x8000_0000 >> bit));
    }

    pub fn get_cr0(&mut self) -> u8 {
        (self.0 >> 28) as u8
    }
}

impl From<u32> for ConditionRegister {
    fn from(v: u32) -> Self {
        ConditionRegister(v)
    }
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    pub struct Hid2(u32);
    impl Debug;
    pub dqoee, _ : 16;
    pub dcmee, _ : 17;
    pub dncee, _ : 18;
    pub dchee, _ : 19;
    pub dqoerr, _ : 20;
    pub dcmerr, _ : 21;
    pub dncerr, _ : 22;
    pub dcherr, _ : 23;
    pub dmaql, _ : 27, 24;
    pub lce, _ : 28;
    pub pse, _ : 29;
    pub wpe, _ : 30;
    pub lsqe, _ : 31;
}

impl From<u32> for Hid2 {
    fn from(v: u32) -> Self {
        Hid2(v)
    }
}

#[derive(Default, Clone)]
pub struct Fpr {
    ps0: u64,
    ps1: u64,
}

impl Fpr {
    pub fn ps0(&self) -> u64 {
        self.ps0
    }

    pub fn ps1(&self) -> u64 {
        self.ps1
    }

    fn set_ps0(&mut self, v: u64) {
        self.ps0 = v;
    }

    fn set_ps1(&mut self, v: u64) {
        self.ps1 = v;
    }

    fn set_ps0_f64(&mut self, v: f64) {
        self.ps0 = f64::to_bits(v);
    }

    fn set_ps1_f64(&mut self, v: f64) {
        self.ps1 = f64::to_bits(v);
    }

    pub fn ps0_as_f64(&self) -> f64 {
        f64::from_bits(self.ps0)
    }

    pub fn ps1_as_f64(&self) -> f64 {
        f64::from_bits(self.ps1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn condition_register() {
        let mut cr = ConditionRegister(0x00F0_F0F0);

        cr.set_bit(2, 1);
        assert_eq!(cr.0, 0x20F0_F0F0);
        assert_eq!(cr.get_bit(2), 1);

        cr.set_bit(2, 0);
        assert_eq!(cr.0, 0x00F0_F0F0);
        assert_eq!(cr.get_bit(2), 0);

        cr.set_field(0, 0xF);
        assert_eq!(cr.0, 0xF0F0_F0F0);

        cr.set_field(0, 0x3);
        assert_eq!(cr.0, 0x30F0_F0F0);

        cr.set_field(0, 0x0);
        assert_eq!(cr.0, 0x00F0_F0F0);
    }
}
