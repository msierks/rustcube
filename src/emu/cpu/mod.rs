mod instruction;
mod mmu;

use super::Context;
use instruction::*;
use mmu::{translate_address, Mmu};

const NUM_GPR: usize = 32;
const NUM_FPR: usize = 32;
const NUM_SPR: usize = 1022;
const NUM_GQR: usize = 8;
const NUM_SR: usize = 16;

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

const PROCESSOR_VERSION: u32 = 0x00083214;

pub struct Cpu {
    /// Current Instruction Address
    pc: u32,
    /// Next Instruction Address
    npc: u32,
    /// General-Purpose Registers
    gpr: [u32; NUM_GPR],
    /// Special-Purpose Registers
    spr: [u32; NUM_SPR],
    /// Segment Registers
    sr: [u32; NUM_SR],
    /// Link Register
    lr: u32,
    /// Count Register
    ctr: u32,
    /// Machine State Register
    msr: MachineStateRegister,
    // Exceptions
    exceptions: u32,
    // Memory Management Unit
    mmu: Mmu,
    /// Primary Opcode Table
    optable: [fn(&mut Context, Instruction); 64],
    /// SubOpcode 19 Table
    optable19: [fn(&mut Context, Instruction); 1024],
    /// SubOpcode 31 Table
    optable31: [fn(&mut Context, Instruction); 1024],
    /// SubOpcode 59 Table
    optable59: [fn(&mut Context, Instruction); 32],
    /// SubOpcode 63 Table
    optable63: [fn(&mut Context, Instruction); 1024],
}

impl Default for Cpu {
    fn default() -> Self {
        let mut optable = [ILLEGAL_OP.2; 64];
        let mut optable19 = [ILLEGAL_OP.2; 1024];
        let mut optable31 = [ILLEGAL_OP.2; 1024];
        let mut optable59 = [ILLEGAL_OP.2; 32];
        let mut optable63 = [ILLEGAL_OP.2; 1024];

        for op in OPCODE_TABLE.iter() {
            optable[op.0 as usize] = op.3;
        }

        for op in SUBOPCODE19_TABLE.iter() {
            optable19[op.0 as usize] = op.3;
        }

        for op in SUBOPCODE31_TABLE.iter() {
            optable31[op.0 as usize] = op.3;
        }

        for op in SUBOPCODE59_TABLE.iter() {
            optable59[op.0 as usize] = op.3;
        }

        for op in SUBOPCODE63_TABLE.iter() {
            optable63[op.0 as usize] = op.3;
        }

        let mut spr = [0; NUM_SPR];

        spr[SPR_PVR] = PROCESSOR_VERSION;

        let mut cpu = Cpu {
            pc: 0,
            npc: 0,
            gpr: [0; NUM_GPR],
            spr,
            sr: [0; NUM_SR],
            lr: 0,
            ctr: 0,
            msr: 0x40.into(),
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
    fn check_exceptions(&mut self) {
        if self.exceptions & EXCEPTION_SYSTEM_RESET != 0 {
            if self.msr.exception_prefix() {
                self.pc = 0x100 | 0xFFF0_0000
            } else {
                self.pc = 0x100
            }

            self.exceptions &= !EXCEPTION_SYSTEM_RESET;

            println!("EXCEPTION_SYSTEM_RESET");
        }

        if self.exceptions & EXCEPTION_SYSTEM_CALL != 0 {
            self.spr[SPR_SRR0] = self.npc;
            self.spr[SPR_SRR1] = self.msr.0 & 0x87C0_FFFF;
            self.msr.0 = self.msr.0 & !0x04_EF36;

            self.msr
                .set_little_endian(self.msr.exception_little_endian());

            if self.msr.exception_prefix() {
                self.pc = 0xC00 | 0xFFF0_0000
            } else {
                self.pc = 0xC00
            }

            self.npc = self.pc;

            self.exceptions &= !EXCEPTION_SYSTEM_CALL;

            println!("EXCEPTION_SYSTEM_CALL");
        }

        if self.msr.external_interrupt() && self.exceptions & EXCEPTION_EXTERNAL_INT != 0 {
            self.spr[SPR_SRR0] = self.npc;
            self.spr[SPR_SRR1] = self.msr.0 & 0x87C0_FFFF;
            self.msr.0 = self.msr.0 & !0x04_EF36;

            self.msr
                .set_little_endian(self.msr.exception_little_endian());

            if self.msr.exception_prefix() {
                self.pc = 0x500 | 0xFFF0_0000
            } else {
                self.pc = 0x500
            }

            self.npc = self.pc;

            self.exceptions &= !EXCEPTION_EXTERNAL_INT;

            println!("EXCEPTION_EXTERNAL_INT");
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

    pub fn set_pc(&mut self, pc: u32) {
        self.pc = pc;
    }

    pub fn get_pc(&mut self) -> u32 {
        self.pc
    }

    pub fn translate_instr_address(&self, ea: u32) -> u32 {
        if self.msr.instr_address_translation() {
            translate_address(&self.mmu.ibat, self.msr, ea)
        } else {
            // real addressing mode
            ea
        }
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

pub fn step(ctx: &mut Context) {
    let addr = ctx.cpu.translate_instr_address(ctx.cpu.pc);

    let instr = Instruction(ctx.read_instruction(addr));

    ctx.cpu.npc = ctx.cpu.pc.wrapping_add(4);

    if instr.0 != 0 {
        ctx.cpu.optable[instr.opcode() as usize](ctx, instr);
    } else {
        //self.check_exceptions();
        unimplemented!();
    }

    ctx.cpu.pc = ctx.cpu.npc;
}

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

#[derive(Copy, Clone, Debug)]
pub enum Opcodes {
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
    UNKNOWN,
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

pub const ILLEGAL_OP: (Opcodes, &str, fn(&mut Context, Instruction)) =
    (Opcodes::UNKNOWN, "illegal_instruction", op_illegal);

pub const OPCODE_TABLE: [(u8, Opcodes, &'static str, fn(&mut Context, Instruction)); 44] = [
    (3, Opcodes::TWI, "twi", op_twi),
    (7, Opcodes::MULLI, "mulli", op_mulli),
    (8, Opcodes::SUBFIC, "subfic", op_subfic),
    (10, Opcodes::CMPLI, "cmpli", op_cmpli),
    (11, Opcodes::CMPI, "cmpi", op_cmpi),
    (12, Opcodes::ADDIC, "addic", op_addic),
    (13, Opcodes::ADDICRC, "addic_rc", op_addic_rc),
    (14, Opcodes::ADDI, "addi", op_addi),
    (15, Opcodes::ADDIS, "addis", op_addis),
    (16, Opcodes::BCX, "bcx", op_bcx),
    (17, Opcodes::SC, "sc", op_sc),
    (18, Opcodes::BX, "bx", op_bx),
    (19, Opcodes::TABLE19, "subtable19", op_subtable19),
    (20, Opcodes::RLWIMIX, "rlwimix", op_rlwimix),
    (21, Opcodes::RLWINMX, "rlwinmx", op_rlwinmx),
    (24, Opcodes::ORI, "ori", op_ori),
    (25, Opcodes::ORIS, "oris", op_oris),
    (27, Opcodes::XORIS, "xoris", op_xoris),
    (28, Opcodes::ANDIRC, "andi_rc", op_andi_rc),
    (31, Opcodes::TABLE31, "subtable31", op_subtable31),
    (32, Opcodes::LWZ, "lwz", op_lwz),
    (33, Opcodes::LWZU, "lwzu", op_lwzu),
    (34, Opcodes::LBZ, "lbz", op_lbz),
    (35, Opcodes::LBZU, "lbzu", op_lbzu),
    (36, Opcodes::STW, "stw", op_stw),
    (37, Opcodes::STWU, "stwu", op_stwu),
    (38, Opcodes::STB, "stb", op_stb),
    (39, Opcodes::STBU, "stbu", op_stbu),
    (40, Opcodes::LHZ, "lhz", op_lhz),
    (41, Opcodes::LHZU, "lhzu", op_lhzu),
    (42, Opcodes::LHA, "lha", op_lha),
    (44, Opcodes::STH, "sth", op_sth),
    (45, Opcodes::STHU, "sthu", op_sthu),
    (46, Opcodes::LMW, "lmw", op_lmw),
    (47, Opcodes::STMW, "stmw", op_stmw),
    (48, Opcodes::LFS, "lfs", op_lfs),
    (50, Opcodes::LFD, "lfd", op_lfd),
    (52, Opcodes::STFS, "stfs", op_stfs),
    (53, Opcodes::STFSU, "stfsu", op_stfsu),
    (54, Opcodes::STFD, "stfd", op_stfd),
    (56, Opcodes::PSQL, "psq_l", op_psq_l),
    (59, Opcodes::TABLE59, "subtable59", op_subtable59),
    (60, Opcodes::PSQST, "psq_st", op_psq_st),
    (63, Opcodes::TABLE63, "subtable63", op_subtable63),
];

pub const SUBOPCODE19_TABLE: [(u16, Opcodes, &str, fn(&mut Context, Instruction)); 5] = [
    (16, Opcodes::BCLRX, "bclrx", op_bclrx),
    (50, Opcodes::RFI, "rfi", op_rfi),
    (150, Opcodes::ISYNC, "isync", op_isync),
    (193, Opcodes::CRXOR, "crxor", op_crxor),
    (528, Opcodes::BCCTRX, "bcctrx", op_bcctrx),
];

pub const SUBOPCODE31_TABLE: [(u16, Opcodes, &str, fn(&mut Context, Instruction)); 43] = [
    (0, Opcodes::CMP, "cmp", op_cmp),
    (8, Opcodes::SUBFCX, "subfcx", op_subfcx),
    (10, Opcodes::ADDCX, "addcx", op_addcx),
    (11, Opcodes::MULHWUX, "mulhwux", op_mulhwux),
    (19, Opcodes::MFCR, "mfcr", op_mfcr),
    (23, Opcodes::LWZX, "lwzx", op_lwzx),
    (24, Opcodes::SLWX, "slwx", op_slwx),
    (26, Opcodes::CNTLZWX, "cntlzwx", op_cntlzwx),
    (28, Opcodes::ANDX, "andx", op_andx),
    (32, Opcodes::CMPL, "cmpl", op_cmpl),
    (40, Opcodes::SUBFX, "subfx", op_subfx),
    (60, Opcodes::ANDCX, "andcx", op_andcx),
    (83, Opcodes::MFMSR, "mfmsr", op_mfmsr),
    (86, Opcodes::DCBF, "dcbf", op_dcbf),
    (87, Opcodes::LBZX, "lbzx", op_lbzx),
    (104, Opcodes::NEGX, "negx", op_negx),
    (124, Opcodes::NORX, "norx", op_norx),
    (136, Opcodes::SUBFEX, "subfex", op_subfex),
    (138, Opcodes::ADDEX, "addex", op_addex),
    (144, Opcodes::MTCRF, "mtcrf", op_mtcrf),
    (146, Opcodes::MTMSR, "mtmsr", op_mtmsr),
    (151, Opcodes::STWX, "stwx", op_stwx),
    (200, Opcodes::SUBFZEX, "subfzex", op_subfzex),
    (202, Opcodes::ADDZEX, "addzex", op_addzex),
    (210, Opcodes::MTSR, "mtsr", op_mtsr),
    (215, Opcodes::STBX, "stbx", op_stbx),
    (235, Opcodes::MULLWX, "mullwx", op_mullwx),
    (266, Opcodes::ADDX, "addx", op_addx),
    (316, Opcodes::XORX, "xorx", op_xorx),
    (339, Opcodes::MFSPR, "mfspr", op_mfspr),
    (371, Opcodes::MFTB, "mftb", op_mftb),
    (444, Opcodes::ORX, "orx", op_orx),
    (459, Opcodes::DIVWUX, "divwux", op_divwux),
    (467, Opcodes::MTSPR, "mtspr", op_mtspr),
    (470, Opcodes::DCBI, "dcbi", op_dcbi),
    (491, Opcodes::DIVWX, "divwx", op_divwx),
    (536, Opcodes::SRWX, "srwx", op_srwx),
    (598, Opcodes::SYNC, "sync", op_sync),
    (792, Opcodes::SRAWX, "srawx", op_srawx),
    (824, Opcodes::SRAWIX, "srawix", op_srawix),
    (922, Opcodes::EXTSHX, "extshx", op_extshx),
    (954, Opcodes::EXTSBX, "extsbx", op_extsbx),
    (982, Opcodes::ICBI, "icbi", op_icbi),
];

pub const SUBOPCODE59_TABLE: [(u8, Opcodes, &str, fn(&mut Context, Instruction)); 4] = [
    (18, Opcodes::FDIVSX, "fdivsx", op_fdivsx),
    (20, Opcodes::FSUBSX, "fsubsx", op_fsubsx),
    (21, Opcodes::FADDSX, "faddsx", op_faddsx),
    (25, Opcodes::FMULSX, "fmulsx", op_fmulsx),
];

pub const SUBOPCODE63_TABLE: [(u16, Opcodes, &str, fn(&mut Context, Instruction)); 11] = [
    (0, Opcodes::FCMPU, "fcmpu", op_fcmpu),
    (12, Opcodes::FRSPX, "frspx", op_frspx),
    (15, Opcodes::FCTIWZX, "fctiwzx", op_fctiwzx),
    (20, Opcodes::FSUBX, "fsubx", op_fsubx),
    (25, Opcodes::FMULX, "fmulx", op_fmulx),
    (32, Opcodes::FCMPO, "fcmpo", op_fcmpo),
    (38, Opcodes::MTFSB1X, "mtfsb1x", op_mtfsb1x),
    (40, Opcodes::FNEGX, "fnegx", op_fnegx),
    (72, Opcodes::FMRX, "fmrx", op_fmrx),
    (136, Opcodes::FNABSX, "fnabsx", op_fnabsx),
    (711, Opcodes::MTFSFX, "mtfsfx", op_mtfsfx),
];

fn op_illegal(_ctx: &mut Context, _instr: Instruction) {}

fn op_subtable19(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.optable19[instr.ext_opcode_x() as usize](ctx, instr);
}

fn op_subtable31(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.optable31[instr.ext_opcode_x() as usize](ctx, instr);
}

fn op_subtable59(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.optable59[instr.ext_opcode_a() as usize](ctx, instr);
}

fn op_subtable63(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.optable63[instr.ext_opcode_x() as usize](ctx, instr);
}

include!("cpu_branch.rs");
include!("cpu_condition.rs");
include!("cpu_float.rs");
#[cfg(feature = "gdb")]
include!("cpu_gdb.rs");
include!("cpu_integer.rs");
include!("cpu_load_store.rs");
include!("cpu_system.rs");
