use super::instruction::*;
use super::{
    sign_ext_16, sign_ext_26, Opcode, ILLEGAL_OP, OPCODE19_TABLE, OPCODE31_TABLE, OPCODE59_TABLE,
    OPCODE63_TABLE, OPCODE_TABLE, OPTABLE19_SIZE, OPTABLE31_SIZE, OPTABLE59_SIZE, OPTABLE63_SIZE,
    OPTABLE_SIZE,
};

pub struct Disassembler {
    /// Primary Opcode Table
    optable: [Opcode; OPTABLE_SIZE],
    /// SubOpcode 19 Table
    optable19: [Opcode; OPTABLE19_SIZE],
    /// SubOpcode 31 Table
    optable31: [Opcode; OPTABLE31_SIZE],
    /// SubOpcode 59 Table
    optable59: [Opcode; OPTABLE59_SIZE],
    /// SubOpcode 63 Table
    optable63: [Opcode; OPTABLE63_SIZE],
}

impl Disassembler {
    pub fn default() -> Self {
        let mut optable = [ILLEGAL_OP.0; OPTABLE_SIZE];
        let mut optable19 = [ILLEGAL_OP.0; OPTABLE19_SIZE];
        let mut optable31 = [ILLEGAL_OP.0; OPTABLE31_SIZE];
        let mut optable59 = [ILLEGAL_OP.0; OPTABLE59_SIZE];
        let mut optable63 = [ILLEGAL_OP.0; OPTABLE63_SIZE];

        for op in OPCODE_TABLE.iter() {
            optable[op.0 as usize] = op.1;
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

        for op in OPCODE63_TABLE.iter() {
            optable63[op.0 as usize] = op.1;
        }

        Disassembler {
            optable,
            optable19,
            optable31,
            optable59,
            optable63,
        }
    }

    pub fn decode(&self, addr: u32, code: u32) -> DecodedInstruction {
        let instr = Instruction(code);

        let mut opcode = self.optable[instr.opcd()];

        opcode = match opcode {
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
        Opcode::Illegal => "<illegal>",
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
        Opcode::Table19 => "<subtable19>",
        Opcode::Rlwimix => "rlwimix",
        Opcode::Rlwinmx => "rlwinmx",
        Opcode::Ori => "ori",
        Opcode::Oris => "oris",
        Opcode::Xoris => "xoris",
        Opcode::Andirc => "andi.",
        Opcode::Table31 => "<subtable31>",
        Opcode::Lwz => "lwz",
        Opcode::Lwzu => "lwzu",
        Opcode::Lbz => "lbz",
        Opcode::Lbzu => "lbzu",
        Opcode::Stw => "stw",
        Opcode::Stwu => "stwu",
        Opcode::Stb => "stb",
        Opcode::Stbu => "stbu",
        Opcode::Lhz => "lhz",
        Opcode::Lhzu => "lhzu",
        Opcode::Lha => "lha",
        Opcode::Sth => "sth",
        Opcode::Sthu => "sthu",
        Opcode::Lmw => "lmw",
        Opcode::Stmw => "stmw",
        Opcode::Lfs => "lfs",
        Opcode::Lfd => "lfd",
        Opcode::Stfs => "stfs",
        Opcode::Stfsu => "stfsu",
        Opcode::Stfd => "stfd",
        Opcode::Psql => "psq_l",
        Opcode::Table59 => "<subtable59>",
        Opcode::Psqst => "psq_st",
        Opcode::Table63 => "<subtable63>",
        Opcode::Bclrx => "bclr",
        Opcode::Rfi => "rfi",
        Opcode::Isync => "isync",
        Opcode::Crxor => "crxor",
        Opcode::Bcctrx => "bcctr",
        Opcode::Cmp => "cmp",
        Opcode::Subfcx => "subfc",
        Opcode::Addcx => "addc",
        Opcode::Mulhwux => "mulhwu",
        Opcode::Mfcr => "mfcr",
        Opcode::Lwzx => "lwzx",
        Opcode::Slwx => "slwx",
        Opcode::Cntlzwx => "cntlzw",
        Opcode::Andx => "and",
        Opcode::Cmpl => "cmpl",
        Opcode::Subfx => "subf",
        Opcode::Andcx => "andc",
        Opcode::Mfmsr => "mfmsr",
        Opcode::Dcbf => "dcbf",
        Opcode::Lbzx => "lbzx",
        Opcode::Negx => "neg",
        Opcode::Norx => "nor",
        Opcode::Subfex => "subfe",
        Opcode::Addex => "addex",
        Opcode::Mtcrf => "mtcrf",
        Opcode::Mtmsr => "mtmsr",
        Opcode::Stwx => "stwx",
        Opcode::Subfzex => "subfze",
        Opcode::Addzex => "addze",
        Opcode::Mtsr => "mtsr",
        Opcode::Stbx => "stbx",
        Opcode::Mullwx => "mullw",
        Opcode::Addx => "add",
        Opcode::Xorx => "xor",
        Opcode::Mfspr => "mfspr",
        Opcode::Mftb => "mftb",
        Opcode::Orx => "or",
        Opcode::Divwux => "divwu",
        Opcode::Mtspr => "mtspr",
        Opcode::Dcbi => "dcbi",
        Opcode::Divwx => "divw",
        Opcode::Srwx => "srw",
        Opcode::Sync => "sync",
        Opcode::Srawx => "sraw",
        Opcode::Srawix => "srawi",
        Opcode::Extshx => "extsh",
        Opcode::Extsbx => "extsb",
        Opcode::Icbi => "icbi",
        Opcode::Fdivsx => "fdivs",
        Opcode::Fsubsx => "fsubs",
        Opcode::Faddsx => "fadds",
        Opcode::Fmulsx => "fmuls",
        Opcode::Fcmpu => "fcmpu",
        Opcode::Frspx => "frsp",
        Opcode::Fctiwzx => "fctiwz",
        Opcode::Fsubx => "fsub",
        Opcode::Fmulx => "fmul",
        Opcode::Fcmpo => "fcmpo",
        Opcode::Mtfsb1x => "mtfsb1",
        Opcode::Fnegx => "fneg",
        Opcode::Fmrx => "fmr",
        Opcode::Fnabsx => "fnabs",
        Opcode::Mtfsfx => "mtfsf",
    }
}

pub fn suffix(instr: Instruction, opcode: Opcode) -> &'static str {
    match opcode {
        Opcode::Illegal
        | Opcode::Twi
        | Opcode::Mulli
        | Opcode::Subfic
        | Opcode::Cmpli
        | Opcode::Cmpi
        | Opcode::Addic
        | Opcode::Addicrc
        | Opcode::Addi
        | Opcode::Addis
        | Opcode::Sc
        | Opcode::Table19
        | Opcode::Ori
        | Opcode::Oris
        | Opcode::Xoris
        | Opcode::Andirc
        | Opcode::Table31
        | Opcode::Lwz
        | Opcode::Lwzu
        | Opcode::Lbz
        | Opcode::Lbzu
        | Opcode::Stw
        | Opcode::Stwu
        | Opcode::Stb
        | Opcode::Stbu
        | Opcode::Lhz
        | Opcode::Lhzu
        | Opcode::Lha
        | Opcode::Sth
        | Opcode::Sthu
        | Opcode::Lmw
        | Opcode::Stmw
        | Opcode::Lfs
        | Opcode::Lfd
        | Opcode::Stfs
        | Opcode::Stfsu
        | Opcode::Stfd
        | Opcode::Psql
        | Opcode::Table59
        | Opcode::Psqst
        | Opcode::Table63
        | Opcode::Rfi
        | Opcode::Isync
        | Opcode::Crxor
        | Opcode::Cmp
        | Opcode::Mfcr
        | Opcode::Lwzx
        | Opcode::Slwx
        | Opcode::Cmpl
        | Opcode::Mfmsr
        | Opcode::Dcbf
        | Opcode::Lbzx
        | Opcode::Mtcrf
        | Opcode::Mtmsr
        | Opcode::Stwx
        | Opcode::Mtsr
        | Opcode::Stbx
        | Opcode::Mfspr
        | Opcode::Mftb
        | Opcode::Mtspr
        | Opcode::Dcbi
        | Opcode::Sync
        | Opcode::Icbi
        | Opcode::Fcmpu
        | Opcode::Fcmpo => "",
        Opcode::Bx | Opcode::Bcx => match (instr.aa() != 0, instr.lk() != 0) {
            (false, false) => "",
            (true, false) => "a",
            (false, true) => "l",
            (true, true) => "la",
        },
        Opcode::Bclrx | Opcode::Bcctrx => {
            if instr.lk() != 0 {
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
        | Opcode::Divwx => match (instr.oe(), instr.rc()) {
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
        | Opcode::Mtfsfx => match instr.rc() {
            false => "",
            true => ".",
        },
    }
}

pub fn operands(instr: Instruction, opcode: Opcode, addr: u32) -> String {
    match opcode {
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
            if instr.aa() == 0 {
                target = target.wrapping_add(addr);
            }

            format!("{},{},{:#x}", instr.bo(), instr.bi(), target)
        }
        Opcode::Bx => {
            let mut target = sign_ext_26(instr.li() << 2) as u32;
            if instr.aa() == 0 {
                target = target.wrapping_add(addr);
            }

            format!("{:#x}", target)
        }
        Opcode::Rlwimix | Opcode::Rlwinmx => format!(
            "r{},r{},{},{},{}",
            instr.a(),
            instr.s(),
            instr.sh(),
            instr.mb(),
            instr.me()
        ),
        Opcode::Ori | Opcode::Oris | Opcode::Xoris | Opcode::Andirc => {
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
        Opcode::Stw
        | Opcode::Stwu
        | Opcode::Stb
        | Opcode::Stbu
        | Opcode::Sth
        | Opcode::Sthu
        | Opcode::Stmw => {
            format!("r{},{}(r{})", instr.s(), instr.simm(), instr.a())
        }
        Opcode::Lfs | Opcode::Lfd => {
            format!("f{},{}(r{})", instr.d(), instr.simm(), instr.a())
        }
        Opcode::Stfs | Opcode::Stfsu | Opcode::Stfd => {
            format!("f{},{}(r{})", instr.s(), instr.simm(), instr.a())
        }
        Opcode::Psql => format!(
            "f{},{}(r{}),{},{}",
            instr.d(),
            instr.uimm_1(),
            instr.a(),
            instr.i(),
            instr.w()
        ),
        Opcode::Psqst => format!(
            "f{},{}(r{}),{},{}",
            instr.s(),
            instr.uimm_1(),
            instr.a(),
            instr.i(),
            instr.w()
        ),
        Opcode::Crxor => format!("cr{},cr{},cr{}", instr.d(), instr.a(), instr.b()),
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
        Opcode::Negx | Opcode::Subfzex | Opcode::Addzex => {
            format!("r{},r{}", instr.d(), instr.a())
        }
        Opcode::Mtsr => format!("{},r{}", instr.sr(), instr.s()),
        Opcode::Stwx | Opcode::Stbx => {
            format!("r{},r{},r{}", instr.s(), instr.a(), instr.b())
        }
        Opcode::Subfcx
        | Opcode::Addcx
        | Opcode::Mulhwux
        | Opcode::Lwzx
        | Opcode::Lbzx
        | Opcode::Subfex
        | Opcode::Addex
        | Opcode::Addx => {
            format!("r{},r{},r{}", instr.d(), instr.a(), instr.b())
        }
        Opcode::Mfspr => format!("r{},{}", instr.d(), instr.spr()),
        Opcode::Mftb => format!("r{},{}", instr.d(), instr.tbr()),
        Opcode::Mtspr => format!("{},r{}", instr.spr(), instr.s()),
        Opcode::Subfx | Opcode::Mullwx | Opcode::Divwux | Opcode::Divwx => {
            format!("r{},r{},r{}", instr.d(), instr.a(), instr.b())
        }
        Opcode::Slwx
        | Opcode::Andx
        | Opcode::Andcx
        | Opcode::Norx
        | Opcode::Xorx
        | Opcode::Orx
        | Opcode::Srwx
        | Opcode::Srawx => {
            format!("r{},r{},r{}", instr.a(), instr.s(), instr.b())
        }
        Opcode::Srawix => format!("r{},r{},{}", instr.a(), instr.s(), instr.sh()),
        Opcode::Cntlzwx | Opcode::Extshx | Opcode::Extsbx => {
            format!("r{},r{}", instr.a(), instr.s())
        }
        Opcode::Dcbf | Opcode::Dcbi | Opcode::Icbi => format!("r{},r{}", instr.a(), instr.b()),
        Opcode::Fdivsx | Opcode::Fsubsx | Opcode::Faddsx | Opcode::Fsubx => {
            format!("f{},f{},f{}", instr.d(), instr.a(), instr.b())
        }
        Opcode::Fmulsx | Opcode::Fmulx => {
            format!("f{},f{},f{}", instr.d(), instr.a(), instr.c())
        }
        Opcode::Fcmpu | Opcode::Fcmpo => {
            format!("cr{},f{},f{}", instr.crfd(), instr.a(), instr.b())
        }
        Opcode::Mtfsb1x => format!("cr{}", instr.crbd()),
        Opcode::Frspx | Opcode::Fctiwzx | Opcode::Fnegx | Opcode::Fmrx => {
            format!("f{},f{}", instr.d(), instr.b())
        }
        Opcode::Fnabsx => format!("f{},f{}", instr.d(), instr.b()),
        Opcode::Mtfsfx => format!("{},f{}", instr.fm(), instr.b()),
        Opcode::Illegal
        | Opcode::Bclrx
        | Opcode::Bcctrx
        | Opcode::Sc
        | Opcode::Table19
        | Opcode::Table31
        | Opcode::Table59
        | Opcode::Table63
        | Opcode::Rfi
        | Opcode::Isync
        | Opcode::Sync => String::new(),
    }
}
