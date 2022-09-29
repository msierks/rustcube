mod fpscr;
mod gqr;
mod hid;

pub mod instruction;
pub mod mmu;
pub mod util;

use std::fmt;

use self::fpscr::Fpscr;
use self::gqr::Gqr;
use self::hid::Hid2;
use self::instruction::Instruction;
use self::util::*;
use super::Context;
use mmu::{translate_address, Mmu};

const NUM_FPR: usize = 32;
const NUM_GPR: usize = 32;
const NUM_SPR: usize = 1022;
const NUM_GQR: usize = 8;
const NUM_SR: usize = 16;

const OPTABLE_SIZE: usize = 64;
const OPTABLE19_SIZE: usize = 1024;
const OPTABLE31_SIZE: usize = 1024;
const OPTABLE59_SIZE: usize = 32;
const OPTABLE63_SIZE: usize = 1024;

const SPR_XER: usize = 1;
const SPR_LR: usize = 8;
const SPR_CTR: usize = 9;
const SPR_DSISR: usize = 18;
const SPR_DAR: usize = 19;
const SPR_DEC: usize = 22;
const SPR_SDR1: usize = 25;
const SPR_SRR0: usize = 26;
const SPR_SRR1: usize = 27;
const SPR_SPRG0: usize = 272;
const SPR_EAR: usize = 282;
const SPR_TBL: usize = 284;
const SPR_TBU: usize = 285;
const SPR_PVR: usize = 287;
const SPR_IBAT0U: usize = 528;
const SPR_IBAT0L: usize = 529;
const SPR_IBAT1U: usize = 530;
const SPR_IBAT1L: usize = 531;
const SPR_IBAT2U: usize = 532;
const SPR_IBAT2L: usize = 533;
const SPR_IBAT3U: usize = 534;
const SPR_IBAT3L: usize = 535;
const SPR_DBAT0U: usize = 536;
const SPR_DBAT0L: usize = 537;
const SPR_DBAT1U: usize = 538;
const SPR_DBAT1L: usize = 539;
const SPR_DBAT2U: usize = 540;
const SPR_DBAT2L: usize = 541;
const SPR_DBAT3U: usize = 542;
const SPR_DBAT3L: usize = 543;
const SPR_GQR0: usize = 912;
const SPR_HID2: usize = 920;
const SPR_WPAR: usize = 921;
const SPR_DMAU: usize = 922;
const SPR_UMMCR0: usize = 936;
const SPR_UPMC1: usize = 937;
const SPR_USIA: usize = 939;
const SPR_UMMCR1: usize = 940;
const SPR_UPMC3: usize = 941;
const SPR_UPMC4: usize = 942;
const SPR_MMCR0: usize = 952;
const SPR_PMC1: usize = 953;
const SPR_PMC2: usize = 954;
const SPR_SIA: usize = 955;
const SPR_MMCR1: usize = 956;
const SPR_PMC3: usize = 957;
const SPR_PMC4: usize = 958;
const SPR_IABR: usize = 1010;
const SPR_HID0: usize = 1008;
const SPR_HID1: usize = 1009;
const SPR_DABR: usize = 1013;
const SPR_L2CR: usize = 1017;
const SPR_ICTC: usize = 1019;
const SPR_THRM1: usize = 1020;

const EXCEPTION_SYSTEM_RESET: u32 = 0x1;
const EXCEPTION_MACHINE_CHECK: u32 = 0x2;
const EXCEPTION_DSI: u32 = 0x4;
const EXCEPTION_ISI: u32 = 0x8;
const EXCEPTION_EXTERNAL_INT: u32 = 0x10;
const EXCEPTION_ALIGNMENT: u32 = 0x20;
const EXCEPTION_PROGRAM: u32 = 0x40;
const EXCEPTION_FPU_UNAVAILABLE: u32 = 0x80;
const EXCEPTION_DECREMENTER: u32 = 0x100;
const EXCEPTION_SYSTEM_CALL: u32 = 0x200;
const EXCEPTION_TRACE: u32 = 0x400;
const EXCEPTION_FPU_ASSIST: u32 = 0x800;
const EXCEPTION_PERFORMANCE_MONITOR: u32 = 0x1000; // Gekko Only
const EXCEPTION_IABR: u32 = 0x2000; // Gekko Only
const EXCEPTION_THERMAL_MANAGEMENT: u32 = 0x4000; // Gekko Only

#[derive(Debug)]
pub enum Exception {
    SystemReset = 0x00100,
    MachineCheck = 0x00200,
    DataStorage = 0x00300,
    InstructionStorage = 0x00400,
    External = 0x00500,
    Alignment = 0x00600,
    Program = 0x00700,
    FloatingPointUnavailable = 0x00800,
    Decrementer = 0x00900,
    SystemCall = 0x00C00,
    Trace = 0x00D00,
    FloatingPointAssist = 0x00E00,
    PerformanceMonitor = 0x00F00,           // Gekko Only
    InstructionAddressBreakpoint = 0x01300, // Gekko Only
    ThermalManagement = 0x01700,            // Gekko Only
}

pub struct Cpu {
    /// Current Instruction Address
    pub cia: u32,
    /// Next Instruction Address
    nia: u32,
    /// General-Purpose Registers
    pub gpr: [u32; NUM_GPR],
    /// Floating-Point Registers
    fpr: [u64; NUM_FPR],
    /// Special-Purpose Registers
    spr: [u32; NUM_SPR],
    /// Condition Register
    cr: ConditionRegister,
    /// Floating-Point Status and Control Register
    fpscr: Fpscr,
    /// Integer Exception Register
    xer: Xer,
    /// Link Register
    pub lr: u32,
    /// Count Register
    ctr: u32,
    /// Machine State Register
    pub msr: MachineStateRegister,
    /// Segment Registers
    sr: [u32; NUM_SR],
    /// Machine Status Save/Restore Register 0
    srr0: u32,
    /// Machine Status Save/Restore Register 1
    srr1: u32,
    /// Decrementer Register
    dec: u32,
    /// Hardware Implementation-Dependent Register 0
    hid0: u32,
    /// Hardware Implementation-Dependent Register 1
    hid2: Hid2,
    /// Graphics Quantization Registers
    gqr: [u32; NUM_GQR],
    /// L2 Cache Control Register
    l2cr: u32,
    /// Monitor Mode Control Register 0
    mmcr0: u32,
    /// Monitor Mode Control Register 1
    mmcr1: u32,
    /// Performance Monitor Counter Register 1
    pmc1: u32,
    /// Performance Monitor Counter Register 2
    pmc2: u32,
    /// Performance Monitor Counter Register 3
    pmc3: u32,
    /// Performance Monitor Counter Register 4
    pmc4: u32,
    /// Excceptions
    exceptions: u32,
    /// Memory Management Unit
    mmu: Mmu,
    /// Primary Opcode Table
    optable: [fn(&mut Context, Instruction); OPTABLE_SIZE],
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
        let mut optable19 = [ILLEGAL_OP.1; OPTABLE19_SIZE];
        let mut optable31 = [ILLEGAL_OP.1; OPTABLE31_SIZE];
        let mut optable59 = [ILLEGAL_OP.1; OPTABLE59_SIZE];
        let mut optable63 = [ILLEGAL_OP.1; OPTABLE63_SIZE];

        for op in OPCODE_TABLE.iter() {
            optable[op.0 as usize] = op.2;
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

        for op in OPCODE63_TABLE.iter() {
            optable63[op.0 as usize] = op.2;
        }

        let mut cpu = Cpu {
            cia: 0,
            nia: 0,
            gpr: [0; NUM_GPR],
            fpr: [0; NUM_FPR],
            spr: [0; NUM_SPR],
            cr: Default::default(),
            fpscr: Default::default(),
            xer: Default::default(),
            lr: 0,
            ctr: 0,
            msr: 0x40.into(),
            sr: [0; NUM_SR],
            srr0: 0,
            srr1: 0,
            dec: 0,
            hid0: 0,
            hid2: Default::default(),
            gqr: [0; NUM_GQR],
            l2cr: 0,
            mmcr0: 0,
            mmcr1: 0,
            pmc1: 0,
            pmc2: 0,
            pmc3: 0,
            pmc4: 0,
            exceptions: EXCEPTION_SYSTEM_RESET,
            mmu: Default::default(),
            optable,
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

        // FixMe: populate SPR's accoprdingly
        self.mmu.write_ibatu(0, 0x8000_1fff); // Spr::IBAT0U
        self.mmu.write_ibatl(0, 0x0000_0002); // Spr::IBAT0L
        self.mmu.write_ibatu(3, 0xfff0_001f); // Spr::IBAT3U
        self.mmu.write_ibatl(3, 0xfff0_0001); // Spr::IBAT3L
        self.mmu.write_dbatu(0, 0x8000_1fff); // Spr::DBAT0U
        self.mmu.write_dbatl(0, 0x0000_0002); // Spr::DBAT0L
        self.mmu.write_dbatu(1, 0xc000_1fff); // Spr::DBAT1U
        self.mmu.write_dbatl(1, 0x0000_002a); // Spr::DBAT1L
        self.mmu.write_dbatu(3, 0xfff0_001f); // Spr::DBAT3U
        self.mmu.write_dbatl(3, 0xfff0_0001); // Spr::DBAT3L
    }

    fn check_exceptions(&mut self) {
        if self.exceptions & EXCEPTION_SYSTEM_RESET != 0 {
            if self.msr.exception_prefix() {
                self.cia = 0x100 | 0xFFF0_0000
            } else {
                self.cia = 0x100
            }

            self.exceptions &= !EXCEPTION_SYSTEM_RESET;

            info!("EXCEPTION_SYSTEM_RESET");
        }

        if self.exceptions & EXCEPTION_SYSTEM_CALL != 0 {
            self.spr[SPR_SRR0] = self.nia;
            self.spr[SPR_SRR1] = self.msr.0 & 0x87C0_FFFF;
            self.msr.0 = self.msr.0 & !0x04_EF36;

            self.msr
                .set_little_endian(self.msr.exception_little_endian());

            if self.msr.exception_prefix() {
                self.cia = 0xC00 | 0xFFF0_0000
            } else {
                self.cia = 0xC00
            }

            self.nia = self.cia;

            self.exceptions &= !EXCEPTION_SYSTEM_CALL;

            info!("EXCEPTION_SYSTEM_CALL (PC={:#x})", self.cia);
        }

        if self.msr.external_interrupt() && self.exceptions & EXCEPTION_EXTERNAL_INT != 0 {
            self.spr[SPR_SRR0] = self.nia;
            self.spr[SPR_SRR1] = self.msr.0 & 0x87C0_FFFF;
            self.msr.0 = self.msr.0 & !0x04_EF36;

            self.msr
                .set_little_endian(self.msr.exception_little_endian());

            if self.msr.exception_prefix() {
                self.cia = 0x500 | 0xFFF0_0000
            } else {
                self.cia = 0x500
            }

            self.nia = self.cia;

            self.exceptions &= !EXCEPTION_EXTERNAL_INT;

            info!("EXCEPTION_EXTERNAL_INT");
        }

        if self.exceptions & EXCEPTION_PERFORMANCE_MONITOR != 0 {
            unimplemented!("EXCEPTION_PERFORMANCE_MONITOR");
        }

        if self.exceptions & EXCEPTION_DECREMENTER != 0 {
            unimplemented!("EXCEPTION_PERFORMANCE_MONITOR");
        }

        if self.exceptions & EXCEPTION_THERMAL_MANAGEMENT != 0 {
            unimplemented!("EXCEPTION_THERMAL_MANAGEMENT");
        }
    }

    pub fn translate_instr_address(&self, ea: u32) -> u32 {
        if self.msr.instr_address_translation() {
            translate_address(&self.mmu.ibat, self.msr, ea)
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

    fn update_cr0(&mut self, r: u32) {
        let value = r as i32;

        let mut flags = if value > 0 {
            0x4 // GT
        } else if value < 0 {
            0x8 // LT
        } else {
            0x2 // EQ
        };

        flags |= self.xer.summary_overflow() as u32;

        self.cr.set_field(0, flags);
    }

    pub fn translate_data_address(&self, ea: u32) -> u32 {
        if self.msr.data_address_translation() {
            translate_address(&self.mmu.dbat, self.msr, ea)
        } else {
            // real addressing mode
            ea
        }
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MSR: {:?} gpr: {:?}, sr: {:?}, cr:{:?}",
            self.msr, self.gpr, self.sr, self.cr
        )
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

#[derive(Copy, Clone, Debug)]
pub enum Opcode {
    TWI,
    MULLI,
    SUBFIC,
    CMPLI,
    CMPI,
    ADDIC,
    ADDICRC,
    ADDI,
    ADDIS,
    BCX,
    SC,
    BX,
    RLWIMIX,
    RLWINMX,
    ORI,
    ORIS,
    XORIS,
    ANDIRC,
    LWZ,
    LWZU,
    LBZ,
    LBZU,
    STW,
    STWU,
    STB,
    STBU,
    LHZ,
    LHZU,
    LHA,
    STH,
    STHU,
    LMW,
    STMW,
    LFS,
    LFD,
    STFS,
    STFSU,
    STFD,
    PSQL,
    PSQST,
    TABLE19,
    TABLE31,
    TABLE59,
    TABLE63,
    ILLEGAL,
    // Table19
    BCLRX,
    RFI,
    ISYNC,
    CRXOR,
    BCCTRX,
    // Table31
    CMP,
    SUBFCX,
    ADDCX,
    MULHWUX,
    MFCR,
    LWZX,
    SLWX,
    CNTLZWX,
    ANDX,
    CMPL,
    SUBFX,
    ANDCX,
    MFMSR,
    DCBF,
    LBZX,
    NEGX,
    NORX,
    SUBFEX,
    MTCRF,
    ADDEX,
    MTMSR,
    STWX,
    SUBFZEX,
    ADDZEX,
    MTSR,
    STBX,
    MULLWX,
    ADDX,
    XORX,
    MFSPR,
    MFTB,
    ORX,
    DIVWUX,
    MTSPR,
    DCBI,
    DIVWX,
    SRWX,
    SYNC,
    SRAWX,
    SRAWIX,
    EXTSHX,
    EXTSBX,
    ICBI,
    // Table59
    FDIVSX,
    FSUBSX,
    FADDSX,
    FMULSX,
    // Table63
    FCMPU,
    FRSPX,
    FCTIWZX,
    FSUBX,
    FMULX,
    FCMPO,
    MTFSB1X,
    FNEGX,
    FMRX,
    FNABSX,
    MTFSFX,
}

pub const ILLEGAL_OP: (Opcode, fn(&mut Context, Instruction)) = (Opcode::ILLEGAL, op_illegal);

pub const OPCODE_TABLE: [(u8, Opcode, fn(&mut Context, Instruction)); 44] = [
    (3, Opcode::TWI, op_twi),
    (7, Opcode::MULLI, op_mulli),
    (8, Opcode::SUBFIC, op_subfic),
    (10, Opcode::CMPLI, op_cmpli),
    (11, Opcode::CMPI, op_cmpi),
    (12, Opcode::ADDIC, op_addic),
    (13, Opcode::ADDICRC, op_addic_rc),
    (14, Opcode::ADDI, op_addi),
    (15, Opcode::ADDIS, op_addis),
    (16, Opcode::BCX, op_bcx),
    (17, Opcode::SC, op_sc),
    (18, Opcode::BX, op_bx),
    (19, Opcode::TABLE19, op_subtable19),
    (20, Opcode::RLWIMIX, op_rlwimix),
    (21, Opcode::RLWINMX, op_rlwinmx),
    (24, Opcode::ORI, op_ori),
    (25, Opcode::ORIS, op_oris),
    (27, Opcode::XORIS, op_xoris),
    (28, Opcode::ANDIRC, op_andi_rc),
    (31, Opcode::TABLE31, op_subtable31),
    (32, Opcode::LWZ, op_lwz),
    (33, Opcode::LWZU, op_lwzu),
    (34, Opcode::LBZ, op_lbz),
    (35, Opcode::LBZU, op_lbzu),
    (36, Opcode::STW, op_stw),
    (37, Opcode::STWU, op_stwu),
    (38, Opcode::STB, op_stb),
    (39, Opcode::STBU, op_stbu),
    (40, Opcode::LHZ, op_lhz),
    (41, Opcode::LHZU, op_lhzu),
    (42, Opcode::LHA, op_lha),
    (44, Opcode::STH, op_sth),
    (45, Opcode::STHU, op_sthu),
    (46, Opcode::LMW, op_lmw),
    (47, Opcode::STMW, op_stmw),
    (48, Opcode::LFS, op_lfs),
    (50, Opcode::LFD, op_lfd),
    (52, Opcode::STFS, op_stfs),
    (53, Opcode::STFSU, op_stfsu),
    (54, Opcode::STFD, op_stfd),
    (56, Opcode::PSQL, op_psq_l),
    (59, Opcode::TABLE59, op_subtable59),
    (60, Opcode::PSQST, op_psq_st),
    (63, Opcode::TABLE63, op_subtable63),
];

pub const OPCODE19_TABLE: [(u16, Opcode, fn(&mut Context, Instruction)); 5] = [
    (16, Opcode::BCLRX, op_bclrx),
    (50, Opcode::RFI, op_rfi),
    (150, Opcode::ISYNC, op_isync),
    (193, Opcode::CRXOR, op_crxor),
    (528, Opcode::BCCTRX, op_bcctrx),
];

pub const OPCODE31_TABLE: [(u16, Opcode, fn(&mut Context, Instruction)); 43] = [
    (0, Opcode::CMP, op_cmp),
    (8, Opcode::SUBFCX, op_subfcx),
    (10, Opcode::ADDCX, op_addcx),
    (11, Opcode::MULHWUX, op_mulhwux),
    (19, Opcode::MFCR, op_mfcr),
    (23, Opcode::LWZX, op_lwzx),
    (24, Opcode::SLWX, op_slwx),
    (26, Opcode::CNTLZWX, op_cntlzwx),
    (28, Opcode::ANDX, op_andx),
    (32, Opcode::CMPL, op_cmpl),
    (40, Opcode::SUBFX, op_subfx),
    (60, Opcode::ANDCX, op_andcx),
    (83, Opcode::MFMSR, op_mfmsr),
    (86, Opcode::DCBF, op_dcbf),
    (87, Opcode::LBZX, op_lbzx),
    (104, Opcode::NEGX, op_negx),
    (124, Opcode::NORX, op_norx),
    (136, Opcode::SUBFEX, op_subfex),
    (138, Opcode::ADDEX, op_addex),
    (144, Opcode::MTCRF, op_mtcrf),
    (146, Opcode::MTMSR, op_mtmsr),
    (151, Opcode::STWX, op_stwx),
    (200, Opcode::SUBFZEX, op_subfzex),
    (202, Opcode::ADDZEX, op_addzex),
    (210, Opcode::MTSR, op_mtsr),
    (215, Opcode::STBX, op_stbx),
    (235, Opcode::MULLWX, op_mullwx),
    (266, Opcode::ADDX, op_addx),
    (316, Opcode::XORX, op_xorx),
    (339, Opcode::MFSPR, op_mfspr),
    (371, Opcode::MFTB, op_mftb),
    (444, Opcode::ORX, op_orx),
    (459, Opcode::DIVWUX, op_divwux),
    (467, Opcode::MTSPR, op_mtspr),
    (470, Opcode::DCBI, op_dcbi),
    (491, Opcode::DIVWX, op_divwx),
    (536, Opcode::SRWX, op_srwx),
    (598, Opcode::SYNC, op_sync),
    (792, Opcode::SRAWX, op_srawx),
    (824, Opcode::SRAWIX, op_srawix),
    (922, Opcode::EXTSHX, op_extshx),
    (954, Opcode::EXTSBX, op_extsbx),
    (982, Opcode::ICBI, op_icbi),
];

pub const OPCODE59_TABLE: [(u8, Opcode, fn(&mut Context, Instruction)); 4] = [
    (18, Opcode::FDIVSX, op_fdivsx),
    (20, Opcode::FSUBSX, op_fsubsx),
    (21, Opcode::FADDSX, op_faddsx),
    (25, Opcode::FMULSX, op_fmulsx),
];

pub const OPCODE63_TABLE: [(u16, Opcode, fn(&mut Context, Instruction)); 11] = [
    (0, Opcode::FCMPU, op_fcmpu),
    (12, Opcode::FRSPX, op_frspx),
    (15, Opcode::FCTIWZX, op_fctiwzx),
    (20, Opcode::FSUBX, op_fsubx),
    (25, Opcode::FMULX, op_fmulx),
    (32, Opcode::FCMPO, op_fcmpo),
    (38, Opcode::MTFSB1X, op_mtfsb1x),
    (40, Opcode::FNEGX, op_fnegx),
    (72, Opcode::FMRX, op_fmrx),
    (136, Opcode::FNABSX, op_fnabsx),
    (711, Opcode::MTFSFX, op_mtfsfx),
];

fn op_illegal(_ctx: &mut Context, _instr: Instruction) {}

fn op_subtable19(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.optable19[instr.xo_x()](ctx, instr);
}

fn op_subtable31(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.optable31[instr.xo_x()](ctx, instr);
}

fn op_subtable59(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.optable59[instr.xo_a()](ctx, instr);
}

fn op_subtable63(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.optable63[instr.xo_x()](ctx, instr);
}

include!("cpu_branch.rs");
include!("cpu_condition.rs");
include!("cpu_float.rs");
include!("cpu_integer.rs");
include!("cpu_load_store.rs");
include!("cpu_system.rs");

bitfield! {
    #[derive(Copy, Clone)]
    pub struct MachineStateRegister(u32);
    impl Debug;
    pub little_endian, set_little_endian : 0;
    pub reset_recoverable, _ : 1;
    pub performance_monitor_marked, _ : 2;
    pub data_address_translation, _ : 4;
    pub instr_address_translation, _ : 5;
    pub exception_prefix, _ : 6;
    pub fp_exception_mode_1, _ : 8;
    pub branch_trace, _ : 9;
    pub single_step_trace, _ : 10;
    pub fp_exception_mode_0, _ : 11;
    pub machine_check, _ : 12;
    pub floating_point, _ : 13;
    pub privilege_level, _ : 14;
    pub external_interrupt, _ : 15;
    pub exception_little_endian, _ : 16;
    pub power_management, set_power_management : 18;
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
        self.0 = value as u32 | (self.0 & !(0x8000_0000 >> bit));
    }
}

impl From<u32> for ConditionRegister {
    fn from(v: u32) -> Self {
        ConditionRegister(v)
    }
}
