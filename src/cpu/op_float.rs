use super::{float::Nan, instruction::Instruction, Cpu, EXCEPTION_FPU_UNAVAILABLE};
use crate::bus::Bus;

impl Cpu {
    pub fn op_fabsx(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_fabsx");
    }

    pub fn op_faddsx(&mut self, instr: Instruction, _: &mut Bus) {
        if !self.ensure_fp() {
            return;
        }

        let fra = self.fpr[instr.a()].ps0_as_f64();
        let frb = self.fpr[instr.b()].ps0_as_f64();

        let result = fra + frb;

        self.fpr[instr.d()].set_ps0_f64(result);

        if self.hid2.pse() {
            self.fpr[instr.d()].set_ps1_f64(result);
        }

        if instr.rc() {
            self.update_cr1();
        }

        self.tick(1);
    }

    pub fn op_faddx(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_faddx");
    }

    pub fn op_fcmpo(&mut self, instr: Instruction, _: &mut Bus) {
        if !self.ensure_fp() {
            return;
        }

        let fra = self.fpr[instr.a()].ps0_as_f64();
        let frb = self.fpr[instr.b()].ps0_as_f64();

        let c = if fra.is_nan() || frb.is_nan() {
            if fra.is_snan() || frb.is_snan() {
                self.fpscr.set_vxsnan(true);
                if !self.fpscr.ve() {
                    self.fpscr.set_vxvc(true);
                }
            } else {
                self.fpscr.set_vxsnan(true);
            }
            0b1 // ?
        } else if fra < frb {
            0x8 // <
        } else if fra > frb {
            0x4 // >
        } else {
            0x2 // =
        };

        self.fpscr.set_fpcc(c);

        self.cr.set_field(instr.crfd(), c);

        self.tick(1);
    }

    pub fn op_fcmpu(&mut self, instr: Instruction, _: &mut Bus) {
        if !self.ensure_fp() {
            return;
        }

        let fra = self.fpr[instr.a()].ps0_as_f64();
        let frb = self.fpr[instr.b()].ps0_as_f64();

        let c = if fra.is_nan() || frb.is_nan() {
            if fra.is_snan() || frb.is_snan() {
                self.fpscr.set_vxsnan(true);
            }
            0b1 // ?
        } else if fra < frb {
            0x8 // <
        } else if fra > frb {
            0x4 // >
        } else {
            0x2 // =
        };

        self.fpscr.set_fpcc(c);

        self.cr.set_field(instr.crfd(), c);

        self.tick(1);
    }

    pub fn op_fctiwzx(&mut self, instr: Instruction, _: &mut Bus) {
        let frb = self.fpr[instr.b()].ps0_as_f64();

        // TODO: implement more accurate conversion
        let result = ((frb as i32) as u32) as u64;

        self.fpr[instr.d()].set_ps0(result);

        if instr.rc() {
            self.update_cr1();
        }

        self.tick(1);
    }

    pub fn op_fctiwx(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_fctiwx");
    }

    pub fn op_fdivsx(&mut self, instr: Instruction, _: &mut Bus) {
        let fra = self.fpr[instr.a()].ps0_as_f64();
        let frb = self.fpr[instr.b()].ps0_as_f64();

        let result = fra / frb;

        if frb.is_nan() {
            panic!();
        }

        self.fpr[instr.d()].set_ps0_f64(result);

        if self.hid2.pse() {
            self.fpr[instr.d()].set_ps1_f64(result);
        }

        if instr.rc() {
            self.update_cr1();
        }

        self.tick(17);
    }

    pub fn op_fdivx(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_fdivx");
    }

    pub fn op_fmaddsx(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_fmaddsx");
    }

    pub fn op_fmaddx(&mut self, instr: Instruction, _: &mut Bus) {
        if !self.msr.fp() {
            self.state.exceptions |= EXCEPTION_FPU_UNAVAILABLE;
            return;
        }

        let fra = self.fpr[instr.a()].ps0_as_f64();
        let frb = self.fpr[instr.b()].ps0_as_f64();
        let frc = self.fpr[instr.c()].ps0_as_f64();

        let result = fra.mul_add(frc, frb);

        self.fpr[instr.d()].set_ps0_f64(result);

        if instr.rc() {
            self.update_cr1();
        }
    }

    // FIXME: Verify paired single functionality with HID2[PSE] value
    pub fn op_fmrx(&mut self, instr: Instruction, _: &mut Bus) {
        if !self.ensure_fp() {
            return;
        }

        let frb = self.fpr[instr.b()].ps0();

        self.fpr[instr.d()].set_ps0(frb);

        if self.hid2.pse() {
            self.fpr[instr.d()].set_ps1(frb);
        }

        if instr.rc() {
            self.update_cr1();
        }

        self.tick(1);
    }

    pub fn op_fmsubsx(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_fmsubsx");
    }

    pub fn op_fmsubx(&mut self, instr: Instruction, _: &mut Bus) {
        if !self.msr.fp() {
            self.state.exceptions |= EXCEPTION_FPU_UNAVAILABLE;
            return;
        }

        let fra = self.fpr[instr.a()].ps0_as_f64();
        let frb = self.fpr[instr.b()].ps0_as_f64();
        let frc = self.fpr[instr.c()].ps0_as_f64();

        let result = fra.mul_add(frc, -frb);

        self.fpr[instr.d()].set_ps0_f64(result);

        if instr.rc() {
            self.update_cr1();
        }
    }

    pub fn op_fmulsx(&mut self, instr: Instruction, _: &mut Bus) {
        if !self.ensure_fp() {
            return;
        }

        let result = self.fpr[instr.a()].ps0_as_f64() * self.fpr[instr.c()].ps0_as_f64();

        self.fpr[instr.d()].set_ps0_f64(result);

        if self.hid2.pse() {
            self.fpr[instr.d()].set_ps1_f64(result);
        }

        if instr.rc() {
            self.update_cr1();
        }

        self.tick(1);
    }

    pub fn op_fmulx(&mut self, instr: Instruction, _: &mut Bus) {
        if !self.ensure_fp() {
            return;
        }

        let fra = self.fpr[instr.a()].ps0_as_f64();
        let frc = self.fpr[instr.c()].ps0_as_f64();

        let result = fra * frc;

        self.fpr[instr.d()].set_ps0_f64(result);

        if instr.rc() {
            self.update_cr1();
        }
    }

    pub fn op_fnabsx(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_fnabsx");
    }

    pub fn op_fnegx(&mut self, instr: Instruction, _: &mut Bus) {
        self.fpr[instr.d()].set_ps0(self.fpr[instr.b()].ps0() ^ (1_u64 << 63));

        self.tick(1);
    }

    pub fn op_fnmaddsx(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_fnmaddsx");
    }

    pub fn op_fnmaddx(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_fnmaddx");
    }

    pub fn op_fnmsubsx(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_fnsubsx");
    }

    pub fn op_fnmsubx(&mut self, instr: Instruction, _: &mut Bus) {
        if !self.ensure_fp() {
            return;
        }

        let fra = self.fpr[instr.a()].ps0_as_f64();
        let frb = self.fpr[instr.b()].ps0_as_f64();
        let frc = self.fpr[instr.c()].ps0_as_f64();

        let result = fra.mul_add(frc, -frb);

        self.fpr[instr.d()].set_ps0_f64(result);

        if instr.rc() {
            self.update_cr1();
        }

        self.tick(2);
    }

    pub fn op_fresx(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_fresx");
    }

    pub fn op_frspx(&mut self, instr: Instruction, _: &mut Bus) {
        if !self.ensure_fp() {
            return;
        }

        let frb = self.fpr[instr.b()].ps0_as_f64();

        if frb.is_nan() {
            unimplemented!();
        }

        self.fpr[instr.d()].set_ps0_f64(frb);

        if self.hid2.pse() {
            self.fpr[instr.d()].set_ps1_f64(frb);
        }

        if instr.rc() {
            self.update_cr1();
        }
    }

    pub fn op_frsqrtex(&mut self, instr: Instruction, _: &mut Bus) {
        let frb = self.fpr[instr.b()].ps0_as_f64();

        self.fpr[instr.d()].set_ps0_f64(1.0 / frb.sqrt());

        if instr.rc() {
            self.update_cr1();
        }

        self.tick(2);
    }

    pub fn op_fselx(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_fselx");
    }

    pub fn op_fsubsx(&mut self, instr: Instruction, _: &mut Bus) {
        if !self.ensure_fp() {
            return;
        }

        let result = self.fpr[instr.a()].ps0_as_f64() - self.fpr[instr.b()].ps0_as_f64();

        self.fpr[instr.d()].set_ps0_f64(result);

        if self.hid2.pse() {
            self.fpr[instr.d()].set_ps1_f64(result);
        }

        if instr.rc() {
            self.update_cr1();
        }

        self.tick(1);
    }

    pub fn op_ps_absx(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_ps_absx");
    }

    pub fn op_ps_addx(&mut self, instr: Instruction, _: &mut Bus) {
        let fra = self.fpr[instr.a()].ps0_as_f64();
        let frb = self.fpr[instr.b()].ps0_as_f64();

        self.fpr[instr.d()].set_ps0_f64(fra + frb);

        let fra = self.fpr[instr.a()].ps1_as_f64();
        let frb = self.fpr[instr.b()].ps1_as_f64();

        self.fpr[instr.d()].set_ps1_f64(fra + frb);

        if instr.rc() {
            self.update_cr1();
        }

        self.tick(1);
    }

    pub fn op_ps_cmpo0(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_ps_cmpo0");
    }

    pub fn op_ps_cmpo1(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_ps_cmpo1");
    }

    pub fn op_ps_cmpu0(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_ps_cmpu0");
    }

    pub fn op_ps_cmpu1(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_ps_cmpu1");
    }

    pub fn op_ps_divx(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_ps_divx");
    }

    pub fn op_ps_maddx(&mut self, instr: Instruction, _: &mut Bus) {
        let fra = self.fpr[instr.a()].ps0_as_f64();
        let frb = self.fpr[instr.b()].ps0_as_f64();
        let frc = self.fpr[instr.c()].ps0_as_f64();

        self.fpr[instr.d()].set_ps0_f64(fra.mul_add(frc, frb));

        let fra = self.fpr[instr.a()].ps1_as_f64();
        let frb = self.fpr[instr.b()].ps1_as_f64();
        let frc = self.fpr[instr.c()].ps1_as_f64();

        self.fpr[instr.d()].set_ps1_f64(fra.mul_add(frc, frb));

        if instr.rc() {
            self.update_cr1();
        }

        self.tick(1);
    }

    pub fn op_ps_madds0x(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_ps_madds0x");
    }

    pub fn op_ps_madds1x(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_ps_madds1x");
    }

    pub fn op_ps_merge00x(&mut self, instr: Instruction, _: &mut Bus) {
        if !self.ensure_fp() {
            return;
        }

        let fra = self.fpr[instr.a()].ps0();
        let frb = self.fpr[instr.b()].ps0();

        self.fpr[instr.d()].set_ps0(fra);
        self.fpr[instr.d()].set_ps1(frb);

        if instr.rc() {
            self.update_cr1();
        }

        self.tick(1);
    }

    pub fn op_ps_merge01x(&mut self, instr: Instruction, _: &mut Bus) {
        if !self.ensure_fp() {
            return;
        }

        let fra = self.fpr[instr.a()].ps0();
        let frb = self.fpr[instr.b()].ps1();

        self.fpr[instr.d()].set_ps0(fra);
        self.fpr[instr.d()].set_ps1(frb);

        if instr.rc() {
            self.update_cr1();
        }

        self.tick(1);
    }

    pub fn op_ps_merge10x(&mut self, instr: Instruction, _: &mut Bus) {
        if !self.ensure_fp() {
            return;
        }

        let fra = self.fpr[instr.a()].ps1();
        let frb = self.fpr[instr.b()].ps0();

        self.fpr[instr.d()].set_ps0(fra);
        self.fpr[instr.d()].set_ps1(frb);

        if instr.rc() {
            self.update_cr1();
        }

        self.tick(1);
    }

    pub fn op_ps_merge11x(&mut self, instr: Instruction, _: &mut Bus) {
        if !self.ensure_fp() {
            return;
        }

        let fra = self.fpr[instr.a()].ps1();
        let frb = self.fpr[instr.b()].ps1();

        self.fpr[instr.d()].set_ps0(fra);
        self.fpr[instr.d()].set_ps1(frb);

        if instr.rc() {
            self.update_cr1();
        }

        self.tick(1);
    }

    pub fn op_ps_mrx(&mut self, instr: Instruction, _: &mut Bus) {
        if !self.ensure_fp() {
            return;
        }

        self.fpr[instr.d()] = self.fpr[instr.b()].clone();

        if instr.rc() {
            self.update_cr1();
        }

        self.tick(1);
    }

    pub fn op_ps_msubx(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_ps_msubx");
    }

    pub fn op_ps_mulx(&mut self, instr: Instruction, _: &mut Bus) {
        let fra = self.fpr[instr.a()].ps0_as_f64();
        let frc = self.fpr[instr.c()].ps0_as_f64();

        let result = fra * frc;

        self.fpr[instr.d()].set_ps0_f64(result);

        if instr.rc() {
            self.update_cr1();
        }

        self.tick(2);
    }

    pub fn op_ps_muls0x(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_ps_muls0x");
    }

    pub fn op_ps_muls1x(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_ps_muls1x");
    }

    pub fn op_ps_nabsx(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_ps_nabsx");
    }

    pub fn op_ps_negx(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_ps_negx");
    }

    pub fn op_ps_nmaddx(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_ps_nmaddx");
    }

    pub fn op_ps_nmsubx(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_ps_nmsubx");
    }

    pub fn op_ps_resx(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_resx");
    }

    pub fn op_ps_rsqrtex(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_ps_rsqrtex");
    }

    pub fn op_ps_selx(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_selx");
    }

    pub fn op_ps_subx(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_ps_subx");
    }

    pub fn op_ps_sum0x(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_ps_sum0x");
    }

    pub fn op_ps_sum1x(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_ps_sum1x");
    }

    pub fn op_fsubx(&mut self, instr: Instruction, _: &mut Bus) {
        if !self.ensure_fp() {
            return;
        }

        let result = self.fpr[instr.a()].ps0_as_f64() - self.fpr[instr.b()].ps0_as_f64();

        self.fpr[instr.d()].set_ps0_f64(result);

        if self.hid2.pse() {
            self.fpr[instr.d()].set_ps1_f64(result);
        }

        if instr.rc() {
            self.update_cr1();
        }

        self.tick(1);
    }

    pub fn op_mcrfs(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_mcrfs");
    }

    pub fn op_mffsx(&mut self, instr: Instruction, _: &mut Bus) {
        self.fpr[instr.d()].set_ps0(self.fpscr.0 as u64);

        if instr.rc() {
            self.update_cr1();
        }

        self.tick(1);
    }

    // TODO: test this implementation
    pub fn op_mtfsb0x(&mut self, instr: Instruction, _: &mut Bus) {
        let b = 0x8000_0000_u32 >> instr.crbd();

        self.fpscr.0 &= !b;

        if instr.rc() {
            self.update_cr1();
        }

        self.tick(3);
    }

    // TODO: test this implementation
    pub fn op_mtfsb1x(&mut self, instr: Instruction, _: &mut Bus) {
        let b = 0x8000_0000_u32 >> instr.crbd();

        self.fpscr.0 |= b;

        if instr.rc() {
            self.update_cr1();
        }

        self.tick(3);
    }

    pub fn op_mtfsfix(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_mtfsfix");
    }

    // TODO: test this implementation
    pub fn op_mtfsfx(&mut self, instr: Instruction, _: &mut Bus) {
        let (mut m, mut i) = (0, 7);
        let fm = instr.fm();

        while i >= 0 {
            if (fm >> i) & 1 != 0 {
                m |= 0xF;
            }
            m <<= 4;
            i -= 1;
        }

        self.fpscr.0 = (self.fpscr.0 & !m) | (self.fpr[instr.b()].ps0() as u32 & m);

        if instr.rc() {
            self.update_cr1();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn op_fmaddx() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();
        cpu.msr.set_fp(true);

        let (frd, fra, frc, frb) = (6, 4, 5, 7);
        let instr = Instruction::new_fmaddx(frd, fra, frc, frb);

        cpu.fpr[fra].set_ps0_f64(2.0);
        cpu.fpr[frc].set_ps0_f64(3.0);
        cpu.fpr[frb].set_ps0_f64(4.0);
        cpu.op_fmaddx(instr, &mut bus);

        assert_eq!(cpu.fpr[frd].ps0_as_f64(), 10.0);
    }

    #[test]
    fn op_fmsubx() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();
        cpu.msr.set_fp(true);

        let (frd, fra, frc, frb) = (6, 4, 5, 7);
        let instr = Instruction::new_fmsubx(frd, fra, frc, frb);

        cpu.fpr[fra].set_ps0_f64(2.0);
        cpu.fpr[frc].set_ps0_f64(3.0);
        cpu.fpr[frb].set_ps0_f64(4.0);
        cpu.op_fmsubx(instr, &mut bus);

        assert_eq!(cpu.fpr[frd].ps0_as_f64(), 2.0);
    }

    #[test]
    fn op_fnegx() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (frd, frb) = (3, 4);
        let instr = Instruction::new_fnegx(frd, frb);

        cpu.fpr[frb].set_ps0_f64(1.0);
        cpu.op_fnegx(instr, &mut bus);
        assert_eq!(cpu.fpr[frd].ps0_as_f64(), -1.0);

        cpu.fpr[frb].set_ps0_f64(-2.5);
        cpu.op_fnegx(instr, &mut bus);
        assert_eq!(cpu.fpr[frd].ps0_as_f64(), 2.5);
    }
}
