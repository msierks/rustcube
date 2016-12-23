
impl Cpu {
    #[allow(unused_variables)]
    fn dcbf(&mut self, instr: Instruction) {
        //println!("FixMe: dcbf");
    }

    #[allow(unused_variables)]
    fn dcbi(&mut self, instr: Instruction) {
        //println!("FixMe: dcbi");
    }

    #[allow(unused_variables)]
    fn icbi(&mut self, instr: Instruction) {
        //println!("FixMe: icbi");
    }

    fn lbz(&mut self, instr: Instruction) {
        let ea = if instr.a() == 0 {
            instr.simm() as u32
        } else {
            self.gpr[instr.a()].wrapping_add(instr.simm() as u32)
        };

        self.gpr[instr.d()] = self.interconnect.read_u8(&self.msr, ea) as u32;
    }

    fn lbzu(&mut self, instr: Instruction) {
        if instr.a() == 0 || instr.a() == instr.d() {
            panic!("lbzu: invalid instruction");
        }

        let ea   = self.gpr[instr.a()].wrapping_add(instr.simm() as u32);

        self.gpr[instr.d()] = self.interconnect.read_u8(&self.msr, ea) as u32;
        self.gpr[instr.a()] = ea;
    }

    fn lbzx(&mut self, instr: Instruction) {
        let ea = if instr.a() == 0 {
            self.gpr[instr.b()]
        } else {
            self.gpr[instr.a()].wrapping_add(self.gpr[instr.b()])
        };

        self.gpr[instr.d()] = self.interconnect.read_u8(&self.msr, ea) as u32;
    }

    fn lfd(&mut self, instr: Instruction) {
        let ea = if instr.a() == 0 {
            instr.simm() as u32
        } else {
            self.gpr[instr.a()].wrapping_add(instr.simm() as u32)
        };

        self.fpr[instr.d()] = self.interconnect.read_u64(&self.msr, ea);
    }

    fn lfs(&mut self, instr: Instruction) {
        let ea = if instr.a() == 0 {
            instr.simm() as u32
        } else {
            self.gpr[instr.a()].wrapping_add(instr.simm() as u32)
        };

        let val = self.interconnect.read_u32(&self.msr, ea);

        if !self.hid2.paired_single {
            self.fpr[instr.d()] = convert_to_double(val);
        } else {
            self.fpr[instr.d()] = ((val as u64) << 32) & val as u64;
        }
    }

    fn lha(&mut self, instr: Instruction) {
        let ea = if instr.a() == 0 {
            instr.simm() as u32
        } else {
            self.gpr[instr.a()].wrapping_add(instr.simm() as u32)
        };

        self.gpr[instr.d()] = ((self.interconnect.read_u16(&self.msr, ea) as i16) as i32) as u32;
    }

    fn lhz(&mut self, instr: Instruction) {
        let ea = if instr.a() == 0 {
            instr.simm() as u32
        } else {
            self.gpr[instr.a()].wrapping_add(instr.simm() as u32)
        };

        self.gpr[instr.d()] = self.interconnect.read_u16(&self.msr, ea) as u32;
    }

    fn lhzu(&mut self, instr: Instruction) {
        let ea = self.gpr[instr.a()].wrapping_add(instr.simm() as u32);

        self.gpr[instr.d()] = self.interconnect.read_u16(&self.msr, ea) as u32;
        self.gpr[instr.a()] = ea;
    }

    fn lmw(&mut self, instr: Instruction) {
        let mut ea = if instr.a() == 0 {
            instr.simm() as u32
        } else {
            self.gpr[instr.a()].wrapping_add(instr.simm() as u32)
        };

        let mut r = instr.d();

        while r <= 31 {
            self.gpr[r] = self.interconnect.read_u32(&self.msr, ea);

            r  += 1;
            ea += 4;
        }
    }

    fn lwz(&mut self, instr: Instruction) {
        let ea = if instr.a() == 0 {
            instr.simm() as u32
        } else {
            self.gpr[instr.a()].wrapping_add(instr.simm() as u32)
        };

        self.gpr[instr.d()] = self.interconnect.read_u32(&self.msr, ea);
    }

    fn lwzx(&mut self, instr: Instruction) {
        let ea = if instr.a() == 0 {
            self.gpr[instr.b()]
        } else {
            self.gpr[instr.a()].wrapping_add(self.gpr[instr.b()])
        };

        self.gpr[instr.d()] = self.interconnect.read_u32(&self.msr, ea);
    }

    fn lwzu(&mut self, instr: Instruction) {
        let ea = self.gpr[instr.a()].wrapping_add(instr.simm() as u32);

        self.gpr[instr.d()] = self.interconnect.read_u32(&self.msr, ea);
        self.gpr[instr.a()] = ea;
    }

    fn psq_l(&mut self, instr: Instruction) {
        if !self.hid2.paired_single || !self.hid2.load_stored_quantized {
            panic!("FixMe: GoTo illegal instruction handler");
        }

        let ea = if instr.a() == 0 {
            sign_ext_12(instr.uimm_1()) as u32
        } else {
            self.gpr[instr.a()].wrapping_add(sign_ext_12(instr.uimm_1()) as u32)
        };

        let gqr = Gqr(self.gqr[instr.i()]);

        match gqr.ld_type() {
            0 => {
                if instr.w() {
                    let value = self.interconnect.read_u32(&self.msr, ea);

                    let ps0 = value;
                    let ps1 = 1.0;

                    self.fpr[instr.d()] = ((ps0 as u64) << 32) | (ps1 as u64);
                } else {
                    let value = (
                        self.interconnect.read_u32(&self.msr, ea),
                        self.interconnect.read_u32(&self.msr, ea + 4)
                    );

                    self.fpr[instr.d()] = ((value.0 as u64) << 32) | (value.1 as u64);
                }
            },
            4 | 6 =>  panic!("FixMe:..."),
            5 | 7 =>  panic!("FixMe:..."),
            _ => panic!("unrecognized ld_type")
        }
    }

    fn psq_st(&mut self, instr: Instruction) {
        if !self.hid2.paired_single || !self.hid2.load_stored_quantized {
            panic!("FixMe: GoTo illegal instruction handler");
        }

        let ea = if instr.a() == 0 {
            sign_ext_12(instr.uimm_1()) as u32
        } else {
            self.gpr[instr.a()].wrapping_add(sign_ext_12(instr.uimm_1()) as u32)
        };

        let gqr = Gqr(self.gqr[instr.i()]);

        match gqr.st_type() {
            0 => { // single-precision floating-point (no-conversion)
                if instr.w() {
                    let ps0 = (self.fpr[instr.d()] >> 32) as u32;

                    self.interconnect.write_u32(&self.msr, ea, ps0);
                } else {
                    self.interconnect.write_u64(&self.msr, ea, self.fpr[instr.d()]);
                }
            },
            4 | 6 => panic!("FixMe:..."), // unsigned 8 bit integer | signed 8 bit integer
            5 | 7 => panic!("FixMe:..."), // unsigned 16 bit integer | signed 16 bit integer
            _ => panic!("unrecognized st_type")
        }
    }

    fn stb(&mut self, instr: Instruction, debugger: &mut Debugger) {
        let ea = if instr.a() == 0 {
            instr.simm() as u32
        } else {
            self.gpr[instr.a()].wrapping_add(instr.simm() as u32)
        };

        self.interconnect.write_u8(&self.msr, ea, self.gpr[instr.d()] as u8);

        debugger.memory_write(self, ea);
    }

    fn stbu(&mut self, instr: Instruction, debugger: &mut Debugger) {
        let ea = self.gpr[instr.a()].wrapping_add(instr.simm() as u32);

        self.interconnect.write_u8(&self.msr, ea, self.gpr[instr.d()] as u8);

        debugger.memory_write(self, ea);

        self.gpr[instr.a()] = ea;
    }

    fn stfd(&mut self, instr: Instruction, debugger: &mut Debugger) {
        let ea = if instr.a() == 0 {
            instr.simm() as u32
        } else {
            self.gpr[instr.a()].wrapping_add(instr.simm() as u32)
        };

        self.interconnect.write_u64(&self.msr, ea, self.fpr[instr.s()]);

        debugger.memory_write(self, ea);
    }

    fn stfs(&mut self, instr: Instruction, debugger: &mut Debugger) {
        let ea = if instr.a() == 0 {
            instr.simm() as u32
        } else {
            self.gpr[instr.a()].wrapping_add(instr.simm() as u32)
        };

        let val = convert_to_single(self.fpr[instr.s()]);

        self.interconnect.write_u32(&self.msr, ea, val);

        debugger.memory_write(self, ea);
    }

    fn stfsu(&mut self, instr: Instruction, debugger: &mut Debugger) {
        if instr.a() == 0 {
            panic!("stfsu: invalid instruction");
        }

        let ea  = self.gpr[instr.a()].wrapping_add(instr.simm() as u32);
        let val = convert_to_single(self.fpr[instr.s()]);

        self.interconnect.write_u32(&self.msr, ea, val);

        debugger.memory_write(self, ea);
    }

    fn sth(&mut self, instr: Instruction, debugger: &mut Debugger) {
        let ea = if instr.a() == 0 {
            instr.simm() as u32
        } else {
            self.gpr[instr.a()].wrapping_add(instr.simm() as u32)
        };

        self.interconnect.write_u16(&self.msr, ea, self.gpr[instr.s()] as u16);

        debugger.memory_write(self, ea);
    }

    fn sthu(&mut self, instr: Instruction, debugger: &mut Debugger) {
        let ea = if instr.a() == 0 {
            instr.simm() as u32
        } else {
            self.gpr[instr.a()].wrapping_add(instr.simm() as u32)
        };

        self.interconnect.write_u16(&self.msr, ea, self.gpr[instr.s()] as u16);

        debugger.memory_write(self, ea);

        self.gpr[instr.a()] = ea;
    }

    fn stmw(&mut self, instr: Instruction, debugger: &mut Debugger) {
        let mut ea = if instr.a() == 0 {
            instr.simm() as u32
        } else {
            self.gpr[instr.a()].wrapping_add(instr.simm() as u32)
        };

        let mut r = instr.s();

        while r <= 31 {
            self.interconnect.write_u32(&self.msr, ea, self.gpr[r]);

            debugger.memory_write(self, ea);

            r  += 1;
            ea += 4;
        }
    }

    fn stw(&mut self, instr: Instruction, debugger: &mut Debugger) {
        let ea = if instr.a() == 0 {
            instr.simm() as u32
        } else {
            self.gpr[instr.a()].wrapping_add(instr.simm() as u32)
        };

        // TODO: remove this at some point
        // enable devkit mode
        if self.cia == 0x813004c4 {
            self.interconnect.write_u32(&self.msr, ea, 0x10000006);
        } else {
            self.interconnect.write_u32(&self.msr, ea, self.gpr[instr.s()]);
        }

        debugger.memory_write(self, ea);
    }

    fn stwx(&mut self, instr: Instruction, debugger: &mut Debugger) {
        let ea = if instr.a() == 0 {
            self.gpr[instr.b()]
        } else {
            self.gpr[instr.a()].wrapping_add(self.gpr[instr.b()])
        };

        self.interconnect.write_u32(&self.msr, ea, self.gpr[instr.s()]);

        debugger.memory_write(self, ea);
    }

    fn stwu(&mut self, instr: Instruction, debugger: &mut Debugger) {
        if instr.a() == 0 {
            panic!("stwu: invalid instruction");
        }

        let ea = self.gpr[instr.a()].wrapping_add(instr.simm() as u32);

        self.interconnect.write_u32(&self.msr, ea, self.gpr[instr.s()]);

        debugger.memory_write(self, ea);

        self.gpr[instr.a()] = ea; // is this conditional ???
    }

}