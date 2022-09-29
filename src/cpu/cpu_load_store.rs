#[allow(unused_variables)]
fn op_dcbf(_ctx: &mut Context, _instr: Instruction) {
    //println!("FixMe: dcbf");
}

#[allow(unused_variables)]
fn op_dcbi(_ctx: &mut Context, _instr: Instruction) {
    //println!("FixMe: dcbi");
}

#[allow(unused_variables)]
fn op_icbi(_ctx: &mut Context, _instr: Instruction) {
    //println!("FixMe: icbi");
}

fn op_lbz(ctx: &mut Context, instr: Instruction) {
    let ea = if instr.a() == 0 {
        instr.simm() as u32
    } else {
        ctx.cpu.gpr[instr.a()].wrapping_add(instr.simm() as u32)
    };

    ctx.cpu.gpr[instr.d()] = u32::from(ctx.read_u8(ea));
}

fn op_lbzu(ctx: &mut Context, instr: Instruction) {
    if instr.a() == 0 || instr.a() == instr.d() {
        panic!("lbzu: invalid instruction");
    }

    let ea = ctx.cpu.gpr[instr.a()].wrapping_add(instr.simm() as u32);

    ctx.cpu.gpr[instr.d()] = u32::from(ctx.read_u8(ea));
    ctx.cpu.gpr[instr.a()] = ea;
}

fn op_lbzx(ctx: &mut Context, instr: Instruction) {
    let ea = if instr.a() == 0 {
        ctx.cpu.gpr[instr.b()]
    } else {
        ctx.cpu.gpr[instr.a()].wrapping_add(ctx.cpu.gpr[instr.b()])
    };

    ctx.cpu.gpr[instr.d()] = u32::from(ctx.read_u8(ea));
}

fn op_lfd(ctx: &mut Context, instr: Instruction) {
    let ea = if instr.a() == 0 {
        instr.simm() as u32
    } else {
        ctx.cpu.gpr[instr.a()].wrapping_add(instr.simm() as u32)
    };

    ctx.cpu.fpr[instr.d()] = ctx.read_u64(ea);
}

fn op_lfs(ctx: &mut Context, instr: Instruction) {
    let ea = if instr.a() == 0 {
        instr.simm() as u32
    } else {
        ctx.cpu.gpr[instr.a()].wrapping_add(instr.simm() as u32)
    };

    let val = ctx.read_u32(ea);

    if !ctx.cpu.hid2.paired_single {
        ctx.cpu.fpr[instr.d()] = convert_to_double(val);
    } else {
        ctx.cpu.fpr[instr.d()] = (u64::from(val) << 32) & u64::from(val);
    }
}

fn op_lha(ctx: &mut Context, instr: Instruction) {
    let ea = if instr.a() == 0 {
        instr.simm() as u32
    } else {
        ctx.cpu.gpr[instr.a()].wrapping_add(instr.simm() as u32)
    };

    ctx.cpu.gpr[instr.d()] = i32::from(ctx.read_u16(ea) as i16) as u32;
}

fn op_lhz(ctx: &mut Context, instr: Instruction) {
    let ea = if instr.a() == 0 {
        instr.simm() as u32
    } else {
        ctx.cpu.gpr[instr.a()].wrapping_add(instr.simm() as u32)
    };

    ctx.cpu.gpr[instr.d()] = u32::from(ctx.read_u16(ea));
}

fn op_lhzu(ctx: &mut Context, instr: Instruction) {
    let ea = ctx.cpu.gpr[instr.a()].wrapping_add(instr.simm() as u32);

    ctx.cpu.gpr[instr.d()] = u32::from(ctx.read_u16(ea));
    ctx.cpu.gpr[instr.a()] = ea;
}

fn op_lmw(ctx: &mut Context, instr: Instruction) {
    let mut ea = if instr.a() == 0 {
        instr.simm() as u32
    } else {
        ctx.cpu.gpr[instr.a()].wrapping_add(instr.simm() as u32)
    };

    let mut r = instr.d();

    while r <= 31 {
        ctx.cpu.gpr[r] = ctx.read_u32(ea);

        r += 1;
        ea += 4;
    }
}

fn op_lwz(ctx: &mut Context, instr: Instruction) {
    let ea = if instr.a() == 0 {
        instr.simm() as u32
    } else {
        ctx.cpu.gpr[instr.a()].wrapping_add(instr.simm() as u32)
    };

    ctx.cpu.gpr[instr.d()] = ctx.read_u32(ea);
}

fn op_lwzx(ctx: &mut Context, instr: Instruction) {
    let ea = if instr.a() == 0 {
        ctx.cpu.gpr[instr.b()]
    } else {
        ctx.cpu.gpr[instr.a()].wrapping_add(ctx.cpu.gpr[instr.b()])
    };

    ctx.cpu.gpr[instr.d()] = ctx.read_u32(ea);
}

fn op_lwzu(ctx: &mut Context, instr: Instruction) {
    let ea = ctx.cpu.gpr[instr.a()].wrapping_add(instr.simm() as u32);

    ctx.cpu.gpr[instr.d()] = ctx.read_u32(ea);
    ctx.cpu.gpr[instr.a()] = ea;
}

fn op_psq_l(ctx: &mut Context, instr: Instruction) {
    if !ctx.cpu.hid2.paired_single || !ctx.cpu.hid2.load_stored_quantized {
        panic!("FixMe: GoTo illegal instruction handler");
    }

    let ea = if instr.a() == 0 {
        sign_ext_12(instr.uimm_1()) as u32
    } else {
        ctx.cpu.gpr[instr.a()].wrapping_add(sign_ext_12(instr.uimm_1()) as u32)
    };

    let gqr = Gqr(ctx.cpu.gqr[instr.i()]);

    match gqr.ld_type() {
        0 => {
            if instr.w() {
                let value = ctx.read_u32(ea);

                let ps0 = value;
                let ps1 = 1.0;

                ctx.cpu.fpr[instr.d()] = (u64::from(ps0) << 32) | (ps1 as u64);
            } else {
                let value = (ctx.read_u32(ea), ctx.read_u32(ea + 4));

                ctx.cpu.fpr[instr.d()] = (u64::from(value.0) << 32) | u64::from(value.1);
            }
        }
        4 | 6 => panic!("FixMe:..."),
        5 | 7 => panic!("FixMe:..."),
        _ => panic!("unrecognized ld_type"),
    }
}

fn op_psq_st(ctx: &mut Context, instr: Instruction) {
    if !ctx.cpu.hid2.paired_single || !ctx.cpu.hid2.load_stored_quantized {
        panic!("FixMe: GoTo illegal instruction handler");
    }

    let ea = if instr.a() == 0 {
        sign_ext_12(instr.uimm_1()) as u32
    } else {
        ctx.cpu.gpr[instr.a()].wrapping_add(sign_ext_12(instr.uimm_1()) as u32)
    };

    let gqr = Gqr(ctx.cpu.gqr[instr.i()]);

    match gqr.st_type() {
        0 => {
            // single-precision floating-point (no-conversion)
            if instr.w() {
                let ps0 = (ctx.cpu.fpr[instr.d()] >> 32) as u32;

                ctx.write_u32(ea, ps0);
            } else {
                ctx.write_u64(ea, ctx.cpu.fpr[instr.d()]);
            }
        }
        4 | 6 => panic!("FixMe:..."), // unsigned 8 bit integer | signed 8 bit integer
        5 | 7 => panic!("FixMe:..."), // unsigned 16 bit integer | signed 16 bit integer
        _ => panic!("unrecognized st_type"),
    }
}

fn op_stb(ctx: &mut Context, instr: Instruction) {
    let ea = if instr.a() == 0 {
        instr.simm() as u32
    } else {
        ctx.cpu.gpr[instr.a()].wrapping_add(instr.simm() as u32)
    };

    ctx.write_u8(ea, ctx.cpu.gpr[instr.d()] as u8);
}

fn op_stbx(ctx: &mut Context, instr: Instruction) {
    let ea = if instr.a() == 0 {
        ctx.cpu.gpr[instr.a()]
    } else {
        ctx.cpu.gpr[instr.a()].wrapping_add(ctx.cpu.gpr[instr.b()])
    };

    ctx.write_u8(ea, ctx.cpu.gpr[instr.d()] as u8);
}

fn op_stbu(ctx: &mut Context, instr: Instruction) {
    let ea = ctx.cpu.gpr[instr.a()].wrapping_add(instr.simm() as u32);

    ctx.write_u8(ea, ctx.cpu.gpr[instr.d()] as u8);

    ctx.cpu.gpr[instr.a()] = ea;
}

fn op_stfd(ctx: &mut Context, instr: Instruction) {
    let ea = if instr.a() == 0 {
        instr.simm() as u32
    } else {
        ctx.cpu.gpr[instr.a()].wrapping_add(instr.simm() as u32)
    };

    ctx.write_u64(ea, ctx.cpu.fpr[instr.s()]);
}

fn op_stfs(ctx: &mut Context, instr: Instruction) {
    let ea = if instr.a() == 0 {
        instr.simm() as u32
    } else {
        ctx.cpu.gpr[instr.a()].wrapping_add(instr.simm() as u32)
    };

    let val = convert_to_single(ctx.cpu.fpr[instr.s()]);

    ctx.write_u32(ea, val);
}

fn op_stfsu(ctx: &mut Context, instr: Instruction) {
    if instr.a() == 0 {
        panic!("stfsu: invalid instruction");
    }

    let ea = ctx.cpu.gpr[instr.a()].wrapping_add(instr.simm() as u32);
    let val = convert_to_single(ctx.cpu.fpr[instr.s()]);

    ctx.write_u32(ea, val);
}

fn op_sth(ctx: &mut Context, instr: Instruction) {
    let ea = if instr.a() == 0 {
        instr.simm() as u32
    } else {
        ctx.cpu.gpr[instr.a()].wrapping_add(instr.simm() as u32)
    };

    ctx.write_u16(ea, ctx.cpu.gpr[instr.s()] as u16);
}

fn op_sthu(ctx: &mut Context, instr: Instruction) {
    let ea = if instr.a() == 0 {
        instr.simm() as u32
    } else {
        ctx.cpu.gpr[instr.a()].wrapping_add(instr.simm() as u32)
    };

    ctx.write_u16(ea, ctx.cpu.gpr[instr.s()] as u16);

    ctx.cpu.gpr[instr.a()] = ea;
}

fn op_stmw(ctx: &mut Context, instr: Instruction) {
    let mut ea = if instr.a() == 0 {
        instr.simm() as u32
    } else {
        ctx.cpu.gpr[instr.a()].wrapping_add(instr.simm() as u32)
    };

    let mut r = instr.s();

    while r <= 31 {
        ctx.write_u32(ea, ctx.cpu.gpr[r]);

        r += 1;
        ea += 4;
    }
}

fn op_stw(ctx: &mut Context, instr: Instruction) {
    let ea = if instr.a() == 0 {
        instr.simm() as u32
    } else {
        ctx.cpu.gpr[instr.a()].wrapping_add(instr.simm() as u32)
    };

    // TODO: remove this at some point
    // enable devkit mode
    if ctx.cpu.cia == 0x8130_04c4 {
        ctx.write_u32(ea, 0x1000_0006);
    } else {
        ctx.write_u32(ea, ctx.cpu.gpr[instr.s()]);
    }
}

fn op_stwx(ctx: &mut Context, instr: Instruction) {
    let ea = if instr.a() == 0 {
        ctx.cpu.gpr[instr.b()]
    } else {
        ctx.cpu.gpr[instr.a()].wrapping_add(ctx.cpu.gpr[instr.b()])
    };

    ctx.write_u32(ea, ctx.cpu.gpr[instr.s()]);
}

fn op_stwu(ctx: &mut Context, instr: Instruction) {
    if instr.a() == 0 {
        panic!("stwu: invalid instruction");
    }

    let ea = ctx.cpu.gpr[instr.a()].wrapping_add(instr.simm() as u32);

    ctx.write_u32(ea, ctx.cpu.gpr[instr.s()]);

    ctx.cpu.gpr[instr.a()] = ea; // is this conditional ???
}
