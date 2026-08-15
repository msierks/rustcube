use super::{instruction::Instruction, utils::*, Cpu};
use crate::bus::Bus;

const BO_DONT_DECREMENT: u8 = 0x4;

impl Cpu {
    pub fn op_bx(&mut self, instr: Instruction, _: &mut Bus) {
        let address = sign_ext_26(instr.li() << 2) as u32;

        if instr.aa() {
            self.nia = address;
        } else {
            self.nia = self.cia.wrapping_add(address);
        }

        if instr.lk() {
            self.lr = self.cia.wrapping_add(4);
        }

        self.tick(1);
    }

    pub fn op_bcx(&mut self, instr: Instruction, _: &mut Bus) {
        let bo = instr.bo();

        if bo & BO_DONT_DECREMENT == 0 {
            self.ctr = self.ctr.wrapping_sub(1);
        }

        let ctr_ok = (bo >> 2) & 1 != 0 || (((self.ctr != 0) as u8 ^ (bo >> 1)) & 1) != 0;
        let cond_ok = (bo >> 4) & 1 != 0 || (self.cr.get_bit(instr.bi()) == (bo >> 3) & 1);

        if ctr_ok && cond_ok {
            let address = sign_ext_16(instr.bd() << 2) as u32;

            if instr.aa() {
                self.nia = address;
            } else {
                self.nia = self.cia.wrapping_add(address);
            }

            if instr.lk() {
                self.lr = self.cia.wrapping_add(4);
            }
        }

        self.tick(1);
    }

    pub fn op_bcctrx(&mut self, instr: Instruction, _: &mut Bus) {
        let bo = instr.bo();

        if bo & BO_DONT_DECREMENT == 0 {
            panic!("bcctrx: Invalid instruction, BO[2] = 0");
        }

        let cond_ok = ((bo >> 4) | (self.cr.get_bit(instr.bi()) == ((bo >> 3) & 1)) as u8) & 1;

        if cond_ok != 0 {
            self.nia = self.ctr & (!3);

            if instr.lk() {
                self.lr = self.cia.wrapping_add(4);
            }
        }

        self.tick(1);
    }

    pub fn op_bclrx(&mut self, instr: Instruction, _: &mut Bus) {
        let bo = instr.bo();

        if bo & BO_DONT_DECREMENT == 0 {
            self.ctr = self.ctr.wrapping_sub(1);
        }

        let ctr_ok = ((bo >> 2) | ((self.ctr != 0) as u8 ^ (bo >> 1))) & 1;
        let cond_ok = ((bo >> 4) | (self.cr.get_bit(instr.bi()) == ((bo >> 3) & 1)) as u8) & 1;

        if ctr_ok != 0 && cond_ok != 0 {
            self.nia = self.lr & (!3);

            if instr.lk() {
                self.lr = self.cia.wrapping_add(4);
            }
        }

        self.tick(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_bcx() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        // addi 8,0,3
        let (rd, ra, simm) = (8, 0, 0x3);
        let instr = Instruction::new_addi(rd, ra, simm);

        cpu.op_addi(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x0000_0003);

        // mtctr 8
        let instr = Instruction::new_mtspr(0x9, 0x8);
        cpu.op_mtspr(instr, &mut bus);

        // check counter register is set to 0x3
        assert_eq!(cpu.ctr, 0x0000_0003);

        // addic. 9,8,0x1
        let (rd, ra, simm) = (9, 8, 0x1);
        let instr = Instruction::new_addic_rc(rd, ra, simm);

        cpu.op_addic_rc(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x0000_0004);
        assert_eq!(cpu.cr.get_cr0(), 0x0000_0004);

        // bc 0xC,1,0x456
        let (bo, bi, bd) = (0xC, 1, 0x456);
        let instr = Instruction::new_bcx(bo, bi, bd);

        cpu.op_bcx(instr, &mut bus);

        assert_eq!(cpu.nia, 0xFFF0_1258);

        // bcl 0x8,1,0x456
        let (bo, bi, bd, lk) = (0x8, 1, 0x456, 1);
        let instr = Instruction::new_bcx(bo, bi, bd).set_lk(lk);

        cpu.op_bcx(instr, &mut bus);

        assert_eq!(cpu.ctr, 0x2);
        assert_eq!(cpu.lr, 0xFFF0_0104);
    }
}
