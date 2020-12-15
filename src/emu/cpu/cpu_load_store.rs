pub fn convert_to_double(v: u32) -> u64 {
    let x = v as u64;
    let mut exp = (x >> 23) & 0xFF;
    let mut frac = x & 0x007F_FFFF;

    if exp > 0 && exp < 255 {
        // Normalize Operand
        let y = !(exp >> 7);
        let z = y << 61 | y << 60 | y << 59;
        ((x & 0xC000_0000) << 32) | z | ((x & 0x3FFF_FFFF) << 29)
    } else if exp == 0 && frac != 0 {
        // Denormalize Operand
        exp = 1023 - 126;
        while (frac & 0x0080_0000) == 0 {
            frac <<= 1;
            exp -= 1;
        }

        ((x & 0x8000_0000) << 32) | (exp << 52) | ((frac & 0x007F_FFFF) << 29)
    } else {
        // Infinity / QNaN / SNaN / Zero
        let y = exp >> 7;
        let z = y << 61 | y << 60 | y << 59;
        ((x & 0xC000_0000) << 32) | z | ((x & 0x3FFF_FFFF) << 29)
    }
}

pub fn convert_to_single(x: u64) -> u32 {
    let exp = (x >> 52) & 0x7FF;

    if exp > 896 || x & 0x7FFF_FFFF == 0 {
        // No Denormalization
        (((x >> 32) as u32) & 0xC000_0000) | (((x >> 29) as u32) & 0x3FFF_FFFF)
    } else if exp >= 874 {
        // Denormalization
        let mut y: u32 = 0x8000_0000 | ((x & 0x000F_FFFF_FFFF_FFFF) >> 21) as u32;
        y = y >> (905 - exp);
        y |= ((x >> 32) as u32) & 0x8000_0000;
        y
    } else {
        // Undefined
        unimplemented!();
    }
}

fn get_ea(ctx: &Context, instr: Instruction) -> u32 {
    if instr.a() == 0 {
        i32::from(instr.simm()) as u32
    } else {
        ctx.cpu.gpr[instr.a()].wrapping_add(i32::from(instr.simm()) as u32)
    }
}

fn get_ea_u(ctx: &Context, instr: Instruction) -> u32 {
    ctx.cpu.gpr[instr.a()].wrapping_add(i32::from(instr.simm()) as u32)
}

fn get_ea_x(ctx: &Context, instr: Instruction) -> u32 {
    if instr.a() == 0 {
        ctx.cpu.gpr[instr.b()]
    } else {
        ctx.cpu.gpr[instr.a()].wrapping_add(ctx.cpu.gpr[instr.b()])
    }
}

fn op_dcbf(_ctx: &mut Context, _instr: Instruction) {
    // don't do anything
}

fn op_dcbi(_ctx: &mut Context, _instr: Instruction) {
    // don't do anything
}

fn op_icbi(_ctx: &mut Context, _instr: Instruction) {
    // don't do anything
}

fn op_lbz(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.d()] = u32::from(ctx.read_u8(get_ea(ctx, instr)));
}

fn op_lbzu(ctx: &mut Context, instr: Instruction) {
    if instr.a() == 0 || instr.a() == instr.d() {
        panic!("lbzu: invalid instruction");
    }

    let ea = get_ea_u(ctx, instr);

    ctx.cpu.gpr[instr.d()] = u32::from(ctx.read_u8(ea));
    ctx.cpu.gpr[instr.a()] = ea;
}

fn op_lbzx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_lbzx");
}

fn op_lfd(ctx: &mut Context, instr: Instruction) {
    let ea = get_ea(ctx, instr);

    // FixMe: check for DSI exception ???

    ctx.cpu.fpr[instr.d()] = Fpr(ctx.read_u64(ea));
}

fn op_lfs(ctx: &mut Context, instr: Instruction) {
    let ea = get_ea(ctx, instr);

    let val = ctx.read_u32(ea);

    if ctx.cpu.hid2.pse() {
        ctx.cpu.fpr[instr.d()].set_ps0(val);
        ctx.cpu.fpr[instr.d()].set_ps1(val);
    } else {
        ctx.cpu.fpr[instr.d()] = Fpr(convert_to_double(val));
    }
}

fn op_lha(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_lha");
}

fn op_lhz(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.d()] = u32::from(ctx.read_u16(get_ea(ctx, instr)));
}

fn op_lhzu(ctx: &mut Context, instr: Instruction) {
    let ea = get_ea_u(ctx, instr);

    ctx.cpu.gpr[instr.d()] = u32::from(ctx.read_u16(ea));
    ctx.cpu.gpr[instr.a()] = ea;
}

fn op_lmw(ctx: &mut Context, instr: Instruction) {
    let mut ea = get_ea(ctx, instr);
    let mut r = instr.d();

    while r < 32 {
        ctx.cpu.gpr[r] = ctx.read_u32(ea);

        r += 1;
        ea += 4;
    }
}

fn op_lwz(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.d()] = ctx.read_u32(get_ea(ctx, instr));
}

fn op_lwzx(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.d()] = ctx.read_u32(get_ea_x(ctx, instr));
}

fn op_lwzu(ctx: &mut Context, instr: Instruction) {
    let ea = get_ea_u(ctx, instr);

    ctx.cpu.gpr[instr.d()] = ctx.read_u32(ea);
    ctx.cpu.gpr[instr.a()] = ea;
}

fn op_psq_l(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_psq_l");
}

fn op_psq_st(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_psq_st");
}

fn op_stb(ctx: &mut Context, instr: Instruction) {
    ctx.write_u8(get_ea(ctx, instr), ctx.cpu.gpr[instr.s()] as u8);
}

fn op_stbx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_stbx");
}

fn op_stbu(ctx: &mut Context, instr: Instruction) {
    let ea = get_ea_u(ctx, instr);

    ctx.write_u8(ea, ctx.cpu.gpr[instr.s()] as u8);

    ctx.cpu.gpr[instr.a()] = ea;
}

fn op_stfd(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_stfd");
}

fn op_stfs(ctx: &mut Context, instr: Instruction) {
    let ea = get_ea(ctx, instr);

    let val = ctx.cpu.fpr[instr.s()].as_u64();

    ctx.write_u32(ea, convert_to_single(val));
}

fn op_stfsu(ctx: &mut Context, instr: Instruction) {
    let ea = get_ea_u(ctx, instr);

    let val = ctx.cpu.fpr[instr.s()].as_u64();

    ctx.write_u32(ea, convert_to_single(val));

    ctx.cpu.gpr[instr.a()] = ea;
}

fn op_sth(ctx: &mut Context, instr: Instruction) {
    ctx.write_u16(get_ea(ctx, instr), ctx.cpu.gpr[instr.s()] as u16);
}

fn op_sthu(ctx: &mut Context, instr: Instruction) {
    let ea = get_ea_u(ctx, instr);

    ctx.write_u16(ea, ctx.cpu.gpr[instr.s()] as u16);

    ctx.cpu.gpr[instr.a()] = ea;
}

// FixMe: handle alignment interrupt if ea is not multiple of 4
fn op_stmw(ctx: &mut Context, instr: Instruction) {
    let mut ea = get_ea(ctx, instr);
    let mut r = instr.s();

    while r < 32 {
        ctx.write_u32(ea, ctx.cpu.gpr[r]);

        r += 1;
        ea += 4;
    }
}

fn op_stw(ctx: &mut Context, instr: Instruction) {
    if ctx.cpu.cia == 0x8130_04c4 {
        // FixMe: remove at some point, possibly add to gdb script
        ctx.write_u32(get_ea(ctx, instr), 0x1000_0006); // Set Console Type to: latest Devkit HW
    } else {
        ctx.write_u32(get_ea(ctx, instr), ctx.cpu.gpr[instr.s()]);
    }
}

fn op_stwx(ctx: &mut Context, instr: Instruction) {
    let ea = get_ea_x(ctx, instr);

    ctx.write_u32(ea, ctx.cpu.gpr[instr.s()]);
}

fn op_stwu(ctx: &mut Context, instr: Instruction) {
    if instr.a() == 0 {
        panic!("stwu: invalid instruction");
    }

    let ea = get_ea_u(ctx, instr);

    ctx.write_u32(ea, ctx.cpu.gpr[instr.s()]);

    ctx.cpu.gpr[instr.a()] = ea;
}
