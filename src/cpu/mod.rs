#[allow(dead_code)]
pub mod disassembler;
pub mod instruction;
pub mod mmu;
pub mod util;

use self::instruction::Instruction;
use self::util::*;
use super::Context;
use mmu::{translate_address, Mmu};
use std::cmp::Ordering;

pub const NUM_FPR: usize = 32;
pub const NUM_GPR: usize = 32;
pub const NUM_SPR: usize = 1023;
const NUM_SR: usize = 16;

const OPTABLE_SIZE: usize = 64;
const OPTABLE4_SIZE: usize = 1024;
const OPTABLE19_SIZE: usize = 1024;
const OPTABLE31_SIZE: usize = 1024;
const OPTABLE59_SIZE: usize = 32;
const OPTABLE63_SIZE: usize = 1024;

pub const SPR_XER: usize = 1;
pub const SPR_LR: usize = 8;
pub const SPR_CTR: usize = 9;
pub const SPR_DSISR: usize = 18;
pub const SPR_DAR: usize = 19;
pub const SPR_DEC: usize = 22;
pub const SPR_SDR1: usize = 25;
pub const SPR_SRR0: usize = 26;
pub const SPR_SRR1: usize = 27;
pub const SPR_SPRG0: usize = 272;
pub const SPR_EAR: usize = 282;
pub const SPR_TBL: usize = 284;
pub const SPR_TBU: usize = 285;
pub const SPR_PVR: usize = 287;
pub const SPR_IBAT0U: usize = 528;
pub const SPR_IBAT0L: usize = 529;
pub const SPR_IBAT1U: usize = 530;
pub const SPR_IBAT1L: usize = 531;
pub const SPR_IBAT2U: usize = 532;
pub const SPR_IBAT2L: usize = 533;
pub const SPR_IBAT3U: usize = 534;
pub const SPR_IBAT3L: usize = 535;
pub const SPR_DBAT0U: usize = 536;
pub const SPR_DBAT0L: usize = 537;
pub const SPR_DBAT1U: usize = 538;
pub const SPR_DBAT1L: usize = 539;
pub const SPR_DBAT2U: usize = 540;
pub const SPR_DBAT2L: usize = 541;
pub const SPR_DBAT3U: usize = 542;
pub const SPR_DBAT3L: usize = 543;
pub const SPR_GQR0: usize = 912;
pub const SPR_HID2: usize = 920;
pub const SPR_WPAR: usize = 921;
pub const SPR_DMAU: usize = 922;
pub const SPR_UMMCR0: usize = 936;
pub const SPR_UPMC1: usize = 937;
pub const SPR_UPMC2: usize = 938;
pub const SPR_USIA: usize = 939;
pub const SPR_UMMCR1: usize = 940;
pub const SPR_UPMC3: usize = 941;
pub const SPR_UPMC4: usize = 942;
pub const SPR_MMCR0: usize = 952;
pub const SPR_PMC1: usize = 953;
pub const SPR_PMC2: usize = 954;
pub const SPR_SIA: usize = 955;
pub const SPR_MMCR1: usize = 956;
pub const SPR_PMC3: usize = 957;
pub const SPR_PMC4: usize = 958;
pub const SPR_IABR: usize = 1010;
pub const SPR_HID0: usize = 1008;
pub const SPR_HID1: usize = 1009;
pub const SPR_DABR: usize = 1013;
pub const SPR_L2CR: usize = 1017;
pub const SPR_ICTC: usize = 1019;
pub const SPR_THRM1: usize = 1020;

pub const TBR_TBL: usize = 268;
pub const TBR_TBU: usize = 269;

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
            optable[op.0] = op.2;
        }

        for op in OPCODE4X_TABLE.iter() {
            optable4[op.0] = op.2;
        }

        for n in 0..32 {
            let fill = n << 5;
            for op in OPCODE4A_TABLE.iter() {
                let xo_x = (op.0) | fill;
                optable4[xo_x] = op.2;
            }
        }

        for n in 0..16 {
            let fill = n << 6;
            for op in OPCODE4AA_TABLE.iter() {
                let xo_x = (op.0) | fill;
                optable4[xo_x] = op.2;
            }
        }

        for op in OPCODE19_TABLE.iter() {
            optable19[op.0] = op.2;
        }

        for op in OPCODE31_TABLE.iter() {
            optable31[op.0] = op.2;
        }

        for op in OPCODE59_TABLE.iter() {
            optable59[op.0] = op.2;
        }

        for op in OPCODE63X_TABLE.iter() {
            optable63[op.0] = op.2;
        }

        for n in 0..32 {
            let fill = n << 5;
            for op in OPCODE63A_TABLE.iter() {
                let xo_x = (op.0) | fill;
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

#[derive(Copy, Clone, Debug)]
pub enum Opcode {
    Twi,
    Mulli,
    Subfic,
    Cmpli,
    Cmpi,
    Addic,
    Addicrc,
    Addi,
    Addis,
    Bcx,
    Sc,
    Bx,
    Rlwimix,
    Rlwinmx,
    Rlwnmx,
    Ori,
    Oris,
    Xori,
    Xoris,
    Andirc,
    Andisrc,
    Lwz,
    Lwzu,
    Lbz,
    Lbzu,
    Stw,
    Stwu,
    Stb,
    Stbu,
    Lhz,
    Lhzu,
    Lha,
    Lhau,
    Sth,
    Sthu,
    Lmw,
    Stmw,
    Lfs,
    Lfsu,
    Lfd,
    Lfdu,
    Stfs,
    Stfsu,
    Stfd,
    Stfdu,
    PsqL,
    PsqLu,
    PsqSt,
    PsqStu,
    Table4,
    Table19,
    Table31,
    Table59,
    Table63,
    Illegal,
    // Table4,
    PsCmpu0,
    PsqLx,
    PsqStx,
    PsSum0x,
    PsSum1x,
    PsMuls0x,
    PsMuls1x,
    PsMadds0x,
    PsMadds1x,
    PsDivx,
    PsSubx,
    PsAddx,
    PsSelx,
    PsResx,
    PsMulx,
    PsRsqrtex,
    PsMsubx,
    PsMaddx,
    PsNmsubx,
    PsNmaddx,
    PsCmpo0,
    PsqLux,
    PsqStux,
    PsNegx,
    PsCmpu1,
    PsMrx,
    PsCmpo1,
    PsNabsx,
    PsAbsx,
    PsMerge00x,
    PsMerge01x,
    PsMerge10x,
    PsMerge11x,
    DcbzL,
    // Table19
    Mcrf,
    Bclrx,
    Crnor,
    Rfi,
    Crandc,
    Isync,
    Crxor,
    Crnand,
    Crand,
    Creqv,
    Crorc,
    Cror,
    Bcctrx,
    // Table31
    Cmp,
    Tw,
    Subfcx,
    Addcx,
    Mulhwux,
    Mfcr,
    Lwarx,
    Lwzx,
    Slwx,
    Cntlzwx,
    Andx,
    Cmpl,
    Subfx,
    Dcbst,
    Lwzux,
    Andcx,
    Mulhwx,
    Mfmsr,
    Dcbf,
    Lbzx,
    Negx,
    Lbzux,
    Norx,
    Subfex,
    Addex,
    Mtcrf,
    Mtmsr,
    Stwcxrc,
    Stwx,
    Stwux,
    Subfzex,
    Addzex,
    Mtsr,
    Stbx,
    Subfmex,
    Addmex,
    Mullwx,
    Mtsrin,
    Dcbtst,
    Stbux,
    Addx,
    Dcbt,
    Lhzx,
    Eqvx,
    Tlbie,
    Eciwx,
    Lhzux,
    Xorx,
    Mfspr,
    Lhax,
    Mftb,
    Lhaux,
    Sthx,
    Orcx,
    Ecowx,
    Sthux,
    Orx,
    Divwux,
    Mtspr,
    Dcbi,
    Nandx,
    Divwx,
    Mcrxr,
    Lswx,
    Lwbrx,
    Lfsx,
    Srwx,
    Tlbsync,
    Lfsux,
    Mfsr,
    Lswi,
    Sync,
    Lfdx,
    Lfdux,
    Mfsrin,
    Stswx,
    Stwbrx,
    Stfsx,
    Stfsux,
    Stswi,
    Stfdx,
    Stfdux,
    Lhbrx,
    Srawx,
    Srawix,
    Eieio,
    Sthbrx,
    Extshx,
    Extsbx,
    Icbi,
    Stfiwx,
    Dcbz,
    // Table59
    Fdivsx,
    Fsubsx,
    Faddsx,
    Fresx,
    Fmulsx,
    Fmsubsx,
    Fmaddsx,
    Fnmsubsx,
    Fnmaddsx,
    // Table63
    Fcmpu,
    Frspx,
    Fctiwx,
    Fctiwzx,
    Fdivx,
    Fsubx,
    Faddx,
    Fselx,
    Fmulx,
    Frsqrtex,
    Fmsubx,
    Fmaddx,
    Fnmsubx,
    Fnmaddx,
    Fcmpo,
    Mtfsb1x,
    Fnegx,
    Mcrfs,
    Mtfsb0x,
    Fmrx,
    Mtfsfix,
    Fnabsx,
    Fabsx,
    Mffsx,
    Mtfsfx,
}

type OpcodeTableItem = (usize, Opcode, fn(&mut Context, Instruction));

pub const ILLEGAL_OP: (Opcode, fn(&mut Context, Instruction)) = (Opcode::Illegal, op_illegal);

pub const OPCODE_TABLE: [OpcodeTableItem; 54] = [
    (3, Opcode::Twi, op_twi),
    (4, Opcode::Table4, op_subtable4),
    (7, Opcode::Mulli, op_mulli),
    (8, Opcode::Subfic, op_subfic),
    (10, Opcode::Cmpli, op_cmpli),
    (11, Opcode::Cmpi, op_cmpi),
    (12, Opcode::Addic, op_addic),
    (13, Opcode::Addicrc, op_addic_rc),
    (14, Opcode::Addi, op_addi),
    (15, Opcode::Addis, op_addis),
    (16, Opcode::Bcx, op_bcx),
    (17, Opcode::Sc, op_sc),
    (18, Opcode::Bx, op_bx),
    (19, Opcode::Table19, op_subtable19),
    (20, Opcode::Rlwimix, op_rlwimix),
    (21, Opcode::Rlwinmx, op_rlwinmx),
    (23, Opcode::Rlwnmx, op_rlwnmx),
    (24, Opcode::Ori, op_ori),
    (25, Opcode::Oris, op_oris),
    (26, Opcode::Xori, op_xori),
    (27, Opcode::Xoris, op_xoris),
    (28, Opcode::Andirc, op_andi_rc),
    (29, Opcode::Andisrc, op_andis_rc),
    (31, Opcode::Table31, op_subtable31),
    (32, Opcode::Lwz, op_lwz),
    (33, Opcode::Lwzu, op_lwzu),
    (34, Opcode::Lbz, op_lbz),
    (35, Opcode::Lbzu, op_lbzu),
    (36, Opcode::Stw, op_stw),
    (37, Opcode::Stwu, op_stwu),
    (38, Opcode::Stb, op_stb),
    (39, Opcode::Stbu, op_stbu),
    (40, Opcode::Lhz, op_lhz),
    (41, Opcode::Lhzu, op_lhzu),
    (42, Opcode::Lha, op_lha),
    (43, Opcode::Lhau, op_lhau),
    (44, Opcode::Sth, op_sth),
    (45, Opcode::Sthu, op_sthu),
    (46, Opcode::Lmw, op_lmw),
    (47, Opcode::Stmw, op_stmw),
    (48, Opcode::Lfs, op_lfs),
    (49, Opcode::Lfsu, op_lfsu),
    (50, Opcode::Lfd, op_lfd),
    (51, Opcode::Lfdu, op_lfdu),
    (52, Opcode::Stfs, op_stfs),
    (53, Opcode::Stfsu, op_stfsu),
    (54, Opcode::Stfd, op_stfd),
    (55, Opcode::Stfdu, op_stfdu),
    (56, Opcode::PsqL, op_psq_l),
    (57, Opcode::PsqLu, op_psq_lu),
    (59, Opcode::Table59, op_subtable59),
    (60, Opcode::PsqSt, op_psq_st),
    (61, Opcode::PsqStu, op_psq_stu),
    (63, Opcode::Table63, op_subtable63),
];

pub const OPCODE4X_TABLE: [OpcodeTableItem; 13] = [
    (0, Opcode::PsCmpu0, op_ps_cmpu0),
    (32, Opcode::PsCmpo0, op_ps_cmpo0),
    (40, Opcode::PsNegx, op_ps_negx),
    (64, Opcode::PsCmpu1, op_ps_cmpu1),
    (72, Opcode::PsMrx, op_ps_mrx),
    (96, Opcode::PsCmpo1, op_ps_cmpo1),
    (136, Opcode::PsNabsx, op_ps_nabsx),
    (264, Opcode::PsAbsx, op_ps_absx),
    (528, Opcode::PsMerge00x, op_ps_merge00x),
    (560, Opcode::PsMerge01x, op_ps_merge01x),
    (592, Opcode::PsMerge10x, op_ps_merge10x),
    (624, Opcode::PsMerge11x, op_ps_merge11x),
    (1014, Opcode::DcbzL, op_dcbz_l),
];

pub const OPCODE4A_TABLE: [OpcodeTableItem; 17] = [
    (10, Opcode::PsSum0x, op_ps_sum0x),
    (11, Opcode::PsSum1x, op_ps_sum1x),
    (12, Opcode::PsMuls0x, op_ps_muls0x),
    (13, Opcode::PsMuls1x, op_ps_muls1x),
    (14, Opcode::PsMadds0x, op_ps_madds0x),
    (15, Opcode::PsMadds1x, op_ps_madds1x),
    (18, Opcode::PsDivx, op_ps_divx),
    (20, Opcode::PsSubx, op_ps_subx),
    (21, Opcode::PsAddx, op_ps_addx),
    (23, Opcode::PsSelx, op_ps_selx),
    (24, Opcode::PsResx, op_ps_resx),
    (25, Opcode::PsMulx, op_ps_mulx),
    (26, Opcode::PsRsqrtex, op_ps_rsqrtex),
    (28, Opcode::PsMsubx, op_ps_msubx),
    (29, Opcode::PsMaddx, op_ps_maddx),
    (30, Opcode::PsNmsubx, op_ps_nmsubx),
    (31, Opcode::PsNmaddx, op_ps_nmaddx),
];

pub const OPCODE4AA_TABLE: [OpcodeTableItem; 4] = [
    (6, Opcode::PsqLx, op_psq_lx),
    (7, Opcode::PsqStx, op_psq_stx),
    (38, Opcode::PsqLux, op_psq_lux),
    (39, Opcode::PsqStux, op_psq_stux),
];

pub const OPCODE19_TABLE: [OpcodeTableItem; 13] = [
    (0, Opcode::Mcrf, op_mcrf),
    (16, Opcode::Bclrx, op_bclrx),
    (33, Opcode::Crnor, op_crnor),
    (50, Opcode::Rfi, op_rfi),
    (129, Opcode::Crandc, op_crandc),
    (150, Opcode::Isync, op_isync),
    (193, Opcode::Crxor, op_crxor),
    (225, Opcode::Crnand, op_crnand),
    (257, Opcode::Crand, op_crand),
    (289, Opcode::Creqv, op_creqv),
    (417, Opcode::Crorc, op_crorc),
    (449, Opcode::Cror, op_cror),
    (528, Opcode::Bcctrx, op_bcctrx),
];

pub const OPCODE31_TABLE: [OpcodeTableItem; 108] = [
    (0, Opcode::Cmp, op_cmp),
    (4, Opcode::Tw, op_tw),
    (8, Opcode::Subfcx, op_subfcx),
    (10, Opcode::Addcx, op_addcx),
    (11, Opcode::Mulhwux, op_mulhwux),
    (19, Opcode::Mfcr, op_mfcr),
    (20, Opcode::Lwarx, op_lwarx),
    (23, Opcode::Lwzx, op_lwzx),
    (24, Opcode::Slwx, op_slwx),
    (26, Opcode::Cntlzwx, op_cntlzwx),
    (28, Opcode::Andx, op_andx),
    (32, Opcode::Cmpl, op_cmpl),
    (40, Opcode::Subfx, op_subfx),
    (54, Opcode::Dcbst, op_dcbst),
    (55, Opcode::Lwzux, op_lwzux),
    (60, Opcode::Andcx, op_andcx),
    (75, Opcode::Mulhwx, op_mulhwx),
    (83, Opcode::Mfmsr, op_mfmsr),
    (86, Opcode::Dcbf, op_dcbf),
    (87, Opcode::Lbzx, op_lbzx),
    (104, Opcode::Negx, op_negx),
    (119, Opcode::Lbzux, op_lbzux),
    (124, Opcode::Norx, op_norx),
    (136, Opcode::Subfex, op_subfex),
    (138, Opcode::Addex, op_addex),
    (144, Opcode::Mtcrf, op_mtcrf),
    (146, Opcode::Mtmsr, op_mtmsr),
    (150, Opcode::Stwcxrc, op_stwcx_rc),
    (151, Opcode::Stwx, op_stwx),
    (183, Opcode::Stwux, op_stwux),
    (200, Opcode::Subfzex, op_subfzex),
    (202, Opcode::Addzex, op_addzex),
    (210, Opcode::Mtsr, op_mtsr),
    (215, Opcode::Stbx, op_stbx),
    (232, Opcode::Subfmex, op_subfmex),
    (234, Opcode::Addmex, op_addmex),
    (235, Opcode::Mullwx, op_mullwx),
    (242, Opcode::Mtsrin, op_mtsrin),
    (246, Opcode::Dcbtst, op_dcbtst),
    (247, Opcode::Stbux, op_stbux),
    (266, Opcode::Addx, op_addx),
    (278, Opcode::Dcbt, op_dcbt),
    (279, Opcode::Lhzx, op_lhzx),
    (284, Opcode::Eqvx, op_eqvx),
    (306, Opcode::Tlbie, op_tlbie),
    (310, Opcode::Eciwx, op_eciwx),
    (311, Opcode::Lhzux, op_lhzux),
    (316, Opcode::Xorx, op_xorx),
    (339, Opcode::Mfspr, op_mfspr),
    (343, Opcode::Lhax, op_lhax),
    (371, Opcode::Mftb, op_mftb),
    (375, Opcode::Lhaux, op_lhaux),
    (407, Opcode::Sthx, op_sthx),
    (412, Opcode::Orcx, op_orcx),
    (438, Opcode::Ecowx, op_ecowx),
    (439, Opcode::Sthux, op_sthux),
    (444, Opcode::Orx, op_orx),
    (459, Opcode::Divwux, op_divwux),
    (467, Opcode::Mtspr, op_mtspr),
    (470, Opcode::Dcbi, op_dcbi),
    (476, Opcode::Nandx, op_nandx),
    (491, Opcode::Divwx, op_divwx),
    (512, Opcode::Mcrxr, op_mcrxr),
    (520, Opcode::Subfcx, op_subfcx),   // oe = 1
    (522, Opcode::Addcx, op_addcx),     // oe = 1
    (523, Opcode::Mulhwux, op_mulhwux), // 21(reserved) = 1
    (533, Opcode::Lswx, op_lswx),
    (534, Opcode::Lwbrx, op_lwbrx),
    (535, Opcode::Lfsx, op_lfsx),
    (536, Opcode::Srwx, op_srwx),
    (552, Opcode::Subfx, op_subfx), // oe = 1
    (566, Opcode::Tlbsync, op_tlbsync),
    (567, Opcode::Lfsux, op_lfsux),
    (587, Opcode::Mulhwx, op_mulhwx), // 21(reserved) = 1
    (595, Opcode::Mfsr, op_mfsr),
    (597, Opcode::Lswi, op_lswi),
    (598, Opcode::Sync, op_sync),
    (599, Opcode::Lfdx, op_lfdx),
    (616, Opcode::Negx, op_negx), // oe = 1
    (631, Opcode::Lfdux, op_lfdux),
    (648, Opcode::Subfex, op_subfex), // oe = 1
    (650, Opcode::Addex, op_addex),   // oe = 1
    (659, Opcode::Mfsrin, op_mfsrin),
    (661, Opcode::Stswx, op_stswx),
    (662, Opcode::Stwbrx, op_stwbrx),
    (663, Opcode::Stfsx, op_stfsx),
    (695, Opcode::Stfsux, op_stfsux),
    (712, Opcode::Subfzex, op_subfzex), // oe = 1
    (714, Opcode::Addzex, op_addzex),   // oe = 1
    (725, Opcode::Stswi, op_stswi),
    (727, Opcode::Stfdx, op_stfdx),
    (744, Opcode::Subfmex, op_subfmex), // oe = 1
    (746, Opcode::Addmex, op_addmex),   // oe = 1
    (747, Opcode::Mullwx, op_mullwx),   // oe = 1
    (759, Opcode::Stfdux, op_stfdux),
    (778, Opcode::Addx, op_addx), // oe = 1
    (790, Opcode::Lhbrx, op_lhbrx),
    (792, Opcode::Srawx, op_srawx),
    (824, Opcode::Srawix, op_srawix),
    (854, Opcode::Eieio, op_eieio),
    (918, Opcode::Sthbrx, op_sthbrx),
    (922, Opcode::Extshx, op_extshx),
    (954, Opcode::Extsbx, op_extsbx),
    (971, Opcode::Divwux, op_divwux), // oe = 1
    (982, Opcode::Icbi, op_icbi),
    (983, Opcode::Stfiwx, op_stfiwx),
    (1003, Opcode::Divwx, op_divwx), // oe = 1
    (1014, Opcode::Dcbz, op_dcbz),
];

pub const OPCODE59_TABLE: [OpcodeTableItem; 9] = [
    (18, Opcode::Fdivsx, op_fdivsx),
    (20, Opcode::Fsubsx, op_fsubsx),
    (21, Opcode::Faddsx, op_faddsx),
    (24, Opcode::Fresx, op_fresx),
    (25, Opcode::Fmulsx, op_fmulsx),
    (28, Opcode::Fmsubsx, op_fmsubsx),
    (29, Opcode::Fmaddsx, op_fmaddsx),
    (30, Opcode::Fnmsubsx, op_fnmsubsx),
    (31, Opcode::Fnmaddsx, op_fnmaddsx),
];

pub const OPCODE63X_TABLE: [OpcodeTableItem; 15] = [
    (0, Opcode::Fcmpu, op_fcmpu),
    (12, Opcode::Frspx, op_frspx),
    (14, Opcode::Fctiwx, op_fctiwx),
    (15, Opcode::Fctiwzx, op_fctiwzx),
    (32, Opcode::Fcmpo, op_fcmpo),
    (38, Opcode::Mtfsb1x, op_mtfsb1x),
    (40, Opcode::Fnegx, op_fnegx),
    (64, Opcode::Mcrfs, op_mcrfs),
    (70, Opcode::Mtfsb0x, op_mtfsb0x),
    (72, Opcode::Fmrx, op_fmrx),
    (134, Opcode::Mtfsfix, op_mtfsfix),
    (136, Opcode::Fnabsx, op_fnabsx),
    (264, Opcode::Fabsx, op_fabsx),
    (583, Opcode::Mffsx, op_mffsx),
    (711, Opcode::Mtfsfx, op_mtfsfx),
];

pub const OPCODE63A_TABLE: [OpcodeTableItem; 10] = [
    (18, Opcode::Fdivx, op_fdivx),
    (20, Opcode::Fsubx, op_fsubx),
    (21, Opcode::Faddx, op_faddx),
    (23, Opcode::Fselx, op_fselx),
    (25, Opcode::Fmulx, op_fmulx),
    (26, Opcode::Frsqrtex, op_frsqrtex),
    (28, Opcode::Fmsubx, op_fmsubx),
    (29, Opcode::Fmaddx, op_fmaddx),
    (30, Opcode::Fnmsubx, op_fnmsubx),
    (31, Opcode::Fnmaddx, op_fnmaddx),
];

fn op_illegal(_ctx: &mut Context, _instr: Instruction) {}

fn op_subtable4(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.optable4[instr.xo_x()](ctx, instr);
}

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
include!("cpu_tests.rs");

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
    vxsqrt, _ : 9;           // Floating-point invalid operation exception for invalid square root
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

const QUANTIZE_FLOAT: u32 = 0; // Single-precision floating-point (no conversion)
const QUANTIZE_U8: u32 = 4; // unsigned 8 bit integer
const QUANTIZE_U16: u32 = 5; // unsigned 16 bit integer
const QUANTIZE_I8: u32 = 6; // signed 8 bit integer
const QUANTIZE_I16: u32 = 7; // signed 16 bit integer

// Paired-single store scale
const QUANTIZE_TABLE: [f32; 64] = [
    (1_u32 << 0) as f32,
    (1_u32 << 1) as f32,
    (1_u32 << 2) as f32,
    (1_u32 << 3) as f32,
    (1_u32 << 4) as f32,
    (1_u32 << 5) as f32,
    (1_u32 << 6) as f32,
    (1_u32 << 7) as f32,
    (1_u32 << 8) as f32,
    (1_u32 << 9) as f32,
    (1_u32 << 10) as f32,
    (1_u32 << 11) as f32,
    (1_u32 << 12) as f32,
    (1_u32 << 13) as f32,
    (1_u32 << 14) as f32,
    (1_u32 << 15) as f32,
    (1_u32 << 16) as f32,
    (1_u32 << 17) as f32,
    (1_u32 << 18) as f32,
    (1_u32 << 19) as f32,
    (1_u32 << 20) as f32,
    (1_u32 << 21) as f32,
    (1_u32 << 22) as f32,
    (1_u32 << 23) as f32,
    (1_u32 << 24) as f32,
    (1_u32 << 25) as f32,
    (1_u32 << 26) as f32,
    (1_u32 << 27) as f32,
    (1_u32 << 28) as f32,
    (1_u32 << 29) as f32,
    (1_u32 << 30) as f32,
    (1_u32 << 31) as f32,
    1.0 / (1_u64 << 32) as f32,
    1.0 / (1_u32 << 31) as f32,
    1.0 / (1_u32 << 30) as f32,
    1.0 / (1_u32 << 29) as f32,
    1.0 / (1_u32 << 28) as f32,
    1.0 / (1_u32 << 27) as f32,
    1.0 / (1_u32 << 26) as f32,
    1.0 / (1_u32 << 25) as f32,
    1.0 / (1_u32 << 24) as f32,
    1.0 / (1_u32 << 23) as f32,
    1.0 / (1_u32 << 22) as f32,
    1.0 / (1_u32 << 21) as f32,
    1.0 / (1_u32 << 20) as f32,
    1.0 / (1_u32 << 19) as f32,
    1.0 / (1_u32 << 18) as f32,
    1.0 / (1_u32 << 17) as f32,
    1.0 / (1_u32 << 16) as f32,
    1.0 / (1_u32 << 15) as f32,
    1.0 / (1_u32 << 14) as f32,
    1.0 / (1_u32 << 13) as f32,
    1.0 / (1_u32 << 12) as f32,
    1.0 / (1_u32 << 11) as f32,
    1.0 / (1_u32 << 10) as f32,
    1.0 / (1_u32 << 9) as f32,
    1.0 / (1_u32 << 8) as f32,
    1.0 / (1_u32 << 7) as f32,
    1.0 / (1_u32 << 6) as f32,
    1.0 / (1_u32 << 5) as f32,
    1.0 / (1_u32 << 4) as f32,
    1.0 / (1_u32 << 3) as f32,
    1.0 / (1_u32 << 2) as f32,
    1.0 / (1_u32 << 1) as f32,
];

// paired-single load scale
const DEQUANTIZE_TABLE: [f32; 64] = [
    1.0 / (1_u32 << 0) as f32,
    1.0 / (1_u32 << 1) as f32,
    1.0 / (1_u32 << 2) as f32,
    1.0 / (1_u32 << 3) as f32,
    1.0 / (1_u32 << 4) as f32,
    1.0 / (1_u32 << 5) as f32,
    1.0 / (1_u32 << 6) as f32,
    1.0 / (1_u32 << 7) as f32,
    1.0 / (1_u32 << 8) as f32,
    1.0 / (1_u32 << 9) as f32,
    1.0 / (1_u32 << 10) as f32,
    1.0 / (1_u32 << 11) as f32,
    1.0 / (1_u32 << 12) as f32,
    1.0 / (1_u32 << 13) as f32,
    1.0 / (1_u32 << 14) as f32,
    1.0 / (1_u32 << 15) as f32,
    1.0 / (1_u32 << 16) as f32,
    1.0 / (1_u32 << 17) as f32,
    1.0 / (1_u32 << 18) as f32,
    1.0 / (1_u32 << 19) as f32,
    1.0 / (1_u32 << 20) as f32,
    1.0 / (1_u32 << 21) as f32,
    1.0 / (1_u32 << 22) as f32,
    1.0 / (1_u32 << 23) as f32,
    1.0 / (1_u32 << 24) as f32,
    1.0 / (1_u32 << 25) as f32,
    1.0 / (1_u32 << 26) as f32,
    1.0 / (1_u32 << 27) as f32,
    1.0 / (1_u32 << 28) as f32,
    1.0 / (1_u32 << 29) as f32,
    1.0 / (1_u32 << 30) as f32,
    1.0 / (1_u32 << 31) as f32,
    (1_u64 << 32) as f32,
    (1_u32 << 31) as f32,
    (1_u32 << 30) as f32,
    (1_u32 << 29) as f32,
    (1_u32 << 28) as f32,
    (1_u32 << 27) as f32,
    (1_u32 << 26) as f32,
    (1_u32 << 25) as f32,
    (1_u32 << 24) as f32,
    (1_u32 << 23) as f32,
    (1_u32 << 22) as f32,
    (1_u32 << 21) as f32,
    (1_u32 << 20) as f32,
    (1_u32 << 19) as f32,
    (1_u32 << 18) as f32,
    (1_u32 << 17) as f32,
    (1_u32 << 16) as f32,
    (1_u32 << 15) as f32,
    (1_u32 << 14) as f32,
    (1_u32 << 13) as f32,
    (1_u32 << 12) as f32,
    (1_u32 << 11) as f32,
    (1_u32 << 10) as f32,
    (1_u32 << 9) as f32,
    (1_u32 << 8) as f32,
    (1_u32 << 7) as f32,
    (1_u32 << 6) as f32,
    (1_u32 << 5) as f32,
    (1_u32 << 4) as f32,
    (1_u32 << 3) as f32,
    (1_u32 << 2) as f32,
    (1_u32 << 1) as f32,
];

fn quantize(mut value: f32, st_type: u32, st_scale: u32) -> u32 {
    value *= QUANTIZE_TABLE[st_scale as usize];

    match st_type {
        QUANTIZE_FLOAT => f32::to_bits(value),
        QUANTIZE_U8 => (value.clamp(u8::MIN as f32, u8::MAX as f32) as u8) as u32,
        QUANTIZE_U16 => (value.clamp(u16::MIN as f32, u16::MAX as f32) as u16) as u32,
        QUANTIZE_I8 => ((value.clamp(i8::MIN as f32, i8::MAX as f32) as i8) as i32) as u32,
        QUANTIZE_I16 => ((value.clamp(i16::MIN as f32, i16::MAX as f32) as i16) as i32) as u32,
        _ => {
            warn!("Unrecognized quantize type {st_type}.");
            f32::to_bits(value)
        }
    }
}

fn dequantize(value: u32, ld_type: u32, ld_scale: u32) -> f32 {
    let result = match ld_type {
        QUANTIZE_FLOAT => f32::from_bits(value),
        QUANTIZE_U8 => (value as u8) as f32,
        QUANTIZE_U16 => (value as u16) as f32,
        QUANTIZE_I8 => (value as i8) as f32,
        QUANTIZE_I16 => (value as i16) as f32,
        _ => {
            warn!("unrecognized dequantize unknown type {ld_type}.");
            f32::from_bits(value)
        }
    };

    result * DEQUANTIZE_TABLE[ld_scale as usize]
}

pub trait Nan {
    fn is_snan(&self) -> bool;
    fn is_qnan(&self) -> bool;
}

impl Nan for f32 {
    fn is_snan(&self) -> bool {
        let v = f32::to_bits(*self);
        v & 0x7FC0_0000 == 0x7F80_0000 && v & 0x003F_FFFF != 0
    }

    fn is_qnan(&self) -> bool {
        let v = f32::to_bits(*self);
        v & 0x7FC0_0000 == 0x7FC0_0000
    }
}

impl Nan for f64 {
    fn is_snan(&self) -> bool {
        let v = f64::to_bits(*self);
        v & 0x7FF8_0000_0000_0000 == 0x7FF0_0000_0000_0000 && v & 0x000F_FFFF_FFFF_FFFF != 0
    }

    fn is_qnan(&self) -> bool {
        let v = f64::to_bits(*self);
        v & 0x7FF8_0000_0000_0000 == 0x7FF8_0000_0000_0000
    }
}
