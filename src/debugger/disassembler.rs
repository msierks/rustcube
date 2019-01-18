use super::super::cpu::instruction::Instruction;
use super::super::cpu::util::*;
use super::super::cpu::Cpu;

#[derive(Default)]
pub struct Disassembler {
    pub opcode: String,
    pub operands: String,
}

impl Disassembler {
    pub fn disassemble(&mut self, cpu: &mut Cpu, instr: Instruction) {
        match instr.opcode() {
            7 => self.mulli(instr),
            8 => self.subfic(instr),
            10 => self.cmpli(instr),
            11 => self.cmpi(instr),
            13 => self.addi(instr, "c."),
            14 => self.addi(instr, ""),
            15 => self.addi(instr, "s"),
            16 => self.bcx(cpu, instr),
            17 => self.sc(),
            18 => self.bx(cpu, instr),
            19 => match instr.ext_opcode_x() {
                16 => self.bclrx(instr),
                150 => self.sync("i"),
                193 => self.crxor(instr),
                528 => self.bcctrx(instr),
                _ => {
                    self.opcode = format!(
                        "unrecognized opcode {}:{}",
                        instr.opcode(),
                        instr.ext_opcode_x()
                    );
                    self.operands = String::new();
                }
            },
            20 => self.rlwimix(instr),
            21 => self.rlwinmx(instr),
            24 => self.ori(instr, ""),
            25 => self.oris(instr),
            28 => self.andi(instr),
            31 => match instr.ext_opcode_x() {
                0 => self.cmp(instr),
                23 => self.lwzx(instr),
                24 => self.slwx(instr),
                26 => self.cntlzwx(instr),
                28 => self.andx(instr),
                32 => self.cmpl(instr),
                40 => self.subfx(instr),
                60 => self.andcx(instr),
                83 => self.mfmsr(instr),
                86 => self.dcbf(instr),
                124 => self.norx(instr),
                146 => self.mtmsr(instr),
                151 => self.stwx(instr),
                210 => self.mtsr(instr),
                266 => self.addx(instr),
                339 => self.mfspr(instr),
                371 => self.mftb(instr),
                444 => self.orx(instr),
                467 => self.mtspr(instr),
                470 => self.cbi("d", instr),
                536 => self.srwx(instr),
                598 => self.sync(""),
                922 => self.extshx(instr),
                982 => self.cbi("i", instr),
                _ => {
                    self.opcode = format!(
                        "unrecognized opcode {}:{}",
                        instr.opcode(),
                        instr.ext_opcode_x()
                    );
                    self.operands = String::new();
                }
            },
            32 => self.lwz(instr),
            33 => self.lwzu(instr),
            34 => self.lbz(instr),
            35 => self.lbzu(instr),
            37 => self.stw(instr, "u"),
            36 => self.stw(instr, ""),
            38 => self.stb(instr),
            39 => self.stbu(instr),
            40 => self.lhz(instr),
            41 => self.lhzu(instr),
            44 => self.sth(instr),
            46 => self.lmw(instr),
            47 => self.stmw(instr),
            48 => self.lfs(instr),
            50 => self.lfd(instr),
            52 => self.stfs(instr),
            53 => self.stfsu(instr),
            63 => match instr.ext_opcode_x() {
                72 => self.fmrx(instr),
                _ => {
                    self.opcode = format!(
                        "unrecognized opcode {}:{}",
                        instr.opcode(),
                        instr.ext_opcode_x()
                    );
                    self.operands = String::new();
                }
            },
            _ => {
                self.opcode = format!("unrecognized opcode {}", instr.opcode());
                self.operands = String::new();
            }
        }
    }

    fn addi(&mut self, instr: Instruction, ext: &str) {
        if ext == "s" {
            self.opcode = String::from("lis");
            self.operands = format!("r{},{:#x}", instr.d(), (instr.simm() as u32) << 16);
        } else if instr.a() == 0 {
            self.opcode = String::from("li");
            self.operands = format!("r{},{}", instr.d(), instr.simm());
        } else {
            self.opcode = format!("addi{}", ext);
            self.operands = format!("r{},r{},{}", instr.d(), instr.a(), instr.simm());
        }
    }

    fn addx(&mut self, instr: Instruction) {
        if instr.oe() {
            if instr.rc() {
                self.opcode = String::from("addo.");
            } else {
                self.opcode = String::from("addo");
            }
        } else if instr.rc() {
            self.opcode = String::from("add.");
        } else {
            self.opcode = String::from("add");
        }

        self.operands = format!("r{},r{},r{}", instr.d(), instr.a(), instr.b());
    }

    fn andi(&mut self, instr: Instruction) {
        self.opcode = String::from("andi.");
        self.operands = format!("r{},r{},{}", instr.a(), instr.s(), instr.uimm());
    }

    fn andx(&mut self, instr: Instruction) {
        if !instr.rc() {
            self.opcode = String::from("and");
        } else {
            self.opcode = String::from("and.");
        }

        self.operands = format!("r{},r{},r{}", instr.a(), instr.s(), instr.b());
    }

    fn bx(&mut self, cpu: &Cpu, instr: Instruction) {
        if instr.aa() == 0 {
            if instr.lk() == 0 {
                self.opcode = String::from("b");
            } else {
                self.opcode = String::from("bl");
            }
        } else if instr.lk() == 0 {
            self.opcode = String::from("ba");
        } else {
            self.opcode = String::from("bla");
        }

        let loc = if instr.aa() == 1 {
            sign_ext_26(instr.li() << 2) as u32
        } else {
            cpu.cia.wrapping_add(sign_ext_26(instr.li() << 2) as u32)
        };

        self.operands = format!("{:#010x}", loc);
    }

    fn bcx(&mut self, cpu: &Cpu, instr: Instruction) {
        let bit = instr.bi() % 4;

        let ext = if instr.aa() == 0 {
            if instr.lk() == 0 {
                ""
            } else {
                "l"
            }
        } else if instr.lk() == 0 {
            "a"
        } else {
            "la"
        };

        if instr.bo() == 4 {
            self.opcode = match bit {
                0 => format!("bge{}", ext),
                1 => format!("ble{}", ext),
                2 => format!("bne{}", ext),
                3 => format!("bns{}", ext),
                _ => unreachable!(),
            };
        } else if instr.bo() == 12 || instr.bo() == 13 {
            self.opcode = match bit {
                0 => format!("blt{}", ext),
                1 => format!("bgt{}", ext),
                2 => format!("beq{}", ext),
                3 => format!("bso{}", ext),
                _ => unreachable!(),
            };
        } else if instr.bo() == 16 && instr.bi() == 0 {
            self.opcode = String::from("bdnz");
        } else {
            panic!("bo {}", instr.bo());
        }

        let loc = if instr.aa() == 1 {
            sign_ext_16(instr.bd() << 2) as u32
        } else {
            cpu.cia.wrapping_add(sign_ext_16(instr.bd() << 2) as u32)
        };

        self.operands = format!("{:#010x}", loc);
    }

    fn bclrx(&mut self, instr: Instruction) {
        let bit = instr.bi() % 4;

        let ext = if instr.aa() == 0 {
            if instr.lk() == 0 {
                ""
            } else {
                "l"
            }
        } else if instr.lk() == 0 {
            "a"
        } else {
            "la"
        };

        self.opcode = match instr.bo() {
            4 => match bit {
                0 => format!("bgelr{}", ext),
                1 => format!("blelr{}", ext),
                2 => format!("bnelr{}", ext),
                3 => format!("bnslr{}", ext),
                _ => unreachable!(),
            },
            12 => match bit {
                0 => format!("bltlr{}", ext),
                1 => format!("bgtlr{}", ext),
                2 => format!("beqlr{}", ext),
                3 => format!("bsolr{}", ext),
                _ => unreachable!(),
            },
            20 => {
                if instr.bi() == 0 {
                    format!("blr{}", ext)
                } else {
                    panic!("invalid instruction")
                }
            }
            _ => panic!("unhandled bo: {}", instr.bo()),
        };

        self.operands = String::new();
    }

    fn cmp(&mut self, instr: Instruction) {
        if instr.l() {
            self.opcode = String::from("cmp");
        } else {
            self.opcode = String::from("cmpw");
        }
        self.operands = format!("r{},r{}", instr.a(), instr.b());
    }

    fn cmpi(&mut self, instr: Instruction) {
        if !instr.l() {
            self.opcode = String::from("cmpwi");
        } else {
            self.opcode = String::from("cmpi");
        }
        self.operands = format!("r{},{}", instr.a(), instr.uimm());
    }

    fn cmpli(&mut self, instr: Instruction) {
        if !instr.l() {
            self.opcode = String::from("cmplwi");
            self.operands = format!("r{},{}", instr.a(), instr.uimm());
        } else {
            self.opcode = String::from("cmpli");
            self.operands = format!("{},r{},{}", instr.l() as u8, instr.a(), instr.uimm());
        }
    }

    fn cmpl(&mut self, instr: Instruction) {
        if !instr.l() {
            self.opcode = String::from("cmplw");
            self.operands = format!("{},r{},r{}", instr.crfd(), instr.a(), instr.b());
        } else {
            self.opcode = String::from("cmpl");
            self.operands = format!(
                "crf{},{},r{},r{}",
                instr.crfd(),
                instr.l() as u8,
                instr.a(),
                instr.b()
            );
        }
    }

    fn sync(&mut self, pre: &str) {
        self.opcode = format!("{}sync", pre);
        self.operands = String::new();
    }

    fn extshx(&mut self, instr: Instruction) {
        if !instr.rc() {
            self.opcode = String::from("extsh");
        } else {
            self.opcode = String::from("extsh.");
        }
        self.operands = format!("r{},r{}", instr.a(), instr.s());
    }

    fn mfmsr(&mut self, instr: Instruction) {
        self.opcode = String::from("mfmsr");
        self.operands = format!("r{}", instr.d());
    }

    fn mtmsr(&mut self, instr: Instruction) {
        self.opcode = String::from("mtmsr");
        self.operands = format!("r{}", instr.s());
    }

    fn mfspr(&mut self, instr: Instruction) {
        match instr.spr() as u32 {
            1 => {
                self.opcode = String::from("mfxer");
                self.operands = format!("r{}", instr.s());
            }
            8 => {
                self.opcode = String::from("mflr");
                self.operands = format!("r{}", instr.s());
            }
            9 => {
                self.opcode = String::from("mfctr");
                self.operands = format!("r{}", instr.s());
            }
            _ => {
                self.opcode = String::from("mfspr");
                self.operands = format!("r{},{}", instr.s(), instr.spr() as u32);
            }
        }
    }

    fn mtspr(&mut self, instr: Instruction) {
        match instr.spr() as u32 {
            1 => {
                self.opcode = String::from("mtxer");
                self.operands = format!("r{}", instr.s());
            }
            8 => {
                self.opcode = String::from("mtlr");
                self.operands = format!("r{}", instr.s());
            }
            9 => {
                self.opcode = String::from("mtctr");
                self.operands = format!("r{}", instr.s());
            }
            _ => {
                self.opcode = String::from("mtspr");
                self.operands = format!("{},r{}", instr.spr() as u32, instr.s());
            }
        }
    }

    fn mtsr(&mut self, instr: Instruction) {
        self.opcode = String::from("mtsr");
        self.operands = format!("{},r{}", instr.sr(), instr.s());
    }

    fn mftb(&mut self, instr: Instruction) {
        self.opcode = String::from("mftb");
        self.operands = format!("r{},{}", instr.d(), instr.tbr() as u32);
    }

    fn mulli(&mut self, instr: Instruction) {
        self.opcode = String::from("mulli");
        self.operands = format!("r{},r{},{}", instr.a(), instr.d(), instr.simm());
    }

    fn subfic(&mut self, instr: Instruction) {
        self.opcode = String::from("subfic");
        self.operands = format!("r{},r{},{}", instr.d(), instr.a(), instr.simm());
    }

    fn ori(&mut self, instr: Instruction, ext: &str) {
        if instr.a() == 0 && instr.s() == 0 && instr.uimm() == 0 {
            self.opcode = String::from("nop");
            self.operands = String::new();
        } else {
            self.opcode = format!("ori{}", ext);
            self.operands = format!("r{},r{},{}", instr.a(), instr.s(), instr.uimm());
        }
    }

    fn oris(&mut self, instr: Instruction) {
        self.opcode = String::from("oris");
        self.operands = format!("r{},r{},{}", instr.a(), instr.s(), instr.uimm());
    }

    fn orx(&mut self, instr: Instruction) {
        if instr.s() == instr.b() {
            self.opcode = String::from("mr");
            self.operands = format!("r{},r{}", instr.a(), instr.s());
        } else {
            if !instr.rc() {
                self.opcode = String::from("or");
            } else {
                self.opcode = String::from("or.");
            }

            self.operands = format!("r{},r{},r{}", instr.a(), instr.s(), instr.b());
        }
    }

    fn norx(&mut self, instr: Instruction) {
        if instr.s() == instr.b() {
            self.opcode = String::from("not");
            self.operands = format!("r{},r{}", instr.a(), instr.s());
        } else {
            if !instr.rc() {
                self.opcode = String::from("nor");
            } else {
                self.opcode = String::from("nor.");
            }
            self.operands = format!("r{},r{},r{}", instr.a(), instr.s(), instr.b());
        }
    }

    fn crxor(&mut self, instr: Instruction) {
        if instr.d() == instr.a() && instr.d() == instr.b() {
            self.opcode = String::from("crclr");
            self.operands = format!("crb{},crb{},crb{}", instr.d(), instr.a(), instr.b());
        } else {
            self.opcode = String::from("crxor");
            self.operands = format!("crb{}", instr.d());
        }
    }

    fn bcctrx(&mut self, instr: Instruction) {
        if instr.bo() == 12 && instr.bi() == 0 {
            self.opcode = String::from("bltctr");
            self.operands = String::new();
        } else if instr.bo() == 4 && instr.bi() == 10 {
            self.opcode = String::from("bnectr");
            self.operands = String::from("cr2");
        } else if instr.lk() == 1 {
            if instr.bo() == 20 {
                self.opcode = String::from("bctrl");
            } else {
                self.opcode = String::from("bcctrl");
            }
            self.operands = String::new();
        } else {
            if instr.bo() == 20 {
                self.opcode = String::from("bctr");
            } else {
                self.opcode = String::from("bcctr");
            }
            self.operands = String::new();
        }
    }

    fn rlwimix(&mut self, instr: Instruction) {
        if !instr.rc() {
            self.opcode = String::from("rlwimi");
        } else {
            self.opcode = String::from("rlwimi.");
        }
        self.operands = format!(
            "r{},r{},{},{},{}",
            instr.a(),
            instr.s(),
            instr.sh(),
            instr.mb(),
            instr.me()
        );
    }

    // FixMe: ???
    fn rlwinmx(&mut self, instr: Instruction) {
        if instr.sh() == 0 && instr.me() == 31 {
            self.opcode = String::from("clrlwi");
            self.operands = format!("r{},r{},{}", instr.a(), instr.s(), instr.mb());
        } else if instr.sh() == 0 && instr.mb() == 0 {
            self.opcode = String::from("clrrwi");
            self.operands = format!("r{},r{},{}", instr.a(), instr.s(), 31 - instr.me());
        } else {
            if !instr.rc() {
                self.opcode = String::from("rlwinm");
            } else {
                self.opcode = String::from("rlwinm.");
            }
            self.operands = format!(
                "r{},r{},{},{},{}",
                instr.a(),
                instr.s(),
                instr.sh(),
                instr.mb(),
                instr.me()
            );
        }
    }

    fn subfx(&mut self, instr: Instruction) {
        self.opcode = String::from("subf");
        self.operands = format!("r{},r{},r{}", instr.d(), instr.a(), instr.b());
    }

    fn andcx(&mut self, instr: Instruction) {
        if !instr.rc() {
            self.opcode = String::from("andc");
        } else {
            self.opcode = String::from("andc.");
        }
        self.operands = format!("r{},r{},r{}", instr.a(), instr.s(), instr.b());
    }

    fn sth(&mut self, instr: Instruction) {
        self.opcode = String::from("sth");
        self.operands = format!("r{},{}(r{})", instr.s(), instr.simm(), instr.a());
    }

    fn lwz(&mut self, instr: Instruction) {
        self.opcode = String::from("lwz");
        self.operands = format!("r{},{}(r{})", instr.d(), instr.simm(), instr.a());
    }

    fn lwzu(&mut self, instr: Instruction) {
        self.opcode = String::from("lwzu");
        self.operands = format!("r{},{}(r{})", instr.d(), instr.simm(), instr.a());
    }

    fn lhz(&mut self, instr: Instruction) {
        self.opcode = String::from("lhz");
        self.operands = format!("r{},{}(r{})", instr.d(), instr.simm(), instr.a());
    }

    fn lhzu(&mut self, instr: Instruction) {
        self.opcode = String::from("lhzu");
        self.operands = format!("r{},{}(r{})", instr.d(), instr.simm(), instr.a());
    }

    fn stw(&mut self, instr: Instruction, ext: &str) {
        self.opcode = format!("stw{}", ext);
        self.operands = format!("r{},{}(r{})", instr.s(), instr.simm(), instr.a());
    }

    fn stmw(&mut self, instr: Instruction) {
        self.opcode = String::from("stmw");
        self.operands = format!("r{},{}(r{})", instr.s(), instr.simm(), instr.a());
    }

    fn lmw(&mut self, instr: Instruction) {
        self.opcode = String::from("lmw");
        self.operands = format!("r{},{}(r{})", instr.d(), instr.uimm(), instr.a());
    }

    fn cbi(&mut self, pre: &str, instr: Instruction) {
        self.opcode = format!("{}cbi", pre);
        self.operands = format!("r{},r{}", instr.a(), instr.b());
    }

    fn lbz(&mut self, instr: Instruction) {
        self.opcode = String::from("lbz");
        self.operands = format!("r{},{}(r{})", instr.d(), instr.uimm(), instr.a());
    }

    fn lbzu(&mut self, instr: Instruction) {
        self.opcode = String::from("lbzu");
        self.operands = format!("r{},{}(r{})", instr.d(), instr.uimm(), instr.a());
    }

    fn stb(&mut self, instr: Instruction) {
        self.opcode = String::from("stb");
        self.operands = format!("r{},{}(r{})", instr.s(), instr.uimm(), instr.a());
    }

    fn stbu(&mut self, instr: Instruction) {
        self.opcode = String::from("stbu");
        self.operands = format!("r{},{}(r{})", instr.s(), instr.uimm(), instr.a());
    }

    fn stwx(&mut self, instr: Instruction) {
        self.opcode = String::from("stwx");
        self.operands = format!("r{},r{},r{}", instr.s(), instr.a(), instr.b());
    }

    fn lfs(&mut self, instr: Instruction) {
        self.opcode = String::from("lfs");
        self.operands = format!("fr{},{}(r{})", instr.d(), instr.uimm(), instr.a());
    }

    fn lfd(&mut self, instr: Instruction) {
        self.opcode = String::from("lfd");
        self.operands = format!("fr{},{}(r{})", instr.d(), instr.uimm(), instr.a());
    }

    fn stfs(&mut self, instr: Instruction) {
        self.opcode = String::from("stfs");
        self.operands = format!("fr{},{}(r{})", instr.s(), instr.uimm(), instr.a());
    }

    fn stfsu(&mut self, instr: Instruction) {
        self.opcode = String::from("stfsu");
        self.operands = format!("fr{},{}(r{})", instr.s(), instr.uimm(), instr.a());
    }

    fn fmrx(&mut self, instr: Instruction) {
        if !instr.rc() {
            self.opcode = String::from("fmrx");
        } else {
            self.opcode = String::from("fmrx.");
        }
        self.operands = format!("fr{},fr{}", instr.d(), instr.b());
    }

    fn lwzx(&mut self, instr: Instruction) {
        self.opcode = String::from("lwzx");
        self.operands = format!("r{},r{},r{}", instr.d(), instr.a(), instr.b());
    }

    fn slwx(&mut self, instr: Instruction) {
        if !instr.rc() {
            self.opcode = String::from("slw");
        } else {
            self.opcode = String::from("slw.");
        }
        self.operands = format!("r{},r{},r{}", instr.a(), instr.s(), instr.b());
    }

    fn cntlzwx(&mut self, instr: Instruction) {
        if !instr.rc() {
            self.opcode = String::from("cntlzw");
        } else {
            self.opcode = String::from("cntlzw.");
        }
        self.operands = format!("r{},r{}", instr.a(), instr.s());
    }

    fn dcbf(&mut self, instr: Instruction) {
        self.opcode = String::from("dcbf");
        self.operands = format!("r{},r{}", instr.a(), instr.b());
    }

    fn sc(&mut self) {
        self.opcode = String::from("sc");
        self.operands = String::new();
    }

    fn srwx(&mut self, instr: Instruction) {
        if !instr.rc() {
            self.opcode = String::from("srw");
        } else {
            self.opcode = String::from("srw.");
        }
        self.operands = format!("r{},r{},r{}", instr.a(), instr.s(), instr.b());
    }
}
