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

    fn get_ea_psq_u(&self, instr: Instruction) -> u32 {
        self.gpr[instr.a()].wrapping_add(sign_ext_12(instr.uimm_1()) as u32)
    }

    pub fn op_dcbf(&mut self, _instr: Instruction, _: &mut Bus) {
        // don't do anything

        self.tick(3);
    }

    pub fn op_dcbi(&mut self, _instr: Instruction, _: &mut Bus) {
        if self.msr.pr() {
            self.generate_program_exception(ProgramException::PrivilegedInstruction);
            return;
        }

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

    pub fn op_dcbz_l(&mut self, instr: Instruction, bus: &mut Bus) {
        // Illegal when locked cache is disabled (HID2[LCE] = 0).
        if !self.hid2.lce() {
            self.generate_program_exception(ProgramException::IllegalInstruction);
            return;
        }

        let ea = self.get_ea_x(instr) & !0x1F;
        self.write_bytes(bus, ea, &[0u8; 32]);

        self.tick(3);
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

        if let Some(val) = self.read::<u8>(bus, ea) {
            self.gpr[instr.d()] = u32::from(val);
        }

        self.tick(2);
    }

    pub fn op_lbzu(&mut self, instr: Instruction, bus: &mut Bus) {
        if instr.a() == 0 || instr.a() == instr.d() {
            panic!("lbzu: invalid instruction");
        }

        let ea = self.gpr[instr.a()].wrapping_add(instr.simm() as u32);

        if let Some(val) = self.read::<u8>(bus, ea) {
            self.gpr[instr.d()] = u32::from(val);
            self.gpr[instr.a()] = ea;
        }

        self.tick(2);
    }

    pub fn op_lbzux(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_lbzux");
    }

    pub fn op_lbzx(&mut self, instr: Instruction, bus: &mut Bus) {
        let ea = self.get_ea_x(instr);

        if let Some(val) = self.read::<u8>(bus, ea) {
            self.gpr[instr.d()] = u32::from(val);
        }

        self.tick(2);
    }

    pub fn op_lfd(&mut self, instr: Instruction, bus: &mut Bus) {
        if !self.ensure_fp() {
            return;
        }

        let ea = self.get_ea(instr);

        if let Some(val) = self.read::<u64>(bus, ea) {
            self.fpr[instr.d()].set_ps0(val);
        }

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
        if !self.ensure_fp() {
            return;
        }

        let ea = self.get_ea(instr);

        if let Some(raw) = self.read::<u32>(bus, ea) {
            let val = convert_to_double(raw);
            self.fpr[instr.d()].set_ps0(val);
            if self.hid2.pse() {
                self.fpr[instr.d()].set_ps1(val);
            }
        }

        self.tick(2);
    }

    pub fn op_lfsu(&mut self, instr: Instruction, bus: &mut Bus) {
        if !self.ensure_fp() {
            return;
        }

        let ea = self.get_ea_u(instr);

        if let Some(raw) = self.read::<u32>(bus, ea) {
            let val = convert_to_double(raw);
            self.fpr[instr.d()].set_ps0(val);
            if self.hid2.pse() {
                self.fpr[instr.d()].set_ps1(val);
            }
            self.gpr[instr.a()] = ea;
        }
    }

    pub fn op_lfsux(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_lfsux");
    }

    pub fn op_lfsx(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_lfsx");
    }

    pub fn op_lha(&mut self, instr: Instruction, bus: &mut Bus) {
        let ea = self.get_ea(instr);

        if let Some(val) = self.read::<u16>(bus, ea) {
            self.gpr[instr.d()] = i32::from(val as i16) as u32;
        }

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

        if let Some(val) = self.read::<u16>(bus, ea) {
            self.gpr[instr.d()] = u32::from(val);
        }

        self.tick(2);
    }

    pub fn op_lhzu(&mut self, instr: Instruction, bus: &mut Bus) {
        let ea = self.get_ea_u(instr);

        if let Some(val) = self.read::<u16>(bus, ea) {
            self.gpr[instr.d()] = u32::from(val);
            self.gpr[instr.a()] = ea;
        }

        self.tick(2);
    }

    pub fn op_lhzux(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_lhzux");
    }

    pub fn op_lhzx(&mut self, instr: Instruction, bus: &mut Bus) {
        let ea = self.get_ea_x(instr);

        if let Some(val) = self.read::<u16>(bus, ea) {
            self.gpr[instr.d()] = u32::from(val);
        }
    }

    pub fn op_lmw(&mut self, instr: Instruction, bus: &mut Bus) {
        let mut ea = self.get_ea(instr);
        let mut r = instr.d();
        let n = (32 - r) as u32;

        while r < 32 {
            match self.read::<u32>(bus, ea) {
                Some(val) => self.gpr[r] = val,
                None => break,
            }

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

        if let Some(val) = self.read::<u32>(bus, ea) {
            self.gpr[instr.d()] = val;
        }

        self.tick(2);
    }

    pub fn op_lwzu(&mut self, instr: Instruction, bus: &mut Bus) {
        let ea = self.get_ea_u(instr);

        if let Some(val) = self.read::<u32>(bus, ea) {
            self.gpr[instr.d()] = val;
            self.gpr[instr.a()] = ea;
        }

        self.tick(2);
    }

    pub fn op_lwzux(&mut self, _instr: Instruction, _: &mut Bus) {
        unimplemented!("op_lwzux");
    }

    pub fn op_lwzx(&mut self, instr: Instruction, bus: &mut Bus) {
        let ea = self.get_ea_x(instr);

        if let Some(val) = self.read::<u32>(bus, ea) {
            self.gpr[instr.d()] = val;
        }

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
                QUANTIZE_FLOAT => self
                    .read::<u32>(bus, ea)
                    .map(|v| f64::from_bits(convert_to_double(v))),
                QUANTIZE_U8 | QUANTIZE_I8 => self
                    .read::<u8>(bus, ea)
                    .map(|v| dequantize(u32::from(v), ld_type, ld_scale) as f64),
                QUANTIZE_U16 | QUANTIZE_I16 => self
                    .read::<u16>(bus, ea)
                    .map(|v| dequantize(u32::from(v), ld_type, ld_scale) as f64),
                _ => panic!("psq_l: invalid type {:}", ld_type),
            };

            if let Some(val) = val {
                self.fpr[instr.d()].set_ps0_f64(val);
                self.fpr[instr.d()].set_ps1_f64(1.0);
            }
        } else {
            let pair = match ld_type {
                QUANTIZE_FLOAT => self.read::<u32>(bus, ea).and_then(|a| {
                    self.read::<u32>(bus, ea + 4)
                        .map(|b| (f32::from_bits(a), f32::from_bits(b)))
                }),
                QUANTIZE_U8 | QUANTIZE_I8 => self.read::<u8>(bus, ea).and_then(|a| {
                    self.read::<u8>(bus, ea + 1).map(|b| {
                        (
                            dequantize(u32::from(a), ld_type, ld_scale),
                            dequantize(u32::from(b), ld_type, ld_scale),
                        )
                    })
                }),
                QUANTIZE_U16 | QUANTIZE_I16 => self.read::<u16>(bus, ea).and_then(|a| {
                    self.read::<u16>(bus, ea + 2).map(|b| {
                        (
                            dequantize(u32::from(a), ld_type, ld_scale),
                            dequantize(u32::from(b), ld_type, ld_scale),
                        )
                    })
                }),
                _ => panic!("psq_l: invalid type {:}", ld_type),
            };
            if let Some((val1, val2)) = pair {
                self.fpr[instr.d()].set_ps0_f64(val1 as f64);
                self.fpr[instr.d()].set_ps1_f64(val2 as f64);
            }
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
        self.store_psq(bus, ea, instr);

        self.tick(2);
    }

    pub fn op_psq_stu(&mut self, instr: Instruction, bus: &mut Bus) {
        if !self.ensure_ps() {
            return;
        }

        let ea = self.get_ea_psq_u(instr);
        if self.store_psq(bus, ea, instr) {
            self.gpr[instr.a()] = ea;
        }

        self.tick(2);
    }

    /// Quantized paired-single store at `ea`. Returns false if a write took a DSI.
    fn store_psq(&mut self, bus: &mut Bus, ea: u32, instr: Instruction) -> bool {
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
                    self.write::<u8>(bus, ea, quantize(ps0_f32, st_type, st_scale) as u8)
                }
                QUANTIZE_U16 | QUANTIZE_I16 => {
                    self.write::<u16>(bus, ea, quantize(ps0_f32, st_type, st_scale) as u16)
                }
                _ => panic!("psq_st: invalid type {:}", st_type),
            }
        } else {
            match st_type {
                QUANTIZE_FLOAT => {
                    self.write::<u32>(bus, ea, convert_to_single(ps0))
                        && self.write::<u32>(bus, ea.wrapping_add(4), convert_to_single(ps1))
                }
                QUANTIZE_U8 | QUANTIZE_I8 => {
                    self.write::<u8>(bus, ea, quantize(ps0_f32, st_type, st_scale) as u8)
                        && self.write::<u8>(bus, ea + 1, quantize(ps1_f32, st_type, st_scale) as u8)
                }
                QUANTIZE_U16 | QUANTIZE_I16 => {
                    self.write::<u16>(bus, ea, quantize(ps0_f32, st_type, st_scale) as u16)
                        && self.write::<u16>(
                            bus,
                            ea + 2,
                            quantize(ps1_f32, st_type, st_scale) as u16,
                        )
                }
                _ => panic!("psq_st: invalid type {:}", st_type),
            }
        }
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

        if self.write::<u8>(bus, ea, self.gpr[instr.s()] as u8) {
            self.gpr[instr.a()] = ea;
        }

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
        if !self.ensure_fp() {
            return;
        }

        let ea = self.get_ea_u(instr);

        let val = self.fpr[instr.s()].ps0();

        if self.write::<u32>(bus, ea, convert_to_single(val)) {
            self.gpr[instr.a()] = ea;
        }

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

        if self.write::<u16>(bus, ea, self.gpr[instr.s()] as u16) {
            self.gpr[instr.a()] = ea;
        }

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
            if !self.write::<u32>(bus, ea, self.gpr[r]) {
                break;
            }

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

        if self.write::<u32>(bus, ea, self.gpr[instr.s()]) {
            self.gpr[instr.a()] = ea;
        }

        self.tick(2);
    }

    pub fn op_stwux(&mut self, instr: Instruction, bus: &mut Bus) {
        if instr.a() == 0 {
            panic!("stwu: invalid instruction");
        }

        let ea = self.get_ea_ux(instr);

        if self.write::<u32>(bus, ea, self.gpr[instr.s()]) {
            self.gpr[instr.a()] = ea;
        }

        self.tick(2);
    }

    pub fn op_stwx(&mut self, instr: Instruction, bus: &mut Bus) {
        let ea = self.get_ea_x(instr);

        self.write::<u32>(bus, ea, self.gpr[instr.s()]);

        self.tick(2);
    }

    pub fn op_tlbie(&mut self, instr: Instruction, _: &mut Bus) {
        if self.msr.pr() {
            self.generate_program_exception(ProgramException::PrivilegedInstruction);
            return;
        }

        let ea = self.gpr[instr.b()];
        self.immu.invalidate_tlb_entry(ea);
        self.dmmu.invalidate_tlb_entry(ea);

        self.tick(1);
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

    #[test]
    fn op_dcbz_l() {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();

        let (ra, rb) = (4, 3);
        let instr = Instruction::new_dcbz_l(ra, rb);
        cpu.gpr[ra] = 0x0000_1000;
        cpu.gpr[rb] = 0x10; // ea = 0x1010 -> aligned to 0x1000

        // HID2[LCE] = 0 -> illegal instruction
        cpu.state.exceptions = 0;
        cpu.op_dcbz_l(instr, &mut bus);
        assert_eq!(cpu.state.exceptions, super::super::EXCEPTION_PROGRAM);
        assert_eq!(
            cpu.program_exception_srr1,
            ProgramException::IllegalInstruction.srr1_bits()
        );

        // HID2[LCE] = 1 -> zero 32-byte cache line
        cpu.hid2 = (1 << 28).into();
        cpu.state.exceptions = 0;
        cpu.program_exception_srr1 = 0;
        for i in 0..8 {
            cpu.write::<u32>(&mut bus, 0x0000_1000 + i * 4, 0xDEAD_BEEF);
        }
        // Neighboring line marker
        cpu.write::<u32>(&mut bus, 0x0000_1020, 0xCAFE_BABE);

        cpu.op_dcbz_l(instr, &mut bus);
        assert_eq!(cpu.state.exceptions, 0);
        for i in 0..8 {
            assert_eq!(cpu.read::<u32>(&mut bus, 0x0000_1000 + i * 4), Some(0));
        }
        assert_eq!(cpu.read::<u32>(&mut bus, 0x0000_1020), Some(0xCAFE_BABE));
    }
}
