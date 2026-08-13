use super::{float::Nan, instruction::Instruction, Cpu};
use crate::bus::Bus;

impl Cpu {
    pub fn op_fabsx(&mut self, instr: Instruction, _: &mut Bus) {
        if !self.ensure_fp() {
            return;
        }

        self.fpr[instr.d()].set_ps0_f64(self.fpr[instr.b()].ps0_as_f64().abs());

        if instr.rc() {
            self.update_cr1();
        }
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

    pub fn op_faddx(&mut self, instr: Instruction, _: &mut Bus) {
        if !self.ensure_fp() {
            return;
        }

        let fra = self.fpr[instr.a()].ps0_as_f64();
        let frb = self.fpr[instr.b()].ps0_as_f64();

        let result = fra + frb;

        self.fpr[instr.d()].set_ps0_f64(result);

        if instr.rc() {
            self.update_cr1();
        }

        self.tick(1);
    }

    fn float_compare_ordered(&mut self, crfd: usize, fa: f64, fb: f64) {
        let c = if fa.is_nan() || fb.is_nan() {
            if fa.is_snan() || fb.is_snan() {
                self.fpscr.set_vxsnan(true);
                if !self.fpscr.ve() {
                    self.fpscr.set_vxvc(true);
                }
            } else {
                // QNaN: invalid compare (VXVC), not VXSNAN
                self.fpscr.set_vxvc(true);
            }
            0x1 // unordered
        } else if fa < fb {
            0x8 // <
        } else if fa > fb {
            0x4 // >
        } else {
            0x2 // =
        };

        self.fpscr.set_fpcc(c);
        self.cr.set_field(crfd, c);
    }

    pub fn op_fcmpo(&mut self, instr: Instruction, _: &mut Bus) {
        if !self.ensure_fp() {
            return;
        }

        let fra = self.fpr[instr.a()].ps0_as_f64();
        let frb = self.fpr[instr.b()].ps0_as_f64();
        self.float_compare_ordered(instr.crfd(), fra, frb);

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
            0x1 // ?
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
        if !self.ensure_fp() {
            return;
        }

        let fra = self.fpr[instr.a()].ps0_as_f64();
        let frb = self.fpr[instr.b()].ps0_as_f64();

        if fra.is_snan() || frb.is_snan() {
            self.fpscr.set_vxsnan(true);
            if self.fpscr.ve() {
                if instr.rc() {
                    self.update_cr1();
                }
                self.tick(17);
                return;
            }
        }

        let mut result = fra / frb;
        if result.is_snan() {
            // Deliver a quiet NaN when VE is disabled.
            result = f64::from_bits(result.to_bits() | 0x0008_0000_0000_0000);
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

    pub fn op_fdivx(&mut self, instr: Instruction, _: &mut Bus) {
        if !self.ensure_fp() {
            return;
        }

        let fra = self.fpr[instr.a()].ps0_as_f64();
        let frb = self.fpr[instr.b()].ps0_as_f64();

        if fra.is_snan() || frb.is_snan() {
            self.fpscr.set_vxsnan(true);
            if self.fpscr.ve() {
                if instr.rc() {
                    self.update_cr1();
                }
                self.tick(17);
                return;
            }
        }

        let mut result = fra / frb;
        if result.is_snan() {
            // Deliver a quiet NaN when VE is disabled.
            result = f64::from_bits(result.to_bits() | 0x0008_0000_0000_0000);
        }

        self.fpr[instr.d()].set_ps0_f64(result);

        if instr.rc() {
            self.update_cr1();
        }

        self.tick(17);
    }

    pub fn op_fmaddsx(&mut self, instr: Instruction, _: &mut Bus) {
        if !self.ensure_fp() {
            return;
        }

        let fra = self.fpr[instr.a()].ps0_as_f64();
        let frb = self.fpr[instr.b()].ps0_as_f64();
        let frc = self.fpr[instr.c()].ps0_as_f64();

        let result = fra.mul_add(frc, frb);

        self.fpr[instr.d()].set_ps0_f64(result);

        if self.hid2.pse() {
            self.fpr[instr.d()].set_ps1_f64(result);
        }

        if instr.rc() {
            self.update_cr1();
        }
    }

    pub fn op_fmaddx(&mut self, instr: Instruction, _: &mut Bus) {
        if !self.ensure_fp() {
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

    pub fn op_fnabsx(&mut self, instr: Instruction, _: &mut Bus) {
        if !self.ensure_fp() {
            return;
        }

        let result = self.fpr[instr.b()].ps0() | 0x8000_0000_0000_0000;

        self.fpr[instr.d()].set_ps0(result);

        if instr.rc() {
            self.update_cr1();
        }
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

    pub fn op_fnmsubsx(&mut self, instr: Instruction, _: &mut Bus) {
        if !self.ensure_fp() {
            return;
        }

        let fra = self.fpr[instr.a()].ps0_as_f64();
        let frb = self.fpr[instr.b()].ps0_as_f64();
        let frc = self.fpr[instr.c()].ps0_as_f64();

        let result = -fra.mul_add(frc, -frb);

        self.fpr[instr.d()].set_ps0_f64(result);

        if self.hid2.pse() {
            self.fpr[instr.d()].set_ps1_f64(result);
        }

        if instr.rc() {
            self.update_cr1();
        }
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

    pub fn op_fresx(&mut self, instr: Instruction, _: &mut Bus) {
        if !self.ensure_fp() {
            return;
        }

        let frb = self.fpr[instr.b()].ps0_as_f64();
        let result = 1.0 / frb;

        if frb == 0.0 {
            self.fpscr.set_zx(true);
        }

        self.fpr[instr.d()].set_ps0_f64(result);

        if self.hid2.pse() {
            self.fpr[instr.d()].set_ps1_f64(result);
        }

        if instr.rc() {
            self.update_cr1();
        }

        self.tick(2);
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

    pub fn op_ps_cmpo0(&mut self, instr: Instruction, _: &mut Bus) {
        if !self.ensure_ps() {
            return;
        }

        let fra = self.fpr[instr.a()].ps0_as_f64();
        let frb = self.fpr[instr.b()].ps0_as_f64();
        self.float_compare_ordered(instr.crfd(), fra, frb);

        self.tick(1);
    }

    pub fn op_ps_cmpo1(&mut self, instr: Instruction, _: &mut Bus) {
        if !self.ensure_ps() {
            return;
        }

        let fra = self.fpr[instr.a()].ps1_as_f64();
        let frb = self.fpr[instr.b()].ps1_as_f64();
        self.float_compare_ordered(instr.crfd(), fra, frb);

        self.tick(1);
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
        if !self.ensure_fp() {
            return;
        }

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

    pub fn op_ps_madds0x(&mut self, instr: Instruction, _: &mut Bus) {
        if !self.ensure_fp() {
            return;
        }

        let frc = self.fpr[instr.c()].ps0_as_f64();

        let fra0 = self.fpr[instr.a()].ps0_as_f64();
        let frb0 = self.fpr[instr.b()].ps0_as_f64();
        self.fpr[instr.d()].set_ps0_f64(fra0.mul_add(frc, frb0));

        let fra1 = self.fpr[instr.a()].ps1_as_f64();
        let frb1 = self.fpr[instr.b()].ps1_as_f64();
        self.fpr[instr.d()].set_ps1_f64(fra1.mul_add(frc, frb1));

        if instr.rc() {
            self.update_cr1();
        }

        self.tick(1);
    }

    pub fn op_ps_madds1x(&mut self, instr: Instruction, _: &mut Bus) {
        if !self.ensure_fp() {
            return;
        }

        let frc = self.fpr[instr.c()].ps1_as_f64();

        let fra0 = self.fpr[instr.a()].ps0_as_f64();
        let frb0 = self.fpr[instr.b()].ps0_as_f64();
        self.fpr[instr.d()].set_ps0_f64(fra0.mul_add(frc, frb0));

        let fra1 = self.fpr[instr.a()].ps1_as_f64();
        let frb1 = self.fpr[instr.b()].ps1_as_f64();
        self.fpr[instr.d()].set_ps1_f64(fra1.mul_add(frc, frb1));

        if instr.rc() {
            self.update_cr1();
        }

        self.tick(1);
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

    pub fn op_ps_msubx(&mut self, instr: Instruction, _: &mut Bus) {
        if !self.ensure_fp() {
            return;
        }

        let fra = self.fpr[instr.a()].ps0_as_f64();
        let frb = self.fpr[instr.b()].ps0_as_f64();
        let frc = self.fpr[instr.c()].ps0_as_f64();

        self.fpr[instr.d()].set_ps0_f64(fra.mul_add(frc, -frb));

        let fra = self.fpr[instr.a()].ps1_as_f64();
        let frb = self.fpr[instr.b()].ps1_as_f64();
        let frc = self.fpr[instr.c()].ps1_as_f64();

        self.fpr[instr.d()].set_ps1_f64(fra.mul_add(frc, -frb));

        if instr.rc() {
            self.update_cr1();
        }

        self.tick(1);
    }

    pub fn op_ps_mulx(&mut self, instr: Instruction, _: &mut Bus) {
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

        self.tick(2);
    }

    pub fn op_ps_muls0x(&mut self, instr: Instruction, _: &mut Bus) {
        if !self.ensure_fp() {
            return;
        }

        let frc = self.fpr[instr.c()].ps0_as_f64();

        let fra0 = self.fpr[instr.a()].ps0_as_f64();
        self.fpr[instr.d()].set_ps0_f64(fra0 * frc);

        let fra1 = self.fpr[instr.a()].ps1_as_f64();
        self.fpr[instr.d()].set_ps1_f64(fra1 * frc);

        if instr.rc() {
            self.update_cr1();
        }

        self.tick(1);
    }

    pub fn op_ps_muls1x(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_ps_muls1x");
    }

    pub fn op_ps_nabsx(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_ps_nabsx");
    }

    pub fn op_ps_negx(&mut self, instr: Instruction, _: &mut Bus) {
        if !self.ensure_fp() {
            return;
        }

        let frb = &self.fpr[instr.b()];
        let ps0 = frb.ps0() ^ (1_u64 << 63);
        let ps1 = frb.ps1() ^ (1_u64 << 63);

        self.fpr[instr.d()].set_ps0(ps0);
        self.fpr[instr.d()].set_ps1(ps1);

        if instr.rc() {
            self.update_cr1();
        }

        self.tick(1);
    }

    pub fn op_ps_nmaddx(&mut self, instr: Instruction, _: &mut Bus) {
        if !self.ensure_fp() {
            return;
        }

        let fra = self.fpr[instr.a()].ps0_as_f64();
        let frb = self.fpr[instr.b()].ps0_as_f64();
        let frc = self.fpr[instr.c()].ps0_as_f64();

        self.fpr[instr.d()].set_ps0_f64(-fra.mul_add(frc, frb));

        let fra = self.fpr[instr.a()].ps1_as_f64();
        let frb = self.fpr[instr.b()].ps1_as_f64();
        let frc = self.fpr[instr.c()].ps1_as_f64();

        self.fpr[instr.d()].set_ps1_f64(-fra.mul_add(frc, frb));

        if instr.rc() {
            self.update_cr1();
        }

        self.tick(1);
    }

    pub fn op_ps_nmsubx(&mut self, instr: Instruction, _: &mut Bus) {
        if !self.ensure_fp() {
            return;
        }

        let fra = self.fpr[instr.a()].ps0_as_f64();
        let frb = self.fpr[instr.b()].ps0_as_f64();
        let frc = self.fpr[instr.c()].ps0_as_f64();

        self.fpr[instr.d()].set_ps0_f64(-fra.mul_add(frc, -frb));

        let fra = self.fpr[instr.a()].ps1_as_f64();
        let frb = self.fpr[instr.b()].ps1_as_f64();
        let frc = self.fpr[instr.c()].ps1_as_f64();

        self.fpr[instr.d()].set_ps1_f64(-fra.mul_add(frc, -frb));

        if instr.rc() {
            self.update_cr1();
        }

        self.tick(1);
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

    pub fn op_ps_subx(&mut self, instr: Instruction, _: &mut Bus) {
        if !self.ensure_fp() {
            return;
        }

        let fra = self.fpr[instr.a()].ps0_as_f64();
        let frb = self.fpr[instr.b()].ps0_as_f64();

        self.fpr[instr.d()].set_ps0_f64(fra - frb);

        let fra = self.fpr[instr.a()].ps1_as_f64();
        let frb = self.fpr[instr.b()].ps1_as_f64();

        self.fpr[instr.d()].set_ps1_f64(fra - frb);

        if instr.rc() {
            self.update_cr1();
        }

        self.tick(1);
    }

    pub fn op_ps_sum0x(&mut self, instr: Instruction, _: &mut Bus) {
        if !self.ensure_fp() {
            return;
        }

        let ps0 = self.fpr[instr.a()].ps0_as_f64() + self.fpr[instr.b()].ps1_as_f64();
        let ps1 = self.fpr[instr.c()].ps1_as_f64();

        self.fpr[instr.d()].set_ps0_f64(ps0);
        self.fpr[instr.d()].set_ps1_f64(ps1);

        if instr.rc() {
            self.update_cr1();
        }

        self.tick(1);
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
    pub fn op_fabsx() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();
        cpu.msr.set_fp(true);

        let (frd, frb) = (6, 4);
        let instr = Instruction::new_fabsx(frd, frb);

        cpu.fpr[frb].set_ps0_f64(-123.45);
        cpu.op_fabsx(instr, &mut bus);

        assert_eq!(cpu.fpr[frd].ps0_as_f64(), 123.45);
    }

    #[test]
    fn op_fmaddsx() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();
        cpu.msr.set_fp(true);

        let (frd, fra, frc, frb) = (6, 4, 5, 7);
        let instr = Instruction::new_fmaddsx(frd, fra, frc, frb);

        cpu.fpr[fra].set_ps0_f64(2.0);
        cpu.fpr[frc].set_ps0_f64(3.0);
        cpu.fpr[frb].set_ps0_f64(4.0);
        cpu.op_fmaddsx(instr, &mut bus);

        assert_eq!(cpu.fpr[frd].ps0_as_f64(), 10.0);
    }

    #[test]
    fn op_fnmsubsx() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();
        cpu.msr.set_fp(true);

        let (frd, fra, frc, frb) = (6, 4, 5, 7);
        let instr = Instruction::new_fnmsubsx(frd, fra, frc, frb);

        cpu.fpr[fra].set_ps0_f64(2.0);
        cpu.fpr[frc].set_ps0_f64(3.0);
        cpu.fpr[frb].set_ps0_f64(1.0);
        cpu.op_fnmsubsx(instr, &mut bus);

        assert_eq!(cpu.fpr[frd].ps0_as_f64(), -5.0); // -(2*3-1)
    }

    #[test]
    pub fn op_faddx() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();
        cpu.msr.set_fp(true);

        let (frd, fra, frb) = (6, 4, 5);
        let instr = Instruction::new_faddx(frd, fra, frb);

        cpu.fpr[fra].set_ps0_f64(1.5);
        cpu.fpr[frb].set_ps0_f64(2.5);
        cpu.op_faddx(instr, &mut bus);

        assert_eq!(cpu.fpr[frd].ps0_as_f64(), 4.0);
    }

    #[test]
    pub fn op_fdivx() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();
        cpu.msr.set_fp(true);

        let (frd, fra, frb) = (6, 4, 5);
        let instr = Instruction::new_fdivx(frd, fra, frb);

        cpu.fpr[fra].set_ps0_f64(20.0);
        cpu.fpr[frb].set_ps0_f64(2.0);
        cpu.op_fdivx(instr, &mut bus);

        assert_eq!(cpu.fpr[frd].ps0_as_f64(), 10.0);

        // QNaN divisor: propagate NaN, no VXSNAN
        cpu.fpr[frd].set_ps0_f64(1.0);
        cpu.fpr[fra].set_ps0_f64(1.0);
        cpu.fpr[frb].set_ps0(0x7FF8_0000_0000_0001);
        cpu.op_fdivx(instr, &mut bus);
        assert!(cpu.fpr[frd].ps0_as_f64().is_nan());
        assert!(cpu.fpr[frd].ps0_as_f64().is_qnan());
        assert!(!cpu.fpscr.vxsnan());

        // SNaN divisor, VE clear: set VXSNAN and write QNaN
        cpu.fpscr.set_vxsnan(false);
        cpu.fpr[frd].set_ps0_f64(1.0);
        cpu.fpr[frb].set_ps0(0x7FF0_0000_0000_0001);
        cpu.op_fdivx(instr, &mut bus);
        assert!(cpu.fpscr.vxsnan());
        assert!(cpu.fpr[frd].ps0_as_f64().is_qnan());

        // SNaN divisor, VE set: set VXSNAN, do not update frD
        cpu.fpscr.set_ve(true);
        cpu.fpscr.set_vxsnan(false);
        cpu.fpr[frd].set_ps0_f64(42.0);
        cpu.fpr[frb].set_ps0(0x7FF0_0000_0000_0001);
        cpu.op_fdivx(instr, &mut bus);
        assert!(cpu.fpscr.vxsnan());
        assert_eq!(cpu.fpr[frd].ps0_as_f64(), 42.0);
    }

    #[test]
    pub fn op_fdivsx() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();
        cpu.msr.set_fp(true);
        cpu.hid2.set_pse(true);

        let (frd, fra, frb) = (6, 4, 5);
        let instr = Instruction::new_fdivsx(frd, fra, frb);

        cpu.fpr[fra].set_ps0_f64(20.0);
        cpu.fpr[frb].set_ps0_f64(2.0);
        cpu.op_fdivsx(instr, &mut bus);
        assert_eq!(cpu.fpr[frd].ps0_as_f64(), 10.0);
        assert_eq!(cpu.fpr[frd].ps1_as_f64(), 10.0);

        cpu.fpscr.set_vxsnan(false);
        cpu.fpr[fra].set_ps0_f64(1.0);
        cpu.fpr[frb].set_ps0(0x7FF0_0000_0000_0001);
        cpu.op_fdivsx(instr, &mut bus);
        assert!(cpu.fpscr.vxsnan());
        assert!(cpu.fpr[frd].ps0_as_f64().is_qnan());
        assert!(cpu.fpr[frd].ps1_as_f64().is_qnan());
    }

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
    pub fn op_fnabsx() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();
        cpu.msr.set_fp(true);

        let (frd, frb) = (6, 4);
        let instr = Instruction::new_fnabsx(frd, frb);

        cpu.fpr[frb].set_ps0_f64(77.1234);
        cpu.op_fnabsx(instr, &mut bus);

        assert_eq!(cpu.fpr[frd].ps0_as_f64(), -77.1234);

        cpu.fpr[frb].set_ps0_f64(-10.5);
        cpu.op_fnabsx(instr, &mut bus);

        assert_eq!(cpu.fpr[frd].ps0_as_f64(), -10.5);
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

    #[test]
    fn op_ps_negx() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();
        cpu.msr.set_fp(true);

        let (frd, frb) = (3, 4);
        let instr = Instruction::new_ps_negx(frd, frb);

        cpu.fpr[frb].set_ps0_f64(1.0);
        cpu.fpr[frb].set_ps1_f64(-2.5);
        cpu.op_ps_negx(instr, &mut bus);

        assert_eq!(cpu.fpr[frd].ps0_as_f64(), -1.0);
        assert_eq!(cpu.fpr[frd].ps1_as_f64(), 2.5);
    }

    #[test]
    fn op_fresx() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();
        cpu.msr.set_fp(true);

        let (frd, frb) = (6, 4);
        let instr = Instruction::new_fresx(frd, frb);

        cpu.fpr[frb].set_ps0_f64(4.0);
        cpu.op_fresx(instr, &mut bus);
        assert_eq!(cpu.fpr[frd].ps0_as_f64(), 0.25);

        cpu.fpr[frb].set_ps0_f64(-2.0);
        cpu.op_fresx(instr, &mut bus);
        assert_eq!(cpu.fpr[frd].ps0_as_f64(), -0.5);

        cpu.fpr[frb].set_ps0_f64(0.0);
        cpu.op_fresx(instr, &mut bus);
        assert!(cpu.fpr[frd].ps0_as_f64().is_infinite());
        assert!(cpu.fpscr.zx());
    }

    #[test]
    fn op_ps_cmpo0() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();
        cpu.msr.set_fp(true);
        cpu.hid2 = (1 << 29).into(); // HID2[PSE]

        let (crfd, fra, frb) = (2, 4, 5);
        let instr = Instruction::new_ps_cmpo0(crfd, fra, frb);

        // PS0 less-than
        // PS1 must not affect the compare
        cpu.fpr[fra].set_ps0_f64(1.0);
        cpu.fpr[fra].set_ps1_f64(100.0);
        cpu.fpr[frb].set_ps0_f64(2.0);
        cpu.fpr[frb].set_ps1_f64(-100.0);
        cpu.op_ps_cmpo0(instr, &mut bus);
        assert_eq!(cpu.fpscr.fpcc(), 0x8);
        assert_eq!((cpu.cr.as_u32() >> ((7 - crfd) * 4)) & 0xF, 0x8);

        cpu.fpr[fra].set_ps0_f64(3.0);
        cpu.fpr[frb].set_ps0_f64(1.0);
        cpu.op_ps_cmpo0(instr, &mut bus);
        assert_eq!(cpu.fpscr.fpcc(), 0x4);

        cpu.fpr[fra].set_ps0_f64(5.0);
        cpu.fpr[frb].set_ps0_f64(5.0);
        cpu.op_ps_cmpo0(instr, &mut bus);
        assert_eq!(cpu.fpscr.fpcc(), 0x2);

        cpu.fpr[fra].set_ps0_f64(f64::NAN);
        cpu.fpr[frb].set_ps0_f64(1.0);
        cpu.op_ps_cmpo0(instr, &mut bus);
        assert_eq!(cpu.fpscr.fpcc(), 0x1);
        assert!(cpu.fpscr.vxvc());
        assert!(!cpu.fpscr.vxsnan());
    }

    #[test]
    fn op_ps_cmpo1() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();
        cpu.msr.set_fp(true);
        cpu.hid2 = (1 << 29).into(); // HID2[PSE]

        let (crfd, fra, frb) = (3, 4, 5);
        let instr = Instruction::new_ps_cmpo1(crfd, fra, frb);

        // PS1 less-than
        // PS0 must not affect the compare
        cpu.fpr[fra].set_ps0_f64(100.0);
        cpu.fpr[fra].set_ps1_f64(1.0);
        cpu.fpr[frb].set_ps0_f64(-100.0);
        cpu.fpr[frb].set_ps1_f64(2.0);
        cpu.op_ps_cmpo1(instr, &mut bus);
        assert_eq!(cpu.fpscr.fpcc(), 0x8);
        assert_eq!((cpu.cr.as_u32() >> ((7 - crfd) * 4)) & 0xF, 0x8);

        cpu.fpr[fra].set_ps1_f64(3.0);
        cpu.fpr[frb].set_ps1_f64(1.0);
        cpu.op_ps_cmpo1(instr, &mut bus);
        assert_eq!(cpu.fpscr.fpcc(), 0x4);

        cpu.fpr[fra].set_ps1_f64(5.0);
        cpu.fpr[frb].set_ps1_f64(5.0);
        cpu.op_ps_cmpo1(instr, &mut bus);
        assert_eq!(cpu.fpscr.fpcc(), 0x2);

        cpu.fpr[fra].set_ps1_f64(f64::NAN);
        cpu.fpr[frb].set_ps1_f64(1.0);
        cpu.op_ps_cmpo1(instr, &mut bus);
        assert_eq!(cpu.fpscr.fpcc(), 0x1);
        assert!(cpu.fpscr.vxvc());
        assert!(!cpu.fpscr.vxsnan());
    }

    #[test]
    pub fn op_ps_madds0x() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();
        cpu.msr.set_fp(true);

        let (frd, fra, frc, frb) = (6, 4, 5, 7);
        let instr = Instruction::new_ps_madds0x(frd, fra, frc, frb);

        cpu.fpr[fra].set_ps0_f64(2.0);
        cpu.fpr[fra].set_ps1_f64(3.0);
        cpu.fpr[frc].set_ps0_f64(4.0);
        cpu.fpr[frc].set_ps1_f64(99.0); // unused, both lanes use c.ps0
        cpu.fpr[frb].set_ps0_f64(1.0);
        cpu.fpr[frb].set_ps1_f64(5.0);
        cpu.op_ps_madds0x(instr, &mut bus);

        assert_eq!(cpu.fpr[frd].ps0_as_f64(), 9.0); // 2*4+1
        assert_eq!(cpu.fpr[frd].ps1_as_f64(), 17.0); // 3*4+5
    }

    #[test]
    pub fn op_ps_madds1x() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();
        cpu.msr.set_fp(true);

        let (frd, fra, frc, frb) = (6, 4, 5, 7);
        let instr = Instruction::new_ps_madds1x(frd, fra, frc, frb);

        cpu.fpr[fra].set_ps0_f64(2.0);
        cpu.fpr[fra].set_ps1_f64(3.0);
        cpu.fpr[frc].set_ps0_f64(99.0); // unused, both lanes use c.ps1
        cpu.fpr[frc].set_ps1_f64(4.0);
        cpu.fpr[frb].set_ps0_f64(1.0);
        cpu.fpr[frb].set_ps1_f64(5.0);
        cpu.op_ps_madds1x(instr, &mut bus);

        assert_eq!(cpu.fpr[frd].ps0_as_f64(), 9.0); // 2*4+1
        assert_eq!(cpu.fpr[frd].ps1_as_f64(), 17.0); // 3*4+5
    }

    #[test]
    pub fn op_ps_msubx() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();
        cpu.msr.set_fp(true);

        let (frd, fra, frc, frb) = (6, 4, 5, 7);
        let instr = Instruction::new_ps_msubx(frd, fra, frc, frb);

        cpu.fpr[fra].set_ps0_f64(2.0);
        cpu.fpr[fra].set_ps1_f64(3.0);
        cpu.fpr[frc].set_ps0_f64(4.0);
        cpu.fpr[frc].set_ps1_f64(5.0);
        cpu.fpr[frb].set_ps0_f64(1.0);
        cpu.fpr[frb].set_ps1_f64(2.0);
        cpu.op_ps_msubx(instr, &mut bus);

        assert_eq!(cpu.fpr[frd].ps0_as_f64(), 7.0); // 2*4-1
        assert_eq!(cpu.fpr[frd].ps1_as_f64(), 13.0); // 3*5-2
    }

    #[test]
    pub fn op_ps_nmsubx() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();
        cpu.msr.set_fp(true);

        let (frd, fra, frc, frb) = (6, 4, 5, 7);
        let instr = Instruction::new_ps_nmsubx(frd, fra, frc, frb);

        cpu.fpr[fra].set_ps0_f64(2.0);
        cpu.fpr[fra].set_ps1_f64(3.0);
        cpu.fpr[frc].set_ps0_f64(4.0);
        cpu.fpr[frc].set_ps1_f64(5.0);
        cpu.fpr[frb].set_ps0_f64(1.0);
        cpu.fpr[frb].set_ps1_f64(2.0);
        cpu.op_ps_nmsubx(instr, &mut bus);

        assert_eq!(cpu.fpr[frd].ps0_as_f64(), -7.0); // -(2*4-1)
        assert_eq!(cpu.fpr[frd].ps1_as_f64(), -13.0); // -(3*5-2)
    }

    #[test]
    pub fn op_ps_nmaddx() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();
        cpu.msr.set_fp(true);

        let (frd, fra, frc, frb) = (6, 4, 5, 7);
        let instr = Instruction::new_ps_nmaddx(frd, fra, frc, frb);

        cpu.fpr[fra].set_ps0_f64(2.0);
        cpu.fpr[fra].set_ps1_f64(3.0);
        cpu.fpr[frc].set_ps0_f64(4.0);
        cpu.fpr[frc].set_ps1_f64(5.0);
        cpu.fpr[frb].set_ps0_f64(1.0);
        cpu.fpr[frb].set_ps1_f64(2.0);
        cpu.op_ps_nmaddx(instr, &mut bus);

        assert_eq!(cpu.fpr[frd].ps0_as_f64(), -9.0); // -(2*4+1)
        assert_eq!(cpu.fpr[frd].ps1_as_f64(), -17.0); // -(3*5+2)
    }

    #[test]
    pub fn op_ps_sum0x() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();
        cpu.msr.set_fp(true);

        let (frd, fra, frc, frb) = (6, 4, 5, 7);
        let instr = Instruction::new_ps_sum0x(frd, fra, frc, frb);

        cpu.fpr[fra].set_ps0_f64(2.0);
        cpu.fpr[fra].set_ps1_f64(99.0); // unused
        cpu.fpr[frb].set_ps0_f64(88.0); // unused
        cpu.fpr[frb].set_ps1_f64(3.0);
        cpu.fpr[frc].set_ps0_f64(77.0); // unused
        cpu.fpr[frc].set_ps1_f64(7.0);
        cpu.op_ps_sum0x(instr, &mut bus);

        assert_eq!(cpu.fpr[frd].ps0_as_f64(), 5.0); // A.ps0 + B.ps1
        assert_eq!(cpu.fpr[frd].ps1_as_f64(), 7.0); // C.ps1
    }

    #[test]
    pub fn op_ps_muls0x() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();
        cpu.msr.set_fp(true);

        let (frd, fra, frc) = (6, 4, 5);
        let instr = Instruction::new_ps_muls0x(frd, fra, frc);

        cpu.fpr[fra].set_ps0_f64(2.0);
        cpu.fpr[fra].set_ps1_f64(3.0);
        cpu.fpr[frc].set_ps0_f64(4.0);
        cpu.fpr[frc].set_ps1_f64(99.0); // unused, both lanes use c.ps0
        cpu.op_ps_muls0x(instr, &mut bus);

        assert_eq!(cpu.fpr[frd].ps0_as_f64(), 8.0);
        assert_eq!(cpu.fpr[frd].ps1_as_f64(), 12.0);
    }

    #[test]
    pub fn op_ps_subx() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();
        cpu.msr.set_fp(true);

        let (frd, fra, frb) = (6, 4, 5);
        let instr = Instruction::new_ps_subx(frd, fra, frb);

        cpu.fpr[fra].set_ps0_f64(10.0);
        cpu.fpr[fra].set_ps1_f64(5.0);
        cpu.fpr[frb].set_ps0_f64(3.0);
        cpu.fpr[frb].set_ps1_f64(2.0);
        cpu.op_ps_subx(instr, &mut bus);

        assert_eq!(cpu.fpr[frd].ps0_as_f64(), 7.0);
        assert_eq!(cpu.fpr[frd].ps1_as_f64(), 3.0);
    }
}
