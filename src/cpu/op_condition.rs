use super::{instruction::Instruction, Cpu};
use crate::bus::Bus;

impl Cpu {
    pub fn op_crand(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_crand");
    }

    pub fn op_crandc(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_crandc");
    }

    pub fn op_creqv(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_creqv");
    }

    pub fn op_crnand(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_crnand");
    }

    pub fn op_crnor(&mut self, instr: Instruction, _: &mut Bus) {
        let d = !(self.cr.get_bit(instr.a()) | self.cr.get_bit(instr.b())) & 1;

        self.cr.set_bit(instr.d(), d);

        self.tick(1);
    }

    pub fn op_cror(&mut self, instr: Instruction, _: &mut Bus) {
        let d = self.cr.get_bit(instr.a()) | self.cr.get_bit(instr.b());

        self.cr.set_bit(instr.d(), d);

        self.tick(1);
    }

    pub fn op_crorc(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_crorc");
    }

    pub fn op_crxor(&mut self, instr: Instruction, _: &mut Bus) {
        let d = self.cr.get_bit(instr.a()) ^ self.cr.get_bit(instr.b());

        self.cr.set_bit(instr.d(), d);

        self.tick(1);
    }

    pub fn op_mcrf(&mut self, instr: Instruction, _: &mut Bus) {
        let cr_f = self.cr.get_field(instr.crfs());
        self.cr.set_field(instr.crfd(), cr_f);

        self.tick(1);
    }

    pub fn op_mcrxr(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_mcrxr");
    }

    pub fn op_mfcr(&mut self, instr: Instruction, _: &mut Bus) {
        self.gpr[instr.d()] = self.cr.as_u32();

        self.tick(1);
    }

    pub fn op_mtcrf(&mut self, instr: Instruction, _: &mut Bus) {
        let crm = instr.crm();

        if crm == 0xFF {
            self.cr.set(self.gpr[instr.s()]);
        } else {
            let mut mask = 0;

            for i in 0..8 {
                if (crm & (1 << i)) != 0 {
                    mask |= 0xF << (i * 4);
                }
            }

            let cr = (self.gpr[instr.s()] & mask) | (self.cr.as_u32() & !mask);

            self.cr.set(cr);
        }

        self.tick(1);
    }
}
