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

    ctx.tick(1);
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
        ctx.cpu.update_cr0(rd);
    }

    ctx.tick(1);
}

fn op_addi(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.d()] = if instr.a() == 0 {
        i32::from(instr.simm()) as u32
    } else {
        ctx.cpu.gpr[instr.a()].wrapping_add(i32::from(instr.simm()) as u32)
    };

    ctx.tick(1);
}

fn op_addic(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()];
    let imm = i32::from(instr.simm()) as u32;

    let (rd, ca) = ra.overflowing_add(imm);

    ctx.cpu.gpr[instr.d()] = rd;

    ctx.cpu.xer.set_carry(ca);

    ctx.tick(1);
}

fn op_addic_rc(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()];
    let imm = i32::from(instr.simm()) as u32;

    let (rd, ca) = ra.overflowing_add(imm);

    ctx.cpu.gpr[instr.d()] = rd;

    ctx.cpu.xer.set_carry(ca);

    ctx.cpu.update_cr0(rd);

    ctx.tick(1);
}

fn op_addis(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.d()] = if instr.a() == 0 {
        instr.uimm() << 16
    } else {
        ctx.cpu.gpr[instr.a()].wrapping_add(instr.uimm() << 16)
    };

    ctx.tick(1);
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

    ctx.tick(1);
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

    ctx.tick(1);
}

fn op_andcx(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.s()] & (!ctx.cpu.gpr[instr.b()]);

    ctx.cpu.gpr[instr.a()] = ra;

    if instr.rc() {
        ctx.cpu.update_cr0(ra);
    }

    ctx.tick(1);
}

fn op_andi_rc(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.s()] & instr.uimm();

    ctx.cpu.gpr[instr.a()] = ra;

    ctx.cpu.update_cr0(ra);

    ctx.tick(1);
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

    ctx.tick(1);
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

    ctx.tick(1);
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

    ctx.tick(1);
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

    ctx.tick(1);
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

    ctx.tick(1);
}

fn op_cntlzwx(ctx: &mut Context, instr: Instruction) {
    let n = ctx.cpu.gpr[instr.s()].leading_zeros();

    ctx.cpu.gpr[instr.a()] = n;

    if instr.rc() {
        ctx.cpu.update_cr0(n);
    }

    ctx.tick(1);
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

    ctx.tick(19);
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

    ctx.tick(19);
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

    ctx.tick(1);
}

fn op_extshx(ctx: &mut Context, instr: Instruction) {
    let ra = ((ctx.cpu.gpr[instr.s()] as i16) as i32) as u32;

    ctx.cpu.gpr[instr.a()] = ra;

    if instr.rc() {
        ctx.cpu.update_cr0(ra);
    }

    ctx.tick(1);
}

fn op_mulhwux(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()] as u64;
    let rb = ctx.cpu.gpr[instr.b()] as u64;

    let rd = ((ra * rb) >> 32) as u32;

    ctx.cpu.gpr[instr.d()] = rd;

    if instr.rc() {
        ctx.cpu.update_cr0(rd);
    }

    ctx.tick(2);
}

fn op_mulhwx(ctx: &mut Context, instr: Instruction) {
    let ra = (ctx.cpu.gpr[instr.a()] as i32) as i64;
    let rb = (ctx.cpu.gpr[instr.b()] as i32) as i64;

    let rd = ((ra * rb) >> 32) as u32;

    ctx.cpu.gpr[instr.d()] = rd;

    if instr.rc() {
        ctx.cpu.update_cr0(rd);
    }

    ctx.tick(2);
}

// TODO: review this implementation
fn op_mulli(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.d()] =
        (ctx.cpu.gpr[instr.a()] as i32).wrapping_mul(i32::from(instr.simm())) as u32;

    ctx.tick(2);
}

fn op_mullwx(ctx: &mut Context, instr: Instruction) {
    let ra = (ctx.cpu.gpr[instr.a()] as i32) as i64;
    let rb = (ctx.cpu.gpr[instr.b()] as i32) as i64;

    let rd = ra.wrapping_mul(rb);

    ctx.cpu.gpr[instr.d()] = rd as u32;

    if instr.oe() {
        ctx.cpu
            .set_xer_so(!(-0x8000_0000..=0x7FFF_FFFF).contains(&rd));
    }

    if instr.rc() {
        ctx.cpu.update_cr0(rd as u32);
    }

    ctx.tick(2);
}

fn op_nandx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_nandx");
}

fn op_negx(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()];
    let rd = (!ra).wrapping_add(1);

    ctx.cpu.gpr[instr.d()] = rd;

    if instr.oe() {
        ctx.cpu.set_xer_so(ra == 0x8000_0000);
    }

    if instr.rc() {
        ctx.cpu.update_cr0(rd);
    }

    ctx.tick(1);
}

fn op_norx(ctx: &mut Context, instr: Instruction) {
    let ra = !(ctx.cpu.gpr[instr.s()] | ctx.cpu.gpr[instr.b()]);

    ctx.cpu.gpr[instr.a()] = ra;

    if instr.rc() {
        ctx.cpu.update_cr0(ra);
    }

    ctx.tick(1);
}

fn op_orcx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_orcx");
}

fn op_ori(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.a()] = ctx.cpu.gpr[instr.s()] | instr.uimm();

    ctx.tick(1);
}

fn op_oris(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.a()] = ctx.cpu.gpr[instr.s()] | (instr.uimm() << 16);

    ctx.tick(1);
}

fn op_orx(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.s()] | ctx.cpu.gpr[instr.b()];

    ctx.cpu.gpr[instr.a()] = ra;

    if instr.rc() {
        ctx.cpu.update_cr0(ra);
    }

    ctx.tick(1);
}

fn op_rlwimix(ctx: &mut Context, instr: Instruction) {
    let m = mask(instr.mb(), instr.me());
    let r = ctx.cpu.gpr[instr.s()].rotate_left(instr.sh());

    let ra = (r & m) | (ctx.cpu.gpr[instr.a()] & !m);

    ctx.cpu.gpr[instr.a()] = ra;

    if instr.rc() {
        ctx.cpu.update_cr0(ra);
    }

    ctx.tick(1);
}

fn op_rlwinmx(ctx: &mut Context, instr: Instruction) {
    let m = mask(instr.mb(), instr.me());
    let r = ctx.cpu.gpr[instr.s()].rotate_left(instr.sh());

    let ra = r & m;

    ctx.cpu.gpr[instr.a()] = ra;

    if instr.rc() {
        ctx.cpu.update_cr0(ra);
    }

    ctx.tick(1);
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

    ctx.tick(1);
}

fn op_srawix(ctx: &mut Context, instr: Instruction) {
    let rs = ctx.cpu.gpr[instr.s()] as i32;
    let s = instr.s();

    ctx.cpu.gpr[instr.a()] = (rs >> instr.sh()) as u32;
    ctx.cpu
        .xer
        .set_carry(rs < 0 && ((rs as u32) << (32 - s)) != 0);

    ctx.tick(1);
}

// TODO: review this implementation
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

            ctx.cpu.xer.set_carry(rs < 0 && (rs << (32 - n) != 0));
        } else {
            ctx.cpu.gpr[instr.a()] = ctx.cpu.gpr[instr.s()];
            ctx.cpu.xer.set_carry(false);
        }
    }

    if instr.rc() {
        ctx.cpu.update_cr0(ctx.cpu.gpr[instr.a()]);
    }

    ctx.tick(1);
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

    ctx.tick(1);
}

fn op_subfcx(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()];
    let rb = ctx.cpu.gpr[instr.b()];

    let (rd, ca1) = (!ra).overflowing_add(rb);
    let (rd, ca2) = rd.overflowing_add(1);

    ctx.cpu.gpr[instr.d()] = rd;

    ctx.cpu.xer.set_carry(ca1 || ca2);

    if instr.oe() {
        ctx.cpu.set_xer_so(check_overflowed(!ra, rb, rd));
    }

    if instr.rc() {
        ctx.cpu.update_cr0(rd);
    }

    ctx.tick(1);
}

fn op_subfex(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()];
    let rb = ctx.cpu.gpr[instr.b()];

    let (rd, ca1) = (!ra).overflowing_add(rb);
    let (rd, ca2) = rd.overflowing_add(ctx.cpu.xer.carry() as u32);

    ctx.cpu.gpr[instr.d()] = rd;

    ctx.cpu.xer.set_carry(ca1 | ca2);

    if instr.oe() {
        ctx.cpu.set_xer_so(check_overflowed(!ra, rb, rd));
    }

    if instr.rc() {
        ctx.cpu.update_cr0(rd);
    }

    ctx.tick(1);
}

fn op_subfic(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()];
    let simm = (instr.simm() as i32) as u32;

    let (rd, ca) = simm.overflowing_sub(ra);

    ctx.cpu.gpr[instr.d()] = rd;

    ctx.cpu.xer.set_carry(ca);

    ctx.tick(1);
}

fn op_subfmex(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_subfmex");
}

fn op_subfzex(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()];
    let ca = ctx.cpu.xer.carry() as u32;

    let rd = (!ra).wrapping_add(ca);

    ctx.cpu.gpr[instr.d()] = rd;

    ctx.cpu.xer.set_carry(ca > ra);

    if instr.rc() {
        ctx.cpu.update_cr0(rd);
    }

    if instr.oe() {
        panic!("OE: subfzex");
    }

    ctx.tick(1);
}

fn op_subfx(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()] as i32;
    let rb = ctx.cpu.gpr[instr.b()] as i32;

    let (rd, ov) = rb.overflowing_sub(ra);
    let rd = rd as u32;

    ctx.cpu.gpr[instr.d()] = rd;

    if instr.oe() {
        ctx.cpu.set_xer_so(ov);
    }

    if instr.rc() {
        ctx.cpu.update_cr0(rd);
    }

    ctx.tick(1);
}

fn op_tw(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_tw");
}

fn op_twi(ctx: &mut Context, instr: Instruction) {
    let a = ctx.cpu.gpr[instr.a()] as i32;
    let simm = instr.simm() as i32;
    let to = instr.to();

    if (a < simm && (to & 0x10) != 0)
        || (a > simm && (to & 0x80) != 0)
        || (a == simm && (to & 0x04) != 0)
        || ((a as u32) < simm as u32 && (to & 0x02) != 0)
        || (a as u32 > simm as u32 && (to & 0x01) != 0)
    {
        ctx.cpu.exceptions |= EXCEPTION_PROGRAM;
        // Set trap program exception flag
        ctx.cpu.spr[SPR_SRR1] = 1 << (31 - 14);
    }
}

fn op_xori(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_xori");
}

fn op_xoris(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.a()] = ctx.cpu.gpr[instr.s()] ^ (instr.uimm() << 16);

    ctx.tick(1);
}

fn op_xorx(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.a()] = ctx.cpu.gpr[instr.s()] ^ ctx.cpu.gpr[instr.b()];

    if instr.rc() {
        ctx.cpu.update_cr0(ctx.cpu.gpr[instr.a()]);
    }

    ctx.tick(1);
}
