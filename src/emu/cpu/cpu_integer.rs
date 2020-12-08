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

    ctx.cpu.xer.set_carry(imm > !ra);
}

fn op_addic_rc(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()];
    let imm = i32::from(instr.simm()) as u32;

    ctx.cpu.gpr[instr.d()] = ra.wrapping_add(imm);

    ctx.cpu.xer.set_carry(imm > !ra);

    ctx.cpu.update_cr0(ctx.cpu.gpr[instr.d()]);
}

fn op_addis(ctx: &mut Context, instr: Instruction) {
    if instr.a() == 0 {
        ctx.cpu.gpr[instr.d()] = instr.uimm() << 16;
    } else {
        ctx.cpu.gpr[instr.d()] = ctx.cpu.gpr[instr.a()].wrapping_add(instr.uimm() << 16);
    }
}

fn op_addex(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_addex");
}

fn op_addzex(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_addzex");
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

fn op_addcx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_addcx");
}

fn op_andx(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.a()] = ctx.cpu.gpr[instr.s()] & ctx.cpu.gpr[instr.b()];

    if instr.rc() {
        ctx.cpu.update_cr0(ctx.cpu.gpr[instr.a()]);
    }
}

fn op_andcx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_andcx");
}

fn op_andi_rc(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.a()] = ctx.cpu.gpr[instr.s()] & instr.uimm();

    ctx.cpu.update_cr0(ctx.cpu.gpr[instr.a()]);
}

fn op_cmp(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_cmp");
}

fn op_cmpi(ctx: &mut Context, instr: Instruction) {
    if instr.l() {
        panic!("cmpi: invalid instruction");
    }

    let a = ctx.cpu.gpr[instr.a()] as i32;
    let b = i32::from(instr.simm());

    let mut c: u8 = if a < b {
        0b1000
    } else if a > b {
        0b0100
    } else {
        0b0010
    };

    c |= ctx.cpu.xer.summary_overflow() as u8;

    ctx.cpu.cr.set_field(instr.crfd(), c as u32);
}

fn op_cmpl(ctx: &mut Context, instr: Instruction) {
    if instr.l() {
        panic!("cmpl: invalid instruction");
    }

    let a = ctx.cpu.gpr[instr.a()];
    let b = ctx.cpu.gpr[instr.b()];

    let mut c: u8 = if a < b {
        0x8
    } else if a > b {
        0x4
    } else {
        0x2
    };

    c |= ctx.cpu.xer.summary_overflow() as u8;

    ctx.cpu.cr.set_field(instr.crfd(), c as u32);
}

fn op_cmpli(ctx: &mut Context, instr: Instruction) {
    if instr.l() {
        panic!("cmpli: invalid instruction");
    }

    let a = ctx.cpu.gpr[instr.a()];
    let b = instr.uimm();

    let mut c: u8 = if a < b {
        0b1000
    } else if a > b {
        0b0100
    } else {
        0b0010
    };

    c |= ctx.cpu.xer.summary_overflow() as u8;

    ctx.cpu.cr.set_field(instr.crfd(), c as u32);
}

fn op_cntlzwx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_cntlzwx");
}

fn op_divwx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_divwx");
}

fn op_divwux(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_divwux");
}

fn op_extsbx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_extsbx");
}

fn op_extshx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_extshx");
}

fn op_mulhwux(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_mulhwux");
}

fn op_mulli(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_mulli");
}

fn op_mullwx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_mullwx");
}

fn op_negx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_negx");
}

fn op_norx(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.a()] = !(ctx.cpu.gpr[instr.s()] | ctx.cpu.gpr[instr.b()]);

    if instr.rc() {
        ctx.cpu.update_cr0(ctx.cpu.gpr[instr.a()]);
    }
}

fn op_orx(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.a()] = ctx.cpu.gpr[instr.s()] | ctx.cpu.gpr[instr.b()];

    if instr.rc() {
        ctx.cpu.update_cr0(ctx.cpu.gpr[instr.a()]);
    }
}

fn op_ori(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.a()] = ctx.cpu.gpr[instr.s()] | instr.uimm();
}

fn op_oris(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.a()] = ctx.cpu.gpr[instr.s()] | (instr.uimm() << 16);
}

fn op_rlwimix(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_rlwimix");
}

fn op_rlwinmx(ctx: &mut Context, instr: Instruction) {
    let mask = mask(instr.mb(), instr.me());

    ctx.cpu.gpr[instr.a()] = (ctx.cpu.gpr[instr.s()].rotate_left(u32::from(instr.sh()))) & mask;

    if instr.rc() {
        ctx.cpu.update_cr0(ctx.cpu.gpr[instr.a()]);
    }
}

fn op_slwx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_slwx");
}

fn op_srawx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_srawx");
}

fn op_srawix(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_srawix");
}

fn op_srwx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_srwx");
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

fn op_subfcx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_subfcx");
}

fn op_subfex(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_subfex");
}

fn op_subfic(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()] as i32;
    let simm = instr.simm() as i32;

    ctx.cpu.gpr[instr.d()] = (simm - ra) as u32;

    ctx.cpu.xer.set_carry(simm == 0); // FixMe: Is this correct?
}

fn op_subfzex(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_subfzex");
}

fn op_xorx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_xorx");
}

fn op_xoris(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_xoris");
}
