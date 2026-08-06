// Primary Opcodes
pub(crate) const OPCODE_TWI: u32 = 3;
pub(crate) const OPCODE_EXTENDED4: u32 = 4;
pub(crate) const OPCODE_MULLI: u32 = 7;
pub(crate) const OPCODE_SUBFIC: u32 = 8;
pub(crate) const OPCODE_CMPLI: u32 = 10;
pub(crate) const OPCODE_CMPI: u32 = 11;
pub(crate) const OPCODE_ADDIC: u32 = 12;
pub(crate) const OPCODE_ADDIC_RC: u32 = 13;
pub(crate) const OPCODE_ADDI: u32 = 14;
pub(crate) const OPCODE_ADDIS: u32 = 15;
pub(crate) const OPCODE_BCX: u32 = 16;
pub(crate) const OPCODE_SC: u32 = 17;
pub(crate) const OPCODE_BX: u32 = 18;
pub(crate) const OPCODE_EXTENDED19: u32 = 19;
pub(crate) const OPCODE_RLWIMIX: u32 = 20;
pub(crate) const OPCODE_RLWINMX: u32 = 21;
pub(crate) const OPCODE_RLWNMX: u32 = 23;
pub(crate) const OPCODE_ORI: u32 = 24;
pub(crate) const OPCODE_ORIS: u32 = 25;
pub(crate) const OPCODE_XORI: u32 = 26;
pub(crate) const OPCODE_XORIS: u32 = 27;
pub(crate) const OPCODE_ANDI_RC: u32 = 28;
pub(crate) const OPCODE_ANDIS_RC: u32 = 29;
pub(crate) const OPCODE_EXTENDED31: u32 = 31;
pub(crate) const OPCODE_LWZ: u32 = 32;
pub(crate) const OPCODE_LWZU: u32 = 33;
pub(crate) const OPCODE_LBZ: u32 = 34;
pub(crate) const OPCODE_LBZU: u32 = 35;
pub(crate) const OPCODE_STW: u32 = 36;
pub(crate) const OPCODE_STWU: u32 = 37;
pub(crate) const OPCODE_STB: u32 = 38;
pub(crate) const OPCODE_STBU: u32 = 39;
pub(crate) const OPCODE_LHZ: u32 = 40;
pub(crate) const OPCODE_LHZU: u32 = 41;
pub(crate) const OPCODE_LHA: u32 = 42;
pub(crate) const OPCODE_LHAU: u32 = 43;
pub(crate) const OPCODE_STH: u32 = 44;
pub(crate) const OPCODE_STHU: u32 = 45;
pub(crate) const OPCODE_LMW: u32 = 46;
pub(crate) const OPCODE_STMW: u32 = 47;
pub(crate) const OPCODE_LFS: u32 = 48;
pub(crate) const OPCODE_LFSU: u32 = 49;
pub(crate) const OPCODE_LFD: u32 = 50;
pub(crate) const OPCODE_LFDU: u32 = 51;
pub(crate) const OPCODE_STFS: u32 = 52;
pub(crate) const OPCODE_STFSU: u32 = 53;
pub(crate) const OPCODE_STFD: u32 = 54;
pub(crate) const OPCODE_STFDU: u32 = 55;
pub(crate) const OPCODE_PSQ_L: u32 = 56;
pub(crate) const OPCODE_PSQ_LU: u32 = 57;
pub(crate) const OPCODE_EXTENDED59: u32 = 59;
pub(crate) const OPCODE_PSQ_ST: u32 = 60;
pub(crate) const OPCODE_PSQ_STU: u32 = 61;
pub(crate) const OPCODE_EXTENDED63: u32 = 63;

// 4X Extended Opcodes
pub(crate) const OPCODE_PS_CMPU0: u32 = 0;
pub(crate) const OPCODE_PS_CMPO0: u32 = 32;
pub(crate) const OPCODE_PS_NEGX: u32 = 40;
pub(crate) const OPCODE_PS_CMPU1: u32 = 64;
pub(crate) const OPCODE_PS_MRX: u32 = 72;
pub(crate) const OPCODE_PS_CMPO1: u32 = 96;
pub(crate) const OPCODE_PS_NABSX: u32 = 136;
pub(crate) const OPCODE_PS_ABSX: u32 = 264;
pub(crate) const OPCODE_PS_MERGE_00X: u32 = 528;
pub(crate) const OPCODE_PS_MERGE_01X: u32 = 560;
pub(crate) const OPCODE_PS_MERGE_10X: u32 = 592;
pub(crate) const OPCODE_PS_MERGE_11X: u32 = 624;
pub(crate) const OPCODE_DCBZ_L: u32 = 1014;

// 4A Extended Opcodes
pub(crate) const OPCODE_PS_SUM0X: u32 = 10;
pub(crate) const OPCODE_PS_SUM1X: u32 = 11;
pub(crate) const OPCODE_PS_MULS0X: u32 = 12;
pub(crate) const OPCODE_PS_MULS1X: u32 = 13;
pub(crate) const OPCODE_PS_MADDS0X: u32 = 14;
pub(crate) const OPCODE_PS_MADDS1X: u32 = 15;
pub(crate) const OPCODE_PS_DIVX: u32 = 18;
pub(crate) const OPCODE_PS_SUBX: u32 = 20;
pub(crate) const OPCODE_PS_ADDX: u32 = 21;
pub(crate) const OPCODE_PS_SELX: u32 = 23;
pub(crate) const OPCODE_PS_RESX: u32 = 24;
pub(crate) const OPCODE_PS_MULX: u32 = 25;
pub(crate) const OPCODE_PS_RSQRTEX: u32 = 26;
pub(crate) const OPCODE_PS_MSUBX: u32 = 28;
pub(crate) const OPCODE_PS_MADDX: u32 = 29;
pub(crate) const OPCODE_PS_NMSUBX: u32 = 30;
pub(crate) const OPCODE_PS_NMADDX: u32 = 31;

// 4AA Extended Opcodes
pub(crate) const OPCODE_PSQ_LX: u32 = 6;
pub(crate) const OPCODE_PSQ_STX: u32 = 7;
pub(crate) const OPCODE_PSQ_LUX: u32 = 38;
pub(crate) const OPCODE_PSQ_STUX: u32 = 39;

// 19 Extended Opcodes
pub(crate) const OPCODE_MCRF: u32 = 0;
pub(crate) const OPCODE_BCLRX: u32 = 16;
pub(crate) const OPCODE_CRNOR: u32 = 33;
pub(crate) const OPCODE_RFI: u32 = 50;
pub(crate) const OPCODE_CRANDC: u32 = 129;
pub(crate) const OPCODE_ISYNC: u32 = 150;
pub(crate) const OPCODE_CRXOR: u32 = 193;
pub(crate) const OPCODE_CRNAND: u32 = 225;
pub(crate) const OPCODE_CRAND: u32 = 257;
pub(crate) const OPCODE_CREQV: u32 = 289;
pub(crate) const OPCODE_CRORC: u32 = 417;
pub(crate) const OPCODE_CROR: u32 = 449;
pub(crate) const OPCODE_BCCTRX: u32 = 528;

// 31 Extended Opcodes
pub(crate) const OPCODE_CMP: u32 = 0;
pub(crate) const OPCODE_TW: u32 = 4;
pub(crate) const OPCODE_SUBFCX: u32 = 8;
pub(crate) const OPCODE_ADDCX: u32 = 10;
pub(crate) const OPCODE_MULHWUX: u32 = 11;
pub(crate) const OPCODE_MFCR: u32 = 19;
pub(crate) const OPCODE_LWARX: u32 = 20;
pub(crate) const OPCODE_LWZX: u32 = 23;
pub(crate) const OPCODE_SLWX: u32 = 24;
pub(crate) const OPCODE_CNTLZWX: u32 = 26;
pub(crate) const OPCODE_ANDX: u32 = 28;
pub(crate) const OPCODE_CMPL: u32 = 32;
pub(crate) const OPCODE_SUBFX: u32 = 40;
pub(crate) const OPCODE_DCBST: u32 = 54;
pub(crate) const OPCODE_LWZUX: u32 = 55;
pub(crate) const OPCODE_ANDCX: u32 = 60;
pub(crate) const OPCODE_MULHWX: u32 = 75;
pub(crate) const OPCODE_MFMSR: u32 = 83;
pub(crate) const OPCODE_DCBF: u32 = 86;
pub(crate) const OPCODE_LBZX: u32 = 87;
pub(crate) const OPCODE_NEGX: u32 = 104;
pub(crate) const OPCODE_LBZUX: u32 = 119;
pub(crate) const OPCODE_NORX: u32 = 124;
pub(crate) const OPCODE_SUBFEX: u32 = 136;
pub(crate) const OPCODE_ADDEX: u32 = 138;
pub(crate) const OPCODE_MTCRF: u32 = 144;
pub(crate) const OPCODE_MTMSR: u32 = 146;
pub(crate) const OPCODE_STWCX_RC: u32 = 150;
pub(crate) const OPCODE_STWX: u32 = 151;
pub(crate) const OPCODE_STWUX: u32 = 183;
pub(crate) const OPCODE_SUBFZEX: u32 = 200;
pub(crate) const OPCODE_ADDZEX: u32 = 202;
pub(crate) const OPCODE_MTSR: u32 = 210;
pub(crate) const OPCODE_STBX: u32 = 215;
pub(crate) const OPCODE_SUBFMEX: u32 = 232;
pub(crate) const OPCODE_ADDMEX: u32 = 234;
pub(crate) const OPCODE_MULLWX: u32 = 235;
pub(crate) const OPCODE_MTSRIN: u32 = 242;
pub(crate) const OPCODE_DCBTST: u32 = 246;
pub(crate) const OPCODE_STBUX: u32 = 247;
pub(crate) const OPCODE_ADDX: u32 = 266;
pub(crate) const OPCODE_DCBT: u32 = 278;
pub(crate) const OPCODE_LHZX: u32 = 279;
pub(crate) const OPCODE_EQVX: u32 = 284;
pub(crate) const OPCODE_TBLIE: u32 = 306;
pub(crate) const OPCODE_ECIWX: u32 = 310;
pub(crate) const OPCODE_LHZUX: u32 = 311;
pub(crate) const OPCODE_XORX: u32 = 316;
pub(crate) const OPCODE_MFSPR: u32 = 339;
pub(crate) const OPCODE_LHAX: u32 = 343;
pub(crate) const OPCODE_MFTB: u32 = 371;
pub(crate) const OPCODE_LHAUX: u32 = 375;
pub(crate) const OPCODE_STHX: u32 = 407;
pub(crate) const OPCODE_ORCX: u32 = 412;
pub(crate) const OPCODE_ECOWX: u32 = 438;
pub(crate) const OPCODE_STHUX: u32 = 439;
pub(crate) const OPCODE_ORX: u32 = 444;
pub(crate) const OPCODE_DIVWUX: u32 = 459;
pub(crate) const OPCODE_MTSPR: u32 = 467;
pub(crate) const OPCODE_DCBI: u32 = 470;
pub(crate) const OPCODE_NANDX: u32 = 476;
pub(crate) const OPCODE_DIVWX: u32 = 491;
pub(crate) const OPCODE_MCRXR: u32 = 512;
pub(crate) const OPCODE_SUBFCX_OE: u32 = 520;
pub(crate) const OPCODE_ADDCX_OE: u32 = 522;
pub(crate) const OPCODE_MULHWUX_21: u32 = 523;
pub(crate) const OPCODE_LSWX: u32 = 533;
pub(crate) const OPCODE_LWBRX: u32 = 534;
pub(crate) const OPCODE_LFSX: u32 = 535;
pub(crate) const OPCODE_SRWX: u32 = 536;
pub(crate) const OPCODE_SUBFX_OE: u32 = 552;
pub(crate) const OPCODE_TLBSYNC: u32 = 566;
pub(crate) const OPCODE_LFSUX: u32 = 567;
pub(crate) const OPCODE_MULHWX_21: u32 = 587;
pub(crate) const OPCODE_MFSR: u32 = 595;
pub(crate) const OPCODE_LSWI: u32 = 597;
pub(crate) const OPCODE_SYNC: u32 = 598;
pub(crate) const OPCODE_LFDX: u32 = 599;
pub(crate) const OPCODE_NEGX_OE: u32 = 616;
pub(crate) const OPCODE_LFDUX: u32 = 631;
pub(crate) const OPCODE_SUBFEX_OE: u32 = 648;
pub(crate) const OPCODE_ADDEX_OE: u32 = 650;
pub(crate) const OPCODE_MFSRIN: u32 = 659;
pub(crate) const OPCODE_STSWX: u32 = 661;
pub(crate) const OPCODE_STWBRX: u32 = 662;
pub(crate) const OPCODE_STFSX: u32 = 663;
pub(crate) const OPCODE_STFSUX: u32 = 695;
pub(crate) const OPCODE_SUBFZEX_OE: u32 = 712;
pub(crate) const OPCODE_ADDZEX_OE: u32 = 714;
pub(crate) const OPCODE_STSWI: u32 = 725;
pub(crate) const OPCODE_STFDX: u32 = 727;
pub(crate) const OPCODE_SUBFMEX_OE: u32 = 744;
pub(crate) const OPCODE_ADDMEX_OE: u32 = 746;
pub(crate) const OPCODE_MULLWX_OE: u32 = 747;
pub(crate) const OPCODE_STFDUX: u32 = 759;
pub(crate) const OPCODE_ADDX_OE: u32 = 778;
pub(crate) const OPCODE_LHBRX: u32 = 790;
pub(crate) const OPCODE_SRAWX: u32 = 792;
pub(crate) const OPCODE_SRAWIX: u32 = 824;
pub(crate) const OPCODE_EIEIO: u32 = 854;
pub(crate) const OPCODE_STHBRX: u32 = 918;
pub(crate) const OPCODE_EXTSHX: u32 = 922;
pub(crate) const OPCODE_EXTSBX: u32 = 954;
pub(crate) const OPCODE_DIVWUX_OE: u32 = 971;
pub(crate) const OPCODE_ICBI: u32 = 982;
pub(crate) const OPCODE_STFIWX: u32 = 983;
pub(crate) const OPCODE_DIVWX_OE: u32 = 1003;
pub(crate) const OPCODE_DCBZ: u32 = 1014;

// 59 Extended Opcodes
pub(crate) const OPCODE_FDIVSX: u32 = 18;
pub(crate) const OPCODE_FSUBSX: u32 = 20;
pub(crate) const OPCODE_FADDSX: u32 = 21;
pub(crate) const OPCODE_FRESX: u32 = 24;
pub(crate) const OPCODE_FMULSX: u32 = 25;
pub(crate) const OPCODE_FMSUBSX: u32 = 28;
pub(crate) const OPCODE_FMADDSX: u32 = 29;
pub(crate) const OPCODE_FNMSUBSX: u32 = 30;
pub(crate) const OPCODE_FNMADDSX: u32 = 31;

// 63X Extended Opcodes
pub(crate) const OPCODE_FCMPU: u32 = 0;
pub(crate) const OPCODE_FRSPX: u32 = 12;
pub(crate) const OPCODE_FCTIWX: u32 = 14;
pub(crate) const OPCODE_FCTIWZX: u32 = 15;
pub(crate) const OPCODE_FCMPO: u32 = 32;
pub(crate) const OPCODE_MTFSB1X: u32 = 38;
pub(crate) const OPCODE_FNEGX: u32 = 40;
pub(crate) const OPCODE_MCRFS: u32 = 64;
pub(crate) const OPCODE_MTFSB0X: u32 = 70;
pub(crate) const OPCODE_FMRX: u32 = 72;
pub(crate) const OPCODE_MTFSFIX: u32 = 134;
pub(crate) const OPCODE_FNABSX: u32 = 136;
pub(crate) const OPCODE_FABSX: u32 = 264;
pub(crate) const OPCODE_MFFSX: u32 = 583;
pub(crate) const OPCODE_MTFSFX: u32 = 711;

// 63A Extended Opcodes
pub(crate) const OPCODE_FDIVX: u32 = 18;
pub(crate) const OPCODE_FSUBX: u32 = 20;
pub(crate) const OPCODE_FADDX: u32 = 21;
pub(crate) const OPCODE_FSELX: u32 = 23;
pub(crate) const OPCODE_FMULX: u32 = 25;
pub(crate) const OPCODE_FRSQRTEX: u32 = 26;
pub(crate) const OPCODE_FMSUBX: u32 = 28;
pub(crate) const OPCODE_FMADDX: u32 = 29;
pub(crate) const OPCODE_FNMSUBX: u32 = 30;
pub(crate) const OPCODE_FNMADDX: u32 = 31;

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
