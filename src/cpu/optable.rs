use super::{instruction::Instruction, opcodes::*, Cpu};
use crate::bus::Bus;

pub(crate) use super::opcodes::Opcode;

pub(crate) const OPTABLE_SIZE: usize = 64;
pub(crate) const OPTABLE4_SIZE: usize = 1024;
pub(crate) const OPTABLE19_SIZE: usize = 1024;
pub(crate) const OPTABLE31_SIZE: usize = 1024;
pub(crate) const OPTABLE59_SIZE: usize = 32;
pub(crate) const OPTABLE63_SIZE: usize = 1024;

pub(crate) type OpFn = fn(&mut Cpu, Instruction, &mut Bus);
pub(crate) type OpcodeTableItem = (u32, Opcode, OpFn);

pub(crate) const ILLEGAL_OP: (Opcode, OpFn) = (Opcode::Illegal, op_illegal);

fn op_illegal(_: &mut Cpu, _instr: Instruction, _: &mut Bus) {}

fn op_subtable4(cpu: &mut Cpu, instr: Instruction, bus: &mut Bus) {
    OPTABLE4[instr.xo_x()](cpu, instr, bus);
}

fn op_subtable19(cpu: &mut Cpu, instr: Instruction, bus: &mut Bus) {
    OPTABLE19[instr.xo_x()](cpu, instr, bus);
}

fn op_subtable31(cpu: &mut Cpu, instr: Instruction, bus: &mut Bus) {
    OPTABLE31[instr.xo_x()](cpu, instr, bus);
}

fn op_subtable59(cpu: &mut Cpu, instr: Instruction, bus: &mut Bus) {
    OPTABLE59[instr.xo_a()](cpu, instr, bus);
}

fn op_subtable63(cpu: &mut Cpu, instr: Instruction, bus: &mut Bus) {
    OPTABLE63[instr.xo_x()](cpu, instr, bus);
}

pub(crate) const OPCODE_TABLE: [OpcodeTableItem; 54] = [
    (OPCODE_TWI, Opcode::Twi, Cpu::op_twi),
    (OPCODE_EXTENDED4, Opcode::Table4, op_subtable4),
    (OPCODE_MULLI, Opcode::Mulli, Cpu::op_mulli),
    (OPCODE_SUBFIC, Opcode::Subfic, Cpu::op_subfic),
    (OPCODE_CMPLI, Opcode::Cmpli, Cpu::op_cmpli),
    (OPCODE_CMPI, Opcode::Cmpi, Cpu::op_cmpi),
    (OPCODE_ADDIC, Opcode::Addic, Cpu::op_addic),
    (OPCODE_ADDIC_RC, Opcode::Addicrc, Cpu::op_addic_rc),
    (OPCODE_ADDI, Opcode::Addi, Cpu::op_addi),
    (OPCODE_ADDIS, Opcode::Addis, Cpu::op_addis),
    (OPCODE_BCX, Opcode::Bcx, Cpu::op_bcx),
    (OPCODE_SC, Opcode::Sc, Cpu::op_sc),
    (OPCODE_BX, Opcode::Bx, Cpu::op_bx),
    (OPCODE_EXTENDED19, Opcode::Table19, op_subtable19),
    (OPCODE_RLWIMIX, Opcode::Rlwimix, Cpu::op_rlwimix),
    (OPCODE_RLWINMX, Opcode::Rlwinmx, Cpu::op_rlwinmx),
    (OPCODE_RLWNMX, Opcode::Rlwnmx, Cpu::op_rlwnmx),
    (OPCODE_ORI, Opcode::Ori, Cpu::op_ori),
    (OPCODE_ORIS, Opcode::Oris, Cpu::op_oris),
    (OPCODE_XORI, Opcode::Xori, Cpu::op_xori),
    (OPCODE_XORIS, Opcode::Xoris, Cpu::op_xoris),
    (OPCODE_ANDI_RC, Opcode::Andirc, Cpu::op_andi_rc),
    (OPCODE_ANDIS_RC, Opcode::Andisrc, Cpu::op_andis_rc),
    (OPCODE_EXTENDED31, Opcode::Table31, op_subtable31),
    (OPCODE_LWZ, Opcode::Lwz, Cpu::op_lwz),
    (OPCODE_LWZU, Opcode::Lwzu, Cpu::op_lwzu),
    (OPCODE_LBZ, Opcode::Lbz, Cpu::op_lbz),
    (OPCODE_LBZU, Opcode::Lbzu, Cpu::op_lbzu),
    (OPCODE_STW, Opcode::Stw, Cpu::op_stw),
    (OPCODE_STWU, Opcode::Stwu, Cpu::op_stwu),
    (OPCODE_STB, Opcode::Stb, Cpu::op_stb),
    (OPCODE_STBU, Opcode::Stbu, Cpu::op_stbu),
    (OPCODE_LHZ, Opcode::Lhz, Cpu::op_lhz),
    (OPCODE_LHZU, Opcode::Lhzu, Cpu::op_lhzu),
    (OPCODE_LHA, Opcode::Lha, Cpu::op_lha),
    (OPCODE_LHAU, Opcode::Lhau, Cpu::op_lhau),
    (OPCODE_STH, Opcode::Sth, Cpu::op_sth),
    (OPCODE_STHU, Opcode::Sthu, Cpu::op_sthu),
    (OPCODE_LMW, Opcode::Lmw, Cpu::op_lmw),
    (OPCODE_STMW, Opcode::Stmw, Cpu::op_stmw),
    (OPCODE_LFS, Opcode::Lfs, Cpu::op_lfs),
    (OPCODE_LFSU, Opcode::Lfsu, Cpu::op_lfsu),
    (OPCODE_LFD, Opcode::Lfd, Cpu::op_lfd),
    (OPCODE_LFDU, Opcode::Lfdu, Cpu::op_lfdu),
    (OPCODE_STFS, Opcode::Stfs, Cpu::op_stfs),
    (OPCODE_STFSU, Opcode::Stfsu, Cpu::op_stfsu),
    (OPCODE_STFD, Opcode::Stfd, Cpu::op_stfd),
    (OPCODE_STFDU, Opcode::Stfdu, Cpu::op_stfdu),
    (OPCODE_PSQ_L, Opcode::PsqL, Cpu::op_psq_l),
    (OPCODE_PSQ_LU, Opcode::PsqLu, Cpu::op_psq_lu),
    (OPCODE_EXTENDED59, Opcode::Table59, op_subtable59),
    (OPCODE_PSQ_ST, Opcode::PsqSt, Cpu::op_psq_st),
    (OPCODE_PSQ_STU, Opcode::PsqStu, Cpu::op_psq_stu),
    (OPCODE_EXTENDED63, Opcode::Table63, op_subtable63),
];

pub(crate) const OPCODE4X_TABLE: [OpcodeTableItem; 13] = [
    (OPCODE_PS_CMPU0, Opcode::PsCmpu0, Cpu::op_ps_cmpu0),
    (OPCODE_PS_CMPO0, Opcode::PsCmpo0, Cpu::op_ps_cmpo0),
    (OPCODE_PS_NEGX, Opcode::PsNegx, Cpu::op_ps_negx),
    (OPCODE_PS_CMPU1, Opcode::PsCmpu1, Cpu::op_ps_cmpu1),
    (OPCODE_PS_MRX, Opcode::PsMrx, Cpu::op_ps_mrx),
    (OPCODE_PS_CMPO1, Opcode::PsCmpo1, Cpu::op_ps_cmpo1),
    (OPCODE_PS_NABSX, Opcode::PsNabsx, Cpu::op_ps_nabsx),
    (OPCODE_PS_ABSX, Opcode::PsAbsx, Cpu::op_ps_absx),
    (OPCODE_PS_MERGE_00X, Opcode::PsMerge00x, Cpu::op_ps_merge00x),
    (OPCODE_PS_MERGE_01X, Opcode::PsMerge01x, Cpu::op_ps_merge01x),
    (OPCODE_PS_MERGE_10X, Opcode::PsMerge10x, Cpu::op_ps_merge10x),
    (OPCODE_PS_MERGE_11X, Opcode::PsMerge11x, Cpu::op_ps_merge11x),
    (OPCODE_DCBZ_L, Opcode::DcbzL, Cpu::op_dcbz_l),
];

pub(crate) const OPCODE4A_TABLE: [OpcodeTableItem; 17] = [
    (OPCODE_PS_SUM0X, Opcode::PsSum0x, Cpu::op_ps_sum0x),
    (OPCODE_PS_SUM1X, Opcode::PsSum1x, Cpu::op_ps_sum1x),
    (OPCODE_PS_MULS0X, Opcode::PsMuls0x, Cpu::op_ps_muls0x),
    (OPCODE_PS_MULS1X, Opcode::PsMuls1x, Cpu::op_ps_muls1x),
    (OPCODE_PS_MADDS0X, Opcode::PsMadds0x, Cpu::op_ps_madds0x),
    (OPCODE_PS_MADDS1X, Opcode::PsMadds1x, Cpu::op_ps_madds1x),
    (OPCODE_PS_DIVX, Opcode::PsDivx, Cpu::op_ps_divx),
    (OPCODE_PS_SUBX, Opcode::PsSubx, Cpu::op_ps_subx),
    (OPCODE_PS_ADDX, Opcode::PsAddx, Cpu::op_ps_addx),
    (OPCODE_PS_SELX, Opcode::PsSelx, Cpu::op_ps_selx),
    (OPCODE_PS_RESX, Opcode::PsResx, Cpu::op_ps_resx),
    (OPCODE_PS_MULX, Opcode::PsMulx, Cpu::op_ps_mulx),
    (OPCODE_PS_RSQRTEX, Opcode::PsRsqrtex, Cpu::op_ps_rsqrtex),
    (OPCODE_PS_MSUBX, Opcode::PsMsubx, Cpu::op_ps_msubx),
    (OPCODE_PS_MADDX, Opcode::PsMaddx, Cpu::op_ps_maddx),
    (OPCODE_PS_NMSUBX, Opcode::PsNmsubx, Cpu::op_ps_nmsubx),
    (OPCODE_PS_NMADDX, Opcode::PsNmaddx, Cpu::op_ps_nmaddx),
];

pub(crate) const OPCODE4AA_TABLE: [OpcodeTableItem; 4] = [
    (OPCODE_PSQ_LX, Opcode::PsqLx, Cpu::op_psq_lx),
    (OPCODE_PSQ_STX, Opcode::PsqStx, Cpu::op_psq_stx),
    (OPCODE_PSQ_LUX, Opcode::PsqLux, Cpu::op_psq_lux),
    (OPCODE_PSQ_STUX, Opcode::PsqStux, Cpu::op_psq_stux),
];

pub(crate) const OPCODE19_TABLE: [OpcodeTableItem; 13] = [
    (OPCODE_MCRF, Opcode::Mcrf, Cpu::op_mcrf),
    (OPCODE_BCLRX, Opcode::Bclrx, Cpu::op_bclrx),
    (OPCODE_CRNOR, Opcode::Crnor, Cpu::op_crnor),
    (OPCODE_RFI, Opcode::Rfi, Cpu::op_rfi),
    (OPCODE_CRANDC, Opcode::Crandc, Cpu::op_crandc),
    (OPCODE_ISYNC, Opcode::Isync, Cpu::op_isync),
    (OPCODE_CRXOR, Opcode::Crxor, Cpu::op_crxor),
    (OPCODE_CRNAND, Opcode::Crnand, Cpu::op_crnand),
    (OPCODE_CRAND, Opcode::Crand, Cpu::op_crand),
    (OPCODE_CREQV, Opcode::Creqv, Cpu::op_creqv),
    (OPCODE_CRORC, Opcode::Crorc, Cpu::op_crorc),
    (OPCODE_CROR, Opcode::Cror, Cpu::op_cror),
    (OPCODE_BCCTRX, Opcode::Bcctrx, Cpu::op_bcctrx),
];

pub(crate) const OPCODE31_TABLE: [OpcodeTableItem; 108] = [
    (OPCODE_CMP, Opcode::Cmp, Cpu::op_cmp),
    (OPCODE_TW, Opcode::Tw, Cpu::op_tw),
    (OPCODE_SUBFCX, Opcode::Subfcx, Cpu::op_subfcx),
    (OPCODE_ADDCX, Opcode::Addcx, Cpu::op_addcx),
    (OPCODE_MULHWUX, Opcode::Mulhwux, Cpu::op_mulhwux),
    (OPCODE_MFCR, Opcode::Mfcr, Cpu::op_mfcr),
    (OPCODE_LWARX, Opcode::Lwarx, Cpu::op_lwarx),
    (OPCODE_LWZX, Opcode::Lwzx, Cpu::op_lwzx),
    (OPCODE_SLWX, Opcode::Slwx, Cpu::op_slwx),
    (OPCODE_CNTLZWX, Opcode::Cntlzwx, Cpu::op_cntlzwx),
    (OPCODE_ANDX, Opcode::Andx, Cpu::op_andx),
    (OPCODE_CMPL, Opcode::Cmpl, Cpu::op_cmpl),
    (OPCODE_SUBFX, Opcode::Subfx, Cpu::op_subfx),
    (OPCODE_DCBST, Opcode::Dcbst, Cpu::op_dcbst),
    (OPCODE_LWZUX, Opcode::Lwzux, Cpu::op_lwzux),
    (OPCODE_ANDCX, Opcode::Andcx, Cpu::op_andcx),
    (OPCODE_MULHWX, Opcode::Mulhwx, Cpu::op_mulhwx),
    (OPCODE_MFMSR, Opcode::Mfmsr, Cpu::op_mfmsr),
    (OPCODE_DCBF, Opcode::Dcbf, Cpu::op_dcbf),
    (OPCODE_LBZX, Opcode::Lbzx, Cpu::op_lbzx),
    (OPCODE_NEGX, Opcode::Negx, Cpu::op_negx),
    (OPCODE_LBZUX, Opcode::Lbzux, Cpu::op_lbzux),
    (OPCODE_NORX, Opcode::Norx, Cpu::op_norx),
    (OPCODE_SUBFEX, Opcode::Subfex, Cpu::op_subfex),
    (OPCODE_ADDEX, Opcode::Addex, Cpu::op_addex),
    (OPCODE_MTCRF, Opcode::Mtcrf, Cpu::op_mtcrf),
    (OPCODE_MTMSR, Opcode::Mtmsr, Cpu::op_mtmsr),
    (OPCODE_STWCX_RC, Opcode::Stwcxrc, Cpu::op_stwcx_rc),
    (OPCODE_STWX, Opcode::Stwx, Cpu::op_stwx),
    (OPCODE_STWUX, Opcode::Stwux, Cpu::op_stwux),
    (OPCODE_SUBFZEX, Opcode::Subfzex, Cpu::op_subfzex),
    (OPCODE_ADDZEX, Opcode::Addzex, Cpu::op_addzex),
    (OPCODE_MTSR, Opcode::Mtsr, Cpu::op_mtsr),
    (OPCODE_STBX, Opcode::Stbx, Cpu::op_stbx),
    (OPCODE_SUBFMEX, Opcode::Subfmex, Cpu::op_subfmex),
    (OPCODE_ADDMEX, Opcode::Addmex, Cpu::op_addmex),
    (OPCODE_MULLWX, Opcode::Mullwx, Cpu::op_mullwx),
    (OPCODE_MTSRIN, Opcode::Mtsrin, Cpu::op_mtsrin),
    (OPCODE_DCBTST, Opcode::Dcbtst, Cpu::op_dcbtst),
    (OPCODE_STBUX, Opcode::Stbux, Cpu::op_stbux),
    (OPCODE_ADDX, Opcode::Addx, Cpu::op_addx),
    (OPCODE_DCBT, Opcode::Dcbt, Cpu::op_dcbt),
    (OPCODE_LHZX, Opcode::Lhzx, Cpu::op_lhzx),
    (OPCODE_EQVX, Opcode::Eqvx, Cpu::op_eqvx),
    (OPCODE_TBLIE, Opcode::Tlbie, Cpu::op_tlbie),
    (OPCODE_ECIWX, Opcode::Eciwx, Cpu::op_eciwx),
    (OPCODE_LHZUX, Opcode::Lhzux, Cpu::op_lhzux),
    (OPCODE_XORX, Opcode::Xorx, Cpu::op_xorx),
    (OPCODE_MFSPR, Opcode::Mfspr, Cpu::op_mfspr),
    (OPCODE_LHAX, Opcode::Lhax, Cpu::op_lhax),
    (OPCODE_MFTB, Opcode::Mftb, Cpu::op_mftb),
    (OPCODE_LHAUX, Opcode::Lhaux, Cpu::op_lhaux),
    (OPCODE_STHX, Opcode::Sthx, Cpu::op_sthx),
    (OPCODE_ORCX, Opcode::Orcx, Cpu::op_orcx),
    (OPCODE_ECOWX, Opcode::Ecowx, Cpu::op_ecowx),
    (OPCODE_STHUX, Opcode::Sthux, Cpu::op_sthux),
    (OPCODE_ORX, Opcode::Orx, Cpu::op_orx),
    (OPCODE_DIVWUX, Opcode::Divwux, Cpu::op_divwux),
    (OPCODE_MTSPR, Opcode::Mtspr, Cpu::op_mtspr),
    (OPCODE_DCBI, Opcode::Dcbi, Cpu::op_dcbi),
    (OPCODE_NANDX, Opcode::Nandx, Cpu::op_nandx),
    (OPCODE_DIVWX, Opcode::Divwx, Cpu::op_divwx),
    (OPCODE_MCRXR, Opcode::Mcrxr, Cpu::op_mcrxr),
    (OPCODE_SUBFCX_OE, Opcode::Subfcx, Cpu::op_subfcx), // oe = 1
    (OPCODE_ADDCX_OE, Opcode::Addcx, Cpu::op_addcx),    // oe = 1
    (OPCODE_MULHWUX_21, Opcode::Mulhwux, Cpu::op_mulhwux), // 21(reserved) = 1
    (OPCODE_LSWX, Opcode::Lswx, Cpu::op_lswx),
    (OPCODE_LWBRX, Opcode::Lwbrx, Cpu::op_lwbrx),
    (OPCODE_LFSX, Opcode::Lfsx, Cpu::op_lfsx),
    (OPCODE_SRWX, Opcode::Srwx, Cpu::op_srwx),
    (OPCODE_SUBFX_OE, Opcode::Subfx, Cpu::op_subfx), // oe = 1
    (OPCODE_TLBSYNC, Opcode::Tlbsync, Cpu::op_tlbsync),
    (OPCODE_LFSUX, Opcode::Lfsux, Cpu::op_lfsux),
    (OPCODE_MULHWX_21, Opcode::Mulhwx, Cpu::op_mulhwx), // 21(reserved) = 1
    (OPCODE_MFSR, Opcode::Mfsr, Cpu::op_mfsr),
    (OPCODE_LSWI, Opcode::Lswi, Cpu::op_lswi),
    (OPCODE_SYNC, Opcode::Sync, Cpu::op_sync),
    (OPCODE_LFDX, Opcode::Lfdx, Cpu::op_lfdx),
    (OPCODE_NEGX_OE, Opcode::Negx, Cpu::op_negx), // oe = 1
    (OPCODE_LFDUX, Opcode::Lfdux, Cpu::op_lfdux),
    (OPCODE_SUBFEX_OE, Opcode::Subfex, Cpu::op_subfex), // oe = 1
    (OPCODE_ADDEX_OE, Opcode::Addex, Cpu::op_addex),    // oe = 1
    (OPCODE_MFSRIN, Opcode::Mfsrin, Cpu::op_mfsrin),
    (OPCODE_STSWX, Opcode::Stswx, Cpu::op_stswx),
    (OPCODE_STWBRX, Opcode::Stwbrx, Cpu::op_stwbrx),
    (OPCODE_STFSX, Opcode::Stfsx, Cpu::op_stfsx),
    (OPCODE_STFSUX, Opcode::Stfsux, Cpu::op_stfsux),
    (OPCODE_SUBFZEX_OE, Opcode::Subfzex, Cpu::op_subfzex), // oe = 1
    (OPCODE_ADDZEX_OE, Opcode::Addzex, Cpu::op_addzex),    // oe = 1
    (OPCODE_STSWI, Opcode::Stswi, Cpu::op_stswi),
    (OPCODE_STFDX, Opcode::Stfdx, Cpu::op_stfdx),
    (OPCODE_SUBFMEX_OE, Opcode::Subfmex, Cpu::op_subfmex), // oe = 1
    (OPCODE_ADDMEX_OE, Opcode::Addmex, Cpu::op_addmex),    // oe = 1
    (OPCODE_MULLWX_OE, Opcode::Mullwx, Cpu::op_mullwx),    // oe = 1
    (OPCODE_STFDUX, Opcode::Stfdux, Cpu::op_stfdux),
    (OPCODE_ADDX_OE, Opcode::Addx, Cpu::op_addx), // oe = 1
    (OPCODE_LHBRX, Opcode::Lhbrx, Cpu::op_lhbrx),
    (OPCODE_SRAWX, Opcode::Srawx, Cpu::op_srawx),
    (OPCODE_SRAWIX, Opcode::Srawix, Cpu::op_srawix),
    (OPCODE_EIEIO, Opcode::Eieio, Cpu::op_eieio),
    (OPCODE_STHBRX, Opcode::Sthbrx, Cpu::op_sthbrx),
    (OPCODE_EXTSHX, Opcode::Extshx, Cpu::op_extshx),
    (OPCODE_EXTSBX, Opcode::Extsbx, Cpu::op_extsbx),
    (OPCODE_DIVWUX_OE, Opcode::Divwux, Cpu::op_divwux), // oe = 1
    (OPCODE_ICBI, Opcode::Icbi, Cpu::op_icbi),
    (OPCODE_STFIWX, Opcode::Stfiwx, Cpu::op_stfiwx),
    (OPCODE_DIVWX_OE, Opcode::Divwx, Cpu::op_divwx), // oe = 1
    (OPCODE_DCBZ, Opcode::Dcbz, Cpu::op_dcbz),
];

pub(crate) const OPCODE59_TABLE: [OpcodeTableItem; 9] = [
    (OPCODE_FDIVSX, Opcode::Fdivsx, Cpu::op_fdivsx),
    (OPCODE_FSUBSX, Opcode::Fsubsx, Cpu::op_fsubsx),
    (OPCODE_FADDSX, Opcode::Faddsx, Cpu::op_faddsx),
    (OPCODE_FRESX, Opcode::Fresx, Cpu::op_fresx),
    (OPCODE_FMULSX, Opcode::Fmulsx, Cpu::op_fmulsx),
    (OPCODE_FMSUBSX, Opcode::Fmsubsx, Cpu::op_fmsubsx),
    (OPCODE_FMADDSX, Opcode::Fmaddsx, Cpu::op_fmaddsx),
    (OPCODE_FNMSUBSX, Opcode::Fnmsubsx, Cpu::op_fnmsubsx),
    (OPCODE_FNMADDSX, Opcode::Fnmaddsx, Cpu::op_fnmaddsx),
];

pub(crate) const OPCODE63X_TABLE: [OpcodeTableItem; 15] = [
    (OPCODE_FCMPU, Opcode::Fcmpu, Cpu::op_fcmpu),
    (OPCODE_FRSPX, Opcode::Frspx, Cpu::op_frspx),
    (OPCODE_FCTIWX, Opcode::Fctiwx, Cpu::op_fctiwx),
    (OPCODE_FCTIWZX, Opcode::Fctiwzx, Cpu::op_fctiwzx),
    (OPCODE_FCMPO, Opcode::Fcmpo, Cpu::op_fcmpo),
    (OPCODE_MTFSB1X, Opcode::Mtfsb1x, Cpu::op_mtfsb1x),
    (OPCODE_FNEGX, Opcode::Fnegx, Cpu::op_fnegx),
    (OPCODE_MCRFS, Opcode::Mcrfs, Cpu::op_mcrfs),
    (OPCODE_MTFSB0X, Opcode::Mtfsb0x, Cpu::op_mtfsb0x),
    (OPCODE_FMRX, Opcode::Fmrx, Cpu::op_fmrx),
    (OPCODE_MTFSFIX, Opcode::Mtfsfix, Cpu::op_mtfsfix),
    (OPCODE_FNABSX, Opcode::Fnabsx, Cpu::op_fnabsx),
    (OPCODE_FABSX, Opcode::Fabsx, Cpu::op_fabsx),
    (OPCODE_MFFSX, Opcode::Mffsx, Cpu::op_mffsx),
    (OPCODE_MTFSFX, Opcode::Mtfsfx, Cpu::op_mtfsfx),
];

pub(crate) const OPCODE63A_TABLE: [OpcodeTableItem; 10] = [
    (OPCODE_FDIVX, Opcode::Fdivx, Cpu::op_fdivx),
    (OPCODE_FSUBX, Opcode::Fsubx, Cpu::op_fsubx),
    (OPCODE_FADDX, Opcode::Faddx, Cpu::op_faddx),
    (OPCODE_FSELX, Opcode::Fselx, Cpu::op_fselx),
    (OPCODE_FMULX, Opcode::Fmulx, Cpu::op_fmulx),
    (OPCODE_FRSQRTEX, Opcode::Frsqrtex, Cpu::op_frsqrtex),
    (OPCODE_FMSUBX, Opcode::Fmsubx, Cpu::op_fmsubx),
    (OPCODE_FMADDX, Opcode::Fmaddx, Cpu::op_fmaddx),
    (OPCODE_FNMSUBX, Opcode::Fnmsubx, Cpu::op_fnmsubx),
    (OPCODE_FNMADDX, Opcode::Fnmaddx, Cpu::op_fnmaddx),
];

pub(crate) const OPTABLE: [OpFn; OPTABLE_SIZE] = {
    let mut optable = [ILLEGAL_OP.1; OPTABLE_SIZE];

    let mut i = 0;
    while i < OPCODE_TABLE.len() {
        let op = OPCODE_TABLE[i];
        optable[op.0 as usize] = op.2;
        i += 1;
    }

    optable
};

const OPTABLE4: [OpFn; OPTABLE4_SIZE] = {
    let mut optable = [ILLEGAL_OP.1; OPTABLE63_SIZE];

    let mut i = 0;
    while i < OPCODE4X_TABLE.len() {
        let op = OPCODE4X_TABLE[i];
        optable[op.0 as usize] = op.2;
        i += 1;
    }

    let mut n = 0;
    while n < 32 {
        let mut i = 0;
        let fill = n << 5;
        while i < OPCODE4A_TABLE.len() {
            let op = OPCODE4A_TABLE[i];
            let xo_x = (op.0 as usize) | fill;
            optable[xo_x] = op.2;
            i += 1;
        }
        n += 1;
    }

    let mut n = 0;
    while n < 16 {
        let mut i = 0;
        let fill = n << 6;
        while i < OPCODE4AA_TABLE.len() {
            let op = OPCODE4AA_TABLE[i];
            let xo_x = (op.0 as usize) | fill;
            optable[xo_x] = op.2;
            i += 1;
        }
        n += 1;
    }

    optable
};

const OPTABLE19: [OpFn; OPTABLE19_SIZE] = {
    let mut optable = [ILLEGAL_OP.1; OPTABLE19_SIZE];

    let mut i = 0;
    while i < OPCODE19_TABLE.len() {
        let op = OPCODE19_TABLE[i];
        optable[op.0 as usize] = op.2;
        i += 1;
    }

    optable
};

const OPTABLE31: [OpFn; OPTABLE31_SIZE] = {
    let mut optable = [ILLEGAL_OP.1; OPTABLE31_SIZE];

    let mut i = 0;
    while i < OPCODE31_TABLE.len() {
        let op = OPCODE31_TABLE[i];
        optable[op.0 as usize] = op.2;
        i += 1;
    }

    optable
};

const OPTABLE59: [OpFn; OPTABLE59_SIZE] = {
    let mut optable = [ILLEGAL_OP.1; OPTABLE59_SIZE];

    let mut i = 0;
    while i < OPCODE59_TABLE.len() {
        let op = OPCODE59_TABLE[i];
        optable[op.0 as usize] = op.2;
        i += 1;
    }

    optable
};

const OPTABLE63: [OpFn; OPTABLE63_SIZE] = {
    let mut optable = [ILLEGAL_OP.1; OPTABLE63_SIZE];

    let mut i = 0;
    while i < OPCODE63X_TABLE.len() {
        let op = OPCODE63X_TABLE[i];
        optable[op.0 as usize] = op.2;
        i += 1;
    }

    let mut n = 0;
    while n < 32 {
        let mut i = 0;
        let fill = n << 5;
        while i < OPCODE63A_TABLE.len() {
            let op = OPCODE63A_TABLE[i];
            let xo_x = (op.0 as usize) | fill;
            optable[xo_x] = op.2;
            i += 1;
        }
        n += 1;
    }

    optable
};

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
