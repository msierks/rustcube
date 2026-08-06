use std::cmp::Ordering;

use super::{
    instruction::Instruction,
    utils::{check_overflowed, mask},
    Cpu, ProgramException,
};
use crate::bus::Bus;

impl Cpu {
    pub fn op_addcx(&mut self, instr: Instruction, _: &mut Bus) {
        let ra = self.gpr[instr.a()];
        let rb = self.gpr[instr.b()];

        let (rd, ca) = ra.overflowing_add(rb);

        self.gpr[instr.d()] = rd;

        self.xer.set_carry(ca);

        if instr.oe() {
            self.set_xer_so(check_overflowed(ra, rb, rd));
        }

        if instr.rc() {
            self.update_cr0(rd);
        }

        self.tick(1);
    }

    pub fn op_addx(&mut self, instr: Instruction, _: &mut Bus) {
        let ra = self.gpr[instr.a()];
        let rb = self.gpr[instr.b()];
        let rd = ra.wrapping_add(rb);

        self.gpr[instr.d()] = rd;

        if instr.oe() {
            self.set_xer_so(check_overflowed(ra, rb, rd));
        }

        if instr.rc() {
            self.update_cr0(rd);
        }

        self.tick(1);
    }

    pub fn op_addi(&mut self, instr: Instruction, _: &mut Bus) {
        self.gpr[instr.d()] = if instr.a() == 0 {
            i32::from(instr.simm()) as u32
        } else {
            self.gpr[instr.a()].wrapping_add(i32::from(instr.simm()) as u32)
        };

        self.tick(1);
    }

    pub fn op_addic(&mut self, instr: Instruction, _: &mut Bus) {
        let ra = self.gpr[instr.a()];
        let imm = i32::from(instr.simm()) as u32;

        let (rd, ca) = ra.overflowing_add(imm);

        self.gpr[instr.d()] = rd;

        self.xer.set_carry(ca);

        self.tick(1);
    }

    pub fn op_addic_rc(&mut self, instr: Instruction, _: &mut Bus) {
        let ra = self.gpr[instr.a()];
        let imm = i32::from(instr.simm()) as u32;

        let (rd, ca) = ra.overflowing_add(imm);

        self.gpr[instr.d()] = rd;

        self.xer.set_carry(ca);

        self.update_cr0(rd);

        self.tick(1);
    }

    pub fn op_addis(&mut self, instr: Instruction, _: &mut Bus) {
        self.gpr[instr.d()] = if instr.a() == 0 {
            instr.uimm() << 16
        } else {
            self.gpr[instr.a()].wrapping_add(instr.uimm() << 16)
        };

        self.tick(1);
    }

    pub fn op_addmex(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_addmex");
    }

    pub fn op_addex(&mut self, instr: Instruction, _: &mut Bus) {
        let ra = self.gpr[instr.a()];
        let rb = self.gpr[instr.b()];

        let (rd, ca1) = ra.overflowing_add(rb);
        let (rd, ca2) = rd.overflowing_add(self.xer.carry() as u32);

        self.gpr[instr.d()] = rd;

        self.xer.set_carry(ca1 | ca2);

        if instr.oe() {
            self.set_xer_so(check_overflowed(ra, rb, rd));
        }

        if instr.rc() {
            self.update_cr0(rd);
        }

        self.tick(1);
    }

    pub fn op_addzex(&mut self, instr: Instruction, _: &mut Bus) {
        let ra = self.gpr[instr.a()];

        let (rd, ca) = ra.overflowing_add(self.xer.carry() as u32);

        self.gpr[instr.d()] = rd;

        self.xer.set_carry(ca);

        if instr.oe() {
            self.set_xer_so(check_overflowed(ra, 0, rd));
        }

        if instr.rc() {
            self.update_cr0(rd);
        }

        self.tick(1);
    }

    pub fn op_andcx(&mut self, instr: Instruction, _: &mut Bus) {
        let ra = self.gpr[instr.s()] & (!self.gpr[instr.b()]);

        self.gpr[instr.a()] = ra;

        if instr.rc() {
            self.update_cr0(ra);
        }

        self.tick(1);
    }

    pub fn op_andi_rc(&mut self, instr: Instruction, _: &mut Bus) {
        let ra = self.gpr[instr.s()] & instr.uimm();

        self.gpr[instr.a()] = ra;

        self.update_cr0(ra);

        self.tick(1);
    }

    pub fn op_andis_rc(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_andis_rc");
    }

    pub fn op_andx(&mut self, instr: Instruction, _: &mut Bus) {
        let ra = self.gpr[instr.s()] & self.gpr[instr.b()];

        self.gpr[instr.a()] = ra;

        if instr.rc() {
            self.update_cr0(ra);
        }

        self.tick(1);
    }

    pub fn op_cmp(&mut self, instr: Instruction, _: &mut Bus) {
        let ra = self.gpr[instr.a()] as i32;
        let rb = self.gpr[instr.b()] as i32;

        let mut c = match ra.cmp(&rb) {
            Ordering::Less => 0x8,
            Ordering::Greater => 0x4,
            Ordering::Equal => 0x2,
        };

        c |= self.xer.summary_overflow() as u32;

        self.cr.set_field(instr.crfd(), c);

        self.tick(1);
    }

    pub fn op_cmpi(&mut self, instr: Instruction, _: &mut Bus) {
        if instr.l() {
            panic!("cmpi: invalid instruction");
        }

        let ra = self.gpr[instr.a()] as i32;
        let simm = i32::from(instr.simm());

        let mut c = match ra.cmp(&simm) {
            Ordering::Less => 0x8,
            Ordering::Greater => 0x4,
            Ordering::Equal => 0x2,
        };

        c |= self.xer.summary_overflow() as u32;

        self.cr.set_field(instr.crfd(), c);

        self.tick(1);
    }

    pub fn op_cmpl(&mut self, instr: Instruction, _: &mut Bus) {
        if instr.l() {
            panic!("cmpl: invalid instruction");
        }

        let ra = self.gpr[instr.a()];
        let rb = self.gpr[instr.b()];

        let mut c = match ra.cmp(&rb) {
            Ordering::Less => 0x8,
            Ordering::Greater => 0x4,
            Ordering::Equal => 0x2,
        };

        c |= self.xer.summary_overflow() as u32;

        self.cr.set_field(instr.crfd(), c);

        self.tick(1);
    }

    pub fn op_cmpli(&mut self, instr: Instruction, _: &mut Bus) {
        if instr.l() {
            panic!("cmpli: invalid instruction");
        }

        let ra = self.gpr[instr.a()];
        let uimm = instr.uimm();

        let mut c = match ra.cmp(&uimm) {
            Ordering::Less => 0x8,
            Ordering::Greater => 0x4,
            Ordering::Equal => 0x2,
        };

        c |= self.xer.summary_overflow() as u32;

        self.cr.set_field(instr.crfd(), c);

        self.tick(1);
    }

    pub fn op_cntlzwx(&mut self, instr: Instruction, _: &mut Bus) {
        let n = self.gpr[instr.s()].leading_zeros();

        self.gpr[instr.a()] = n;

        if instr.rc() {
            self.update_cr0(n);
        }

        self.tick(1);
    }

    pub fn op_divwux(&mut self, instr: Instruction, _: &mut Bus) {
        let ra = self.gpr[instr.a()];
        let rb = self.gpr[instr.b()];
        let overflow = rb == 0;

        let rd = if overflow { 0 } else { ra / rb };

        self.gpr[instr.d()] = rd;

        if instr.oe() {
            self.set_xer_so(overflow);
        }

        if instr.rc() {
            self.update_cr0(rd);
        }

        self.tick(19);
    }

    // TODO: review this implementation
    pub fn op_divwx(&mut self, instr: Instruction, _: &mut Bus) {
        let ra = self.gpr[instr.a()] as i32;
        let rb = self.gpr[instr.b()] as i32;
        let overflow = rb == 0 || (ra as u32 == 0x8000_0000 && rb == -1);

        let rd = if overflow {
            if ra as u32 == 0x8000_0000 && rb == 0 {
                0xFFFF_FFFF
            } else {
                0
            }
        } else {
            (ra / rb) as u32
        };

        self.gpr[instr.d()] = rd;

        if instr.oe() {
            self.set_xer_so(overflow);
        }

        if instr.rc() {
            self.update_cr0(rd);
        }

        self.tick(19);
    }

    pub fn op_eqvx(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_eqvx");
    }

    pub fn op_extsbx(&mut self, instr: Instruction, _: &mut Bus) {
        let ra = ((self.gpr[instr.s()] as i8) as i32) as u32;

        self.gpr[instr.a()] = ra;

        if instr.rc() {
            self.update_cr0(ra);
        }

        self.tick(1);
    }

    pub fn op_extshx(&mut self, instr: Instruction, _: &mut Bus) {
        let ra = ((self.gpr[instr.s()] as i16) as i32) as u32;

        self.gpr[instr.a()] = ra;

        if instr.rc() {
            self.update_cr0(ra);
        }

        self.tick(1);
    }

    pub fn op_mulhwux(&mut self, instr: Instruction, _: &mut Bus) {
        let ra = self.gpr[instr.a()] as u64;
        let rb = self.gpr[instr.b()] as u64;

        let rd = ((ra * rb) >> 32) as u32;

        self.gpr[instr.d()] = rd;

        if instr.rc() {
            self.update_cr0(rd);
        }

        self.tick(2);
    }

    pub fn op_mulhwx(&mut self, instr: Instruction, _: &mut Bus) {
        let ra = (self.gpr[instr.a()] as i32) as i64;
        let rb = (self.gpr[instr.b()] as i32) as i64;

        let rd = ((ra * rb) >> 32) as u32;

        self.gpr[instr.d()] = rd;

        if instr.rc() {
            self.update_cr0(rd);
        }

        self.tick(2);
    }

    // TODO: review this implementation
    pub fn op_mulli(&mut self, instr: Instruction, _: &mut Bus) {
        self.gpr[instr.d()] =
            (self.gpr[instr.a()] as i32).wrapping_mul(i32::from(instr.simm())) as u32;

        self.tick(2);
    }

    pub fn op_mullwx(&mut self, instr: Instruction, _: &mut Bus) {
        let ra = (self.gpr[instr.a()] as i32) as i64;
        let rb = (self.gpr[instr.b()] as i32) as i64;

        let rd = ra.wrapping_mul(rb);

        self.gpr[instr.d()] = rd as u32;

        if instr.oe() {
            self.set_xer_so(!(-0x8000_0000..=0x7FFF_FFFF).contains(&rd));
        }

        if instr.rc() {
            self.update_cr0(rd as u32);
        }

        self.tick(2);
    }

    pub fn op_nandx(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_nandx");
    }

    pub fn op_negx(&mut self, instr: Instruction, _: &mut Bus) {
        let ra = self.gpr[instr.a()];
        let rd = (!ra).wrapping_add(1);

        self.gpr[instr.d()] = rd;

        if instr.oe() {
            self.set_xer_so(ra == 0x8000_0000);
        }

        if instr.rc() {
            self.update_cr0(rd);
        }

        self.tick(1);
    }

    pub fn op_norx(&mut self, instr: Instruction, _: &mut Bus) {
        let ra = !(self.gpr[instr.s()] | self.gpr[instr.b()]);

        self.gpr[instr.a()] = ra;

        if instr.rc() {
            self.update_cr0(ra);
        }

        self.tick(1);
    }

    pub fn op_orcx(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_orcx");
    }

    pub fn op_ori(&mut self, instr: Instruction, _: &mut Bus) {
        self.gpr[instr.a()] = self.gpr[instr.s()] | instr.uimm();

        self.tick(1);
    }

    pub fn op_oris(&mut self, instr: Instruction, _: &mut Bus) {
        self.gpr[instr.a()] = self.gpr[instr.s()] | (instr.uimm() << 16);

        self.tick(1);
    }

    pub fn op_orx(&mut self, instr: Instruction, _: &mut Bus) {
        let ra = self.gpr[instr.s()] | self.gpr[instr.b()];

        self.gpr[instr.a()] = ra;

        if instr.rc() {
            self.update_cr0(ra);
        }

        self.tick(1);
    }

    pub fn op_rlwimix(&mut self, instr: Instruction, _: &mut Bus) {
        let m = mask(instr.mb(), instr.me());

        let ra = (self.gpr[instr.a()] & !m) | (self.gpr[instr.s()].rotate_left(instr.sh()) & m);

        self.gpr[instr.a()] = ra;

        if instr.rc() {
            self.update_cr0(ra);
        }

        self.tick(1);
    }

    pub fn op_rlwinmx(&mut self, instr: Instruction, _: &mut Bus) {
        let mask = mask(instr.mb(), instr.me());

        let ra = (self.gpr[instr.s()].rotate_left(instr.sh())) & mask;

        self.gpr[instr.a()] = ra;

        if instr.rc() {
            self.update_cr0(ra);
        }

        self.tick(1);
    }

    pub fn op_rlwnmx(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_rlwnmx");
    }

    pub fn op_slwx(&mut self, instr: Instruction, _: &mut Bus) {
        let rb = self.gpr[instr.b()];

        let ra = if rb & 0x20 != 0 {
            0
        } else {
            self.gpr[instr.s()] << (rb & 0x1F)
        };

        self.gpr[instr.a()] = ra;

        if instr.rc() {
            self.update_cr0(ra);
        }

        self.tick(1);
    }

    pub fn op_srawix(&mut self, instr: Instruction, _: &mut Bus) {
        let rs = self.gpr[instr.s()] as i32;
        let sh = instr.sh();

        self.gpr[instr.a()] = (rs >> sh) as u32;

        if sh != 0 {
            self.xer
                .set_carry(rs < 0 && ((rs as u32) << (32 - sh)) != 0);
        } else {
            self.xer.set_carry(false);
        }

        if instr.rc() {
            self.update_cr0(self.gpr[instr.a()]);
        }

        self.tick(1);
    }

    // TODO: review this implementation
    pub fn op_srawx(&mut self, instr: Instruction, _: &mut Bus) {
        let rb = self.gpr[instr.b()];

        if rb & 0x20 != 0 {
            if self.gpr[instr.s()] & 0x8000_0000 != 0 {
                self.gpr[instr.a()] = 0xFFFF_FFFF;
                self.xer.set_carry(true);
            } else {
                self.gpr[instr.a()] = 0;
                self.xer.set_carry(false);
            }
        } else {
            let n = rb & 0x1F;

            if n != 0 {
                let rs = self.gpr[instr.s()] as i32;

                self.gpr[instr.a()] = (rs >> n) as u32;

                self.xer.set_carry(rs < 0 && (rs << (32 - n) != 0));
            } else {
                self.gpr[instr.a()] = self.gpr[instr.s()];
                self.xer.set_carry(false);
            }
        }

        if instr.rc() {
            self.update_cr0(self.gpr[instr.a()]);
        }

        self.tick(1);
    }

    pub fn op_srwx(&mut self, instr: Instruction, _: &mut Bus) {
        let rb = self.gpr[instr.b()];

        let ra = if rb & 0x20 != 0 {
            0
        } else {
            self.gpr[instr.s()] >> (rb & 0x1F)
        };

        self.gpr[instr.a()] = ra;

        if instr.rc() {
            self.update_cr0(ra);
        }

        self.tick(1);
    }

    pub fn op_subfcx(&mut self, instr: Instruction, _: &mut Bus) {
        let ra = self.gpr[instr.a()];
        let rb = self.gpr[instr.b()];

        let (rd, ca1) = (!ra).overflowing_add(rb);
        let (rd, ca2) = rd.overflowing_add(1);

        self.gpr[instr.d()] = rd;

        self.xer.set_carry(ca1 || ca2);

        if instr.oe() {
            self.set_xer_so(check_overflowed(!ra, rb, rd));
        }

        if instr.rc() {
            self.update_cr0(rd);
        }

        self.tick(1);
    }

    pub fn op_subfex(&mut self, instr: Instruction, _: &mut Bus) {
        let ra = self.gpr[instr.a()];
        let rb = self.gpr[instr.b()];

        let (rd, ca1) = (!ra).overflowing_add(rb);
        let (rd, ca2) = rd.overflowing_add(self.xer.carry() as u32);

        self.gpr[instr.d()] = rd;

        self.xer.set_carry(ca1 | ca2);

        if instr.oe() {
            self.set_xer_so(check_overflowed(!ra, rb, rd));
        }

        if instr.rc() {
            self.update_cr0(rd);
        }

        self.tick(1);
    }

    pub fn op_subfic(&mut self, instr: Instruction, _: &mut Bus) {
        let ra = self.gpr[instr.a()];
        let simm = (instr.simm() as i32) as u32;

        let (rd, ca1) = (!ra).overflowing_add(simm);
        let (rd, ca2) = rd.overflowing_add(1);

        self.gpr[instr.d()] = rd;

        self.xer.set_carry(ca1 || ca2);

        self.tick(1);
    }

    pub fn op_subfmex(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_subfmex");
    }

    pub fn op_subfzex(&mut self, instr: Instruction, _: &mut Bus) {
        let ra = self.gpr[instr.a()];
        let ca = self.xer.carry() as u32;

        let rd = (!ra).wrapping_add(ca);

        self.gpr[instr.d()] = rd;

        self.xer.set_carry(ca > ra);

        if instr.rc() {
            self.update_cr0(rd);
        }

        if instr.oe() {
            panic!("OE: subfzex");
        }

        self.tick(1);
    }

    pub fn op_subfx(&mut self, instr: Instruction, _: &mut Bus) {
        let ra = self.gpr[instr.a()] as i32;
        let rb = self.gpr[instr.b()] as i32;

        let (rd, ov) = rb.overflowing_sub(ra);
        let rd = rd as u32;

        self.gpr[instr.d()] = rd;

        if instr.oe() {
            self.set_xer_so(ov);
        }

        if instr.rc() {
            self.update_cr0(rd);
        }

        self.tick(1);
    }

    pub fn op_tw(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_tw");
    }

    pub fn op_twi(&mut self, instr: Instruction, _: &mut Bus) {
        let a = self.gpr[instr.a()] as i32;
        let simm = instr.simm() as i32;
        let to = instr.to();

        if (a < simm && (to & 0x10) != 0)
            || (a > simm && (to & 0x08) != 0)
            || (a == simm && (to & 0x04) != 0)
            || ((a as u32) < simm as u32 && (to & 0x02) != 0)
            || ((a as u32) > simm as u32 && (to & 0x01) != 0)
        {
            self.generate_program_exception(ProgramException::Trap);
        }
    }

    pub fn op_xori(&mut self, instr: Instruction, _: &mut Bus) {
        self.gpr[instr.a()] = self.gpr[instr.s()] ^ instr.uimm();

        self.tick(1);
    }

    pub fn op_xoris(&mut self, instr: Instruction, _: &mut Bus) {
        self.gpr[instr.a()] = self.gpr[instr.s()] ^ (instr.uimm() << 16);

        self.tick(1);
    }

    pub fn op_xorx(&mut self, instr: Instruction, _: &mut Bus) {
        self.gpr[instr.a()] = self.gpr[instr.s()] ^ self.gpr[instr.b()];

        if instr.rc() {
            self.update_cr0(self.gpr[instr.a()]);
        }

        self.tick(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::{EXCEPTION_PROGRAM, SPR_SRR0, SPR_SRR1};

    #[test]
    fn op_addi() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (rd, ra, simm) = (4, 5, 0x8FF0);
        let instr = Instruction::new_addi(rd, ra, simm);

        cpu.gpr[ra] = 0x0000_0900;
        cpu.op_addi(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0xFFFF_98F0);
    }

    #[test]
    fn op_addic() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (rd, ra, simm) = (6, 4, 0xFFFF);
        let instr = Instruction::new_addic(rd, ra, simm);

        cpu.gpr[ra] = 0x0000_2346;

        cpu.op_addic(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x0000_2345)
    }

    #[test]
    fn op_addic_rc() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();
        let (rd, ra, simm) = (31, 3, 1);
        let instr = Instruction::new_addic_rc(rd, ra, simm);

        cpu.gpr[rd] = 0xDEAD_BEEF;
        cpu.gpr[ra] = 0xFFFF_FFFF;

        cpu.op_addic_rc(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x0);
        assert_eq!(cpu.gpr[ra], 0xFFFF_FFFF); // confirm gpr source remains unmodified
        assert!(cpu.xer.carry());

        cpu.gpr[ra] = 0xFFFF_FFFE;

        cpu.op_addic_rc(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0xFFFF_FFFF);
        assert!(!cpu.xer.carry());
    }

    #[test]
    fn op_addis() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (rd, ra, simm) = (7, 6, 0x0011);
        let instr = Instruction::new_addis(rd, ra, simm);

        cpu.gpr[ra] = 0x0000_4000;
        cpu.op_addis(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x0011_4000);
    }

    #[test]
    fn op_addex() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (rd, ra, rb) = (6, 4, 10);
        let instr = Instruction::new_addex(rd, ra, rb);

        cpu.gpr[ra] = 0x1000_0400;
        cpu.gpr[rb] = 0x1000_0400;
        cpu.xer.set_carry(true);
        cpu.op_addex(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x2000_0801);

        cpu.gpr[ra] = 0x9000_3000;
        cpu.gpr[rb] = 0x7B41_92C0;
        cpu.xer.set_carry(false);
        cpu.op_addex(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x0B41_C2C0);

        cpu.gpr[ra] = 0x1000_0400;
        cpu.gpr[rb] = 0xEFFF_FFFF;
        cpu.xer.set_carry(true);
        cpu.op_addex(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x0000_0400);

        cpu.gpr[ra] = 0x9000_3000;
        cpu.gpr[rb] = 0x8000_7000;
        cpu.xer.set_carry(false);
        cpu.op_addex(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x1000_A000);
    }

    #[test]
    fn op_addzex() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (rd, ra) = (6, 4);
        let instr = Instruction::new_addzex(rd, ra);
        cpu.gpr[ra] = 0x7B41_92C0;
        cpu.xer.set_carry(false);
        cpu.op_addzex(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x7B41_92C0);

        cpu.gpr[ra] = 0xEFFF_FFFF;
        cpu.xer.set_carry(true);
        cpu.op_addzex(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0xF000_0000);

        cpu.gpr[ra] = 0x9000_3000;
        cpu.xer.set_carry(true);
        cpu.op_addzex(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x9000_3001);

        cpu.gpr[ra] = 0xEFFF_FFFF;
        cpu.xer.set_carry(false);
        cpu.op_addzex(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0xEFFF_FFFF);
    }

    #[test]
    fn op_addx() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (rd, ra, rb) = (4, 6, 3);
        let instr = Instruction::new_addx(rd, ra, rb);
        cpu.gpr[ra] = 0x0004_0000;
        cpu.gpr[rb] = 0x0000_4000;
        cpu.op_addx(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x0004_4000);
        assert!(!cpu.xer.carry());

        let instr = instr.set_rc(1);

        cpu.gpr[ra] = 0x8000_7000;
        cpu.gpr[rb] = 0x7000_8000;
        cpu.op_addx(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0xF000_F000);
        assert!(!cpu.xer.carry());
        assert_eq!(cpu.cr.get_cr0(), 0x8); // LT

        cpu.gpr[ra] = 0xEFFF_FFFF;
        cpu.gpr[rb] = 0x8000_0000;
        cpu.op_addx(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x6FFF_FFFF);
        // FIXME: check Summary Overflow, Overflow and carry bits are set in Fixed point
        // register

        cpu.gpr[ra] = 0xEFFF_FFFF;
        cpu.gpr[rb] = 0xEFFF_FFFF;
        cpu.op_addx(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0xDFFF_FFFE);
        // FIXME: check check Summary Overflow, Overflow and carry bits are set in Fixed point
        // register, as well as condition register field 0 updated
    }

    #[test]
    fn op_addcx() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (rd, ra, rb) = (6, 4, 10);
        let instr = Instruction::new_addcx(rd, ra, rb);
        cpu.gpr[ra] = 0x9000_3000;
        cpu.gpr[rb] = 0x8000_7000;
        cpu.op_addcx(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x1000_A000);

        let instr = instr.set_rc(1);

        cpu.gpr[ra] = 0x7000_3000;
        cpu.gpr[rb] = 0xFFFF_FFFF;
        cpu.op_addcx(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x7000_2FFF);
        assert_eq!(cpu.cr.get_cr0(), 0x4); // GT

        let instr = instr.set_oe(1);

        cpu.gpr[ra] = 0x9000_3000;
        cpu.gpr[rb] = 0x7B41_92C0;
        cpu.op_addcx(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x0B41_C2C0);
        assert!(cpu.xer.carry());
        // FIXME: check Summary Overflow and Overflow bits are set

        cpu.gpr[ra] = 0x8000_0000;
        cpu.gpr[rb] = 0x8000_7000;
        cpu.xer.set_carry(false);
        cpu.op_addcx(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x0000_7000);
        assert!(cpu.xer.carry());
        assert_eq!(cpu.cr.get_cr0(), 0x5); // GT, SO

        // FIXME: check Summery Overflow and Overflow bits set
    }

    #[test]
    fn op_andx() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (ra, rs, rb) = (6, 4, 7);
        let instr = Instruction::new_andx(ra, rs, rb);

        cpu.gpr[rs] = 0xFFF2_5730;
        cpu.gpr[rb] = 0x7B41_92C0;
        cpu.op_andx(instr, &mut bus);

        assert_eq!(cpu.gpr[ra], 0x7B40_1200);

        let instr = instr.set_rc(1);

        cpu.gpr[rs] = 0xFFF2_5730;
        cpu.gpr[rb] = 0xFFFF_EFFF;
        cpu.op_andx(instr, &mut bus);

        assert_eq!(cpu.gpr[ra], 0xFFF2_4730);
        assert_eq!(cpu.cr.get_cr0(), 0x8); // LT
    }

    #[test]
    fn op_andcx() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (ra, rs, rb) = (6, 4, 5);
        let instr = Instruction::new_andcx(ra, rs, rb);

        cpu.gpr[rs] = 0x9000_3000;
        cpu.gpr[rb] = 0xFFFF_FFFF;
        cpu.op_andcx(instr, &mut bus);

        assert_eq!(cpu.gpr[ra], 0x0000_0000);

        let instr = instr.set_rc(1);

        cpu.gpr[rs] = 0xB004_3000;
        cpu.gpr[rb] = 0x7676_7676;
        cpu.op_andcx(instr, &mut bus);

        assert_eq!(cpu.gpr[ra], 0x8000_0000);
        assert_eq!(cpu.cr.get_cr0(), 0x8); // LT
    }

    #[test]
    fn op_andi_rc() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (ra, rs) = (6, 4);
        let uimm = 0x5730;
        let instr = Instruction::new_andi_rc(ra, rs, uimm);

        cpu.gpr[rs] = 0x7B41_92C0;
        cpu.op_andi_rc(instr, &mut bus);

        assert_eq!(cpu.gpr[ra], 0x0000_1200);
        assert_eq!(cpu.cr.get_cr0(), 0x4); // GT
    }

    #[test]
    fn op_cmp() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (crfd, l, ra, rb) = (0, 0, 4, 6);
        let instr = Instruction::new_cmp(crfd, l, ra, rb);

        cpu.gpr[ra] = 0xFFFF_FFE7;
        cpu.gpr[rb] = 0x0000_0011;
        cpu.op_cmp(instr, &mut bus);

        assert_eq!(cpu.cr.get_cr0(), 0x8); // LT
    }

    #[test]
    fn op_cmpi() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (crfd, l, ra, simm) = (0, 0, 4, 0x11);
        let instr = Instruction::new_cmpi(crfd, l, ra, simm);

        cpu.gpr[ra] = 0xFFFF_FFE7;
        cpu.op_cmpi(instr, &mut bus);

        assert_eq!(cpu.cr.get_cr0(), 0x8); // LT
    }

    #[test]
    fn op_cmpl() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (crfd, l, ra, rb) = (0, 0, 4, 5);
        let instr = Instruction::new_cmpl(crfd, l, ra, rb);

        cpu.gpr[ra] = 0xFFFF_0000;
        cpu.gpr[rb] = 0x7FFF_0000;
        cpu.op_cmpl(instr, &mut bus);

        assert_eq!(cpu.cr.get_cr0(), 0x4); // GT
    }

    #[test]
    fn op_cmpli() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (crfd, l, ra, uimm) = (0, 0, 4, 0xFF);
        let instr = Instruction::new_cmpli(crfd, l, ra, uimm);

        cpu.gpr[ra] = 0x0000_00FF;
        cpu.op_cmpli(instr, &mut bus);

        assert_eq!(cpu.cr.get_cr0(), 0x2); // EQ
    }

    #[test]
    fn op_cntlzwx() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (ra, rs) = (3, 3);
        let instr = Instruction::new_cntlzwx(ra, rs);

        cpu.gpr[ra] = 0x0061_9920;
        cpu.op_cntlzwx(instr, &mut bus);

        assert_eq!(cpu.gpr[rs], 0x0000_0009);
    }

    #[test]
    fn op_divwx() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (rd, ra, rb) = (4, 4, 6);
        let instr = Instruction::new_divwx(rd, ra, rb);

        cpu.gpr[ra] = 0x0000_0000;
        cpu.gpr[rb] = 0x0000_0002;
        cpu.op_divwx(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x0000_0000);

        let instr = instr.set_rc(1);

        cpu.gpr[ra] = 0x0000_0002;
        cpu.gpr[rb] = 0x0000_0002;
        cpu.op_divwx(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x0000_0001);
        assert_eq!(cpu.cr.get_cr0(), 0x4); // GT

        cpu.gpr[ra] = 0x0000_0001;
        cpu.gpr[rb] = 0x0000_0002;
        cpu.op_divwx(instr, &mut bus);

        // Undefined
        assert_eq!(cpu.gpr[rd], 0x0000_0000);

        cpu.gpr[ra] = 0x8000_0000;
        cpu.gpr[rb] = 0xFFFF_FFFF;
        cpu.op_divwx(instr, &mut bus);

        // Undefined
        assert_eq!(cpu.gpr[rd], 0x0000_0000);
    }

    #[test]
    fn op_divwux() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (rd, ra, rb) = (4, 4, 6);
        let instr = Instruction::new_divwux(rd, ra, rb);

        cpu.gpr[ra] = 0x0000_0000;
        cpu.gpr[rb] = 0x0000_0002;
        cpu.op_divwux(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x0000_0000);

        let instr = instr.set_rc(1);

        cpu.gpr[ra] = 0x0000_0002;
        cpu.gpr[rb] = 0x0000_0002;
        cpu.op_divwux(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x0000_0001);
        assert_eq!(cpu.cr.get_cr0(), 0x4); // GT

        cpu.gpr[ra] = 0x0000_0001;
        cpu.gpr[rb] = 0x0000_0002;
        cpu.op_divwux(instr, &mut bus);

        // Undefined
        assert_eq!(cpu.gpr[rd], 0x0000_0000);

        cpu.gpr[ra] = 0x8000_0000;
        cpu.gpr[rb] = 0xFFFF_FFFF;
        cpu.op_divwux(instr, &mut bus);

        // Undefined
        assert_eq!(cpu.gpr[rd], 0x0000_0000);
    }

    #[test]
    fn op_extsbx() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (ra, rs) = (4, 6);
        let instr = Instruction::new_extsbx(ra, rs);

        cpu.gpr[rs] = 0x5A5A_5A5A;
        cpu.op_extsbx(instr, &mut bus);

        assert_eq!(cpu.gpr[ra], 0x0000_005A);

        cpu.gpr[rs] = 0xA5A5_A5A5;
        cpu.op_extsbx(instr, &mut bus);

        assert_eq!(cpu.gpr[ra], 0xFFFF_FFA5);
    }

    #[test]
    fn op_extshx() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (ra, rs) = (4, 6);
        let instr = Instruction::new_extshx(ra, rs);

        cpu.gpr[rs] = 0x0000_FFFF;
        cpu.op_extshx(instr, &mut bus);

        assert_eq!(cpu.gpr[ra], 0xFFFF_FFFF);

        cpu.gpr[rs] = 0x0000_2FFF;
        cpu.op_extshx(instr, &mut bus);

        assert_eq!(cpu.gpr[ra], 0x0000_2FFF);
    }

    #[test]
    fn op_mulhwux() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (rd, ra, rb) = (6, 4, 10);
        let instr = Instruction::new_mulhwux(rd, ra, rb);

        cpu.gpr[ra] = 0x0000_0003;
        cpu.gpr[rb] = 0x0000_0002;
        cpu.op_mulhwux(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x0000_0000);

        let instr = instr.set_rc(1);

        cpu.gpr[ra] = 0x0000_4500;
        cpu.gpr[rb] = 0x8000_7000;
        cpu.op_mulhwux(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x0000_2280);
        assert_eq!(cpu.cr.get_cr0(), 0x4); // GT
    }

    #[test]
    fn op_mulli() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (rd, ra, simm) = (6, 4, 10);
        let instr = Instruction::new_mulli(rd, ra, simm);

        cpu.gpr[ra] = 0x0000_3000;
        cpu.op_mulli(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x0001_E000);
    }

    #[test]
    fn op_mullwx() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (rd, ra, rb) = (6, 4, 10);
        let instr = Instruction::new_mullwx(rd, ra, rb);

        cpu.gpr[ra] = 0x0000_3000;
        cpu.gpr[rb] = 0x0000_7000;
        cpu.op_mullwx(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x1500_0000);

        let instr = instr.set_rc(1);

        cpu.gpr[ra] = 0x0000_4500;
        cpu.gpr[rb] = 0x0000_7000;
        cpu.op_mullwx(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x1E30_0000);
        assert_eq!(cpu.cr.get_cr0(), 0x4); // GT

        let instr = instr.set_oe(1);

        cpu.gpr[ra] = 0x0000_4500;
        cpu.gpr[rb] = 0x0007_0000;
        cpu.op_mullwx(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0xE300_0000);
        // FIXME: check summary overflow and overflow

        cpu.gpr[ra] = 0x0000_4500;
        cpu.gpr[rb] = 0x7FFF_FFFF;
        cpu.op_mullwx(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0xFFFF_BB00);
        // FIXME: check summary overflow and overflow
        assert_eq!(cpu.cr.get_cr0(), 0x9); // LT SO
    }

    #[test]
    fn op_negx() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (rd, ra) = (6, 4);
        let instr = Instruction::new_negx(rd, ra);

        cpu.gpr[ra] = 0x9000_3000;
        cpu.op_negx(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x6FFF_D000);

        let instr = instr.set_rc(1);

        cpu.gpr[ra] = 0x789A_789B;
        cpu.op_negx(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x8765_8765);
        assert_eq!(cpu.cr.get_cr0(), 0x8); // LT

        cpu.gpr[ra] = 0x9000_3000;
        cpu.op_negx(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x6FFF_D000);
        // FIXME: check summary overflow and overflow bits

        cpu.gpr[ra] = 0x8000_0000;
        cpu.op_negx(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x8000_0000);
        assert_eq!(cpu.cr.get_cr0(), 0x8); // LT

        // FIXME: check summary overflow and overflow bits
    }

    #[test]
    fn op_norx() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (ra, rs, rb) = (6, 4, 7);
        let instr = Instruction::new_norx(ra, rs, rb);

        cpu.gpr[rs] = 0x9000_3000;
        cpu.gpr[rb] = 0x789A_789B;
        cpu.op_norx(instr, &mut bus);

        assert_eq!(cpu.gpr[ra], 0x0765_8764);

        let instr = instr.set_rc(1);

        cpu.gpr[rs] = 0xB004_3000;
        cpu.gpr[rb] = 0x789A_789B;
        cpu.op_norx(instr, &mut bus);

        assert_eq!(cpu.gpr[ra], 0x0761_8764);
        assert_eq!(cpu.cr.get_cr0(), 0x4); // GT
    }

    #[test]
    fn op_orx() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (ra, rs, rb) = (6, 4, 7);
        let instr = Instruction::new_orx(ra, rs, rb);

        cpu.gpr[rs] = 0x9000_3000;
        cpu.gpr[rb] = 0x789A_789B;
        cpu.op_orx(instr, &mut bus);

        assert_eq!(cpu.gpr[ra], 0xF89A_789B);

        let instr = instr.set_rc(1);

        cpu.gpr[rs] = 0xB004_3000;
        cpu.gpr[rb] = 0x789A_789B;
        cpu.op_orx(instr, &mut bus);

        assert_eq!(cpu.gpr[ra], 0xF89E_789B);
        assert_eq!(cpu.cr.get_cr0(), 0x8); // LT
    }

    #[test]
    fn op_ori() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (rs, ra, uimm) = (6, 4, 0x0079);
        let instr = Instruction::new_ori(ra, rs, uimm);

        cpu.gpr[rs] = 0x9000_3000;
        cpu.op_ori(instr, &mut bus);

        assert_eq!(cpu.gpr[ra], 0x9000_3079);
    }

    #[test]
    fn op_oris() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (rs, ra, uimm) = (6, 4, 0x0079);
        let instr = Instruction::new_oris(ra, rs, uimm);

        cpu.gpr[rs] = 0x9000_3000;
        cpu.op_oris(instr, &mut bus);

        assert_eq!(cpu.gpr[ra], 0x9079_3000);
    }

    #[test]
    fn op_rlwimix() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (ra, rs, sh, mb, me) = (6, 4, 2, 0, 0x1D);
        let instr = Instruction::new_rlwimix(ra, rs, sh, mb, me);

        cpu.gpr[rs] = 0x9000_3000;
        cpu.gpr[ra] = 0x0000_0003;
        cpu.op_rlwimix(instr, &mut bus);

        assert_eq!(cpu.gpr[ra], 0x4000_C003);

        let (mb, me) = (0, 0x1A);
        let instr = Instruction::new_rlwimix(ra, rs, sh, mb, me).set_rc(1);

        cpu.gpr[rs] = 0x789A_789B;
        cpu.gpr[ra] = 0x3000_0003;
        cpu.op_rlwimix(instr, &mut bus);

        assert_eq!(cpu.gpr[ra], 0xE269_E263);
        assert_eq!(cpu.cr.get_cr0(), 0x8); // LT
    }

    #[test]
    fn op_rlwinmx() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (ra, rs, sh, mb, me) = (6, 4, 2, 0, 0x1D);
        let instr = Instruction::new_rlwinmx(ra, rs, sh, mb, me);

        cpu.gpr[rs] = 0x9000_3000;
        cpu.gpr[ra] = 0xFFFF_FFFF;
        cpu.op_rlwinmx(instr, &mut bus);

        assert_eq!(cpu.gpr[ra], 0x4000_C000);

        let instr = instr.set_rc(1);

        cpu.gpr[rs] = 0xB004_3000;
        cpu.gpr[ra] = 0xFFFF_FFFF;
        cpu.op_rlwinmx(instr, &mut bus);

        assert_eq!(cpu.gpr[ra], 0xC010_C000);
        assert_eq!(cpu.cr.get_cr0(), 0x8); // LT
    }

    #[test]
    fn op_slwx() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (ra, rs, rb) = (6, 4, 5);
        let instr = Instruction::new_slwx(ra, rs, rb);

        cpu.gpr[rb] = 0x0000_002F;
        cpu.gpr[rs] = 0xFFFF_FFFF;
        cpu.op_slwx(instr, &mut bus);

        assert_eq!(cpu.gpr[ra], 0x0000_0000);

        let instr = instr.set_rc(1);

        cpu.gpr[rb] = 0x0000_0005;
        cpu.gpr[rs] = 0xB004_3000;
        cpu.op_slwx(instr, &mut bus);

        assert_eq!(cpu.gpr[ra], 0x0086_0000);
        assert_eq!(cpu.cr.get_cr0(), 0x4); // GT
    }

    #[test]
    fn op_srawx() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (ra, rs, rb) = (6, 4, 5);
        let instr = Instruction::new_srawx(ra, rs, rb);

        cpu.gpr[rs] = 0x9000_3000;
        cpu.gpr[rb] = 0x0000_0024;
        cpu.op_srawx(instr, &mut bus);

        assert_eq!(cpu.gpr[ra], 0xFFFF_FFFF);
        assert!(cpu.xer.carry());

        let instr = instr.set_rc(1);

        cpu.gpr[rs] = 0xB004_3000;
        cpu.gpr[rb] = 0x0000_0004;
        cpu.op_srawx(instr, &mut bus);

        assert_eq!(cpu.gpr[ra], 0xFB00_4300);
        assert_eq!(cpu.cr.get_cr0(), 0x8); // LT

        //assert_eq!(cpu.xer.carry(), true);
    }

    #[test]
    fn op_srawix() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (ra, rs, sh) = (6, 3, 0x4);
        let instr = Instruction::new_srawix(ra, rs, sh);

        cpu.gpr[rs] = 0x9000_3000;
        cpu.op_srawix(instr, &mut bus);

        assert_eq!(cpu.gpr[ra], 0xF900_0300);
        assert!(!cpu.xer.carry());

        cpu.gpr[rs] = 0xB004_3008;
        cpu.op_srawix(instr, &mut bus);

        assert_eq!(cpu.gpr[ra], 0xFB00_4300);
        assert!(cpu.xer.carry());

        let instr = instr.set_rc(1);
        cpu.gpr[rs] = 0x8000_0001;
        cpu.op_srawix(instr, &mut bus);

        assert_eq!(cpu.gpr[ra], 0xF800_0000);
        assert!(cpu.xer.carry());
        assert_eq!(cpu.cr.get_cr0(), 0x8); // LT

        let instr = Instruction::new_srawix(ra, rs, 0);
        cpu.gpr[rs] = 0x8000_0001;
        cpu.xer.set_carry(true);
        cpu.op_srawix(instr, &mut bus);

        assert_eq!(cpu.gpr[ra], 0x8000_0001);
        assert!(!cpu.xer.carry());
    }

    #[test]
    fn op_srwx() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (ra, rs, rb) = (6, 4, 5);
        let instr = Instruction::new_srwx(ra, rs, rb);

        cpu.gpr[rs] = 0x9000_3000;
        cpu.gpr[rb] = 0x0000_0024;
        cpu.op_srwx(instr, &mut bus);

        assert_eq!(cpu.gpr[ra], 0x0000_0000);

        let instr = instr.set_rc(1);

        cpu.gpr[rs] = 0xB004_3001;
        cpu.gpr[rb] = 0x0000_0004;
        cpu.op_srwx(instr, &mut bus);

        assert_eq!(cpu.gpr[ra], 0x0B00_4300);
        assert_eq!(cpu.cr.get_cr0(), 0x4); // GT
    }

    #[test]
    fn op_subfx() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (rd, ra, rb) = (6, 4, 10);
        let instr = Instruction::new_subfx(rd, ra, rb);

        cpu.gpr[ra] = 0x8000_7000;
        cpu.gpr[rb] = 0x9000_3000;
        cpu.op_subfx(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x0FFF_C000);

        let instr = instr.set_rc(1);

        cpu.gpr[ra] = 0x0000_4500;
        cpu.gpr[rb] = 0x8000_7000;
        cpu.op_subfx(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x8000_2B00);
        assert_eq!(cpu.cr.get_cr0(), 0x8); // LT

        let instr = instr.set_oe(1);

        cpu.gpr[ra] = 0x8000_0000;
        cpu.gpr[rb] = 0x0000_4500;
        cpu.op_subfx(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x8000_4500);
        // FIXME: check SO and O

        cpu.gpr[ra] = 0x8000_0000;
        cpu.gpr[rb] = 0x0000_7000;
        cpu.op_subfx(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x8000_7000);
        assert_eq!(cpu.cr.get_cr0(), 0x9); // LT

        // FIXME: check SO and O
    }

    #[test]
    fn op_subfcx() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (rd, ra, rb) = (6, 4, 10);
        let instr = Instruction::new_subfcx(rd, ra, rb);

        cpu.gpr[ra] = 0x8000_7000;
        cpu.gpr[rb] = 0x9000_3000;
        cpu.op_subfcx(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x0FFF_C000);
        assert!(cpu.xer.carry());

        let instr = instr.set_rc(1);

        cpu.gpr[ra] = 0x0000_4500;
        cpu.gpr[rb] = 0x8000_7000;
        cpu.op_subfcx(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x8000_2B00);
        assert!(cpu.xer.carry());
        assert_eq!(cpu.cr.get_cr0(), 0x8); // LT

        let instr = instr.set_oe(1);

        cpu.gpr[ra] = 0x8000_0000;
        cpu.gpr[rb] = 0x0000_4500;
        cpu.op_subfcx(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x8000_4500);
        assert!(!cpu.xer.carry());
        assert_eq!(cpu.cr.get_cr0(), 0x9); // LT

        cpu.gpr[ra] = 0x8000_0000;
        cpu.gpr[rb] = 0x0000_7000;
        cpu.op_subfcx(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x8000_7000);
        assert!(!cpu.xer.carry());
        assert_eq!(cpu.cr.get_cr0(), 0x9); // LT
    }

    #[test]
    fn op_subfex() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (rd, ra, rb) = (6, 4, 10);
        let instr = Instruction::new_subfex(rd, ra, rb);

        cpu.gpr[ra] = 0x9000_3000;
        cpu.gpr[rb] = 0x8000_7000;
        cpu.xer.set_carry(true);
        cpu.op_subfex(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0xF000_4000);
        assert!(!cpu.xer.carry());

        let instr = instr.set_rc(1);

        cpu.gpr[ra] = 0x0000_4500;
        cpu.gpr[rb] = 0x8000_7000;
        cpu.xer.set_carry(false);
        cpu.op_subfex(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x8000_2AFF);
        assert!(cpu.xer.carry());
        assert_eq!(cpu.cr.get_cr0(), 0x8); // LT

        let instr = instr.set_oe(1);

        cpu.gpr[ra] = 0x8000_0000;
        cpu.gpr[rb] = 0xEFFF_FFFF;
        cpu.xer.set_carry(true);
        cpu.op_subfex(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x6FFF_FFFF);
        assert!(cpu.xer.carry());
        assert_eq!(cpu.cr.get_cr0(), 0x4); // GT

        cpu.gpr[ra] = 0x8000_0000;
        cpu.gpr[rb] = 0xEFFF_FFFF;
        cpu.xer.set_carry(false);
        cpu.op_subfex(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x6FFF_FFFE);
        assert!(cpu.xer.carry());
        assert_eq!(cpu.cr.get_cr0(), 0x4); // GT
    }

    #[test]
    fn op_subfic() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (rd, ra, simm) = (6, 4, 0x7000);
        let instr = Instruction::new_subfic(rd, ra, simm);

        // SIMM < RA -> borrow -> CA clear
        cpu.gpr[ra] = 0x9000_3000;
        cpu.xer.set_carry(true);
        cpu.op_subfic(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x7000_4000);
        assert!(!cpu.xer.carry());

        // SIMM >= RA -> no borrow -> CA set
        cpu.gpr[ra] = 0x0000_1000;
        cpu.xer.set_carry(false);
        cpu.op_subfic(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x0000_6000);
        assert!(cpu.xer.carry());

        // Equal -> result 0, CA set
        cpu.gpr[ra] = 0x0000_7000;
        cpu.xer.set_carry(false);
        cpu.op_subfic(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0);
        assert!(cpu.xer.carry());
    }

    #[test]
    fn op_subfzex() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (rd, ra) = (6, 4);
        let instr = Instruction::new_subfzex(rd, ra);

        cpu.gpr[ra] = 0x9000_3000;
        cpu.xer.set_carry(true);
        cpu.op_subfzex(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x6FFF_D000);
        assert!(!cpu.xer.carry());

        cpu.gpr[ra] = 0xB004_3000;
        cpu.xer.set_carry(true);
        cpu.op_subfzex(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x4FFB_D000);
        assert!(!cpu.xer.carry());

        cpu.gpr[ra] = 0xEFFF_FFFF;
        cpu.xer.set_carry(false);
        cpu.op_subfzex(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x1000_0000);
        assert!(!cpu.xer.carry());

        cpu.gpr[ra] = 0x70FB_6500;
        cpu.xer.set_carry(false);
        cpu.op_subfzex(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x8F04_9AFF);
        assert!(!cpu.xer.carry());
    }

    #[test]
    fn op_twi() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();
        let trap = 1 << (31 - 14);

        let a = 4;

        let instr = Instruction::new_twi(0x4, a, 0x10);
        cpu.cia = 0x8000_1234;
        cpu.gpr[a] = 0x0000_0010;
        cpu.state.exceptions = 0;
        cpu.msr.0 = 0x0000_2030;
        cpu.op_twi(instr, &mut bus);
        assert_eq!(cpu.state.exceptions, EXCEPTION_PROGRAM);
        assert_eq!(cpu.spr[SPR_SRR1], 0);
        assert_eq!(cpu.program_exception_srr1, trap);
        assert_eq!(cpu.program_exception_srr0, 0x8000_1234);

        cpu.check_exceptions();
        assert_eq!(cpu.state.exceptions & EXCEPTION_PROGRAM, 0);
        assert_eq!(cpu.spr[SPR_SRR0], 0x8000_1234);
        assert_eq!(cpu.spr[SPR_SRR1] & trap, trap);
        assert_eq!(cpu.spr[SPR_SRR1] & 0x87C0_FFFF, 0x0000_2030 & 0x87C0_FFFF);
        assert_eq!(cpu.program_exception_srr1, 0);

        let instr = Instruction::new_twi(0x8, a, 0x10);
        cpu.cia = 0x8000_2000;
        cpu.gpr[a] = 0x0000_0020;
        cpu.state.exceptions = 0;
        cpu.spr[SPR_SRR1] = 0;
        cpu.op_twi(instr, &mut bus);
        assert_eq!(cpu.state.exceptions, EXCEPTION_PROGRAM);
        assert_eq!(cpu.program_exception_srr1, trap);

        cpu.gpr[a] = 0x0000_0008;
        cpu.state.exceptions = 0;
        cpu.program_exception_srr1 = 0;
        cpu.op_twi(instr, &mut bus);
        assert_eq!(cpu.state.exceptions, 0);
        assert_eq!(cpu.program_exception_srr1, 0);

        let instr = Instruction::new_twi(0x10, a, 0x10);
        cpu.gpr[a] = 0x0000_0020;
        cpu.state.exceptions = 0;
        cpu.op_twi(instr, &mut bus);
        assert_eq!(cpu.state.exceptions, 0);
    }

    #[test]
    fn op_xorx() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (ra, rs, rb) = (6, 4, 3);
        let instr = Instruction::new_xorx(ra, rs, rb);

        cpu.gpr[rs] = 0x9000_3000;
        cpu.gpr[rb] = 0x789A_789B;
        cpu.op_xorx(instr, &mut bus);

        assert_eq!(cpu.gpr[ra], 0xE89A_489B);

        let instr = instr.set_rc(1);

        cpu.gpr[rs] = 0xB004_3000;
        cpu.gpr[rb] = 0x789A_789B;
        cpu.op_xorx(instr, &mut bus);

        assert_eq!(cpu.gpr[ra], 0xC89E_489B);
        assert_eq!(cpu.cr.get_cr0(), 0x8); // LT
    }

    #[test]
    fn op_xoris() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (ra, rs, uimm) = (6, 4, 0x0079);
        let instr = Instruction::new_xoris(ra, rs, uimm);

        cpu.gpr[rs] = 0x9000_3000;
        cpu.op_xoris(instr, &mut bus);

        assert_eq!(cpu.gpr[ra], 0x9079_3000);
    }
}
