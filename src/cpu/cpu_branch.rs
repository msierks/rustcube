
impl Cpu {
    fn bx(&mut self, instr: Instruction) {
        if instr.aa() == 1 {
            self.nia = sign_ext_26(instr.li() << 2) as u32;
        } else {
            self.nia = self.cia.wrapping_add(sign_ext_26(instr.li() << 2) as u32);
        }

        if instr.lk() == 1 {
            self.lr = self.cia + 4;
        }
    }

    fn bcx(&mut self, instr: Instruction) {
        let bo = instr.bo();

        let ctr_ok = if bon(bo, 2) == 0 {
            self.ctr = self.ctr.wrapping_sub(1);

            if bon(bo, 3) != 0 {
                self.ctr == 0
            } else {
                self.ctr != 0
            }
        } else {
            true
        };

        let cond_ok = if bon(bo, 0) == 0 {
            bon(bo, 1) == self.cr.get_bit(instr.bi())
        } else {
            true
        };

        //println!("BO: {} ctr_ok: {} cond_ok: {}", bo, ctr_ok, cond_ok);

        if ctr_ok && cond_ok {
            if instr.aa() == 1 {
                self.nia = sign_ext_16(instr.bd() << 2) as u32;
            } else {
                self.nia = self.cia.wrapping_add(sign_ext_16(instr.bd() << 2) as u32);
            }

            if instr.lk() == 1 {
                self.lr = self.cia + 4;
            }
        }
    }

    fn bcctrx(&mut self, instr: Instruction) {
        let bo = instr.bo();

        let cond_ok = if bon(bo, 0) == 0 {
            self.cr.get_bit(instr.bi()) == bon(bo, 1)
        } else {
            true
        };

        if cond_ok {
            self.nia = self.ctr & 0xFFFFFFFC;

            if instr.lk() == 1 {
                self.lr = self.cia + 4;
            }
        }
    }

    fn bclrx(&mut self, instr: Instruction) {
        let bo = instr.bo();

        let ctr_ok = if bon(bo, 2) == 0 {
            self.ctr = self.ctr.wrapping_sub(1);

            if bon(bo, 3) != 0 {
                self.ctr == 0
            } else {
                self.ctr != 0
            }
        } else {
            true
        };

        let cond_ok = if bon(bo, 0) == 0 {
            bon(bo, 1) == self.cr.get_bit(instr.bi())
        } else {
            true
        };

        if ctr_ok && cond_ok {
            self.nia = self.lr & 0xFFFFFFFC;

            if instr.lk() == 1 {
                self.lr = self.cia + 4;
            }
        }
    }
}
