use super::instruction::*;
use super::{
    sign_ext_16, sign_ext_26, Opcode, ILLEGAL_OP, OPCODE19_TABLE, OPCODE31_TABLE, OPCODE4AA_TABLE,
    OPCODE4A_TABLE, OPCODE4X_TABLE, OPCODE59_TABLE, OPCODE63A_TABLE, OPCODE63X_TABLE, OPCODE_TABLE,
    OPTABLE19_SIZE, OPTABLE31_SIZE, OPTABLE4_SIZE, OPTABLE59_SIZE, OPTABLE63_SIZE, OPTABLE_SIZE,
};

pub struct Disassembler {
    /// Primary Opcode Table
    optable: [Opcode; OPTABLE_SIZE],
    /// SubOpcode 4 Table
    optable4: [Opcode; OPTABLE4_SIZE],
    /// SubOpcode 19 Table
    optable19: [Opcode; OPTABLE19_SIZE],
    /// SubOpcode 31 Table
    optable31: [Opcode; OPTABLE31_SIZE],
    /// SubOpcode 59 Table
    optable59: [Opcode; OPTABLE59_SIZE],
    /// SubOpcode 63 Table
    optable63: [Opcode; OPTABLE63_SIZE],
}

impl Default for Disassembler {
    fn default() -> Self {
        let mut optable = [ILLEGAL_OP.0; OPTABLE_SIZE];
        let mut optable4 = [ILLEGAL_OP.0; OPTABLE4_SIZE];
        let mut optable19 = [ILLEGAL_OP.0; OPTABLE19_SIZE];
        let mut optable31 = [ILLEGAL_OP.0; OPTABLE31_SIZE];
        let mut optable59 = [ILLEGAL_OP.0; OPTABLE59_SIZE];
        let mut optable63 = [ILLEGAL_OP.0; OPTABLE63_SIZE];

        for op in OPCODE_TABLE.iter() {
            optable[op.0] = op.1;
        }

        for op in OPCODE4X_TABLE.iter() {
            optable4[op.0] = op.1;
        }

        for n in 0..32 {
            let fill = n << 5;
            for op in OPCODE4A_TABLE.iter() {
                let xo_x = op.0 | fill;
                optable4[xo_x] = op.1;
            }
        }

        for n in 0..16 {
            let fill = n << 6;
            for op in OPCODE4AA_TABLE.iter() {
                let xo_x = op.0 | fill;
                optable4[xo_x] = op.1;
            }
        }

        for op in OPCODE19_TABLE.iter() {
            optable19[op.0] = op.1;
        }

        for op in OPCODE31_TABLE.iter() {
            optable31[op.0] = op.1;
        }

        for op in OPCODE59_TABLE.iter() {
            optable59[op.0] = op.1;
        }

        for op in OPCODE63X_TABLE.iter() {
            optable63[op.0] = op.1;
        }

        for n in 0..32 {
            let fill = n << 5;
            for op in OPCODE63A_TABLE.iter() {
                let xo_x = op.0 | fill;
                optable63[xo_x] = op.1;
            }
        }

        Disassembler {
            optable,
            optable4,
            optable19,
            optable31,
            optable59,
            optable63,
        }
    }
}

impl Disassembler {
    pub fn decode(&self, addr: u32, code: u32) -> DecodedInstruction {
        let instr = Instruction(code);

        let mut opcode = self.optable[instr.opcd()];

        opcode = match opcode {
            Opcode::Table4 => self.optable4[instr.xo_x()],
            Opcode::Table19 => self.optable19[instr.xo_x()],
            Opcode::Table31 => self.optable31[instr.xo_x()],
            Opcode::Table59 => self.optable59[instr.xo_a()],
            Opcode::Table63 => self.optable63[instr.xo_x()],
            _ => opcode,
        };

        DecodedInstruction::new(instr, opcode, addr)
    }
}

pub struct DecodedInstruction {
    pub instr: Instruction,
    pub opcode: Opcode,
    pub addr: u32,
    pub mnemonic: String,
    pub operands: String,
}

impl DecodedInstruction {
    pub fn new(instr: Instruction, opcode: Opcode, addr: u32) -> Self {
        let mut mnemonic = mnemonic(opcode).to_string();
        mnemonic.push_str(suffix(instr, opcode));

        let operands = operands(instr, opcode, addr);

        DecodedInstruction {
            instr,
            opcode,
            addr,
            mnemonic,
            operands,
        }
    }
}

pub fn mnemonic(opcode: Opcode) -> &'static str {
    match opcode {
        Opcode::Twi => "twi",
        Opcode::Mulli => "mulli",
        Opcode::Subfic => "subfic",
        Opcode::Cmpli => "cmpli",
        Opcode::Cmpi => "cmpi",
        Opcode::Addic => "addic",
        Opcode::Addicrc => "addic.",
        Opcode::Addi => "addi",
        Opcode::Addis => "addis",
        Opcode::Bcx => "bc",
        Opcode::Sc => "sc",
        Opcode::Bx => "b",
        Opcode::Rlwimix => "rlwimix",
        Opcode::Rlwinmx => "rlwinmx",
        Opcode::Rlwnmx => "rlwnmx",
        Opcode::Ori => "ori",
        Opcode::Oris => "oris",
        Opcode::Xori => "xori",
        Opcode::Xoris => "xoris",
        Opcode::Andirc => "andi.",
        Opcode::Andisrc => "andis.",
        Opcode::Lwz => "lwz",
        Opcode::Lwzu => "lwzu",
        Opcode::Lbz => "lbz",
        Opcode::Lbzu => "lbzu",
        Opcode::Stw => "stw",
        Opcode::Stwu => "stwu",
        Opcode::Stb => "stbu",
        Opcode::Stbu => "stbu",
        Opcode::Lhz => "lhz",
        Opcode::Lhzu => "lhzu",
        Opcode::Lha => "lha",
        Opcode::Lhau => "lhau",
        Opcode::Sth => "sth",
        Opcode::Sthu => "sthu",
        Opcode::Lmw => "lmw",
        Opcode::Stmw => "stmw",
        Opcode::Lfs => "lfs",
        Opcode::Lfsu => "lfsu",
        Opcode::Lfd => "lfd",
        Opcode::Lfdu => "lfdu",
        Opcode::Stfs => "stfs",
        Opcode::Stfsu => "stfsu",
        Opcode::Stfd => "stfdu",
        Opcode::Stfdu => "stfdu",
        Opcode::PsqL => "psq_l",
        Opcode::PsqLu => "psq_lu",
        Opcode::PsqSt => "psq_st",
        Opcode::PsqStu => "psq_stu",
        Opcode::Table4 => "<subtable4>",
        Opcode::Table19 => "<subtable19>",
        Opcode::Table31 => "<subtable31>",
        Opcode::Table59 => "<subtable59>",
        Opcode::Table63 => "<subtable63>",
        Opcode::Illegal => "<illegal>",
        // Table4,
        Opcode::PsCmpu0 => "ps_cmpu0",
        Opcode::PsqLx => "ps_lx",
        Opcode::PsqStx => "psq_stx",
        Opcode::PsSum0x => "ps_sum0",
        Opcode::PsSum1x => "ps_sum1",
        Opcode::PsMuls0x => "ps_muls0",
        Opcode::PsMuls1x => "ps_muls1",
        Opcode::PsMadds0x => "ps_madds0",
        Opcode::PsMadds1x => "ps_madds1",
        Opcode::PsDivx => "ps_div",
        Opcode::PsSubx => "ps_sub",
        Opcode::PsAddx => "ps_add",
        Opcode::PsSelx => "ps_sel",
        Opcode::PsResx => "ps_res",
        Opcode::PsMulx => "ps_mul",
        Opcode::PsRsqrtex => "ps_rsqrte",
        Opcode::PsMsubx => "ps_msub",
        Opcode::PsMaddx => "ps_madd",
        Opcode::PsNmsubx => "ps_nmsub",
        Opcode::PsNmaddx => "ps_nmadd",
        Opcode::PsCmpo0 => "ps_cmp0",
        Opcode::PsqLux => "psq_lux",
        Opcode::PsqStux => "ps_Stux",
        Opcode::PsNegx => "ps_neg",
        Opcode::PsCmpu1 => "ps_cmpu1",
        Opcode::PsMrx => "ps_mr",
        Opcode::PsCmpo1 => "ps_cmpo1",
        Opcode::PsNabsx => "ps_nabs",
        Opcode::PsAbsx => "ps_abs",
        Opcode::PsMerge00x => "ps_merge00",
        Opcode::PsMerge01x => "ps_merge01",
        Opcode::PsMerge10x => "ps_merge10",
        Opcode::PsMerge11x => "ps_merge11",
        Opcode::DcbzL => "dcbzl",
        // Table19
        Opcode::Mcrf => "mcrf",
        Opcode::Bclrx => "bclr",
        Opcode::Crnor => "crnor",
        Opcode::Rfi => "rfi",
        Opcode::Crandc => "crandc",
        Opcode::Isync => "isync",
        Opcode::Crxor => "crxor",
        Opcode::Crnand => "crnand",
        Opcode::Crand => "crand",
        Opcode::Creqv => "creqv",
        Opcode::Crorc => "crorc",
        Opcode::Cror => "cror",
        Opcode::Bcctrx => "bcctr",
        // Table31
        Opcode::Cmp => "cmp",
        Opcode::Tw => "tw",
        Opcode::Subfcx => "subfc",
        Opcode::Addcx => "addc",
        Opcode::Mulhwux => "mulhwu",
        Opcode::Mfcr => "mfcr",
        Opcode::Lwarx => "lwarx",
        Opcode::Lwzx => "lwzx",
        Opcode::Slwx => "slw",
        Opcode::Cntlzwx => "cntlzw",
        Opcode::Andx => "and",
        Opcode::Cmpl => "cmpl",
        Opcode::Subfx => "subfx",
        Opcode::Dcbst => "dcbst",
        Opcode::Lwzux => "lwzux",
        Opcode::Andcx => "andc",
        Opcode::Mulhwx => "mulhw",
        Opcode::Mfmsr => "mfmsr",
        Opcode::Dcbf => "dcbf",
        Opcode::Lbzx => "lbzx",
        Opcode::Negx => "neg",
        Opcode::Lbzux => "lbzux",
        Opcode::Norx => "nor",
        Opcode::Subfex => "subfe",
        Opcode::Addex => "adde",
        Opcode::Mtcrf => "mtcrf",
        Opcode::Mtmsr => "mtmsr",
        Opcode::Stwcxrc => "stwcx.",
        Opcode::Stwx => "stwx",
        Opcode::Stwux => "stwux",
        Opcode::Subfzex => "subfze",
        Opcode::Addzex => "addze",
        Opcode::Mtsr => "mtsr",
        Opcode::Stbx => "stbx",
        Opcode::Subfmex => "subfme",
        Opcode::Addmex => "addme",
        Opcode::Mullwx => "mullw",
        Opcode::Mtsrin => "mtsrin",
        Opcode::Dcbtst => "dcbtst",
        Opcode::Stbux => "stbux",
        Opcode::Addx => "add",
        Opcode::Dcbt => "dcbt",
        Opcode::Lhzx => "lhzx",
        Opcode::Eqvx => "eqv",
        Opcode::Tlbie => "tlbie",
        Opcode::Eciwx => "eciwx",
        Opcode::Lhzux => "lhzux",
        Opcode::Xorx => "xor",
        Opcode::Mfspr => "mfspr",
        Opcode::Lhax => "lhax",
        Opcode::Mftb => "mftb",
        Opcode::Lhaux => "lhaux",
        Opcode::Sthx => "sthx",
        Opcode::Orcx => "orc",
        Opcode::Ecowx => "ecowx",
        Opcode::Sthux => "sthux",
        Opcode::Orx => "or",
        Opcode::Divwux => "divwu",
        Opcode::Mtspr => "mtspr",
        Opcode::Dcbi => "dcbi",
        Opcode::Nandx => "nand",
        Opcode::Divwx => "divw",
        Opcode::Mcrxr => "mcrxr",
        Opcode::Lswx => "lswx",
        Opcode::Lwbrx => "lwbrx",
        Opcode::Lfsx => "lfsx",
        Opcode::Srwx => "srw",
        Opcode::Tlbsync => "tlbsync",
        Opcode::Lfsux => "lfsux",
        Opcode::Mfsr => "mfsr",
        Opcode::Lswi => "lswi",
        Opcode::Sync => "sync",
        Opcode::Lfdx => "lfdx",
        Opcode::Lfdux => "lfdux",
        Opcode::Mfsrin => "mfsrin",
        Opcode::Stswx => "stswx",
        Opcode::Stwbrx => "stwbrx",
        Opcode::Stfsx => "stfsx",
        Opcode::Stfsux => "stfsux",
        Opcode::Stswi => "stswi",
        Opcode::Stfdx => "stfdx",
        Opcode::Stfdux => "stfdux",
        Opcode::Lhbrx => "lhbrx",
        Opcode::Srawx => "sraw",
        Opcode::Srawix => "srawi",
        Opcode::Eieio => "eieio",
        Opcode::Sthbrx => "sthbrx",
        Opcode::Extshx => "extsh",
        Opcode::Extsbx => "extsb",
        Opcode::Icbi => "icbi",
        Opcode::Stfiwx => "stfiwx",
        Opcode::Dcbz => "dcbz",
        // Table59
        Opcode::Fdivsx => "fdivs",
        Opcode::Fsubsx => "fsubs",
        Opcode::Faddsx => "fadds",
        Opcode::Fresx => "fres",
        Opcode::Fmulsx => "fmuls",
        Opcode::Fmsubsx => "fmsubs",
        Opcode::Fmaddsx => "fmadds",
        Opcode::Fnmsubsx => "fnmsubs",
        Opcode::Fnmaddsx => "fnmadds",
        // Table63
        Opcode::Fcmpu => "fcmpu",
        Opcode::Frspx => "frsp",
        Opcode::Fctiwx => "fctiw",
        Opcode::Fctiwzx => "fctiwz",
        Opcode::Fdivx => "fdiv",
        Opcode::Fsubx => "fsub",
        Opcode::Faddx => "fadd",
        Opcode::Fselx => "fsel",
        Opcode::Fmulx => "fmul",
        Opcode::Frsqrtex => "frsqrte",
        Opcode::Fmsubx => "fmsub",
        Opcode::Fmaddx => "fmadd",
        Opcode::Fnmsubx => "fnmsub",
        Opcode::Fnmaddx => "fnmadd",
        Opcode::Fcmpo => "fcmpo",
        Opcode::Mtfsb1x => "mtfsb1",
        Opcode::Fnegx => "fneg",
        Opcode::Mcrfs => "mcrfs",
        Opcode::Mtfsb0x => "mtfsb0",
        Opcode::Fmrx => "fmr",
        Opcode::Mtfsfix => "mtfsfi",
        Opcode::Fnabsx => "fnabs",
        Opcode::Fabsx => "fabs",
        Opcode::Mffsx => "mffs",
        Opcode::Mtfsfx => "mtfsfx",
    }
}

pub fn suffix(instr: Instruction, opcode: Opcode) -> &'static str {
    match opcode {
        Opcode::Bx | Opcode::Bcx => match (instr.aa(), instr.lk()) {
            (false, false) => "",
            (true, false) => "a",
            (false, true) => "l",
            (true, true) => "la",
        },
        Opcode::Bclrx | Opcode::Bcctrx => {
            if instr.lk() {
                ""
            } else {
                "l"
            }
        }
        Opcode::Subfcx
        | Opcode::Addcx
        | Opcode::Mulhwux
        | Opcode::Subfx
        | Opcode::Negx
        | Opcode::Subfex
        | Opcode::Addex
        | Opcode::Subfzex
        | Opcode::Addzex
        | Opcode::Mullwx
        | Opcode::Addx
        | Opcode::Divwux
        | Opcode::Divwx
        | Opcode::Subfmex
        | Opcode::Addmex => match (instr.oe(), instr.rc()) {
            (false, false) => "",
            (false, true) => ".",
            (true, false) => "o",
            (true, true) => "o.",
        },
        Opcode::Rlwimix
        | Opcode::Rlwinmx
        | Opcode::Cntlzwx
        | Opcode::Andx
        | Opcode::Andcx
        | Opcode::Norx
        | Opcode::Xorx
        | Opcode::Orx
        | Opcode::Srwx
        | Opcode::Srawx
        | Opcode::Srawix
        | Opcode::Extshx
        | Opcode::Extsbx
        | Opcode::Fdivsx
        | Opcode::Fsubsx
        | Opcode::Faddsx
        | Opcode::Fmulsx
        | Opcode::Frspx
        | Opcode::Fctiwzx
        | Opcode::Fsubx
        | Opcode::Fmulx
        | Opcode::Mtfsb1x
        | Opcode::Fnegx
        | Opcode::Fmrx
        | Opcode::Fnabsx
        | Opcode::Rlwnmx
        | Opcode::PsSum0x
        | Opcode::PsSum1x
        | Opcode::PsMuls0x
        | Opcode::PsMuls1x
        | Opcode::PsMadds0x
        | Opcode::PsMadds1x
        | Opcode::PsDivx
        | Opcode::PsSubx
        | Opcode::PsAddx
        | Opcode::PsSelx
        | Opcode::PsResx
        | Opcode::PsMulx
        | Opcode::PsRsqrtex
        | Opcode::PsMsubx
        | Opcode::PsMaddx
        | Opcode::PsNmsubx
        | Opcode::PsNmaddx
        | Opcode::PsNegx
        | Opcode::PsMrx
        | Opcode::PsNabsx
        | Opcode::PsAbsx
        | Opcode::PsMerge00x
        | Opcode::PsMerge01x
        | Opcode::PsMerge10x
        | Opcode::PsMerge11x
        | Opcode::Eqvx
        | Opcode::Mtfsfx
        | Opcode::Orcx
        | Opcode::Nandx
        | Opcode::Fresx
        | Opcode::Fmsubsx
        | Opcode::Fmaddsx
        | Opcode::Fnmsubsx
        | Opcode::Fnmaddsx
        | Opcode::Fctiwx
        | Opcode::Fdivx
        | Opcode::Faddx
        | Opcode::Fselx
        | Opcode::Frsqrtex
        | Opcode::Fmsubx
        | Opcode::Fmaddx
        | Opcode::Fnmsubx
        | Opcode::Fnmaddx
        | Opcode::Mtfsb0x
        | Opcode::Mtfsfix
        | Opcode::Fabsx
        | Opcode::Mffsx => match instr.rc() {
            false => "",
            true => ".",
        },
        _ => "",
    }
}

pub fn operands(instr: Instruction, opcode: Opcode, addr: u32) -> String {
    match opcode {
        Opcode::Tw => format!("{},r{},r{}", instr.s(), instr.a(), instr.b()),
        Opcode::Twi => format!("{},r{},{}", instr.to(), instr.a(), instr.simm()),
        Opcode::Mulli => format!("r{},r{},{}", instr.d(), instr.a(), instr.simm()),
        Opcode::Subfic => format!("r{},r{},{}", instr.d(), instr.a(), instr.simm()),
        Opcode::Cmpli => format!(
            "cr{},{},r{},{}",
            instr.crfd(),
            instr.l() as u8,
            instr.a(),
            instr.uimm()
        ),
        Opcode::Cmpi => format!(
            "cr{},{},r{},{}",
            instr.crfd(),
            instr.l() as u8,
            instr.a(),
            instr.simm()
        ),
        Opcode::Addic => format!("r{},r{},{}", instr.d(), instr.a(), instr.simm()),
        Opcode::Addicrc => format!("r{},r{},{}", instr.d(), instr.a(), instr.simm()),
        Opcode::Addi | Opcode::Addis => {
            if instr.a() != 0 {
                format!("r{},r{},{}", instr.d(), instr.a(), instr.simm())
            } else {
                format!("r{},0,{}", instr.d(), instr.simm())
            }
        }
        Opcode::Bcx => {
            let mut target = sign_ext_16(instr.bd() << 2) as u32;
            if !instr.aa() {
                target = target.wrapping_add(addr);
            }

            format!("{},{},{:#x}", instr.bo(), instr.bi(), target)
        }
        Opcode::Bx => {
            let mut target = sign_ext_26(instr.li() << 2) as u32;
            if !instr.aa() {
                target = target.wrapping_add(addr);
            }

            format!("{target:#x}")
        }
        Opcode::Rlwimix | Opcode::Rlwinmx => format!(
            "r{},r{},{},{},{}",
            instr.a(),
            instr.s(),
            instr.sh(),
            instr.mb(),
            instr.me()
        ),
        Opcode::Rlwnmx => format!(
            "r{},r{},r{},{},{}",
            instr.a(),
            instr.s(),
            instr.b(),
            instr.mb(),
            instr.me()
        ),
        Opcode::Ori
        | Opcode::Oris
        | Opcode::Xoris
        | Opcode::Andirc
        | Opcode::Xori
        | Opcode::Andisrc => {
            format!("r{},r{},{}", instr.a(), instr.s(), instr.uimm())
        }
        Opcode::Lwz
        | Opcode::Lwzu
        | Opcode::Lbz
        | Opcode::Lbzu
        | Opcode::Lhz
        | Opcode::Lhzu
        | Opcode::Lha
        | Opcode::Lmw => {
            format!("r{},{}(r{})", instr.d(), instr.simm(), instr.a())
        }
        Opcode::Lhau => {
            format!("r{},{}(r{})", instr.d(), instr.uimm(), instr.a())
        }
        Opcode::Stw
        | Opcode::Stwu
        | Opcode::Stb
        | Opcode::Stbu
        | Opcode::Sth
        | Opcode::Sthu
        | Opcode::Stmw => {
            format!("r{},{}(r{})", instr.s(), instr.simm(), instr.a())
        }
        Opcode::Lfs | Opcode::Lfd | Opcode::Lfsu | Opcode::Lfdu => {
            format!("f{},{}(r{})", instr.d(), instr.simm(), instr.a())
        }
        Opcode::Stfs | Opcode::Stfsu | Opcode::Stfd | Opcode::Stfdu => {
            format!("f{},{}(r{})", instr.s(), instr.simm(), instr.a())
        }
        Opcode::PsqL | Opcode::PsqLu => format!(
            "f{},{}(r{}),{},{}",
            instr.d(),
            instr.uimm_1(),
            instr.a(),
            instr.w(),
            instr.i()
        ),
        Opcode::PsqLx | Opcode::PsqLux => format!(
            "f{},r{},r{},{},{}",
            instr.d(),
            instr.a(),
            instr.b(),
            instr.w(),
            instr.i()
        ),
        Opcode::PsqSt | Opcode::PsqStu => format!(
            "f{},{}(r{}),{},{}",
            instr.s(),
            instr.uimm_1(),
            instr.a(),
            instr.w(),
            instr.i()
        ),
        Opcode::PsqStx | Opcode::PsqStux => format!(
            "f{},r{},r{},{},{}",
            instr.s(),
            instr.a(),
            instr.b(),
            instr.w(),
            instr.i()
        ),
        Opcode::Crxor
        | Opcode::Crnor
        | Opcode::Crandc
        | Opcode::Crnand
        | Opcode::Crand
        | Opcode::Creqv
        | Opcode::Crorc
        | Opcode::Cror => format!("cr{},cr{},cr{}", instr.d(), instr.a(), instr.b()),
        Opcode::Cmp | Opcode::Cmpl => format!(
            "cr{},{},r{},r{}",
            instr.crfd(),
            instr.l() as u8,
            instr.a(),
            instr.b()
        ),
        Opcode::Mfcr | Opcode::Mfmsr => format!("r{}", instr.d()),
        Opcode::Mtcrf => format!("{},r{}", instr.crm(), instr.s()),
        Opcode::Mtmsr => format!("r{}", instr.s()),
        Opcode::Negx | Opcode::Subfzex | Opcode::Addzex | Opcode::Subfmex | Opcode::Addmex => {
            format!("r{},r{}", instr.d(), instr.a())
        }
        Opcode::Mtsr => format!("{},r{}", instr.sr(), instr.s()),
        Opcode::Mfsr => format!("r{},{}", instr.d(), instr.sr()),
        Opcode::Stwx
        | Opcode::Stbx
        | Opcode::Stwcxrc
        | Opcode::Stwux
        | Opcode::Stbux
        | Opcode::Sthx
        | Opcode::Ecowx
        | Opcode::Sthux
        | Opcode::Stswx
        | Opcode::Stwbrx
        | Opcode::Sthbrx => {
            format!("r{},r{},r{}", instr.s(), instr.a(), instr.b())
        }
        Opcode::Subfcx
        | Opcode::Addcx
        | Opcode::Mulhwux
        | Opcode::Lwzx
        | Opcode::Lbzx
        | Opcode::Subfex
        | Opcode::Addex
        | Opcode::Addx
        | Opcode::Lwarx
        | Opcode::Lwzux
        | Opcode::Mulhwx
        | Opcode::Lbzux
        | Opcode::Lhzx
        | Opcode::Eciwx
        | Opcode::Lhzux
        | Opcode::Lhax
        | Opcode::Lhaux
        | Opcode::Lhbrx
        | Opcode::Subfx
        | Opcode::Mullwx
        | Opcode::Divwux
        | Opcode::Divwx
        | Opcode::Lswx
        | Opcode::Lwbrx => {
            format!("r{},r{},r{}", instr.d(), instr.a(), instr.b())
        }
        Opcode::Mfspr => format!("r{},{}", instr.d(), instr.spr()),
        Opcode::Mftb => format!("r{},{}", instr.d(), instr.tbr()),
        Opcode::Mtspr => format!("{},r{}", instr.spr(), instr.s()),
        Opcode::Mtsrin => format!("r{},r{}", instr.s(), instr.b()),
        Opcode::Slwx
        | Opcode::Andx
        | Opcode::Andcx
        | Opcode::Norx
        | Opcode::Xorx
        | Opcode::Orx
        | Opcode::Srwx
        | Opcode::Srawx
        | Opcode::Eqvx
        | Opcode::Orcx
        | Opcode::Nandx => {
            format!("r{},r{},r{}", instr.a(), instr.s(), instr.b())
        }
        Opcode::Srawix => format!("r{},r{},{}", instr.a(), instr.s(), instr.sh()),
        Opcode::Cntlzwx | Opcode::Extshx | Opcode::Extsbx => {
            format!("r{},r{}", instr.a(), instr.s())
        }
        Opcode::Dcbf
        | Opcode::Dcbi
        | Opcode::Icbi
        | Opcode::DcbzL
        | Opcode::Dcbst
        | Opcode::Dcbtst
        | Opcode::Dcbt
        | Opcode::Dcbz => {
            format!("r{},r{}", instr.a(), instr.b())
        }
        Opcode::Mfsrin => format!("r{},r{}", instr.d(), instr.b()),
        Opcode::Fdivsx
        | Opcode::Fsubsx
        | Opcode::Faddsx
        | Opcode::Fsubx
        | Opcode::PsDivx
        | Opcode::PsSubx
        | Opcode::PsAddx
        | Opcode::PsMerge00x
        | Opcode::PsMerge01x
        | Opcode::PsMerge10x
        | Opcode::PsMerge11x
        | Opcode::Fdivx
        | Opcode::Faddx => {
            format!("f{},f{},f{}", instr.d(), instr.a(), instr.b())
        }
        Opcode::Fmulsx | Opcode::Fmulx | Opcode::PsMuls0x | Opcode::PsMuls1x | Opcode::PsMulx => {
            format!("f{},f{},f{}", instr.d(), instr.a(), instr.c())
        }
        Opcode::Fcmpu
        | Opcode::Fcmpo
        | Opcode::PsCmpu0
        | Opcode::PsCmpo0
        | Opcode::PsCmpu1
        | Opcode::PsCmpo1 => {
            format!("cr{},f{},f{}", instr.crfd(), instr.a(), instr.b())
        }
        Opcode::Mcrxr => format!("cr{}", instr.crfd()),
        Opcode::Mtfsb1x | Opcode::Mtfsb0x => format!("cr{}", instr.crbd()),

        Opcode::Mtfsfix => format!("cr{},{}", instr.crfd(), instr.imm()),
        Opcode::Frspx
        | Opcode::Fctiwzx
        | Opcode::Fnegx
        | Opcode::Fmrx
        | Opcode::PsNabsx
        | Opcode::Fnabsx
        | Opcode::PsResx
        | Opcode::PsRsqrtex
        | Opcode::PsNegx
        | Opcode::PsMrx
        | Opcode::PsAbsx
        | Opcode::Fresx
        | Opcode::Fctiwx
        | Opcode::Frsqrtex
        | Opcode::Fabsx => {
            format!("f{},f{}", instr.d(), instr.b())
        }
        Opcode::Mtfsfx => format!("{},f{}", instr.fm(), instr.b()),
        Opcode::PsSum0x
        | Opcode::PsSum1x
        | Opcode::PsMadds0x
        | Opcode::PsMadds1x
        | Opcode::PsSelx
        | Opcode::PsMsubx
        | Opcode::PsMaddx
        | Opcode::PsNmsubx
        | Opcode::PsNmaddx
        | Opcode::Fmsubsx
        | Opcode::Fmaddsx
        | Opcode::Fnmsubsx
        | Opcode::Fnmaddsx
        | Opcode::Fselx
        | Opcode::Fmsubx
        | Opcode::Fmaddx
        | Opcode::Fnmsubx
        | Opcode::Fnmaddx => format!(
            "f{},f{},f{},f{}",
            instr.d(),
            instr.a(),
            instr.c(),
            instr.b(),
        ),
        Opcode::Mcrf | Opcode::Mcrfs => {
            format!("cr{},cr{}", instr.crfd(), instr.crfs())
        }
        Opcode::Tlbie => format!("r{}", instr.b()),
        Opcode::Lfsx | Opcode::Lfsux | Opcode::Lfdx | Opcode::Lfdux => {
            format!("f{},r{},r{}", instr.d(), instr.a(), instr.b())
        }
        Opcode::Stfsx | Opcode::Stfsux | Opcode::Stfdx | Opcode::Stfdux | Opcode::Stfiwx => {
            format!("f{},r{},r{}", instr.s(), instr.a(), instr.b())
        }
        Opcode::Lswi => format!("r{},r{},{}", instr.d(), instr.a(), instr.nb()),
        Opcode::Stswi => format!("r{},r{},{}", instr.s(), instr.a(), instr.nb()),
        Opcode::Mffsx => format!("f{}", instr.d()),
        _ => String::new(),
    }
}
