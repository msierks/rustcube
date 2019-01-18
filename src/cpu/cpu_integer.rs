fn mask(x: u8, y: u8) -> u32 {
    let mut mask: u32 = 0xFFFF_FFFF >> x;

    if y >= 31 {
        mask ^= 0;
    } else {
        mask ^= 0xFFFF_FFFF >> (y + 1)
    };

    if y < x {
        !mask
    } else {
        mask
    }
}

impl Cpu {
    fn addi(&mut self, instr: Instruction) {
        if instr.a() == 0 {
            self.gpr[instr.d()] = i32::from(instr.simm()) as u32;
        } else {
            self.gpr[instr.d()] = self.gpr[instr.a()].wrapping_add(i32::from(instr.simm()) as u32);
        }
    }

    fn addic(&mut self, instr: Instruction) {
        let ra = self.gpr[instr.a()];
        let imm = i32::from(instr.simm()) as u32;

        self.gpr[instr.d()] = ra.wrapping_add(imm);

        self.xer.carry = ra > !imm;
    }

    fn addic_rc(&mut self, instr: Instruction) {
        let ra = self.gpr[instr.a()];
        let imm = i32::from(instr.simm()) as u32;

        self.gpr[instr.d()] = ra.wrapping_add(imm);

        self.xer.carry = ra > !imm;

        self.cr.update_cr0(self.gpr[instr.d()], &self.xer);
    }

    fn addis(&mut self, instr: Instruction) {
        if instr.a() == 0 {
            self.gpr[instr.d()] = instr.uimm() << 16;
        } else {
            self.gpr[instr.d()] = self.gpr[instr.a()].wrapping_add(instr.uimm() << 16);
        }
    }

    fn addex(&mut self, instr: Instruction) {
        let a = self.gpr[instr.a()];
        let b = self.gpr[instr.b()];

        self.gpr[instr.d()] = a.wrapping_add(b).wrapping_add(self.xer.carry as u32);

        // FixMe: update carry

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.d()], &self.xer);
        }

        if instr.oe() {
            panic!("OE: addex");
        }
    }

    fn addzex(&mut self, instr: Instruction) {
        let carry = self.xer.carry as u32;
        let ra = self.gpr[instr.a()];

        self.gpr[instr.d()] = ra.wrapping_add(carry);

        self.xer.carry = ra > !carry;

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.d()], &self.xer);
        }

        if instr.oe() {
            panic!("OE: addzex");
        }
    }

    fn addx(&mut self, instr: Instruction) {
        self.gpr[instr.d()] = self.gpr[instr.a()].wrapping_add(self.gpr[instr.b()]);

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.d()], &self.xer);
        }

        if instr.oe() {
            panic!("OE: addx");
        }
    }

    fn addcx(&mut self, instr: Instruction) {
        let a = self.gpr[instr.a()];
        let b = self.gpr[instr.b()];

        self.gpr[instr.d()] = a.wrapping_add(b);

        self.xer.carry = a > !b;

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.d()], &self.xer);
        }

        if instr.oe() {
            panic!("OE: addcx");
        }
    }

    fn andx(&mut self, instr: Instruction) {
        self.gpr[instr.a()] = self.gpr[instr.s()] & self.gpr[instr.b()];

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.a()], &self.xer);
        }
    }

    fn andcx(&mut self, instr: Instruction) {
        self.gpr[instr.a()] = self.gpr[instr.s()] & (!self.gpr[instr.b()]);

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.a()], &self.xer);
        }
    }

    fn andi_rc(&mut self, instr: Instruction) {
        self.gpr[instr.a()] = self.gpr[instr.s()] & instr.uimm();

        self.cr.update_cr0(self.gpr[instr.a()], &self.xer);
    }

    fn cmp(&mut self, instr: Instruction) {
        let a = self.gpr[instr.a()] as i32;
        let b = self.gpr[instr.b()] as i32;

        let mut c: u8 = if a < b {
            0b1000
        } else if a > b {
            0b0100
        } else {
            0b0010
        };

        c |= self.xer.summary_overflow as u8;

        self.cr.set_field(instr.crfd(), c);
    }

    fn cmpi(&mut self, instr: Instruction) {
        if instr.l() {
            panic!("cmpi: invalid instruction");
        }

        let a = self.gpr[instr.a()] as i32;
        let b = i32::from(instr.simm());

        let mut c: u8 = if a < b {
            0b1000
        } else if a > b {
            0b0100
        } else {
            0b0010
        };

        c |= self.xer.summary_overflow as u8;

        self.cr.set_field(instr.crfd(), c);
    }

    fn cmpl(&mut self, instr: Instruction) {
        if instr.l() {
            panic!("cmpl: invalid instruction");
        }

        let a = self.gpr[instr.a()];
        let b = self.gpr[instr.b()];

        let mut c: u8 = if a < b {
            0b1000
        } else if a > b {
            0b0100
        } else {
            0b0010
        };

        c |= self.xer.summary_overflow as u8;

        self.cr.set_field(instr.crfd(), c);
    }

    fn cmpli(&mut self, instr: Instruction) {
        if instr.l() {
            panic!("cmpli: invalid instruction");
        }

        let a = self.gpr[instr.a()];
        let b = instr.uimm();

        let mut c: u8 = if a < b {
            0b1000
        } else if a > b {
            0b0100
        } else {
            0b0010
        };

        c |= self.xer.summary_overflow as u8;

        self.cr.set_field(instr.crfd(), c);
    }

    fn cntlzwx(&mut self, instr: Instruction) {
        let mut n = 0;
        let mut mask = 0x8000_0000;
        let s = self.gpr[instr.s()];

        while n < 32 {
            n += 1;
            mask >>= 1;

            if (s & mask) != 0 {
                break;
            }
        }

        self.gpr[instr.a()] = n;

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.a()], &self.xer);
        }
    }

    fn divwx(&mut self, instr: Instruction) {
        let a = self.gpr[instr.a()] as i32;
        let b = self.gpr[instr.b()] as i32;

        if b == 0 || (a as u32 == 0x8000_0000 && b == -1) {
            if instr.oe() {
                panic!("OE: divwx");
            }

            if a as u32 == 0x8000_0000 && b == 0 {
                self.gpr[instr.d()] = 0xFFFF_FFFF;
            } else {
                self.gpr[instr.d()] = 0;
            }
        } else {
            self.gpr[instr.d()] = (a / b) as u32;
        }

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.d()], &self.xer);
        }
    }

    fn divwux(&mut self, instr: Instruction) {
        let a = self.gpr[instr.a()];
        let b = self.gpr[instr.b()];

        if b == 0 {
            if instr.oe() {
                panic!("OE: divwux");
            }

            self.gpr[instr.d()] = 0;
        } else {
            self.gpr[instr.d()] = a / b;
        }

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.d()], &self.xer);
        }
    }

    fn extsbx(&mut self, instr: Instruction) {
        self.gpr[instr.a()] = i32::from(self.gpr[instr.s()] as i8) as u32;

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.a()], &self.xer);
        }
    }

    fn extshx(&mut self, instr: Instruction) {
        self.gpr[instr.a()] = i32::from(self.gpr[instr.s()] as i16) as u32;

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.a()], &self.xer);
        }
    }

    fn mulhwux(&mut self, instr: Instruction) {
        let a = u64::from(self.gpr[instr.a()]);
        let b = u64::from(self.gpr[instr.b()]);

        self.gpr[instr.d()] = ((a * b) >> 32) as u32;

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.d()], &self.xer);
        }
    }

    fn mulli(&mut self, instr: Instruction) {
        self.gpr[instr.d()] =
            (self.gpr[instr.a()] as i32).wrapping_mul(i32::from(instr.simm())) as u32;
    }

    fn mullwx(&mut self, instr: Instruction) {
        let a = self.gpr[instr.a()] as i32;
        let b = self.gpr[instr.b()] as i32;

        self.gpr[instr.d()] = a.wrapping_mul(b) as u32;

        if instr.oe() {
            panic!("OE: mullwx");
        }

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.d()], &self.xer);
        }
    }

    fn negx(&mut self, instr: Instruction) {
        self.gpr[instr.d()] = !(self.gpr[instr.a()]) + 1;

        // FixMe: ???

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.d()], &self.xer);
        }
    }

    fn norx(&mut self, instr: Instruction) {
        self.gpr[instr.a()] = !(self.gpr[instr.s()] | self.gpr[instr.b()]);

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.a()], &self.xer);
        }
    }

    fn orx(&mut self, instr: Instruction) {
        self.gpr[instr.a()] = self.gpr[instr.s()] | self.gpr[instr.b()];

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.a()], &self.xer);
        }
    }

    fn ori(&mut self, instr: Instruction) {
        self.gpr[instr.a()] = self.gpr[instr.s()] | instr.uimm();
    }

    fn oris(&mut self, instr: Instruction) {
        self.gpr[instr.a()] = self.gpr[instr.s()] | (instr.uimm() << 16);
    }

    fn rlwimix(&mut self, instr: Instruction) {
        let m = mask(instr.mb(), instr.me());
        let r = self.gpr[instr.s()].rotate_left(u32::from(instr.sh()));

        self.gpr[instr.a()] = (r & m) | (self.gpr[instr.a()] & !m);

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.a()], &self.xer);
        }
    }

    fn rlwinmx(&mut self, instr: Instruction) {
        let mask = mask(instr.mb(), instr.me());

        self.gpr[instr.a()] = (self.gpr[instr.s()].rotate_left(u32::from(instr.sh()))) & mask;

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.a()], &self.xer);
        }
    }

    fn slwx(&mut self, instr: Instruction) {
        let r = self.gpr[instr.b()];

        self.gpr[instr.a()] = if r & 0x20 != 0 {
            0
        } else {
            self.gpr[instr.s()] << (r & 0x1F)
        };

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.a()], &self.xer);
        }
    }

    fn srawx(&mut self, instr: Instruction) {
        let rb = self.gpr[instr.b()];

        if rb & 0x20 != 0 {
            if self.gpr[instr.s()] & 0x8000_0000 != 0 {
                self.gpr[instr.a()] = 0xFFFF_FFFF;
                self.xer.carry = true;
            } else {
                self.gpr[instr.a()] = 0;
                self.xer.carry = false;
            }
        } else {
            let n = rb & 0x1F;

            if n != 0 {
                let rs = self.gpr[instr.s()] as i32;

                self.gpr[instr.a()] = (rs >> n) as u32;

                if rs < 0 && (rs << (32 - n) != 0) {
                    self.xer.carry = true;
                } else {
                    self.xer.carry = false;
                }
            } else {
                self.gpr[instr.a()] = self.gpr[instr.s()];
                self.xer.carry = false;
            }
        }

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.a()], &self.xer);
        }
    }

    fn srawix(&mut self, instr: Instruction) {
        let n = instr.sh();

        if n != 0 {
            let rs = self.gpr[instr.s()] as i32;

            self.gpr[instr.a()] = (rs >> n) as u32;

            if rs < 0 && (rs << (32 - n) != 0) {
                self.xer.carry = true;
            } else {
                self.xer.carry = false;
            }
        } else {
            self.gpr[instr.a()] = self.gpr[instr.s()];
            self.xer.carry = false;
        }
    }

    fn srwx(&mut self, instr: Instruction) {
        let r = self.gpr[instr.b()];

        self.gpr[instr.a()] = if r & 0x20 != 0 {
            0
        } else {
            self.gpr[instr.s()] >> (r & 0x1F)
        };

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.a()], &self.xer);
        }
    }

    fn subfx(&mut self, instr: Instruction) {
        self.gpr[instr.d()] = self.gpr[instr.b()].wrapping_sub(self.gpr[instr.a()]);

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.d()], &self.xer);
        }

        if instr.oe() {
            panic!("OE: subfx");
        }
    }

    fn subfcx(&mut self, instr: Instruction) {
        let ra = !self.gpr[instr.a()];
        let rb = self.gpr[instr.b()] + 1;

        self.gpr[instr.d()] = ra.wrapping_add(rb);

        self.xer.carry = (self.gpr[instr.a()]) < ra; // FixMe: ???

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.d()], &self.xer);
        }

        if instr.oe() {
            panic!("OE: subfcx");
        }
    }

    fn subfex(&mut self, instr: Instruction) {
        let ra = self.gpr[instr.a()];
        let rb = self.gpr[instr.b()];

        self.gpr[instr.d()] = !ra.wrapping_add(rb).wrapping_add(self.xer.carry as u32);

        //self.xer.carry = (self.gpr[instr.a()]) < ra; // FixMe: ???

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.d()], &self.xer);
        }

        if instr.oe() {
            panic!("OE: subfex");
        }
    }

    fn subfic(&mut self, instr: Instruction) {
        let ra = !self.gpr[instr.a()] as i32;
        let imm = i32::from(instr.simm()) + 1;

        self.gpr[instr.d()] = ra.wrapping_add(imm) as u32;

        self.xer.carry = (self.gpr[instr.a()] as i32) < ra; // FixMe: ???
    }

    fn subfzex(&mut self, instr: Instruction) {
        let ra = self.gpr[instr.a()];
        self.gpr[instr.d()] = (!ra) + self.xer.carry as u32;

        self.xer.carry = ra > self.xer.carry as u32;

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.d()], &self.xer);
        }

        if instr.oe() {
            panic!("OE: subfex");
        }
    }

    fn xorx(&mut self, instr: Instruction) {
        self.gpr[instr.a()] = self.gpr[instr.s()] ^ self.gpr[instr.b()];

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.a()], &self.xer);
        }
    }

    fn xoris(&mut self, instr: Instruction) {
        self.gpr[instr.a()] = self.gpr[instr.s()] ^ (instr.uimm() << 16)
    }
}
