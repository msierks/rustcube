use super::{
    instruction::Instruction, mmu::SegmentRegister, registers::*, Cpu, EXCEPTION_DECREMENTER,
    EXCEPTION_PROGRAM, EXCEPTION_SYSTEM_CALL,
};
use crate::bus::Bus;

impl Cpu {
    pub fn op_eieio(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_eieio");
    }

    pub fn op_isync(&mut self, _instr: Instruction, _: &mut Bus) {
        // don't do anything

        self.tick(2);
    }

    pub fn op_mfmsr(&mut self, instr: Instruction, _: &mut Bus) {
        if self.msr.pr() {
            self.generate_program_exception(ProgramException::PrivilegedInstruction);
            return;
        }

        self.gpr[instr.d()] = self.msr.0;

        self.tick(1);
    }

    pub fn op_mfspr(&mut self, instr: Instruction, _: &mut Bus) {
        let i = instr.spr();

        match i {
            SPR_XER => self.gpr[instr.s()] = self.xer.into(),
            SPR_WPAR => self.gpr[instr.s()] &= !1,
            SPR_DEC => {
                let dec = self.state.timers.get_decrementer();
                self.spr[SPR_DEC] = dec;
                self.gpr[instr.s()] = dec;
            }
            SPR_TBL => {
                self.gpr[instr.s()] = self.state.timers.get_timebase() as u32;
            }
            SPR_TBU => {
                self.gpr[instr.s()] = (self.state.timers.get_timebase() >> 32) as u32;
            }
            SPR_HID2 => self.gpr[instr.s()] = self.hid2.0,
            _ => self.gpr[instr.s()] = self.spr[i],
        }

        // TODO: check privilege level
        if (SPR_IBAT0U..=SPR_DBAT3L).contains(&i) {
            self.tick(3);
        } else {
            self.tick(1);
        }
    }

    pub fn op_mfsr(&mut self, _instr: Instruction, _: &mut Bus) {
        if self.msr.pr() {
            self.generate_program_exception(ProgramException::PrivilegedInstruction);
            return;
        }

        unimplemented!("op_mfsr");
    }

    pub fn op_mfsrin(&mut self, _instr: Instruction, _: &mut Bus) {
        if self.msr.pr() {
            self.generate_program_exception(ProgramException::PrivilegedInstruction);
            return;
        }

        unimplemented!("op_mfsrin");
    }

    pub fn op_mftb(&mut self, instr: Instruction, _: &mut Bus) {
        let timebase = self.state.timers.get_timebase();

        if instr.tbr() == TBR_TBL {
            self.gpr[instr.d()] = timebase as u32;
        } else if instr.tbr() == TBR_TBU {
            self.gpr[instr.d()] = (timebase >> 32) as u32;
        } else {
            panic!("mftb unknown tbr {:#x}", instr.tbr());
        }

        self.tick(1);
    }

    pub fn op_mtmsr(&mut self, instr: Instruction, _: &mut Bus) {
        if self.msr.pr() {
            self.generate_program_exception(ProgramException::PrivilegedInstruction);
            return;
        }

        self.msr = self.gpr[instr.s()].into();

        self.tick(1);
    }

    pub fn op_mtspr(&mut self, instr: Instruction, _: &mut Bus) {
        let i = instr.spr();
        let v = self.gpr[instr.s()];

        self.spr[i] = v;

        match i {
            SPR_XER => self.xer = v.into(),
            _ => {
                if self.msr.pr() {
                    // TODO: properly handle this case
                    self.state.exceptions |= EXCEPTION_PROGRAM;
                    panic!("mtspr: user privilege level prevents setting spr {i:#?}");
                }

                match i {
                    SPR_IBAT0U => self.immu.write_batu(0, v),
                    SPR_IBAT0L => self.immu.write_batl(0, v),
                    SPR_IBAT1U => self.immu.write_batu(1, v),
                    SPR_IBAT1L => self.immu.write_batl(1, v),
                    SPR_IBAT2U => self.immu.write_batu(2, v),
                    SPR_IBAT2L => self.immu.write_batl(2, v),
                    SPR_IBAT3U => self.immu.write_batu(3, v),
                    SPR_IBAT3L => self.immu.write_batl(3, v),
                    SPR_DBAT0U => self.dmmu.write_batu(0, v),
                    SPR_DBAT0L => self.dmmu.write_batl(0, v),
                    SPR_DBAT1U => self.dmmu.write_batu(1, v),
                    SPR_DBAT1L => self.dmmu.write_batl(1, v),
                    SPR_DBAT2U => self.dmmu.write_batu(2, v),
                    SPR_DBAT2L => self.dmmu.write_batl(2, v),
                    SPR_DBAT3U => self.dmmu.write_batu(3, v),
                    SPR_DBAT3L => self.dmmu.write_batl(3, v),
                    SPR_SDR1 => {
                        self.immu.sdr1 = super::mmu::SDR1(v);
                        self.dmmu.sdr1 = super::mmu::SDR1(v);
                    }
                    SPR_DEC => {
                        let old_dec = self.state.timers.get_decrementer();
                        self.state.timers.set_decrementer(v);
                        // Software write that sets MSB (0 -> 1) raises a decrementer exception.
                        if (old_dec >> 31) == 0 && (v >> 31) != 0 {
                            self.state.exceptions |= EXCEPTION_DECREMENTER;
                        }
                    }
                    SPR_HID2 => self.hid2 = v.into(),
                    SPR_TBL => self.state.timers.set_timebase_lower(v),
                    SPR_TBU => self.state.timers.set_timebase_upper(v),
                    SPR_WPAR => {
                        self.spr[i] &= !0x1F;
                        info!("WPAR set to {:#x}", self.spr[i]);
                        // gp_fifo.reset();
                    }
                    _ => {}
                }
            }
        }

        self.tick(2);
    }

    pub fn op_mtsr(&mut self, instr: Instruction, _: &mut Bus) {
        if self.msr.pr() {
            self.generate_program_exception(ProgramException::PrivilegedInstruction);
            return;
        }

        self.sr[instr.sr()] = self.gpr[instr.s()];
        self.immu.sr[instr.sr()] = SegmentRegister(self.gpr[instr.s()]);
        self.dmmu.sr[instr.sr()] = SegmentRegister(self.gpr[instr.s()]);

        self.tick(2);
    }

    pub fn op_mtsrin(&mut self, instr: Instruction, _: &mut Bus) {
        if self.msr.pr() {
            self.generate_program_exception(ProgramException::PrivilegedInstruction);
            return;
        }

        let v = self.gpr[instr.s()];
        let i = (self.gpr[instr.b()] >> 28) as usize;

        self.sr[i] = v;
        self.immu.sr[i] = SegmentRegister(v);
        self.dmmu.sr[i] = SegmentRegister(v);

        self.tick(2);
    }

    pub fn op_rfi(&mut self, _instr: Instruction, _: &mut Bus) {
        if self.msr.pr() {
            self.generate_program_exception(ProgramException::PrivilegedInstruction);
            return;
        }

        let mask = 0x87C0_FF73;

        self.msr.0 = (self.msr.0 & !mask) | (self.spr[SPR_SRR1] & mask);

        self.msr.0 &= 0xFFFB_FFFF;

        self.nia = self.spr[SPR_SRR0] & 0xFFFF_FFFC;

        self.tick(2);
    }

    pub fn op_sc(&mut self, _instr: Instruction, _: &mut Bus) {
        self.state.exceptions |= EXCEPTION_SYSTEM_CALL;

        self.tick(2);
    }

    pub fn op_sync(&mut self, _instr: Instruction, _: &mut Bus) {
        // don't do anything

        self.tick(3);
    }

    pub fn op_tlbsync(&mut self, _instr: Instruction, _: &mut Bus) {
        if self.msr.pr() {
            self.generate_program_exception(ProgramException::PrivilegedInstruction);
            return;
        }

        unimplemented!("op_tlbsync");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_eieio() {}

    #[test]
    fn op_isync() {}

    #[test]
    fn op_mfmsr() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let rd = 6;
        let instr = Instruction::new_mfmsr(rd);

        cpu.msr = 0x0D15_AA5E.into();

        cpu.op_mfmsr(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0x0D15_AA5E);
    }

    #[test]
    fn op_mfspr() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (rd, spr) = (6, SPR_LR as u32); // FIXME: make spr a usize
        let instr = Instruction::new_mfspr(rd, spr);

        cpu.spr[SPR_LR] = 0xDEAD_BEEF;
        cpu.op_mfspr(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 0xDEAD_BEEF);

        // TBL/TBU via mfspr match mftb (SPR 284/285)
        cpu.state.timers.set_timebase_upper(0x0000_00AB);
        cpu.state.timers.set_timebase_lower(0x1234_5678);
        cpu.state.timers.tick(0); // no further advance

        let instr = Instruction::new_mfspr(rd, SPR_TBL as u32);
        cpu.op_mfspr(instr, &mut bus);
        assert_eq!(cpu.gpr[rd], 0x1234_5678);

        let instr = Instruction::new_mfspr(rd, SPR_TBU as u32);
        cpu.op_mfspr(instr, &mut bus);
        assert_eq!(cpu.gpr[rd], 0x0000_00AB);
    }

    #[test]
    fn op_mfsr() {}

    #[test]
    fn op_mfsrin() {}

    #[test]
    fn op_mftb() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (rd, tbr) = (6, TBR_TBL); // FIXME: make tbr usize
        let instr = Instruction::new_mftb(rd, tbr as u32);

        cpu.state.timers.tick(0x1784);
        cpu.op_mftb(instr, &mut bus);

        assert_eq!(cpu.gpr[rd], 501); // FIXME: this needs to be better
    }

    #[test]
    fn op_mtmsr() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let rs = 6;
        let instr = Instruction::new_mtmsr(rs);

        cpu.gpr[rs] = 0x0D15_AA5E;

        cpu.op_mtmsr(instr, &mut bus);

        assert_eq!(cpu.msr.0, 0x0D15_AA5E);
    }

    #[test]
    fn op_mtspr() {}

    #[test]
    fn op_mtsrin() {}

    #[test]
    fn op_rfi() {}

    #[test]
    fn op_sc() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let instr = Instruction::new_sc();

        cpu.op_sc(instr, &mut bus);

        assert_eq!(cpu.state.exceptions, EXCEPTION_SYSTEM_CALL);
    }

    #[test]
    fn op_sync() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let instr = Instruction::new_sync();

        cpu.op_sync(instr, &mut bus);
    }

    #[test]
    #[should_panic]
    fn op_tlbsync() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let instr = Instruction::new_tlbsync();

        cpu.op_tlbsync(instr, &mut bus);
    }
}
