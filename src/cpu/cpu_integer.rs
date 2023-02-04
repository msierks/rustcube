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

/// Helper to check if operation results in an overflow. This is determined by checking if both
/// operands signs bits are the same but the results sign bit is different.
///
/// Note: Overflow flag is only relavent to signed arithmetic
fn check_overflowed(a: u32, b: u32, result: u32) -> bool {
    ((a ^ result) & (b ^ result)) >> 31 != 0
}

fn op_addcx(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()];
    let rb = ctx.cpu.gpr[instr.b()];

    let (rd, ca) = ra.overflowing_add(rb);

    ctx.cpu.gpr[instr.d()] = rd;

    ctx.cpu.xer.set_carry(ca);

    if instr.oe() {
        ctx.cpu.set_xer_so(check_overflowed(ra, rb, rd));
    }

    if instr.rc() {
        ctx.cpu.update_cr0(rd);
    }
}

fn op_addx(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()];
    let rb = ctx.cpu.gpr[instr.b()];
    let rd = ra.wrapping_add(rb);

    ctx.cpu.gpr[instr.d()] = rd;

    if instr.oe() {
        ctx.cpu.set_xer_so(check_overflowed(ra, rb, rd));
    }

    if instr.rc() {
        ctx.cpu.update_cr0(ctx.cpu.gpr[instr.d()]);
    }
}

fn op_addi(ctx: &mut Context, instr: Instruction) {
    if instr.a() == 0 {
        ctx.cpu.gpr[instr.d()] = i32::from(instr.simm()) as u32;
    } else {
        ctx.cpu.gpr[instr.d()] =
            ctx.cpu.gpr[instr.a()].wrapping_add(i32::from(instr.simm()) as u32);
    }
}

fn op_addic(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()];
    let imm = i32::from(instr.simm()) as u32;

    let (rd, ca) = ra.overflowing_add(imm);

    ctx.cpu.gpr[instr.d()] = rd;

    ctx.cpu.xer.set_carry(ca);
}

fn op_addic_rc(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()];
    let imm = i32::from(instr.simm()) as u32;

    let (rd, ca) = ra.overflowing_add(imm);

    ctx.cpu.gpr[instr.d()] = rd;

    ctx.cpu.xer.set_carry(ca);

    ctx.cpu.update_cr0(rd);
}

fn op_addis(ctx: &mut Context, instr: Instruction) {
    if instr.a() == 0 {
        ctx.cpu.gpr[instr.d()] = instr.uimm() << 16;
    } else {
        ctx.cpu.gpr[instr.d()] = ctx.cpu.gpr[instr.a()].wrapping_add(instr.uimm() << 16);
    }
}

fn op_addmex(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_addmex");
}

fn op_addex(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()];
    let rb = ctx.cpu.gpr[instr.b()];

    let (rd, ca1) = ra.overflowing_add(rb);
    let (rd, ca2) = rd.overflowing_add(ctx.cpu.xer.carry() as u32);

    ctx.cpu.gpr[instr.d()] = rd;

    ctx.cpu.xer.set_carry(ca1 | ca2);

    if instr.oe() {
        ctx.cpu.set_xer_so(check_overflowed(ra, rb, rd));
    }

    if instr.rc() {
        ctx.cpu.update_cr0(rd);
    }
}

fn op_addzex(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()];

    let (rd, ca) = ra.overflowing_add(ctx.cpu.xer.carry() as u32);

    ctx.cpu.gpr[instr.d()] = rd;

    ctx.cpu.xer.set_carry(ca);

    if instr.oe() {
        ctx.cpu.set_xer_so(check_overflowed(ra, 0, rd));
    }

    if instr.rc() {
        ctx.cpu.update_cr0(rd);
    }
}

fn op_andcx(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.s()] & (!ctx.cpu.gpr[instr.b()]);

    ctx.cpu.gpr[instr.a()] = ra;

    if instr.rc() {
        ctx.cpu.update_cr0(ra);
    }
}

fn op_andi_rc(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.s()] & instr.uimm();

    ctx.cpu.gpr[instr.a()] = ra;

    ctx.cpu.update_cr0(ra);
}

fn op_andis_rc(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_andis_rc");
}

fn op_andx(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.s()] & ctx.cpu.gpr[instr.b()];

    ctx.cpu.gpr[instr.a()] = ra;

    if instr.rc() {
        ctx.cpu.update_cr0(ra);
    }
}

fn op_cmp(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()] as i32;
    let rb = ctx.cpu.gpr[instr.b()] as i32;

    let mut c = match ra.cmp(&rb) {
        Ordering::Less => 0x8,
        Ordering::Greater => 0x4,
        Ordering::Equal => 0x2,
    };

    c |= ctx.cpu.xer.summary_overflow() as u32;

    ctx.cpu.cr.set_field(instr.crfd(), c);
}

fn op_cmpi(ctx: &mut Context, instr: Instruction) {
    if instr.l() {
        panic!("cmpi: invalid instruction");
    }

    let ra = ctx.cpu.gpr[instr.a()] as i32;
    let simm = i32::from(instr.simm());

    let mut c = match ra.cmp(&simm) {
        Ordering::Less => 0x8,
        Ordering::Greater => 0x4,
        Ordering::Equal => 0x2,
    };

    c |= ctx.cpu.xer.summary_overflow() as u32;

    ctx.cpu.cr.set_field(instr.crfd(), c);
}

fn op_cmpl(ctx: &mut Context, instr: Instruction) {
    if instr.l() {
        panic!("cmpl: invalid instruction");
    }

    let ra = ctx.cpu.gpr[instr.a()];
    let rb = ctx.cpu.gpr[instr.b()];

    let mut c = match ra.cmp(&rb) {
        Ordering::Less => 0x8,
        Ordering::Greater => 0x4,
        Ordering::Equal => 0x2,
    };

    c |= ctx.cpu.xer.summary_overflow() as u32;

    ctx.cpu.cr.set_field(instr.crfd(), c);
}

fn op_cmpli(ctx: &mut Context, instr: Instruction) {
    if instr.l() {
        panic!("cmpli: invalid instruction");
    }

    let ra = ctx.cpu.gpr[instr.a()];
    let uimm = instr.uimm();

    let mut c = match ra.cmp(&uimm) {
        Ordering::Less => 0x8,
        Ordering::Greater => 0x4,
        Ordering::Equal => 0x2,
    };

    c |= ctx.cpu.xer.summary_overflow() as u32;

    ctx.cpu.cr.set_field(instr.crfd(), c);
}

fn op_cntlzwx(ctx: &mut Context, instr: Instruction) {
    let mut n = 0;
    let mut mask = 0x8000_0000;
    let s = ctx.cpu.gpr[instr.s()];

    while n < 32 {
        if (s & mask) != 0 {
            break;
        }

        n += 1;
        mask >>= 1;
    }

    ctx.cpu.gpr[instr.a()] = n;

    if instr.rc() {
        ctx.cpu.update_cr0(n);
    }
}

fn op_divwux(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()];
    let rb = ctx.cpu.gpr[instr.b()];
    let overflow = rb == 0;

    let rd = if overflow { 0 } else { ra / rb };

    ctx.cpu.gpr[instr.d()] = rd;

    if instr.oe() {
        ctx.cpu.set_xer_so(overflow);
    }

    if instr.rc() {
        ctx.cpu.update_cr0(rd);
    }
}

fn op_divwx(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()] as i32;
    let rb = ctx.cpu.gpr[instr.b()] as i32;
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

    ctx.cpu.gpr[instr.d()] = rd;

    if instr.oe() {
        ctx.cpu.set_xer_so(overflow);
    }

    if instr.rc() {
        ctx.cpu.update_cr0(rd);
    }
}

fn op_eqvx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_eqvx");
}

fn op_extsbx(ctx: &mut Context, instr: Instruction) {
    let ra = ((ctx.cpu.gpr[instr.s()] as i8) as i32) as u32;

    ctx.cpu.gpr[instr.a()] = ra;

    if instr.rc() {
        ctx.cpu.update_cr0(ra);
    }
}

fn op_extshx(ctx: &mut Context, instr: Instruction) {
    let ra = ((ctx.cpu.gpr[instr.s()] as i16) as i32) as u32;

    ctx.cpu.gpr[instr.a()] = ra;

    if instr.rc() {
        ctx.cpu.update_cr0(ra);
    }
}

fn op_mulhwux(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()] as u64;
    let rb = ctx.cpu.gpr[instr.b()] as u64;

    let rd = ((ra * rb) >> 32) as u32;

    ctx.cpu.gpr[instr.d()] = rd;

    if instr.rc() {
        ctx.cpu.update_cr0(rd);
    }
}

fn op_mulhwx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_mulhwx");
}

fn op_mulli(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.d()] =
        (ctx.cpu.gpr[instr.a()] as i32).wrapping_mul(i32::from(instr.simm())) as u32;
}

fn op_mullwx(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()] as i32;
    let rb = ctx.cpu.gpr[instr.b()] as i32;
    let (rd, overflow) = ra.overflowing_mul(rb);
    let rd = rd as u32;

    ctx.cpu.gpr[instr.d()] = rd;

    if instr.oe() {
        ctx.cpu.set_xer_so(overflow)
    }

    if instr.rc() {
        ctx.cpu.update_cr0(rd);
    }
}

fn op_nandx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_nandx");
}

fn op_negx(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()];
    let rd = (!ra) + 1;

    ctx.cpu.gpr[instr.d()] = rd;

    if instr.oe() {
        ctx.cpu.set_xer_so(ra == 0x8000_0000);
    }

    if instr.rc() {
        ctx.cpu.update_cr0(rd);
    }
}

fn op_norx(ctx: &mut Context, instr: Instruction) {
    let ra = !(ctx.cpu.gpr[instr.s()] | ctx.cpu.gpr[instr.b()]);

    ctx.cpu.gpr[instr.a()] = ra;

    if instr.rc() {
        ctx.cpu.update_cr0(ra);
    }
}

fn op_orcx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_orcx");
}

fn op_ori(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.a()] = ctx.cpu.gpr[instr.s()] | instr.uimm();
}

fn op_oris(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.a()] = ctx.cpu.gpr[instr.s()] | (instr.uimm() << 16);
}

fn op_orx(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.s()] | ctx.cpu.gpr[instr.b()];

    ctx.cpu.gpr[instr.a()] = ra;

    if instr.rc() {
        ctx.cpu.update_cr0(ra);
    }
}

fn op_rlwimix(ctx: &mut Context, instr: Instruction) {
    let m = mask(instr.mb(), instr.me());
    let r = ctx.cpu.gpr[instr.s()].rotate_left(instr.sh());

    let ra = (r & m) | (ctx.cpu.gpr[instr.a()] & !m);

    ctx.cpu.gpr[instr.a()] = ra;

    if instr.rc() {
        ctx.cpu.update_cr0(ra);
    }
}

fn op_rlwinmx(ctx: &mut Context, instr: Instruction) {
    let m = mask(instr.mb(), instr.me());
    let r = ctx.cpu.gpr[instr.s()].rotate_left(instr.sh());

    let ra = r & m;

    ctx.cpu.gpr[instr.a()] = ra;

    if instr.rc() {
        ctx.cpu.update_cr0(ra);
    }
}

fn op_rlwnmx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_rlwnmx");
}

fn op_slwx(ctx: &mut Context, instr: Instruction) {
    let rb = ctx.cpu.gpr[instr.b()];

    let ra = if rb & 0x20 != 0 {
        0
    } else {
        ctx.cpu.gpr[instr.s()] << (rb & 0x1F)
    };

    ctx.cpu.gpr[instr.a()] = ra;

    if instr.rc() {
        ctx.cpu.update_cr0(ra);
    }
}

fn op_srawix(ctx: &mut Context, instr: Instruction) {
    let rs = ctx.cpu.gpr[instr.s()] as i32;
    let s = instr.s();

    ctx.cpu.gpr[instr.a()] = (rs >> instr.sh()) as u32;
    ctx.cpu
        .xer
        .set_carry(rs < 0 && s > 0 && ((rs as u32) << (32 - s)) != 0);
}

// ToDo: review this implementation
fn op_srawx(ctx: &mut Context, instr: Instruction) {
    let rb = ctx.cpu.gpr[instr.b()];

    if rb & 0x20 != 0 {
        if ctx.cpu.gpr[instr.s()] & 0x8000_0000 != 0 {
            ctx.cpu.gpr[instr.a()] = 0xFFFF_FFFF;
            ctx.cpu.xer.set_carry(true);
        } else {
            ctx.cpu.gpr[instr.a()] = 0;
            ctx.cpu.xer.set_carry(false);
        }
    } else {
        let n = rb & 0x1F;

        if n != 0 {
            let rs = ctx.cpu.gpr[instr.s()] as i32;

            ctx.cpu.gpr[instr.a()] = (rs >> n) as u32;

            if rs < 0 && (rs << (32 - n) != 0) {
                ctx.cpu.xer.set_carry(true);
            } else {
                ctx.cpu.xer.set_carry(false);
            }
        } else {
            ctx.cpu.gpr[instr.a()] = ctx.cpu.gpr[instr.s()];
            ctx.cpu.xer.set_carry(false);
        }
    }

    if instr.rc() {
        ctx.cpu.update_cr0(ctx.cpu.gpr[instr.a()]);
    }
}

fn op_srwx(ctx: &mut Context, instr: Instruction) {
    let rb = ctx.cpu.gpr[instr.b()];

    let ra = if rb & 0x20 != 0 {
        0
    } else {
        ctx.cpu.gpr[instr.s()] >> (rb & 0x1F)
    };

    ctx.cpu.gpr[instr.a()] = ra;

    if instr.rc() {
        ctx.cpu.update_cr0(ra);
    }
}

fn op_subfcx(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()];
    let rb = ctx.cpu.gpr[instr.b()];
    let ca = rb < ra;

    let rd = (!ra).wrapping_add(1).wrapping_add(rb);

    ctx.cpu.gpr[instr.d()] = rd;

    ctx.cpu.xer.set_carry(ca);

    if instr.oe() {
        ctx.cpu.set_xer_so(check_overflowed(ra, rb, rd));
    }

    if instr.rc() {
        ctx.cpu.update_cr0(ctx.cpu.gpr[instr.d()]);
    }
}

fn op_subfex(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()];
    let rb = ctx.cpu.gpr[instr.b()];

    // Wrong ???
    let (rd, ca1) = (!ra).overflowing_add(rb);
    let (rd, ca2) = rd.overflowing_add(ctx.cpu.xer.carry() as u32);

    ctx.cpu.gpr[instr.d()] = rd;

    ctx.cpu.xer.set_carry(ca1 | ca2);

    if instr.oe() {
        ctx.cpu.set_xer_so(check_overflowed(ra, rb, rd));
    }

    if instr.rc() {
        ctx.cpu.update_cr0(rd);
    }
}

fn op_subfic(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()];
    let simm = (instr.simm() as i32) as u32;

    let (rd, ca) = simm.overflowing_sub(ra);

    ctx.cpu.gpr[instr.d()] = rd;

    ctx.cpu.xer.set_carry(ca);
}

fn op_subfmex(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_subfmex");
}

fn op_subfzex(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()];

    let (rd, ca) = (!ra).overflowing_add(ctx.cpu.xer.carry() as u32);

    ctx.cpu.gpr[instr.d()] = rd;

    ctx.cpu.xer.set_carry(ca);

    if instr.rc() {
        ctx.cpu.update_cr0(rd);
    }

    if instr.oe() {
        panic!("OE: subfex");
    }
}

fn op_subfx(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()] as i32;
    let rb = ctx.cpu.gpr[instr.b()] as i32;

    let (rd, ca) = rb.overflowing_sub(ra);
    let rd = rd as u32;

    ctx.cpu.gpr[instr.d()] = rd;

    if instr.oe() {
        ctx.cpu.set_xer_so(ca);
    }

    if instr.rc() {
        ctx.cpu.update_cr0(rd);
    }
}

fn op_tw(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_tw");
}

fn op_twi(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_twi");
}

fn op_xori(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_xori");
}

fn op_xoris(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.a()] = ctx.cpu.gpr[instr.s()] ^ (instr.uimm() << 16)
}

fn op_xorx(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.a()] = ctx.cpu.gpr[instr.s()] ^ ctx.cpu.gpr[instr.b()];

    if instr.rc() {
        ctx.cpu.update_cr0(ctx.cpu.gpr[instr.a()]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op_addi() {
        let mut ctx = Context::default();

        let (d, a) = (4, 5);
        let instr = Instruction(((d as u32) << 21) | ((a as u32) << 16) | 0x8FF0);

        ctx.cpu.gpr[a] = 0x0000_0900;
        op_addi(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0xFFFF_98F0);
    }

    #[test]
    fn test_op_addic() {
        let mut ctx = Context::default();

        let (d, a) = (6, 4);
        let instr = Instruction(((d as u32) << 21) | ((a as u32) << 16) | 0xFFFF);

        ctx.cpu.gpr[a] = 0x0000_2346;

        op_addic(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x0000_2345)
    }

    #[test]
    fn test_op_addic_rc() {
        let a: usize = 3;
        let d: usize = 31;

        let mut ctx = Context::default();
        let instr = Instruction(((d as u32) << 21) | ((a as u32) << 16) | 0x1);

        ctx.cpu.gpr[d] = 0xDEAD_BEEF;
        ctx.cpu.gpr[a] = 0xFFFF_FFFF;

        op_addic_rc(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x0);
        assert_eq!(ctx.cpu.gpr[a], 0xFFFF_FFFF); // confirm gpr source remains unmodified
        assert_eq!(ctx.cpu.xer.carry(), true);

        ctx.cpu.gpr[a] = 0xFFFF_FFFE;

        op_addic_rc(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0xFFFF_FFFF);
        assert_eq!(ctx.cpu.xer.carry(), false);
    }

    #[test]
    fn test_op_addis() {
        let mut ctx = Context::default();

        let (d, a) = (7, 6);
        let instr = Instruction(((d as u32) << 21) | ((a as u32) << 16) | 0x0011);

        ctx.cpu.gpr[a] = 0x0000_4000;
        op_addis(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x0011_4000);
    }

    #[test]
    fn test_op_addex() {
        let mut ctx = Context::default();

        let (d, a, b) = (6, 4, 10);
        let instr = Instruction(((d as u32) << 21) | ((a as u32) << 16) | ((b as u32) << 11));

        ctx.cpu.gpr[a] = 0x1000_0400;
        ctx.cpu.gpr[b] = 0x1000_0400;
        ctx.cpu.xer.set_carry(true);
        op_addex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x2000_0801);

        ctx.cpu.gpr[a] = 0x9000_3000;
        ctx.cpu.gpr[b] = 0x7B41_92C0;
        ctx.cpu.xer.set_carry(false);
        op_addex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x0B41_C2C0);

        ctx.cpu.gpr[a] = 0x1000_0400;
        ctx.cpu.gpr[b] = 0xEFFF_FFFF;
        ctx.cpu.xer.set_carry(true);
        op_addex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x0000_0400);

        ctx.cpu.gpr[a] = 0x9000_3000;
        ctx.cpu.gpr[b] = 0x8000_7000;
        ctx.cpu.xer.set_carry(false);
        op_addex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x1000_A000);
    }

    #[test]
    fn test_op_addzex() {
        let mut ctx = Context::default();

        let (d, a) = (6, 4);
        let instr = Instruction(((d as u32) << 21) | ((a as u32) << 16));

        ctx.cpu.gpr[a] = 0x7B41_92C0;
        ctx.cpu.xer.set_carry(false);
        op_addzex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x7B41_92C0);

        ctx.cpu.gpr[a] = 0xEFFF_FFFF;
        ctx.cpu.xer.set_carry(true);
        op_addzex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0xF000_0000);

        ctx.cpu.gpr[a] = 0x9000_3000;
        ctx.cpu.xer.set_carry(true);
        op_addzex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x9000_3001);

        ctx.cpu.gpr[a] = 0xEFFF_FFFF;
        ctx.cpu.xer.set_carry(false);
        op_addzex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0xEFFF_FFFF);
    }

    #[test]
    fn test_op_addx() {
        let mut ctx = Context::default();

        let (d, a, b) = (4, 6, 3);
        let instr = Instruction(((d as u32) << 21) | ((a as u32) << 16) | ((b as u32) << 11));

        ctx.cpu.gpr[a] = 0x0004_0000;
        ctx.cpu.gpr[b] = 0x0000_4000;
        op_addx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x0004_4000);
        assert_eq!(ctx.cpu.xer.carry(), false);

        let instr = Instruction(instr.0 | 1); // Enable rc

        ctx.cpu.gpr[a] = 0x8000_7000;
        ctx.cpu.gpr[b] = 0x7000_8000;
        op_addx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0xF000_F000);
        assert_eq!(ctx.cpu.xer.carry(), false);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT

        ctx.cpu.gpr[a] = 0xEFFF_FFFF;
        ctx.cpu.gpr[b] = 0x8000_0000;
        op_addx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x6FFF_FFFF);
        // FixMe check Summary Overflow, Overflow and carry bits are set in Fixed point
        // register

        ctx.cpu.gpr[a] = 0xEFFF_FFFF;
        ctx.cpu.gpr[b] = 0xEFFF_FFFF;
        op_addx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0xDFFF_FFFE);
        // FixMe: check check Summary Overflow, Overflow and carry bits are set in Fixed point
        // register, as well as condition register field 0 updated
    }

    #[test]
    fn test_op_addcx() {
        let mut ctx = Context::default();

        let (d, a, b) = (6, 4, 10);
        let instr = Instruction(((d as u32) << 21) | ((a as u32) << 16) | ((b as u32) << 11));

        ctx.cpu.gpr[a] = 0x9000_3000;
        ctx.cpu.gpr[b] = 0x8000_7000;
        op_addcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x1000_A000);

        let instr = Instruction(instr.0 | 1); // Enable rc

        ctx.cpu.gpr[a] = 0x7000_3000;
        ctx.cpu.gpr[b] = 0xFFFF_FFFF;
        op_addcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x7000_2FFF);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT

        let instr = Instruction(instr.0 | (1 << 10)); // Enable oe

        ctx.cpu.gpr[a] = 0x9000_3000;
        ctx.cpu.gpr[b] = 0x7B41_92C0;
        op_addcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x0B41_C2C0);
        assert_eq!(ctx.cpu.xer.carry(), true);
        // FixMe: check Summary Overflow and Overflow bits are set

        ctx.cpu.gpr[a] = 0x8000_0000;
        ctx.cpu.gpr[b] = 0x8000_7000;
        ctx.cpu.xer.set_carry(false);
        op_addcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x0000_7000);
        assert_eq!(ctx.cpu.xer.carry(), true);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x5); // GT, SO

        // FixMe: check Summery Overflow and Overflow bits set
    }

    #[test]
    fn test_op_andx() {
        let mut ctx = Context::default();

        let (a, s, b) = (6, 4, 7);
        let instr = Instruction(((s as u32) << 21) | ((a as u32) << 16) | ((b as u32) << 11));

        ctx.cpu.gpr[s] = 0xFFF2_5730;
        ctx.cpu.gpr[b] = 0x7B41_92C0;
        op_andx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0x7B40_1200);

        let instr = Instruction(instr.0 | 1); // Enable rc

        ctx.cpu.gpr[s] = 0xFFF2_5730;
        ctx.cpu.gpr[b] = 0xFFFF_EFFF;
        op_andx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0xFFF2_4730);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT
    }

    #[test]
    fn test_op_andcx() {
        let mut ctx = Context::default();

        let (a, s, b) = (6, 4, 5);
        let instr = Instruction(((s as u32) << 21) | ((a as u32) << 16) | ((b as u32) << 11));

        ctx.cpu.gpr[s] = 0x9000_3000;
        ctx.cpu.gpr[b] = 0xFFFF_FFFF;
        op_andcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0x0000_0000);

        let instr = Instruction(instr.0 | 1); // Enable rc

        ctx.cpu.gpr[s] = 0xB004_3000;
        ctx.cpu.gpr[b] = 0x7676_7676;
        op_andcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0x8000_0000);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT
    }

    #[test]
    fn test_op_andi_rc() {
        let mut ctx = Context::default();

        let (a, s) = (6, 4);
        let uimm = 0x5730;
        let instr = Instruction(((s as u32) << 21) | ((a as u32) << 16) | uimm);

        ctx.cpu.gpr[s] = 0x7B41_92C0;
        op_andi_rc(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0x0000_1200);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT
    }

    #[test]
    fn test_op_cmp() {
        let mut ctx = Context::default();

        let (a, b) = (4, 6);
        let instr = Instruction(((a as u32) << 16) | ((b as u32) << 11));

        ctx.cpu.gpr[a] = 0xFFFF_FFE7;
        ctx.cpu.gpr[b] = 0x0000_0011;
        op_cmp(&mut ctx, instr);

        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT
    }

    #[test]
    fn test_op_cmpi() {
        let mut ctx = Context::default();

        let a = 4;
        let simm = 0x11;
        let instr = Instruction(((a as u32) << 16) | simm);

        ctx.cpu.gpr[a] = 0xFFFF_FFE7;
        op_cmpi(&mut ctx, instr);

        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT
    }

    #[test]
    fn test_op_cmpl() {
        let mut ctx = Context::default();

        let (a, b) = (4, 5);
        let instr = Instruction(((a as u32) << 16) | ((b as u32) << 11));

        ctx.cpu.gpr[a] = 0xFFFF_0000;
        ctx.cpu.gpr[b] = 0x7FFF_0000;
        op_cmpl(&mut ctx, instr);

        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT
    }

    #[test]
    fn test_op_cmpli() {
        let mut ctx = Context::default();

        let a = 4;
        let uimm = 0xFF;
        let instr = Instruction(((a as u32) << 16) | uimm);

        ctx.cpu.gpr[a] = 0x0000_00FF;
        op_cmpli(&mut ctx, instr);

        assert_eq!(ctx.cpu.cr.get_cr0(), 0x2); // EQ
    }

    #[test]
    fn test_op_cntlzwx() {
        let mut ctx = Context::default();

        let (a, s) = (3, 3);
        let instr = Instruction(((s as u32) << 21) | ((a as u32) << 16));

        ctx.cpu.gpr[a] = 0x0061_9920;
        op_cntlzwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[s], 0x0000_0009);
    }

    #[test]
    fn test_op_divwx() {
        let mut ctx = Context::default();

        let (d, a, b) = (4, 4, 6);
        let instr = Instruction(((d as u32) << 21) | ((a as u32) << 16) | ((b as u32) << 11));

        ctx.cpu.gpr[a] = 0x0000_0000;
        ctx.cpu.gpr[b] = 0x0000_0002;
        op_divwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x0000_0000);

        let instr = Instruction(instr.0 | 1); // Enable rc

        ctx.cpu.gpr[a] = 0x0000_0002;
        ctx.cpu.gpr[b] = 0x0000_0002;
        op_divwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x0000_0001);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT

        ctx.cpu.gpr[a] = 0x0000_0001;
        ctx.cpu.gpr[b] = 0x0000_0002;
        op_divwx(&mut ctx, instr);

        // Undefined
        assert_eq!(ctx.cpu.gpr[d], 0x0000_0000);

        ctx.cpu.gpr[a] = 0x8000_0000;
        ctx.cpu.gpr[b] = 0xFFFF_FFFF;
        op_divwx(&mut ctx, instr);

        // Undefined
        assert_eq!(ctx.cpu.gpr[d], 0x0000_0000);
    }

    #[test]
    fn test_op_divwux() {}

    #[test]
    fn test_op_extsbx() {
        let mut ctx = Context::default();

        let (a, s) = (4, 6);
        let instr = Instruction(((s as u32) << 21) | ((a as u32) << 16));

        ctx.cpu.gpr[s] = 0x5A5A_5A5A;
        op_extsbx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0x0000_005A);

        ctx.cpu.gpr[s] = 0xA5A5_A5A5;
        op_extsbx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0xFFFF_FFA5);
    }

    #[test]
    fn test_op_extshx() {
        let mut ctx = Context::default();

        let (a, s) = (4, 6);
        let instr = Instruction(((s as u32) << 21) | ((a as u32) << 16));

        ctx.cpu.gpr[s] = 0x0000_FFFF;
        op_extshx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0xFFFF_FFFF);

        ctx.cpu.gpr[s] = 0x0000_2FFF;
        op_extshx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0x0000_2FFF);
    }

    #[test]
    fn test_op_mulhwux() {
        let mut ctx = Context::default();

        let (d, a, b) = (6, 4, 10);
        let instr = Instruction(((d as u32) << 21) | ((a as u32) << 16) | ((b as u32) << 11));

        ctx.cpu.gpr[a] = 0x0000_0003;
        ctx.cpu.gpr[b] = 0x0000_0002;
        op_mulhwux(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x0000_0000);

        let instr = Instruction(instr.0 | 1); // Enable rc

        ctx.cpu.gpr[a] = 0x0000_4500;
        ctx.cpu.gpr[b] = 0x8000_7000;
        op_mulhwux(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x0000_2280);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT
    }

    #[test]
    fn test_op_mulli() {
        let mut ctx = Context::default();

        let (d, a, simm) = (6, 4, 10);
        let instr = Instruction(((d as u32) << 21) | ((a as u32) << 16) | simm);

        ctx.cpu.gpr[a] = 0x0000_3000;
        op_mulli(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x0001_E000);
    }

    #[test]
    fn test_op_mullwx() {
        let mut ctx = Context::default();

        let (d, a, b) = (6, 4, 10);
        let instr = Instruction(((d as u32) << 21) | ((a as u32) << 16) | ((b as u32) << 11));

        ctx.cpu.gpr[a] = 0x0000_3000;
        ctx.cpu.gpr[b] = 0x0000_7000;
        op_mullwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x1500_0000);

        let instr = Instruction(instr.0 | 1); // Enable rc

        ctx.cpu.gpr[a] = 0x0000_4500;
        ctx.cpu.gpr[b] = 0x0000_7000;
        op_mullwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x1E30_0000);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT

        let instr = Instruction(instr.0 | (1 << 10)); // Enable oe

        ctx.cpu.gpr[a] = 0x0000_4500;
        ctx.cpu.gpr[b] = 0x0007_0000;
        op_mullwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0xE300_0000);
        // FixMe: check summary overflow and overflow

        ctx.cpu.gpr[a] = 0x0000_4500;
        ctx.cpu.gpr[b] = 0x7FFF_FFFF;
        op_mullwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0xFFFF_BB00);
        // FixMe: check summary overflow and overflow
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x9); // LT SO
    }

    #[test]
    fn test_op_negx() {
        let mut ctx = Context::default();

        let (d, a) = (6, 4);
        let instr = Instruction(((d as u32) << 21) | ((a as u32) << 16));

        ctx.cpu.gpr[a] = 0x9000_3000;
        op_negx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x6FFF_D000);

        let instr = Instruction(instr.0 | 1); // Enable rc

        ctx.cpu.gpr[a] = 0x789A_789B;
        op_negx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x8765_8765);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT

        ctx.cpu.gpr[a] = 0x9000_3000;
        op_negx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x6FFF_D000);
        // FixMe: check summary overflow and overflow bits

        ctx.cpu.gpr[a] = 0x8000_0000;
        op_negx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x8000_0000);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT

        // FixMe: check summary overflow and overflow bits
    }

    #[test]
    fn test_op_norx() {
        let mut ctx = Context::default();

        let (a, s, b) = (6, 4, 7);
        let instr = Instruction(((s as u32) << 21) | ((a as u32) << 16) | ((b as u32) << 11));

        ctx.cpu.gpr[s] = 0x9000_3000;
        ctx.cpu.gpr[b] = 0x789A_789B;
        op_norx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0x0765_8764);

        let instr = Instruction(instr.0 | 1); // Enable rc

        ctx.cpu.gpr[s] = 0xB004_3000;
        ctx.cpu.gpr[b] = 0x789A_789B;
        op_norx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0x0761_8764);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT
    }

    #[test]
    fn test_op_orx() {
        let mut ctx = Context::default();

        let (a, s, b) = (6, 4, 7);
        let instr = Instruction(((s as u32) << 21) | ((a as u32) << 16) | ((b as u32) << 11));

        ctx.cpu.gpr[s] = 0x9000_3000;
        ctx.cpu.gpr[b] = 0x789A_789B;
        op_orx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0xF89A_789B);

        let instr = Instruction(instr.0 | 1); // Enable rc

        ctx.cpu.gpr[s] = 0xB004_3000;
        ctx.cpu.gpr[b] = 0x789A_789B;
        op_orx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0xF89E_789B);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT
    }

    #[test]
    fn test_op_ori() {
        let mut ctx = Context::default();

        let (s, a, uimm) = (6, 4, 0x0079);
        let instr = Instruction(((s as u32) << 21) | ((a as u32) << 16) | uimm);

        ctx.cpu.gpr[s] = 0x9000_3000;
        op_ori(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0x9000_3079);
    }

    #[test]
    fn test_op_oris() {
        let mut ctx = Context::default();

        let (s, a, uimm) = (6, 4, 0x0079);
        let instr = Instruction(((s as u32) << 21) | ((a as u32) << 16) | uimm);

        ctx.cpu.gpr[s] = 0x9000_3000;
        op_oris(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0x9079_3000);
    }

    #[test]
    fn test_op_rlwimix() {
        let mut ctx = Context::default();

        let (a, s, sh, mb, me) = (6, 4, 2, 0, 0x1D);
        let instr =
            Instruction(((s as u32) << 21) | ((a as u32) << 16) | sh << 11 | mb << 6 | me << 1);

        ctx.cpu.gpr[s] = 0x9000_3000;
        ctx.cpu.gpr[a] = 0x0000_0003;
        op_rlwimix(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0x4000_C003);

        let (mb, me) = (0, 0x1A);
        let instr =
            Instruction(((s as u32) << 21) | ((a as u32) << 16) | sh << 11 | mb << 6 | me << 1 | 1); // enable rc

        ctx.cpu.gpr[s] = 0x789A_789B;
        ctx.cpu.gpr[a] = 0x3000_0003;
        op_rlwimix(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0xE269_E263);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT
    }

    #[test]
    fn test_op_rlwinmx() {
        let mut ctx = Context::default();

        let (a, s, sh, mb, me) = (6, 4, 2, 0, 0x1D);
        let instr =
            Instruction(((s as u32) << 21) | ((a as u32) << 16) | sh << 11 | mb << 6 | me << 1);

        ctx.cpu.gpr[s] = 0x9000_3000;
        ctx.cpu.gpr[a] = 0xFFFF_FFFF;
        op_rlwinmx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0x4000_C000);

        let instr = Instruction(instr.0 | 1); // Enable rc

        ctx.cpu.gpr[s] = 0xB004_3000;
        ctx.cpu.gpr[a] = 0xFFFF_FFFF;
        op_rlwinmx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0xC010_C000);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT
    }

    #[test]
    fn test_op_slwx() {
        let mut ctx = Context::default();

        let (a, s, b) = (6, 4, 5);
        let instr = Instruction(((s as u32) << 21) | ((a as u32) << 16) | ((b as u32) << 11));

        ctx.cpu.gpr[b] = 0x0000_002F;
        ctx.cpu.gpr[s] = 0xFFFF_FFFF;
        op_slwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0x0000_0000);

        let instr = Instruction(instr.0 | 1); // Enable rc

        ctx.cpu.gpr[b] = 0x0000_0005;
        ctx.cpu.gpr[s] = 0xB004_3000;
        op_slwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0x0086_0000);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT
    }

    #[test]
    fn test_op_srawx() {
        let mut ctx = Context::default();

        let (a, s, b) = (6, 4, 5);
        let instr = Instruction(((s as u32) << 21) | ((a as u32) << 16) | ((b as u32) << 11));

        ctx.cpu.gpr[s] = 0x9000_3000;
        ctx.cpu.gpr[b] = 0x0000_0024;
        op_srawx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0xFFFF_FFFF);
        assert_eq!(ctx.cpu.xer.carry(), true);

        let instr = Instruction(instr.0 | 1); // Enable rc

        ctx.cpu.gpr[s] = 0xB004_3000;
        ctx.cpu.gpr[b] = 0x0000_0004;
        op_srawx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0xFB00_4300);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT

        //assert_eq!(ctx.cpu.xer.carry(), true);
    }

    #[test]
    fn test_op_srawix() {
        let mut ctx = Context::default();

        let (a, s, sh) = (6, 4, 0x4);
        let instr = Instruction(((s as u32) << 21) | ((a as u32) << 16) | ((sh as u32) << 11));

        ctx.cpu.gpr[s] = 0x9000_3000;
        op_srawix(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0xF900_0300);
        //assert_eq!(ctx.cpu.xer.carry(), true);

        ctx.cpu.gpr[s] = 0xB004_3000;
        op_srawix(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0xFB00_4300);
        //assert_eq!(ctx.cpu.xer.carry(), true);
        // FixMe: check carry properly
    }

    #[test]
    fn test_op_srwx() {
        let mut ctx = Context::default();

        let (a, s, b) = (6, 4, 5);
        let instr = Instruction(((s as u32) << 21) | ((a as u32) << 16) | ((b as u32) << 11));

        ctx.cpu.gpr[s] = 0x9000_3000;
        ctx.cpu.gpr[b] = 0x0000_0024;
        op_srwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0x0000_0000);

        let instr = Instruction(instr.0 | 1); // Enable rc

        ctx.cpu.gpr[s] = 0xB004_3001;
        ctx.cpu.gpr[b] = 0x0000_0004;
        op_srwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0x0B00_4300);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT
    }

    #[test]
    fn test_op_subfx() {
        let mut ctx = Context::default();

        let (d, a, b) = (6, 4, 10);
        let instr = Instruction(((d as u32) << 21) | ((a as u32) << 16) | ((b as u32) << 11));

        ctx.cpu.gpr[a] = 0x8000_7000;
        ctx.cpu.gpr[b] = 0x9000_3000;
        op_subfx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x0FFF_C000);

        let instr = Instruction(instr.0 | 1); // Enable rc

        ctx.cpu.gpr[a] = 0x0000_4500;
        ctx.cpu.gpr[b] = 0x8000_7000;
        op_subfx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x8000_2B00);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT

        let instr = Instruction(instr.0 | (1 << 10)); // Enable oe

        ctx.cpu.gpr[a] = 0x8000_0000;
        ctx.cpu.gpr[b] = 0x0000_4500;
        op_subfx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x8000_4500);
        // FixMe: check SO and O

        ctx.cpu.gpr[a] = 0x8000_0000;
        ctx.cpu.gpr[b] = 0x0000_7000;
        op_subfx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x8000_7000);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x9); // LT

        // FixMe: check SO and O
    }

    #[test]
    fn test_op_subfcx() {
        let mut ctx = Context::default();

        let (d, a, b) = (6, 4, 10);
        let instr = Instruction(((d as u32) << 21) | ((a as u32) << 16) | ((b as u32) << 11));

        ctx.cpu.gpr[a] = 0x8000_7000;
        ctx.cpu.gpr[b] = 0x9000_3000;
        op_subfcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x0FFF_C000);
        assert_eq!(ctx.cpu.xer.carry(), false);

        let instr = Instruction(instr.0 | 1); // Enable rc

        ctx.cpu.gpr[a] = 0x0000_4500;
        ctx.cpu.gpr[b] = 0x8000_7000;
        op_subfcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x8000_2B00);
        assert_eq!(ctx.cpu.xer.carry(), false);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT

        let instr = Instruction(instr.0 | (1 << 10)); // Enable oe

        ctx.cpu.gpr[a] = 0x8000_0000;
        ctx.cpu.gpr[b] = 0x0000_4500;
        op_subfcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x8000_4500);
        assert_eq!(ctx.cpu.xer.carry(), true);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT

        ctx.cpu.gpr[a] = 0x8000_0000;
        ctx.cpu.gpr[b] = 0x0000_7000;
        op_subfcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x8000_7000);
        assert_eq!(ctx.cpu.xer.carry(), true);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT
    }

    #[test]
    fn test_op_subfex() {
        let mut ctx = Context::default();

        let (d, a, b) = (6, 4, 10);
        let instr = Instruction(((d as u32) << 21) | ((a as u32) << 16) | ((b as u32) << 11));

        ctx.cpu.gpr[a] = 0x9000_3000;
        ctx.cpu.gpr[b] = 0x8000_7000;
        ctx.cpu.xer.set_carry(true);
        op_subfex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0xF000_4000);
        assert_eq!(ctx.cpu.xer.carry(), false);

        let instr = Instruction(instr.0 | 1); // Enable rc

        ctx.cpu.gpr[a] = 0x0000_4500;
        ctx.cpu.gpr[b] = 0x8000_7000;
        ctx.cpu.xer.set_carry(false);
        op_subfex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x8000_2AFF);
        assert_eq!(ctx.cpu.xer.carry(), true);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT

        let instr = Instruction(instr.0 | (1 << 10)); // Enable oe

        ctx.cpu.gpr[a] = 0x8000_0000;
        ctx.cpu.gpr[b] = 0xEFFF_FFFF;
        ctx.cpu.xer.set_carry(true);
        op_subfex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x6FFF_FFFF);
        assert_eq!(ctx.cpu.xer.carry(), true);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x5); // GT, SO

        ctx.cpu.gpr[a] = 0x8000_0000;
        ctx.cpu.gpr[b] = 0xEFFF_FFFF;
        ctx.cpu.xer.set_carry(false);
        op_subfex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x6FFF_FFFE);
        assert_eq!(ctx.cpu.xer.carry(), true);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x5); // GT, SO
    }

    #[test]
    fn test_op_subfic() {
        let mut ctx = Context::default();

        let (d, a, simm) = (6, 4, 0x7000);
        let instr = Instruction(((d as u32) << 21) | ((a as u32) << 16) | simm);

        ctx.cpu.gpr[a] = 0x9000_3000;
        op_subfic(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x7000_4000);
    }

    //#[test]
    //fn test_op_subfzex() {}

    //#[test]
    //fn test_op_twi() {}

    #[test]
    fn test_op_xorx() {
        let mut ctx = Context::default();

        let (a, s, b) = (6, 4, 3);
        let instr = Instruction(((s as u32) << 21) | ((a as u32) << 16) | ((b as u32) << 11));

        ctx.cpu.gpr[s] = 0x9000_3000;
        ctx.cpu.gpr[b] = 0x789A_789B;
        op_xorx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0xE89A_489B);

        let instr = Instruction(instr.0 | 1); // Enable rc

        ctx.cpu.gpr[s] = 0xB004_3000;
        ctx.cpu.gpr[b] = 0x789A_789B;
        op_xorx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0xC89E_489B);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT
    }

    #[test]
    fn test_op_xoris() {
        let mut ctx = Context::default();

        let (a, s, uimm) = (6, 4, 0x0079);
        let instr = Instruction(((s as u32) << 21) | ((a as u32) << 16) | uimm);

        ctx.cpu.gpr[s] = 0x9000_3000;
        op_xoris(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0x9079_3000);
    }
}
