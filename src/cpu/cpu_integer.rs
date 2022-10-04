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

fn op_addcx(ctx: &mut Context, instr: Instruction) {
    let a = ctx.cpu.gpr[instr.a()];
    let b = ctx.cpu.gpr[instr.b()];

    ctx.cpu.gpr[instr.d()] = a.wrapping_add(b);

    ctx.cpu.xer.set_carry(a > !b);

    if instr.rc() {
        ctx.cpu.update_cr0(ctx.cpu.gpr[instr.d()]);
    }

    if instr.oe() {
        panic!("OE: addcx");
    }
}

fn op_addex(ctx: &mut Context, instr: Instruction) {
    let a = ctx.cpu.gpr[instr.a()];
    let b = ctx.cpu.gpr[instr.b()];

    ctx.cpu.gpr[instr.d()] = a.wrapping_add(b).wrapping_add(ctx.cpu.xer.carry() as u32);

    // FixMe: update carry

    if instr.rc() {
        ctx.cpu.update_cr0(ctx.cpu.gpr[instr.d()]);
    }

    if instr.oe() {
        panic!("OE: addex");
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

    ctx.cpu.gpr[instr.d()] = ra.wrapping_add(imm);

    ctx.cpu.xer.set_carry(ra > !imm);
}

fn op_addic_rc(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()];
    let imm = i32::from(instr.simm()) as u32;

    ctx.cpu.gpr[instr.d()] = ra.wrapping_add(imm);

    ctx.cpu.xer.set_carry(ra > !imm);

    ctx.cpu.update_cr0(ctx.cpu.gpr[instr.d()]);
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

fn op_addzex(ctx: &mut Context, instr: Instruction) {
    let carry = ctx.cpu.xer.carry() as u32;
    let ra = ctx.cpu.gpr[instr.a()];

    ctx.cpu.gpr[instr.d()] = ra.wrapping_add(carry);

    ctx.cpu.xer.set_carry(ra > !carry);

    if instr.rc() {
        ctx.cpu.update_cr0(ctx.cpu.gpr[instr.d()]);
    }

    if instr.oe() {
        panic!("OE: addzex");
    }
}

fn op_addx(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.d()] = ctx.cpu.gpr[instr.a()].wrapping_add(ctx.cpu.gpr[instr.b()]);

    if instr.rc() {
        ctx.cpu.update_cr0(ctx.cpu.gpr[instr.d()]);
    }

    if instr.oe() {
        panic!("OE: addx");
    }
}

fn op_andcx(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.a()] = ctx.cpu.gpr[instr.s()] & (!ctx.cpu.gpr[instr.b()]);

    if instr.rc() {
        ctx.cpu.update_cr0(ctx.cpu.gpr[instr.a()]);
    }
}

fn op_andi_rc(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.a()] = ctx.cpu.gpr[instr.s()] & instr.uimm();

    ctx.cpu.update_cr0(ctx.cpu.gpr[instr.a()]);
}

fn op_andis_rc(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_andis_rc");
}

fn op_andx(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.a()] = ctx.cpu.gpr[instr.s()] & ctx.cpu.gpr[instr.b()];

    if instr.rc() {
        ctx.cpu.update_cr0(ctx.cpu.gpr[instr.a()]);
    }
}

fn op_cmp(ctx: &mut Context, instr: Instruction) {
    let a = ctx.cpu.gpr[instr.a()] as i32;
    let b = ctx.cpu.gpr[instr.b()] as i32;

    let mut c = match a.cmp(&b) {
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

    let a = ctx.cpu.gpr[instr.a()] as i32;
    let b = i32::from(instr.simm());

    let mut c = match a.cmp(&b) {
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

    let a = ctx.cpu.gpr[instr.a()];
    let b = ctx.cpu.gpr[instr.b()];

    let mut c = match a.cmp(&b) {
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

    let a = ctx.cpu.gpr[instr.a()];
    let b = instr.uimm();

    let mut c = match a.cmp(&b) {
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
        n += 1;
        mask >>= 1;

        if (s & mask) != 0 {
            break;
        }
    }

    ctx.cpu.gpr[instr.a()] = n;

    if instr.rc() {
        ctx.cpu.update_cr0(ctx.cpu.gpr[instr.a()]);
    }
}

fn op_divwux(ctx: &mut Context, instr: Instruction) {
    let a = ctx.cpu.gpr[instr.a()];
    let b = ctx.cpu.gpr[instr.b()];

    if b == 0 {
        if instr.oe() {
            panic!("OE: divwux");
        }

        ctx.cpu.gpr[instr.d()] = 0;
    } else {
        ctx.cpu.gpr[instr.d()] = a / b;
    }

    if instr.rc() {
        ctx.cpu.update_cr0(ctx.cpu.gpr[instr.d()]);
    }
}

fn op_divwx(ctx: &mut Context, instr: Instruction) {
    let a = ctx.cpu.gpr[instr.a()] as i32;
    let b = ctx.cpu.gpr[instr.b()] as i32;

    if b == 0 || (a as u32 == 0x8000_0000 && b == -1) {
        if instr.oe() {
            panic!("OE: divwx");
        }

        if a as u32 == 0x8000_0000 && b == 0 {
            ctx.cpu.gpr[instr.d()] = 0xFFFF_FFFF;
        } else {
            ctx.cpu.gpr[instr.d()] = 0;
        }
    } else {
        ctx.cpu.gpr[instr.d()] = (a / b) as u32;
    }

    if instr.rc() {
        ctx.cpu.update_cr0(ctx.cpu.gpr[instr.d()]);
    }
}

fn op_eqvx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_eqvx");
}

fn op_extsbx(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.a()] = i32::from(ctx.cpu.gpr[instr.s()] as i8) as u32;

    if instr.rc() {
        ctx.cpu.update_cr0(ctx.cpu.gpr[instr.a()]);
    }
}

fn op_extshx(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.a()] = i32::from(ctx.cpu.gpr[instr.s()] as i16) as u32;

    if instr.rc() {
        ctx.cpu.update_cr0(ctx.cpu.gpr[instr.a()]);
    }
}

fn op_mulhwux(ctx: &mut Context, instr: Instruction) {
    let a = u64::from(ctx.cpu.gpr[instr.a()]);
    let b = u64::from(ctx.cpu.gpr[instr.b()]);

    ctx.cpu.gpr[instr.d()] = ((a * b) >> 32) as u32;

    if instr.rc() {
        ctx.cpu.update_cr0(ctx.cpu.gpr[instr.d()]);
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
    let a = ctx.cpu.gpr[instr.a()] as i32;
    let b = ctx.cpu.gpr[instr.b()] as i32;

    ctx.cpu.gpr[instr.d()] = a.wrapping_mul(b) as u32;

    if instr.oe() {
        panic!("OE: mullwx");
    }

    if instr.rc() {
        ctx.cpu.update_cr0(ctx.cpu.gpr[instr.d()]);
    }
}

fn op_nandx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_nandx");
}

fn op_negx(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.d()] = !(ctx.cpu.gpr[instr.a()]) + 1;

    // FixMe: ???

    if instr.rc() {
        ctx.cpu.update_cr0(ctx.cpu.gpr[instr.d()]);
    }
}

fn op_norx(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.a()] = !(ctx.cpu.gpr[instr.s()] | ctx.cpu.gpr[instr.b()]);

    if instr.rc() {
        ctx.cpu.update_cr0(ctx.cpu.gpr[instr.a()]);
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
    ctx.cpu.gpr[instr.a()] = ctx.cpu.gpr[instr.s()] | ctx.cpu.gpr[instr.b()];

    if instr.rc() {
        ctx.cpu.update_cr0(ctx.cpu.gpr[instr.a()]);
    }
}

fn op_rlwimix(ctx: &mut Context, instr: Instruction) {
    let m = mask(instr.mb(), instr.me());
    let r = ctx.cpu.gpr[instr.s()].rotate_left(u32::from(instr.sh()));

    ctx.cpu.gpr[instr.a()] = (r & m) | (ctx.cpu.gpr[instr.a()] & !m);

    if instr.rc() {
        ctx.cpu.update_cr0(ctx.cpu.gpr[instr.a()]);
    }
}

fn op_rlwinmx(ctx: &mut Context, instr: Instruction) {
    let mask = mask(instr.mb(), instr.me());

    ctx.cpu.gpr[instr.a()] = (ctx.cpu.gpr[instr.s()].rotate_left(u32::from(instr.sh()))) & mask;

    if instr.rc() {
        ctx.cpu.update_cr0(ctx.cpu.gpr[instr.a()]);
    }
}

fn op_rlwnmx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_rlwnmx");
}

fn op_slwx(ctx: &mut Context, instr: Instruction) {
    let r = ctx.cpu.gpr[instr.b()];

    ctx.cpu.gpr[instr.a()] = if r & 0x20 != 0 {
        0
    } else {
        ctx.cpu.gpr[instr.s()] << (r & 0x1F)
    };

    if instr.rc() {
        ctx.cpu.update_cr0(ctx.cpu.gpr[instr.a()]);
    }
}

fn op_srawix(ctx: &mut Context, instr: Instruction) {
    let n = instr.sh();

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
    let r = ctx.cpu.gpr[instr.b()];

    ctx.cpu.gpr[instr.a()] = if r & 0x20 != 0 {
        0
    } else {
        ctx.cpu.gpr[instr.s()] >> (r & 0x1F)
    };

    if instr.rc() {
        ctx.cpu.update_cr0(ctx.cpu.gpr[instr.a()]);
    }
}

fn op_subfcx(ctx: &mut Context, instr: Instruction) {
    let ra = !ctx.cpu.gpr[instr.a()];
    let rb = ctx.cpu.gpr[instr.b()] + 1;

    ctx.cpu.gpr[instr.d()] = ra.wrapping_add(rb);

    ctx.cpu.xer.set_carry((ctx.cpu.gpr[instr.a()]) < ra); // FixMe: ???

    if instr.rc() {
        ctx.cpu.update_cr0(ctx.cpu.gpr[instr.d()]);
    }

    if instr.oe() {
        panic!("OE: subfcx");
    }
}

fn op_subfex(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()];
    let rb = ctx.cpu.gpr[instr.b()];

    ctx.cpu.gpr[instr.d()] = !ra.wrapping_add(rb).wrapping_add(ctx.cpu.xer.carry() as u32);

    //self.xer.carry = (self.gpr[instr.a()]) < ra; // FixMe: ???

    if instr.rc() {
        ctx.cpu.update_cr0(ctx.cpu.gpr[instr.d()]);
    }

    if instr.oe() {
        panic!("OE: subfex");
    }
}

fn op_subfic(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()] as i32;
    let simm = instr.simm() as i32;

    let (rd, ca) = ra.overflowing_sub(simm);

    ctx.cpu.gpr[instr.d()] = rd as u32;
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
        ctx.cpu.update_cr0(ctx.cpu.gpr[instr.d()]);
    }

    if instr.oe() {
        panic!("OE: subfex");
    }
}

fn op_subfx(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.d()] = ctx.cpu.gpr[instr.b()].wrapping_sub(ctx.cpu.gpr[instr.a()]);

    if instr.rc() {
        ctx.cpu.update_cr0(ctx.cpu.gpr[instr.d()]);
    }

    if instr.oe() {
        panic!("OE: subfx");
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
    fn test_op_addic_rc() {
        let a: usize = 3;
        let d: usize = 31;

        let mut ctx = Context::default();
        let intr = Instruction(((d as u32) << 21) | ((a as u32) << 16) | 0x1);

        ctx.cpu.gpr[d] = 0xDEAD_BEEF;
        ctx.cpu.gpr[a] = 0xFFFF_FFFF;

        op_addic_rc(&mut ctx, intr);

        assert_eq!(ctx.cpu.gpr[d], 0x0);
        assert_eq!(ctx.cpu.gpr[a], 0xFFFF_FFFF); // confirm gpr source remains unmodified
        assert_eq!(ctx.cpu.xer.carry(), true);

        ctx.cpu.gpr[a] = 0xFFFF_FFFE;

        op_addic_rc(&mut ctx, intr);

        assert_eq!(ctx.cpu.gpr[d], 0xFFFF_FFFF);
        assert_eq!(ctx.cpu.xer.carry(), false);
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
