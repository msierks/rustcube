use super::{
    float::*,
    instruction::Instruction,
    registers::*,
    utils::{convert_to_double, convert_to_single, sign_ext_12},
    Cpu,
};
use crate::bus::Bus;

impl Cpu {
    fn get_ea(&mut self, instr: Instruction) -> u32 {
        if instr.a() == 0 {
            (instr.simm() as i32) as u32
        } else {
            self.gpr[instr.a()].wrapping_add((instr.simm() as i32) as u32)
        }
    }

    fn get_ea_u(&mut self, instr: Instruction) -> u32 {
        self.gpr[instr.a()].wrapping_add((instr.simm() as i32) as u32)
    }

    fn get_ea_x(&mut self, instr: Instruction) -> u32 {
        if instr.a() == 0 {
            self.gpr[instr.b()]
        } else {
            self.gpr[instr.a()].wrapping_add(self.gpr[instr.b()])
        }
    }

    fn get_ea_ux(&mut self, instr: Instruction) -> u32 {
        self.gpr[instr.a()].wrapping_add(self.gpr[instr.b()])
    }

    fn get_ea_psq(&self, instr: Instruction) -> u32 {
        let d = sign_ext_12(instr.uimm_1()) as u32;
        if instr.a() == 0 {
            d
        } else {
            self.gpr[instr.a()].wrapping_add(d)
        }
    }

    pub fn op_dcbf(&mut self, _instr: Instruction, _: &mut Bus) {
        // don't do anything

        self.tick(3);
    }

    pub fn op_dcbi(&mut self, _instr: Instruction, _: &mut Bus) {
        // don't do anything

        self.tick(3);
    }

    pub fn op_dcbst(&mut self, _instr: Instruction, _: &mut Bus) {
        self.tick(3);
    }

    pub fn op_dcbt(&mut self, _instr: Instruction, _: &mut Bus) {
        self.tick(2);
    }

    pub fn op_dcbtst(&mut self, _instr: Instruction, _: &mut Bus) {
        self.tick(2);
    }

    // Ignore this for the time being
    pub fn op_dcbz(&mut self, _instr: Instruction, _: &mut Bus) {
        self.tick(3);
    }

    // Ignore this for the time being
    pub fn op_dcbz_l(&mut self, _instr: Instruction, _: &mut Bus) {
        self.tick(3);
        unimplemented!();
    }

    pub fn op_eciwx(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_eciwx");
    }

    pub fn op_ecowx(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_ecowx");
    }

    pub fn op_icbi(&mut self, _instr: Instruction, _: &mut Bus) {
        // don't do anything

        self.tick(3);
    }

    pub fn op_lbz(&mut self, instr: Instruction, bus: &mut Bus) {
        let ea = self.get_ea(instr);

        self.gpr[instr.d()] = u32::from(self.read::<u8>(bus, ea));

        self.tick(2);
    }

    pub fn op_lbzu(&mut self, instr: Instruction, bus: &mut Bus) {
        if instr.a() == 0 || instr.a() == instr.d() {
            panic!("lbzu: invalid instruction");
        }

        let ea = self.gpr[instr.a()].wrapping_add(instr.simm() as u32);

        self.gpr[instr.d()] = u32::from(self.read::<u8>(bus, ea));
        self.gpr[instr.a()] = ea;

        self.tick(2);
    }

    pub fn op_lbzux(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_lbzux");
    }

    pub fn op_lbzx(&mut self, instr: Instruction, bus: &mut Bus) {
        let ea = self.get_ea_x(instr);

        self.gpr[instr.d()] = u32::from(self.read::<u8>(bus, ea));

        self.tick(2);
    }

    pub fn op_lfd(&mut self, instr: Instruction, bus: &mut Bus) {
        let ea = self.get_ea(instr);

        // FIXME: check for DSI exception ???
        let val = self.read::<u64>(bus, ea);

        self.fpr[instr.d()].set_ps0(val);

        self.tick(2);
    }

    pub fn op_lfdu(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_lfdu");
    }

    pub fn op_lfdux(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_lfdux");
    }

    pub fn op_lfdx(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_lfdx");
    }

    pub fn op_lfs(&mut self, instr: Instruction, bus: &mut Bus) {
        let ea = self.get_ea(instr);

        let val = convert_to_double(self.read::<u32>(bus, ea));

        self.fpr[instr.d()].set_ps0(val);

        if self.hid2.pse() {
            self.fpr[instr.d()].set_ps1(val);
        }

        self.tick(2);
    }

    pub fn op_lfsu(&mut self, instr: Instruction, bus: &mut Bus) {
        let ea = self.get_ea_u(instr);

        let val = convert_to_double(self.read::<u32>(bus, ea));

        self.fpr[instr.d()].set_ps0(val);

        if self.hid2.pse() {
            self.fpr[instr.d()].set_ps1(val);
        }

        self.gpr[instr.a()] = ea;
    }

    pub fn op_lfsux(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_lfsux");
    }

    pub fn op_lfsx(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_lfsx");
    }

    pub fn op_lha(&mut self, instr: Instruction, bus: &mut Bus) {
        let ea = self.get_ea(instr);

        self.gpr[instr.d()] = ((self.read::<u16>(bus, ea) as i16) as i32) as u32;

        self.tick(2);
    }

    pub fn op_lhau(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_lhau");
    }

    pub fn op_lhaux(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_lhaux");
    }

    pub fn op_lhax(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_lhax");
    }

    pub fn op_lhbrx(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_lhbrx");
    }

    pub fn op_lhz(&mut self, instr: Instruction, bus: &mut Bus) {
        let ea = self.get_ea(instr);

        self.gpr[instr.d()] = u32::from(self.read::<u16>(bus, ea));

        self.tick(2);
    }

    pub fn op_lhzu(&mut self, instr: Instruction, bus: &mut Bus) {
        let ea = self.get_ea_u(instr);

        self.gpr[instr.d()] = u32::from(self.read::<u16>(bus, ea));
        self.gpr[instr.a()] = ea;

        self.tick(2);
    }

    pub fn op_lhzux(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_lhzux");
    }

    pub fn op_lhzx(&mut self, instr: Instruction, bus: &mut Bus) {
        let ea = self.get_ea_x(instr);

        self.gpr[instr.d()] = self.read::<u16>(bus, ea) as u32;
    }

    pub fn op_lmw(&mut self, instr: Instruction, bus: &mut Bus) {
        let mut ea = self.get_ea(instr);
        let mut r = instr.d();
        let n = (32 - r) as u32;

        while r < 32 {
            self.gpr[r] = self.read::<u32>(bus, ea);

            r += 1;
            ea += 4;
        }

        self.tick(2 + n);
    }

    pub fn op_lswi(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_lswi");
    }

    pub fn op_lswx(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_lswx");
    }

    pub fn op_lwarx(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_lwarx");
    }

    pub fn op_lwbrx(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_lwbrx");
    }

    pub fn op_lwz(&mut self, instr: Instruction, bus: &mut Bus) {
        let ea = self.get_ea(instr);

        self.gpr[instr.d()] = self.read::<u32>(bus, ea);

        self.tick(2);
    }

    pub fn op_lwzu(&mut self, instr: Instruction, bus: &mut Bus) {
        let ea = self.get_ea_u(instr);

        self.gpr[instr.d()] = self.read::<u32>(bus, ea);
        self.gpr[instr.a()] = ea;

        self.tick(2);
    }

    pub fn op_lwzux(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_lwzux");
    }

    pub fn op_lwzx(&mut self, instr: Instruction, bus: &mut Bus) {
        let ea = self.get_ea_x(instr);

        self.gpr[instr.d()] = self.read::<u32>(bus, ea);

        self.tick(2);
    }

    pub fn op_psq_l(&mut self, instr: Instruction, bus: &mut Bus) {
        if !self.ensure_ps() {
            return;
        }

        let ea = self.get_ea_psq(instr);

        let gqr = Gqr(self.spr[SPR_GQR0 + instr.i()]);
        let ld_type = gqr.lt();
        let ld_scale = gqr.ls();

        if instr.w() {
            let val = match ld_type {
                QUANTIZE_FLOAT => f64::from_bits(convert_to_double(self.read::<u32>(bus, ea))),
                QUANTIZE_U8 | QUANTIZE_I8 => {
                    dequantize(self.read::<u8>(bus, ea) as u32, ld_type, ld_scale) as f64
                }
                QUANTIZE_U16 | QUANTIZE_I16 => {
                    dequantize(self.read::<u16>(bus, ea) as u32, ld_type, ld_scale) as f64
                }
                _ => panic!("psq_l: invalid type {:}", ld_type),
            };

            self.fpr[instr.d()].set_ps0_f64(val as f64);
            self.fpr[instr.d()].set_ps1_f64(1.0);
        } else {
            let (val1, val2) = match ld_type {
                QUANTIZE_FLOAT => (
                    f32::from_bits(self.read::<u32>(bus, ea)),
                    f32::from_bits(self.read::<u32>(bus, ea + 4)),
                ),
                QUANTIZE_U8 | QUANTIZE_I8 => (
                    dequantize(self.read::<u8>(bus, ea) as u32, ld_type, ld_scale),
                    dequantize(self.read::<u8>(bus, ea + 1) as u32, ld_type, ld_scale),
                ),
                QUANTIZE_U16 | QUANTIZE_I16 => (
                    dequantize(self.read::<u16>(bus, ea) as u32, ld_type, ld_scale),
                    dequantize(self.read::<u16>(bus, ea + 2) as u32, ld_type, ld_scale),
                ),
                _ => panic!("psq_l: invalid type {:}", ld_type),
            };
            self.fpr[instr.d()].set_ps0_f64(val1 as f64);
            self.fpr[instr.d()].set_ps1_f64(val2 as f64);
        }

        self.tick(3);
    }

    pub fn op_psq_lu(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_psq_lu");
    }

    pub fn op_psq_lux(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_psq_lux");
    }

    pub fn op_psq_lx(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_psq_lx");
    }

    pub fn op_psq_st(&mut self, instr: Instruction, bus: &mut Bus) {
        if !self.ensure_ps() {
            return;
        }

        let ea = self.get_ea_psq(instr);

        let gqr = Gqr(self.spr[SPR_GQR0 + instr.i()]);
        let st_type = gqr.st();
        let st_scale = gqr.ss();

        let ps0 = self.fpr[instr.s()].ps0();
        let ps1 = self.fpr[instr.s()].ps1();
        let ps0_f32 = self.fpr[instr.s()].ps0_as_f64() as f32;
        let ps1_f32 = self.fpr[instr.s()].ps1_as_f64() as f32;

        if instr.w() {
            match st_type {
                QUANTIZE_FLOAT => self.write::<u32>(bus, ea, convert_to_single(ps0)),
                QUANTIZE_U8 | QUANTIZE_I8 => {
                    self.write::<u8>(bus, ea, quantize(ps0_f32, st_type, st_scale) as u8);
                }
                QUANTIZE_U16 | QUANTIZE_I16 => {
                    self.write::<u16>(bus, ea, quantize(ps0_f32, st_type, st_scale) as u16);
                }
                _ => panic!("psq_st: invalid type {:}", st_type),
            }
        } else {
            match st_type {
                QUANTIZE_FLOAT => {
                    self.write::<u32>(bus, ea, convert_to_single(ps0));
                    self.write::<u32>(bus, ea.wrapping_add(4), convert_to_single(ps1));
                }
                QUANTIZE_U8 | QUANTIZE_I8 => {
                    self.write::<u8>(bus, ea, quantize(ps0_f32, st_type, st_scale) as u8);
                    self.write::<u8>(bus, ea + 1, quantize(ps1_f32, st_type, st_scale) as u8);
                }
                QUANTIZE_U16 | QUANTIZE_I16 => {
                    self.write::<u16>(bus, ea, quantize(ps0_f32, st_type, st_scale) as u16);
                    self.write::<u16>(bus, ea + 2, quantize(ps1_f32, st_type, st_scale) as u16);
                }
                _ => panic!("psq_st: invalid type {:}", st_type),
            }
        }

        self.tick(2);
    }

    pub fn op_psq_stu(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_psq_stu");
    }

    pub fn op_psq_stux(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_psq_stux");
    }

    pub fn op_psq_stx(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_psq_stx");
    }

    pub fn op_stb(&mut self, instr: Instruction, bus: &mut Bus) {
        let ea = self.get_ea(instr);

        self.write::<u8>(bus, ea, self.gpr[instr.s()] as u8);

        self.tick(2);
    }

    pub fn op_stbu(&mut self, instr: Instruction, bus: &mut Bus) {
        let ea = self.get_ea_u(instr);

        self.write::<u8>(bus, ea, self.gpr[instr.s()] as u8);

        self.gpr[instr.a()] = ea;

        self.tick(2);
    }

    pub fn op_stbux(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_stbux");
    }

    pub fn op_stbx(&mut self, instr: Instruction, bus: &mut Bus) {
        let ea = self.get_ea_x(instr);

        self.write::<u8>(bus, ea, self.gpr[instr.s()] as u8);

        self.tick(2);
    }

    pub fn op_stfd(&mut self, instr: Instruction, bus: &mut Bus) {
        let ea = self.get_ea(instr);

        self.write::<u64>(bus, ea, self.fpr[instr.s()].ps0());

        self.tick(2);
    }

    pub fn op_stfdu(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_stfdu");
    }

    pub fn op_stfdux(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_stfdux");
    }

    pub fn op_stfdx(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_stfdx");
    }

    pub fn op_stfiwx(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_stfiwx");
    }

    pub fn op_stfs(&mut self, instr: Instruction, bus: &mut Bus) {
        let ea = self.get_ea(instr);

        let val = self.fpr[instr.s()].ps0();

        self.write::<u32>(bus, ea, convert_to_single(val));

        self.tick(2);
    }

    pub fn op_stfsu(&mut self, instr: Instruction, bus: &mut Bus) {
        let ea = self.get_ea_u(instr);

        let val = self.fpr[instr.s()].ps0();

        self.write::<u32>(bus, ea, convert_to_single(val));

        self.gpr[instr.a()] = ea;

        self.tick(2);
    }

    pub fn op_stfsux(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_stfsux");
    }

    pub fn op_stfsx(&mut self, instr: Instruction, bus: &mut Bus) {
        let ea = self.get_ea_x(instr);

        let val = self.fpr[instr.s()].ps0();

        self.write::<u32>(bus, ea, convert_to_single(val));
    }

    pub fn op_sth(&mut self, instr: Instruction, bus: &mut Bus) {
        let ea = self.get_ea(instr);

        self.write::<u16>(bus, ea, self.gpr[instr.s()] as u16);

        self.tick(2);
    }

    pub fn op_sthbrx(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_sthbrx");
    }

    pub fn op_sthu(&mut self, instr: Instruction, bus: &mut Bus) {
        let ea = self.get_ea_u(instr);

        self.write::<u16>(bus, ea, self.gpr[instr.s()] as u16);

        self.gpr[instr.a()] = ea;

        self.tick(2);
    }

    pub fn op_sthux(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_sthux");
    }

    pub fn op_sthx(&mut self, instr: Instruction, bus: &mut Bus) {
        let ea = self.get_ea_ux(instr);

        self.write::<u16>(bus, ea, self.gpr[instr.s()] as u16);

        self.tick(2);
    }

    // FIXME: handle alignment interrupt if ea is not multiple of 4
    pub fn op_stmw(&mut self, instr: Instruction, bus: &mut Bus) {
        let mut ea = self.get_ea(instr);
        let mut r = instr.s();
        let n = (32 - r) as u32;

        while r < 32 {
            self.write::<u32>(bus, ea, self.gpr[r]);

            r += 1;
            ea += 4;
        }

        self.tick(2 + n);
    }

    pub fn op_stswi(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_stswi");
    }

    pub fn op_stswx(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_stswx");
    }

    pub fn op_stw(&mut self, instr: Instruction, bus: &mut Bus) {
        let addr = self.get_ea(instr);

        self.write::<u32>(bus, addr, self.gpr[instr.s()]);

        self.tick(2);
    }

    pub fn op_stwbrx(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_stwbrx");
    }

    pub fn op_stwcx_rc(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_stwcx_rc");
    }

    pub fn op_stwu(&mut self, instr: Instruction, bus: &mut Bus) {
        if instr.a() == 0 {
            panic!("stwu: invalid instruction");
        }

        let ea = self.get_ea_u(instr);

        self.write::<u32>(bus, ea, self.gpr[instr.s()]);

        self.gpr[instr.a()] = ea;

        self.tick(2);
    }

    pub fn op_stwux(&mut self, instr: Instruction, bus: &mut Bus) {
        if instr.a() == 0 {
            panic!("stwu: invalid instruction");
        }

        let ea = self.get_ea_ux(instr);

        self.write::<u32>(bus, ea, self.gpr[instr.s()]);

        self.gpr[instr.a()] = ea;

        self.tick(2);
    }

    pub fn op_stwx(&mut self, instr: Instruction, bus: &mut Bus) {
        let ea = self.get_ea_x(instr);

        self.write::<u32>(bus, ea, self.gpr[instr.s()]);

        self.tick(2);
    }

    pub fn op_tlbie(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_tlbie");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // load and store ops
    #[test]
    fn op_dcbf() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (ra, rb) = (4, 3);
        let instr = Instruction::new_dcbf(ra, rb);

        cpu.op_dcbf(instr, &mut bus);
    }
}
