use crate::cpu::instruction::Instruction;
use crate::cpu::ops::branch::*;
use crate::cpu::ops::condition::*;
use crate::cpu::ops::float::*;
use crate::cpu::ops::integer::*;
use crate::cpu::ops::load_store::*;
use crate::cpu::ops::system::*;
use crate::Context;

pub const OPTABLE_SIZE: usize = 64;
pub const OPTABLE4_SIZE: usize = 1024;
pub const OPTABLE19_SIZE: usize = 1024;
pub const OPTABLE31_SIZE: usize = 1024;
pub const OPTABLE59_SIZE: usize = 32;
pub const OPTABLE63_SIZE: usize = 1024;

// Primary Opcodes
pub const OPCODE_TWI: u32 = 3;
pub const OPCODE_EXTENDED4: u32 = 4;
pub const OPCODE_MULLI: u32 = 7;
pub const OPCODE_SUBFIC: u32 = 8;
pub const OPCODE_CMPLI: u32 = 10;
pub const OPCODE_CMPI: u32 = 11;
pub const OPCODE_ADDIC: u32 = 12;
pub const OPCODE_ADDIC_RC: u32 = 13;
pub const OPCODE_ADDI: u32 = 14;
pub const OPCODE_ADDIS: u32 = 15;
pub const OPCODE_BCX: u32 = 16;
pub const OPCODE_SC: u32 = 17;
pub const OPCODE_BX: u32 = 18;
pub const OPCODE_EXTENDED19: u32 = 19;
pub const OPCODE_RLWIMIX: u32 = 20;
pub const OPCODE_RLWINMX: u32 = 21;
pub const OPCODE_RLWNMX: u32 = 23;
pub const OPCODE_ORI: u32 = 24;
pub const OPCODE_ORIS: u32 = 25;
pub const OPCODE_XORI: u32 = 26;
pub const OPCODE_XORIS: u32 = 27;
pub const OPCODE_ANDI_RC: u32 = 28;
pub const OPCODE_ANDIS_RC: u32 = 29;
pub const OPCODE_EXTENDED31: u32 = 31;
pub const OPCODE_LWZ: u32 = 32;
pub const OPCODE_LWZU: u32 = 33;
pub const OPCODE_LBZ: u32 = 34;
pub const OPCODE_LBZU: u32 = 35;
pub const OPCODE_STW: u32 = 36;
pub const OPCODE_STWU: u32 = 37;
pub const OPCODE_STB: u32 = 38;
pub const OPCODE_STBU: u32 = 39;
pub const OPCODE_LHZ: u32 = 40;
pub const OPCODE_LHZU: u32 = 41;
pub const OPCODE_LHA: u32 = 42;
pub const OPCODE_LHAU: u32 = 43;
pub const OPCODE_STH: u32 = 44;
pub const OPCODE_STHU: u32 = 45;
pub const OPCODE_LMW: u32 = 46;
pub const OPCODE_STMW: u32 = 47;
pub const OPCODE_LFS: u32 = 48;
pub const OPCODE_LFSU: u32 = 49;
pub const OPCODE_LFD: u32 = 50;
pub const OPCODE_LFDU: u32 = 51;
pub const OPCODE_STFS: u32 = 52;
pub const OPCODE_STFSU: u32 = 53;
pub const OPCODE_STFD: u32 = 54;
pub const OPCODE_STFDU: u32 = 55;
pub const OPCODE_PSQ_L: u32 = 56;
pub const OPCODE_PSQ_LU: u32 = 57;
pub const OPCODE_EXTENDED59: u32 = 59;
pub const OPCODE_PSQ_ST: u32 = 60;
pub const OPCODE_PSQ_STU: u32 = 61;
pub const OPCODE_EXTENDED63: u32 = 63;

// 4X Extended Opcodes
pub const OPCODE_PS_CMPU0: u32 = 0;
pub const OPCODE_PS_CMPO0: u32 = 32;
pub const OPCODE_PS_NEGX: u32 = 40;
pub const OPCODE_PS_CMPU1: u32 = 64;
pub const OPCODE_PS_MRX: u32 = 72;
pub const OPCODE_PS_CMPO1: u32 = 96;
pub const OPCODE_PS_NABSX: u32 = 136;
pub const OPCODE_PS_ABSX: u32 = 264;
pub const OPCODE_PS_MERGE_00X: u32 = 528;
pub const OPCODE_PS_MERGE_01X: u32 = 560;
pub const OPCODE_PS_MERGE_10X: u32 = 592;
pub const OPCODE_PS_MERGE_11X: u32 = 624;
pub const OPCODE_DCBZ_L: u32 = 1014;

// 4A Extended Opcodes
pub const OPCODE_PS_SUM0X: u32 = 10;
pub const OPCODE_PS_SUM1X: u32 = 11;
pub const OPCODE_PS_MULS0X: u32 = 12;
pub const OPCODE_PS_MULS1X: u32 = 13;
pub const OPCODE_PS_MADDS0X: u32 = 14;
pub const OPCODE_PS_MADDS1X: u32 = 15;
pub const OPCODE_PS_DIVX: u32 = 18;
pub const OPCODE_PS_SUBX: u32 = 20;
pub const OPCODE_PS_ADDX: u32 = 21;
pub const OPCODE_PS_SELX: u32 = 23;
pub const OPCODE_PS_RESX: u32 = 24;
pub const OPCODE_PS_MULX: u32 = 25;
pub const OPCODE_PS_RSQRTEX: u32 = 26;
pub const OPCODE_PS_MSUBX: u32 = 28;
pub const OPCODE_PS_MADDX: u32 = 29;
pub const OPCODE_PS_NMSUBX: u32 = 30;
pub const OPCODE_PS_NMADDX: u32 = 31;

// 4AA Extended Opcodes
pub const OPCODE_PSQ_LX: u32 = 6;
pub const OPCODE_PSQ_STX: u32 = 7;
pub const OPCODE_PSQ_LUX: u32 = 38;
pub const OPCODE_PSQ_STUX: u32 = 39;

// 19 Extended Opcodes
pub const OPCODE_MCRF: u32 = 0;
pub const OPCODE_BCLRX: u32 = 16;
pub const OPCODE_CRNOR: u32 = 33;
pub const OPCODE_RFI: u32 = 50;
pub const OPCODE_CRANDC: u32 = 129;
pub const OPCODE_ISYNC: u32 = 150;
pub const OPCODE_CRXOR: u32 = 193;
pub const OPCODE_CRNAND: u32 = 225;
pub const OPCODE_CRAND: u32 = 257;
pub const OPCODE_CREQV: u32 = 289;
pub const OPCODE_CRORC: u32 = 417;
pub const OPCODE_CROR: u32 = 449;
pub const OPCODE_BCCTRX: u32 = 528;

// 31 Extended Opcodes
pub const OPCODE_CMP: u32 = 0;
pub const OPCODE_TW: u32 = 4;
pub const OPCODE_SUBFCX: u32 = 8;
pub const OPCODE_ADDCX: u32 = 10;
pub const OPCODE_MULHWUX: u32 = 11;
pub const OPCODE_MFCR: u32 = 19;
pub const OPCODE_LWARX: u32 = 20;
pub const OPCODE_LWZX: u32 = 23;
pub const OPCODE_SLWX: u32 = 24;
pub const OPCODE_CNTLZWX: u32 = 26;
pub const OPCODE_ANDX: u32 = 28;
pub const OPCODE_CMPL: u32 = 32;
pub const OPCODE_SUBFX: u32 = 40;
pub const OPCODE_DCBST: u32 = 54;
pub const OPCODE_LWZUX: u32 = 55;
pub const OPCODE_ANDCX: u32 = 60;
pub const OPCODE_MULHWX: u32 = 75;
pub const OPCODE_MFMSR: u32 = 83;
pub const OPCODE_DCBF: u32 = 86;
pub const OPCODE_LBZX: u32 = 87;
pub const OPCODE_NEGX: u32 = 104;
pub const OPCODE_LBZUX: u32 = 119;
pub const OPCODE_NORX: u32 = 124;
pub const OPCODE_SUBFEX: u32 = 136;
pub const OPCODE_ADDEX: u32 = 138;
pub const OPCODE_MTCRF: u32 = 144;
pub const OPCODE_MTMSR: u32 = 146;
pub const OPCODE_STWCX_RC: u32 = 150;
pub const OPCODE_STWX: u32 = 151;
pub const OPCODE_STWUX: u32 = 183;
pub const OPCODE_SUBFZEX: u32 = 200;
pub const OPCODE_ADDZEX: u32 = 202;
pub const OPCODE_MTSR: u32 = 210;
pub const OPCODE_STBX: u32 = 215;
pub const OPCODE_SUBFMEX: u32 = 232;
pub const OPCODE_ADDMEX: u32 = 234;
pub const OPCODE_MULLWX: u32 = 235;
pub const OPCODE_MTSRIN: u32 = 242;
pub const OPCODE_DCBTST: u32 = 246;
pub const OPCODE_STBUX: u32 = 247;
pub const OPCODE_ADDX: u32 = 266;
pub const OPCODE_DCBT: u32 = 278;
pub const OPCODE_LHZX: u32 = 279;
pub const OPCODE_EQVX: u32 = 284;
pub const OPCODE_TBLIE: u32 = 306;
pub const OPCODE_ECIWX: u32 = 310;
pub const OPCODE_LHZUX: u32 = 311;
pub const OPCODE_XORX: u32 = 316;
pub const OPCODE_MFSPR: u32 = 339;
pub const OPCODE_LHAX: u32 = 343;
pub const OPCODE_MFTB: u32 = 371;
pub const OPCODE_LHAUX: u32 = 375;
pub const OPCODE_STHX: u32 = 407;
pub const OPCODE_ORCX: u32 = 412;
pub const OPCODE_ECOWX: u32 = 438;
pub const OPCODE_STHUX: u32 = 439;
pub const OPCODE_ORX: u32 = 444;
pub const OPCODE_DIVWUX: u32 = 459;
pub const OPCODE_MTSPR: u32 = 467;
pub const OPCODE_DCBI: u32 = 470;
pub const OPCODE_NANDX: u32 = 476;
pub const OPCODE_DIVWX: u32 = 491;
pub const OPCODE_MCRXR: u32 = 512;
pub const OPCODE_SUBFCX_OE: u32 = 520;
pub const OPCODE_ADDCX_OE: u32 = 522;
pub const OPCODE_MULHWUX_21: u32 = 523;
pub const OPCODE_LSWX: u32 = 533;
pub const OPCODE_LWBRX: u32 = 534;
pub const OPCODE_LFSX: u32 = 535;
pub const OPCODE_SRWX: u32 = 536;
pub const OPCODE_SUBFX_OE: u32 = 552;
pub const OPCODE_TLBSYNC: u32 = 566;
pub const OPCODE_LFSUX: u32 = 567;
pub const OPCODE_MULHWX_21: u32 = 587;
pub const OPCODE_MFSR: u32 = 595;
pub const OPCODE_LSWI: u32 = 597;
pub const OPCODE_SYNC: u32 = 598;
pub const OPCODE_LFDX: u32 = 599;
pub const OPCODE_NEGX_OE: u32 = 616;
pub const OPCODE_LFDUX: u32 = 631;
pub const OPCODE_SUBFEX_OE: u32 = 648;
pub const OPCODE_ADDEX_OE: u32 = 650;
pub const OPCODE_MFSRIN: u32 = 659;
pub const OPCODE_STSWX: u32 = 661;
pub const OPCODE_STWBRX: u32 = 662;
pub const OPCODE_STFSX: u32 = 663;
pub const OPCODE_STFSUX: u32 = 695;
pub const OPCODE_SUBFZEX_OE: u32 = 712;
pub const OPCODE_ADDZEX_OE: u32 = 714;
pub const OPCODE_STSWI: u32 = 725;
pub const OPCODE_STFDX: u32 = 727;
pub const OPCODE_SUBFMEX_OE: u32 = 744;
pub const OPCODE_ADDMEX_OE: u32 = 746;
pub const OPCODE_MULLWX_OE: u32 = 747;
pub const OPCODE_STFDUX: u32 = 759;
pub const OPCODE_ADDX_OE: u32 = 778;
pub const OPCODE_LHBRX: u32 = 790;
pub const OPCODE_SRAWX: u32 = 792;
pub const OPCODE_SRAWIX: u32 = 824;
pub const OPCODE_EIEIO: u32 = 854;
pub const OPCODE_STHBRX: u32 = 918;
pub const OPCODE_EXTSHX: u32 = 922;
pub const OPCODE_EXTSBX: u32 = 954;
pub const OPCODE_DIVWUX_OE: u32 = 971;
pub const OPCODE_ICBI: u32 = 982;
pub const OPCODE_STFIWX: u32 = 983;
pub const OPCODE_DIVWX_OE: u32 = 1003;
pub const OPCODE_DCBZ: u32 = 1014;

// 59 Extended Opcodes
pub const OPCODE_FDIVSX: u32 = 18;
pub const OPCODE_FSUBSX: u32 = 20;
pub const OPCODE_FADDSX: u32 = 21;
pub const OPCODE_FRESX: u32 = 24;
pub const OPCODE_FMULSX: u32 = 25;
pub const OPCODE_FMSUBSX: u32 = 28;
pub const OPCODE_FMADDSX: u32 = 29;
pub const OPCODE_FNMSUBSX: u32 = 30;
pub const OPCODE_FNMADDSX: u32 = 31;

// 63X Extended Opcodes
pub const OPCODE_FCMPU: u32 = 0;
pub const OPCODE_FRSPX: u32 = 12;
pub const OPCODE_FCTIWX: u32 = 14;
pub const OPCODE_FCTIWZX: u32 = 15;
pub const OPCODE_FCMPO: u32 = 32;
pub const OPCODE_MTFSB1X: u32 = 38;
pub const OPCODE_FNEGX: u32 = 40;
pub const OPCODE_MCRFS: u32 = 64;
pub const OPCODE_MTFSB0X: u32 = 70;
pub const OPCODE_FMRX: u32 = 72;
pub const OPCODE_MTFSFIX: u32 = 134;
pub const OPCODE_FNABSX: u32 = 136;
pub const OPCODE_FABSX: u32 = 264;
pub const OPCODE_MFFSX: u32 = 583;
pub const OPCODE_MTFSFX: u32 = 711;

// 63A Extended Opcodes
pub const OPCODE_FDIVX: u32 = 18;
pub const OPCODE_FSUBX: u32 = 20;
pub const OPCODE_FADDX: u32 = 21;
pub const OPCODE_FSELX: u32 = 23;
pub const OPCODE_FMULX: u32 = 25;
pub const OPCODE_FRSQRTEX: u32 = 26;
pub const OPCODE_FMSUBX: u32 = 28;
pub const OPCODE_FMADDX: u32 = 29;
pub const OPCODE_FNMSUBX: u32 = 30;
pub const OPCODE_FNMADDX: u32 = 31;

#[derive(Copy, Clone, Debug, PartialEq)]
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

pub type OpcodeTableItem = (u32, Opcode, fn(&mut Context, Instruction));

pub const ILLEGAL_OP: (Opcode, fn(&mut Context, Instruction)) = (Opcode::Illegal, op_illegal);

pub const OPCODE_TABLE: [OpcodeTableItem; 54] = [
    (OPCODE_TWI, Opcode::Twi, op_twi),
    (OPCODE_EXTENDED4, Opcode::Table4, op_subtable4),
    (OPCODE_MULLI, Opcode::Mulli, op_mulli),
    (OPCODE_SUBFIC, Opcode::Subfic, op_subfic),
    (OPCODE_CMPLI, Opcode::Cmpli, op_cmpli),
    (OPCODE_CMPI, Opcode::Cmpi, op_cmpi),
    (OPCODE_ADDIC, Opcode::Addic, op_addic),
    (OPCODE_ADDIC_RC, Opcode::Addicrc, op_addic_rc),
    (OPCODE_ADDI, Opcode::Addi, op_addi),
    (OPCODE_ADDIS, Opcode::Addis, op_addis),
    (OPCODE_BCX, Opcode::Bcx, op_bcx),
    (OPCODE_SC, Opcode::Sc, op_sc),
    (OPCODE_BX, Opcode::Bx, op_bx),
    (OPCODE_EXTENDED19, Opcode::Table19, op_subtable19),
    (OPCODE_RLWIMIX, Opcode::Rlwimix, op_rlwimix),
    (OPCODE_RLWINMX, Opcode::Rlwinmx, op_rlwinmx),
    (OPCODE_RLWNMX, Opcode::Rlwnmx, op_rlwnmx),
    (OPCODE_ORI, Opcode::Ori, op_ori),
    (OPCODE_ORIS, Opcode::Oris, op_oris),
    (OPCODE_XORI, Opcode::Xori, op_xori),
    (OPCODE_XORIS, Opcode::Xoris, op_xoris),
    (OPCODE_ANDI_RC, Opcode::Andirc, op_andi_rc),
    (OPCODE_ANDIS_RC, Opcode::Andisrc, op_andis_rc),
    (OPCODE_EXTENDED31, Opcode::Table31, op_subtable31),
    (OPCODE_LWZ, Opcode::Lwz, op_lwz),
    (OPCODE_LWZU, Opcode::Lwzu, op_lwzu),
    (OPCODE_LBZ, Opcode::Lbz, op_lbz),
    (OPCODE_LBZU, Opcode::Lbzu, op_lbzu),
    (OPCODE_STW, Opcode::Stw, op_stw),
    (OPCODE_STWU, Opcode::Stwu, op_stwu),
    (OPCODE_STB, Opcode::Stb, op_stb),
    (OPCODE_STBU, Opcode::Stbu, op_stbu),
    (OPCODE_LHZ, Opcode::Lhz, op_lhz),
    (OPCODE_LHZU, Opcode::Lhzu, op_lhzu),
    (OPCODE_LHA, Opcode::Lha, op_lha),
    (OPCODE_LHAU, Opcode::Lhau, op_lhau),
    (OPCODE_STH, Opcode::Sth, op_sth),
    (OPCODE_STHU, Opcode::Sthu, op_sthu),
    (OPCODE_LMW, Opcode::Lmw, op_lmw),
    (OPCODE_STMW, Opcode::Stmw, op_stmw),
    (OPCODE_LFS, Opcode::Lfs, op_lfs),
    (OPCODE_LFSU, Opcode::Lfsu, op_lfsu),
    (OPCODE_LFD, Opcode::Lfd, op_lfd),
    (OPCODE_LFDU, Opcode::Lfdu, op_lfdu),
    (OPCODE_STFS, Opcode::Stfs, op_stfs),
    (OPCODE_STFSU, Opcode::Stfsu, op_stfsu),
    (OPCODE_STFD, Opcode::Stfd, op_stfd),
    (OPCODE_STFDU, Opcode::Stfdu, op_stfdu),
    (OPCODE_PSQ_L, Opcode::PsqL, op_psq_l),
    (OPCODE_PSQ_LU, Opcode::PsqLu, op_psq_lu),
    (OPCODE_EXTENDED59, Opcode::Table59, op_subtable59),
    (OPCODE_PSQ_ST, Opcode::PsqSt, op_psq_st),
    (OPCODE_PSQ_STU, Opcode::PsqStu, op_psq_stu),
    (OPCODE_EXTENDED63, Opcode::Table63, op_subtable63),
];

pub const OPCODE4X_TABLE: [OpcodeTableItem; 13] = [
    (OPCODE_PS_CMPU0, Opcode::PsCmpu0, op_ps_cmpu0),
    (OPCODE_PS_CMPO0, Opcode::PsCmpo0, op_ps_cmpo0),
    (OPCODE_PS_NEGX, Opcode::PsNegx, op_ps_negx),
    (OPCODE_PS_CMPU1, Opcode::PsCmpu1, op_ps_cmpu1),
    (OPCODE_PS_MRX, Opcode::PsMrx, op_ps_mrx),
    (OPCODE_PS_CMPO1, Opcode::PsCmpo1, op_ps_cmpo1),
    (OPCODE_PS_NABSX, Opcode::PsNabsx, op_ps_nabsx),
    (OPCODE_PS_ABSX, Opcode::PsAbsx, op_ps_absx),
    (OPCODE_PS_MERGE_00X, Opcode::PsMerge00x, op_ps_merge00x),
    (OPCODE_PS_MERGE_01X, Opcode::PsMerge01x, op_ps_merge01x),
    (OPCODE_PS_MERGE_10X, Opcode::PsMerge10x, op_ps_merge10x),
    (OPCODE_PS_MERGE_11X, Opcode::PsMerge11x, op_ps_merge11x),
    (OPCODE_DCBZ_L, Opcode::DcbzL, op_dcbz_l),
];

pub const OPCODE4A_TABLE: [OpcodeTableItem; 17] = [
    (OPCODE_PS_SUM0X, Opcode::PsSum0x, op_ps_sum0x),
    (OPCODE_PS_SUM1X, Opcode::PsSum1x, op_ps_sum1x),
    (OPCODE_PS_MULS0X, Opcode::PsMuls0x, op_ps_muls0x),
    (OPCODE_PS_MULS1X, Opcode::PsMuls1x, op_ps_muls1x),
    (OPCODE_PS_MADDS0X, Opcode::PsMadds0x, op_ps_madds0x),
    (OPCODE_PS_MADDS1X, Opcode::PsMadds1x, op_ps_madds1x),
    (OPCODE_PS_DIVX, Opcode::PsDivx, op_ps_divx),
    (OPCODE_PS_SUBX, Opcode::PsSubx, op_ps_subx),
    (OPCODE_PS_ADDX, Opcode::PsAddx, op_ps_addx),
    (OPCODE_PS_SELX, Opcode::PsSelx, op_ps_selx),
    (OPCODE_PS_RESX, Opcode::PsResx, op_ps_resx),
    (OPCODE_PS_MULX, Opcode::PsMulx, op_ps_mulx),
    (OPCODE_PS_RSQRTEX, Opcode::PsRsqrtex, op_ps_rsqrtex),
    (OPCODE_PS_MSUBX, Opcode::PsMsubx, op_ps_msubx),
    (OPCODE_PS_MADDX, Opcode::PsMaddx, op_ps_maddx),
    (OPCODE_PS_NMSUBX, Opcode::PsNmsubx, op_ps_nmsubx),
    (OPCODE_PS_NMADDX, Opcode::PsNmaddx, op_ps_nmaddx),
];

pub const OPCODE4AA_TABLE: [OpcodeTableItem; 4] = [
    (OPCODE_PSQ_LX, Opcode::PsqLx, op_psq_lx),
    (OPCODE_PSQ_STX, Opcode::PsqStx, op_psq_stx),
    (OPCODE_PSQ_LUX, Opcode::PsqLux, op_psq_lux),
    (OPCODE_PSQ_STUX, Opcode::PsqStux, op_psq_stux),
];

pub const OPCODE19_TABLE: [OpcodeTableItem; 13] = [
    (OPCODE_MCRF, Opcode::Mcrf, op_mcrf),
    (OPCODE_BCLRX, Opcode::Bclrx, op_bclrx),
    (OPCODE_CRNOR, Opcode::Crnor, op_crnor),
    (OPCODE_RFI, Opcode::Rfi, op_rfi),
    (OPCODE_CRANDC, Opcode::Crandc, op_crandc),
    (OPCODE_ISYNC, Opcode::Isync, op_isync),
    (OPCODE_CRXOR, Opcode::Crxor, op_crxor),
    (OPCODE_CRNAND, Opcode::Crnand, op_crnand),
    (OPCODE_CRAND, Opcode::Crand, op_crand),
    (OPCODE_CREQV, Opcode::Creqv, op_creqv),
    (OPCODE_CRORC, Opcode::Crorc, op_crorc),
    (OPCODE_CROR, Opcode::Cror, op_cror),
    (OPCODE_BCCTRX, Opcode::Bcctrx, op_bcctrx),
];

pub const OPCODE31_TABLE: [OpcodeTableItem; 108] = [
    (OPCODE_CMP, Opcode::Cmp, op_cmp),
    (OPCODE_TW, Opcode::Tw, op_tw),
    (OPCODE_SUBFCX, Opcode::Subfcx, op_subfcx),
    (OPCODE_ADDCX, Opcode::Addcx, op_addcx),
    (OPCODE_MULHWUX, Opcode::Mulhwux, op_mulhwux),
    (OPCODE_MFCR, Opcode::Mfcr, op_mfcr),
    (OPCODE_LWARX, Opcode::Lwarx, op_lwarx),
    (OPCODE_LWZX, Opcode::Lwzx, op_lwzx),
    (OPCODE_SLWX, Opcode::Slwx, op_slwx),
    (OPCODE_CNTLZWX, Opcode::Cntlzwx, op_cntlzwx),
    (OPCODE_ANDX, Opcode::Andx, op_andx),
    (OPCODE_CMPL, Opcode::Cmpl, op_cmpl),
    (OPCODE_SUBFX, Opcode::Subfx, op_subfx),
    (OPCODE_DCBST, Opcode::Dcbst, op_dcbst),
    (OPCODE_LWZUX, Opcode::Lwzux, op_lwzux),
    (OPCODE_ANDCX, Opcode::Andcx, op_andcx),
    (OPCODE_MULHWX, Opcode::Mulhwx, op_mulhwx),
    (OPCODE_MFMSR, Opcode::Mfmsr, op_mfmsr),
    (OPCODE_DCBF, Opcode::Dcbf, op_dcbf),
    (OPCODE_LBZX, Opcode::Lbzx, op_lbzx),
    (OPCODE_NEGX, Opcode::Negx, op_negx),
    (OPCODE_LBZUX, Opcode::Lbzux, op_lbzux),
    (OPCODE_NORX, Opcode::Norx, op_norx),
    (OPCODE_SUBFEX, Opcode::Subfex, op_subfex),
    (OPCODE_ADDEX, Opcode::Addex, op_addex),
    (OPCODE_MTCRF, Opcode::Mtcrf, op_mtcrf),
    (OPCODE_MTMSR, Opcode::Mtmsr, op_mtmsr),
    (OPCODE_STWCX_RC, Opcode::Stwcxrc, op_stwcx_rc),
    (OPCODE_STWX, Opcode::Stwx, op_stwx),
    (OPCODE_STWUX, Opcode::Stwux, op_stwux),
    (OPCODE_SUBFZEX, Opcode::Subfzex, op_subfzex),
    (OPCODE_ADDZEX, Opcode::Addzex, op_addzex),
    (OPCODE_MTSR, Opcode::Mtsr, op_mtsr),
    (OPCODE_STBX, Opcode::Stbx, op_stbx),
    (OPCODE_SUBFMEX, Opcode::Subfmex, op_subfmex),
    (OPCODE_ADDMEX, Opcode::Addmex, op_addmex),
    (OPCODE_MULLWX, Opcode::Mullwx, op_mullwx),
    (OPCODE_MTSRIN, Opcode::Mtsrin, op_mtsrin),
    (OPCODE_DCBTST, Opcode::Dcbtst, op_dcbtst),
    (OPCODE_STBUX, Opcode::Stbux, op_stbux),
    (OPCODE_ADDX, Opcode::Addx, op_addx),
    (OPCODE_DCBT, Opcode::Dcbt, op_dcbt),
    (OPCODE_LHZX, Opcode::Lhzx, op_lhzx),
    (OPCODE_EQVX, Opcode::Eqvx, op_eqvx),
    (OPCODE_TBLIE, Opcode::Tlbie, op_tlbie),
    (OPCODE_ECIWX, Opcode::Eciwx, op_eciwx),
    (OPCODE_LHZUX, Opcode::Lhzux, op_lhzux),
    (OPCODE_XORX, Opcode::Xorx, op_xorx),
    (OPCODE_MFSPR, Opcode::Mfspr, op_mfspr),
    (OPCODE_LHAX, Opcode::Lhax, op_lhax),
    (OPCODE_MFTB, Opcode::Mftb, op_mftb),
    (OPCODE_LHAUX, Opcode::Lhaux, op_lhaux),
    (OPCODE_STHX, Opcode::Sthx, op_sthx),
    (OPCODE_ORCX, Opcode::Orcx, op_orcx),
    (OPCODE_ECOWX, Opcode::Ecowx, op_ecowx),
    (OPCODE_STHUX, Opcode::Sthux, op_sthux),
    (OPCODE_ORX, Opcode::Orx, op_orx),
    (OPCODE_DIVWUX, Opcode::Divwux, op_divwux),
    (OPCODE_MTSPR, Opcode::Mtspr, op_mtspr),
    (OPCODE_DCBI, Opcode::Dcbi, op_dcbi),
    (OPCODE_NANDX, Opcode::Nandx, op_nandx),
    (OPCODE_DIVWX, Opcode::Divwx, op_divwx),
    (OPCODE_MCRXR, Opcode::Mcrxr, op_mcrxr),
    (OPCODE_SUBFCX_OE, Opcode::Subfcx, op_subfcx), // oe = 1
    (OPCODE_ADDCX_OE, Opcode::Addcx, op_addcx),    // oe = 1
    (OPCODE_MULHWUX_21, Opcode::Mulhwux, op_mulhwux), // 21(reserved) = 1
    (OPCODE_LSWX, Opcode::Lswx, op_lswx),
    (OPCODE_LWBRX, Opcode::Lwbrx, op_lwbrx),
    (OPCODE_LFSX, Opcode::Lfsx, op_lfsx),
    (OPCODE_SRWX, Opcode::Srwx, op_srwx),
    (OPCODE_SUBFX_OE, Opcode::Subfx, op_subfx), // oe = 1
    (OPCODE_TLBSYNC, Opcode::Tlbsync, op_tlbsync),
    (OPCODE_LFSUX, Opcode::Lfsux, op_lfsux),
    (OPCODE_MULHWX_21, Opcode::Mulhwx, op_mulhwx), // 21(reserved) = 1
    (OPCODE_MFSR, Opcode::Mfsr, op_mfsr),
    (OPCODE_LSWI, Opcode::Lswi, op_lswi),
    (OPCODE_SYNC, Opcode::Sync, op_sync),
    (OPCODE_LFDX, Opcode::Lfdx, op_lfdx),
    (OPCODE_NEGX_OE, Opcode::Negx, op_negx), // oe = 1
    (OPCODE_LFDUX, Opcode::Lfdux, op_lfdux),
    (OPCODE_SUBFEX_OE, Opcode::Subfex, op_subfex), // oe = 1
    (OPCODE_ADDEX_OE, Opcode::Addex, op_addex),    // oe = 1
    (OPCODE_MFSRIN, Opcode::Mfsrin, op_mfsrin),
    (OPCODE_STSWX, Opcode::Stswx, op_stswx),
    (OPCODE_STWBRX, Opcode::Stwbrx, op_stwbrx),
    (OPCODE_STFSX, Opcode::Stfsx, op_stfsx),
    (OPCODE_STFSUX, Opcode::Stfsux, op_stfsux),
    (OPCODE_SUBFZEX_OE, Opcode::Subfzex, op_subfzex), // oe = 1
    (OPCODE_ADDZEX_OE, Opcode::Addzex, op_addzex),    // oe = 1
    (OPCODE_STSWI, Opcode::Stswi, op_stswi),
    (OPCODE_STFDX, Opcode::Stfdx, op_stfdx),
    (OPCODE_SUBFMEX_OE, Opcode::Subfmex, op_subfmex), // oe = 1
    (OPCODE_ADDMEX_OE, Opcode::Addmex, op_addmex),    // oe = 1
    (OPCODE_MULLWX_OE, Opcode::Mullwx, op_mullwx),    // oe = 1
    (OPCODE_STFDUX, Opcode::Stfdux, op_stfdux),
    (OPCODE_ADDX_OE, Opcode::Addx, op_addx), // oe = 1
    (OPCODE_LHBRX, Opcode::Lhbrx, op_lhbrx),
    (OPCODE_SRAWX, Opcode::Srawx, op_srawx),
    (OPCODE_SRAWIX, Opcode::Srawix, op_srawix),
    (OPCODE_EIEIO, Opcode::Eieio, op_eieio),
    (OPCODE_STHBRX, Opcode::Sthbrx, op_sthbrx),
    (OPCODE_EXTSHX, Opcode::Extshx, op_extshx),
    (OPCODE_EXTSBX, Opcode::Extsbx, op_extsbx),
    (OPCODE_DIVWUX_OE, Opcode::Divwux, op_divwux), // oe = 1
    (OPCODE_ICBI, Opcode::Icbi, op_icbi),
    (OPCODE_STFIWX, Opcode::Stfiwx, op_stfiwx),
    (OPCODE_DIVWX_OE, Opcode::Divwx, op_divwx), // oe = 1
    (OPCODE_DCBZ, Opcode::Dcbz, op_dcbz),
];

pub const OPCODE59_TABLE: [OpcodeTableItem; 9] = [
    (OPCODE_FDIVSX, Opcode::Fdivsx, op_fdivsx),
    (OPCODE_FSUBSX, Opcode::Fsubsx, op_fsubsx),
    (OPCODE_FADDSX, Opcode::Faddsx, op_faddsx),
    (OPCODE_FRESX, Opcode::Fresx, op_fresx),
    (OPCODE_FMULSX, Opcode::Fmulsx, op_fmulsx),
    (OPCODE_FMSUBSX, Opcode::Fmsubsx, op_fmsubsx),
    (OPCODE_FMADDSX, Opcode::Fmaddsx, op_fmaddsx),
    (OPCODE_FNMSUBSX, Opcode::Fnmsubsx, op_fnmsubsx),
    (OPCODE_FNMADDSX, Opcode::Fnmaddsx, op_fnmaddsx),
];

pub const OPCODE63X_TABLE: [OpcodeTableItem; 15] = [
    (OPCODE_FCMPU, Opcode::Fcmpu, op_fcmpu),
    (OPCODE_FRSPX, Opcode::Frspx, op_frspx),
    (OPCODE_FCTIWX, Opcode::Fctiwx, op_fctiwx),
    (OPCODE_FCTIWZX, Opcode::Fctiwzx, op_fctiwzx),
    (OPCODE_FCMPO, Opcode::Fcmpo, op_fcmpo),
    (OPCODE_MTFSB1X, Opcode::Mtfsb1x, op_mtfsb1x),
    (OPCODE_FNEGX, Opcode::Fnegx, op_fnegx),
    (OPCODE_MCRFS, Opcode::Mcrfs, op_mcrfs),
    (OPCODE_MTFSB0X, Opcode::Mtfsb0x, op_mtfsb0x),
    (OPCODE_FMRX, Opcode::Fmrx, op_fmrx),
    (OPCODE_MTFSFIX, Opcode::Mtfsfix, op_mtfsfix),
    (OPCODE_FNABSX, Opcode::Fnabsx, op_fnabsx),
    (OPCODE_FABSX, Opcode::Fabsx, op_fabsx),
    (OPCODE_MFFSX, Opcode::Mffsx, op_mffsx),
    (OPCODE_MTFSFX, Opcode::Mtfsfx, op_mtfsfx),
];

pub const OPCODE63A_TABLE: [OpcodeTableItem; 10] = [
    (OPCODE_FDIVX, Opcode::Fdivx, op_fdivx),
    (OPCODE_FSUBX, Opcode::Fsubx, op_fsubx),
    (OPCODE_FADDX, Opcode::Faddx, op_faddx),
    (OPCODE_FSELX, Opcode::Fselx, op_fselx),
    (OPCODE_FMULX, Opcode::Fmulx, op_fmulx),
    (OPCODE_FRSQRTEX, Opcode::Frsqrtex, op_frsqrtex),
    (OPCODE_FMSUBX, Opcode::Fmsubx, op_fmsubx),
    (OPCODE_FMADDX, Opcode::Fmaddx, op_fmaddx),
    (OPCODE_FNMSUBX, Opcode::Fnmsubx, op_fnmsubx),
    (OPCODE_FNMADDX, Opcode::Fnmaddx, op_fnmaddx),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup() {
        let mut optable: [Opcode; OPTABLE_SIZE] = [Opcode::Illegal; OPTABLE_SIZE];
        let mut optable4: [Opcode; OPTABLE4_SIZE] = [Opcode::Illegal; OPTABLE4_SIZE];
        let mut optable19: [Opcode; OPTABLE19_SIZE] = [Opcode::Illegal; OPTABLE19_SIZE];
        let mut optable31: [Opcode; OPTABLE31_SIZE] = [Opcode::Illegal; OPTABLE31_SIZE];
        let mut optable59: [Opcode; OPTABLE59_SIZE] = [Opcode::Illegal; OPTABLE59_SIZE];
        let mut optable63: [Opcode; OPTABLE63_SIZE] = [Opcode::Illegal; OPTABLE63_SIZE];

        for op in OPCODE_TABLE.iter() {
            optable[op.0 as usize] = op.1;
        }

        for op in OPCODE4X_TABLE.iter() {
            optable4[op.0 as usize] = op.1;
        }

        for n in 0..32 {
            let fill = n << 5;
            for op in OPCODE4A_TABLE.iter() {
                let xo_x = (op.0 as usize) | fill;
                optable4[xo_x] = op.1;
            }
        }

        for n in 0..16 {
            let fill = n << 6;
            for op in OPCODE4AA_TABLE.iter() {
                let xo_x = (op.0 as usize) | fill;
                optable4[xo_x] = op.1;
            }
        }

        for op in OPCODE19_TABLE.iter() {
            optable19[op.0 as usize] = op.1;
        }

        for op in OPCODE31_TABLE.iter() {
            optable31[op.0 as usize] = op.1;
        }

        for op in OPCODE59_TABLE.iter() {
            optable59[op.0 as usize] = op.1;
        }

        for op in OPCODE63X_TABLE.iter() {
            optable63[op.0 as usize] = op.1;
        }

        for n in 0..32 {
            let fill = n << 5;
            for op in OPCODE63A_TABLE.iter() {
                let xo_x = (op.0 as usize) | fill;
                optable63[xo_x] = op.1;
            }
        }

        let data = [
            (0x7C00_0214, Opcode::Addx),
            (0x7C00_0014, Opcode::Addcx),
            (0x7C00_0114, Opcode::Addex),
            (0x3800_0000, Opcode::Addi),
            (0x3000_0000, Opcode::Addic),
            (0x3400_0000, Opcode::Addicrc),
            (0x3C00_0000, Opcode::Addis),
            (0x7C00_01D4, Opcode::Addmex),
            (0x7C00_0194, Opcode::Addzex),
            (0x7C00_0038, Opcode::Andx),
            (0x7C00_0078, Opcode::Andcx),
            (0x7000_0000, Opcode::Andirc),
            (0x7400_0000, Opcode::Andisrc),
            (0x4800_0000, Opcode::Bx),
            (0x4000_0000, Opcode::Bcx),
            (0x4C00_0420, Opcode::Bcctrx),
            (0x4c00_0020, Opcode::Bclrx),
            (0x7C00_0000, Opcode::Cmp),
            (0x2C00_0000, Opcode::Cmpi),
            (0x7C00_0040, Opcode::Cmpl),
            (0x2800_0000, Opcode::Cmpli),
            (0x7C00_0034, Opcode::Cntlzwx),
            (0x4C00_0202, Opcode::Crand),
            (0x4C00_0102, Opcode::Crandc),
            (0x4C00_0242, Opcode::Creqv),
            (0x4C00_01C2, Opcode::Crnand),
            (0x4C00_0042, Opcode::Crnor),
            (0x4C00_0382, Opcode::Cror),
            (0x4C00_0342, Opcode::Crorc),
            (0x4C00_0182, Opcode::Crxor),
            (0x7C00_00AC, Opcode::Dcbf),
            (0x7C00_03AC, Opcode::Dcbi),
            (0x7C00_006C, Opcode::Dcbst),
            (0x7C00_022C, Opcode::Dcbt),
            (0x7C00_01EC, Opcode::Dcbtst),
            (0x7C00_07EC, Opcode::Dcbz),
            (0x1000_07EC, Opcode::DcbzL),
            (0x7C00_03D6, Opcode::Divwx),
            (0x7C00_0396, Opcode::Divwux),
            (0x7C00_026C, Opcode::Eciwx),
            (0x7C00_036C, Opcode::Ecowx),
            (0x7C00_06AC, Opcode::Eieio),
            (0x7C00_0238, Opcode::Eqvx),
            (0x7C00_0774, Opcode::Extsbx),
            (0x7C00_0734, Opcode::Extshx),
            (0xFC00_0210, Opcode::Fabsx),
            (0xFC00_002A, Opcode::Faddx),
            (0xEC00_002A, Opcode::Faddsx),
            (0xFC00_0040, Opcode::Fcmpo),
            (0xFC00_0000, Opcode::Fcmpu),
            (0xFC00_001C, Opcode::Fctiwx),
            (0xFC00_001E, Opcode::Fctiwzx),
            (0xFC00_0024, Opcode::Fdivx),
            (0xEC00_0024, Opcode::Fdivsx),
            (0xFC00_003A, Opcode::Fmaddx),
            (0xEC00_003A, Opcode::Fmaddsx),
            (0xFC00_0090, Opcode::Fmrx),
            (0xFC00_0038, Opcode::Fmsubx),
            (0xEC00_0038, Opcode::Fmsubsx),
            (0xFC00_0032, Opcode::Fmulx),
            (0xEC00_0032, Opcode::Fmulsx),
            (0xFC00_0110, Opcode::Fnabsx),
            (0xFC00_0050, Opcode::Fnegx),
            (0xFC00_003E, Opcode::Fnmaddx),
            (0xEC00_003E, Opcode::Fnmaddsx),
            (0xFC00_003C, Opcode::Fnmsubx),
            (0xEC00_003C, Opcode::Fnmsubsx),
            (0xEC00_0030, Opcode::Fresx),
            (0xFC00_0018, Opcode::Frspx),
            (0xFC00_0034, Opcode::Frsqrtex),
            (0xFC00_002E, Opcode::Fselx),
            (0xFC00_0028, Opcode::Fsubx),
            (0xEC00_0028, Opcode::Fsubsx),
            (0x7C00_07AC, Opcode::Icbi),
            (0x4C00_012C, Opcode::Isync),
            (0x8800_0000, Opcode::Lbz),
            (0x8C00_0000, Opcode::Lbzu),
            (0x7C00_00EE, Opcode::Lbzux),
            (0x7C00_00AE, Opcode::Lbzx),
            (0xC800_0000, Opcode::Lfd),
            (0xCC00_0000, Opcode::Lfdu),
            (0x7C00_04EE, Opcode::Lfdux),
            (0x7C00_04AE, Opcode::Lfdx),
            (0xC000_0000, Opcode::Lfs),
            (0xC400_0000, Opcode::Lfsu),
            (0x7C00_046E, Opcode::Lfsux),
            (0x7C00_042E, Opcode::Lfsx),
            (0xA800_0000, Opcode::Lha),
            (0xAC00_0000, Opcode::Lhau),
            (0x7C00_02EE, Opcode::Lhaux),
            (0x7C00_02AE, Opcode::Lhax),
            (0x7C00_062C, Opcode::Lhbrx),
            (0xA000_0000, Opcode::Lhz),
            (0xA400_0000, Opcode::Lhzu),
            (0x7C00_026E, Opcode::Lhzux),
            (0x7C00_022E, Opcode::Lhzx),
            (0xB800_0000, Opcode::Lmw),
            (0x7C00_04AA, Opcode::Lswi),
            (0x7C00_042A, Opcode::Lswx),
            (0x7C00_0028, Opcode::Lwarx),
            (0x7C00_042C, Opcode::Lwbrx),
            (0x8000_0000, Opcode::Lwz),
            (0x8400_0000, Opcode::Lwzu),
            (0x7C00_006E, Opcode::Lwzux),
            (0x7C00_002E, Opcode::Lwzx),
            (0x4C00_0000, Opcode::Mcrf),
            (0xFC00_0080, Opcode::Mcrfs),
            (0x7c00_0400, Opcode::Mcrxr),
            (0x7C00_0026, Opcode::Mfcr),
            (0xFC00_048E, Opcode::Mffsx),
            (0x7C00_00A6, Opcode::Mfmsr),
            (0x7C00_02A6, Opcode::Mfspr),
            (0x7C00_04A6, Opcode::Mfsr),
            (0x7C00_0526, Opcode::Mfsrin),
            (0x7C00_02E6, Opcode::Mftb),
            (0x7C00_0120, Opcode::Mtcrf),
            (0xFC00_008C, Opcode::Mtfsb0x),
            (0xFC00_004C, Opcode::Mtfsb1x),
            (0xFC00_058E, Opcode::Mtfsfx),
            (0xFC00_010C, Opcode::Mtfsfix),
            (0x7C00_0124, Opcode::Mtmsr),
            (0x7C00_03A6, Opcode::Mtspr),
            (0x7C00_01A4, Opcode::Mtsr),
            (0x7C00_01E4, Opcode::Mtsrin),
            (0x7C00_0096, Opcode::Mulhwx),
            (0x7C00_0016, Opcode::Mulhwux),
            (0x1C00_0000, Opcode::Mulli),
            (0x7C00_01D6, Opcode::Mullwx),
            (0x7C00_03B8, Opcode::Nandx),
            (0x7C00_00D0, Opcode::Negx),
            (0x7C00_00F8, Opcode::Norx),
            (0x7C00_0378, Opcode::Orx),
            (0x7C00_0338, Opcode::Orcx),
            (0x6000_0000, Opcode::Ori),
            (0x6400_0000, Opcode::Oris),
            (0xE000_0000, Opcode::PsqL),
            (0xE400_0000, Opcode::PsqLu),
            (0x1000_004C, Opcode::PsqLux),
            (0x1000_000C, Opcode::PsqLx),
            (0xF000_0000, Opcode::PsqSt),
            (0xF400_0000, Opcode::PsqStu),
            (0x1000_004E, Opcode::PsqStux),
            (0x1000_000E, Opcode::PsqStx),
            (0x1000_0210, Opcode::PsAbsx),
            (0x1000_002A, Opcode::PsAddx),
            (0x1000_0040, Opcode::PsCmpo0),
            (0x1000_00C0, Opcode::PsCmpo1),
            (0x1000_0000, Opcode::PsCmpu0),
            (0x1000_0080, Opcode::PsCmpu1),
            (0x1000_0024, Opcode::PsDivx),
            (0x1000_003A, Opcode::PsMaddx),
            (0x1000_001C, Opcode::PsMadds0x),
            (0x1000_001E, Opcode::PsMadds1x),
            (0x1000_0420, Opcode::PsMerge00x),
            (0x1000_0460, Opcode::PsMerge01x),
            (0x1000_04A0, Opcode::PsMerge10x),
            (0x1000_04E0, Opcode::PsMerge11x),
            (0x1000_0090, Opcode::PsMrx),
            (0x1000_0038, Opcode::PsMsubx),
            (0x1000_0032, Opcode::PsMulx),
            (0x1000_0018, Opcode::PsMuls0x),
            (0x1000_001A, Opcode::PsMuls1x),
            (0x1000_0110, Opcode::PsNabsx),
            (0x1000_0050, Opcode::PsNegx),
            (0x1000_003E, Opcode::PsNmaddx),
            (0x1000_003C, Opcode::PsNmsubx),
            (0x1000_0030, Opcode::PsResx),
            (0x1000_0034, Opcode::PsRsqrtex),
            (0x1000_002E, Opcode::PsSelx),
            (0x1000_0028, Opcode::PsSubx),
            (0x1000_0014, Opcode::PsSum0x),
            (0x1000_0016, Opcode::PsSum1x),
            (0x4C00_0064, Opcode::Rfi),
            (0x5000_0000, Opcode::Rlwimix),
            (0x5400_0000, Opcode::Rlwinmx),
            (0x5C00_0000, Opcode::Rlwnmx),
            (0x4400_0002, Opcode::Sc),
            (0x7C00_0030, Opcode::Slwx),
            (0x7C00_0630, Opcode::Srawx),
            (0x7C00_0670, Opcode::Srawix),
            (0x7C00_0430, Opcode::Srwx),
            (0x9800_0000, Opcode::Stb),
            (0x9C00_0000, Opcode::Stbu),
            (0x7C00_01EE, Opcode::Stbux),
            (0x7C00_01AE, Opcode::Stbx),
            (0xD800_0000, Opcode::Stfd),
            (0xDC00_0000, Opcode::Stfdu),
            (0x7C00_05EE, Opcode::Stfdux),
            (0x7C00_05AE, Opcode::Stfdx),
            (0x7C00_07AE, Opcode::Stfiwx),
            (0xD000_0000, Opcode::Stfs),
            (0xD400_0000, Opcode::Stfsu),
            (0x7C00_056E, Opcode::Stfsux),
            (0x7C00_052E, Opcode::Stfsx),
            (0xB000_0000, Opcode::Sth),
            (0x7C00_072C, Opcode::Sthbrx),
            (0xB400_0000, Opcode::Sthu),
            (0x7C00_036E, Opcode::Sthux),
            (0x7C00_032E, Opcode::Sthx),
            (0xBC00_0000, Opcode::Stmw),
            (0x7C00_05AA, Opcode::Stswi),
            (0x7C00_052A, Opcode::Stswx),
            (0x9000_0000, Opcode::Stw),
            (0x7C00_052C, Opcode::Stwbrx),
            (0x7C00_012D, Opcode::Stwcxrc),
            (0x9400_0000, Opcode::Stwu),
            (0x7C00_016E, Opcode::Stwux),
            (0x7C00_012E, Opcode::Stwx),
            (0x7C00_0050, Opcode::Subfx),
            (0x7C00_0010, Opcode::Subfcx),
            (0x7C00_0110, Opcode::Subfex),
            (0x2000_0000, Opcode::Subfic),
            (0x7C00_01D0, Opcode::Subfmex),
            (0x7C00_0190, Opcode::Subfzex),
            (0x7C00_04AC, Opcode::Sync),
            (0x7C00_0264, Opcode::Tlbie),
            (0x7C00_046C, Opcode::Tlbsync),
            (0x7C00_0008, Opcode::Tw),
            (0x0C00_0000, Opcode::Twi),
            (0x7C00_0278, Opcode::Xorx),
            (0x6800_0000, Opcode::Xori),
            (0x6C00_0000, Opcode::Xoris),
        ];

        for i in data.iter() {
            let instr = Instruction(i.0);
            let opcode = optable[instr.opcd()];
            match opcode {
                Opcode::Table4 => assert_eq!(optable4[instr.xo_x()], i.1),
                Opcode::Table19 => assert_eq!(optable19[instr.xo_x()], i.1),
                Opcode::Table31 => assert_eq!(optable31[instr.xo_x()], i.1),
                Opcode::Table59 => assert_eq!(optable59[instr.xo_a()], i.1),
                Opcode::Table63 => assert_eq!(optable63[instr.xo_x()], i.1),
                _ => assert_eq!(opcode, i.1),
            }
        }
    }
}
