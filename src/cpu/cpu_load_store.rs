#[allow(unused_variables)]
fn op_dcbf(_ctx: &mut Context, _instr: Instruction) {
    //println!("FixMe: dcbf");
}

#[allow(unused_variables)]
fn op_dcbi(_ctx: &mut Context, _instr: Instruction) {
    //println!("FixMe: dcbi");
}

fn op_dcbst(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_dcbst");
}

fn op_dcbt(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_dcbt");
}

fn op_dcbtst(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_dcbtst");
}

fn op_dcbz(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_dcbz");
}

fn op_dcbz_l(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_dcbz_l");
}

fn op_eciwx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_eciwx");
}

fn op_ecowx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ecowx");
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

fn op_lbzux(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_lbzux");
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

fn op_lfdu(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_lfdu");
}

fn op_lfdux(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_lfdux");
}

fn op_lfdx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_lfdx");
}

fn op_lfs(ctx: &mut Context, instr: Instruction) {
    let ea = if instr.a() == 0 {
        instr.simm() as u32
    } else {
        ctx.cpu.gpr[instr.a()].wrapping_add(instr.simm() as u32)
    };

    let val = ctx.read_u32(ea);

    if !ctx.cpu.hid2.pse() {
        ctx.cpu.fpr[instr.d()] = convert_to_double(val);
    } else {
        ctx.cpu.fpr[instr.d()] = (u64::from(val) << 32) & u64::from(val);
    }
}

fn op_lfsu(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_lfsu");
}

fn op_lfsux(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_lfsux");
}

fn op_lfsx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_lfsx");
}

fn op_lha(ctx: &mut Context, instr: Instruction) {
    let ea = if instr.a() == 0 {
        instr.simm() as u32
    } else {
        ctx.cpu.gpr[instr.a()].wrapping_add(instr.simm() as u32)
    };

    ctx.cpu.gpr[instr.d()] = i32::from(ctx.read_u16(ea) as i16) as u32;
}

fn op_lhau(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_lhau");
}

fn op_lhaux(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_lhaux");
}

fn op_lhax(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_lhax");
}

fn op_lhbrx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_lhbrx");
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

fn op_lhzux(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_lhzux");
}

fn op_lhzx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_lhzx");
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

fn op_lswi(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_lswi");
}

fn op_lswx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_lswx");
}

fn op_lwarx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_lwarx");
}

fn op_lwbrx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_lwbrx");
}

fn op_lwz(ctx: &mut Context, instr: Instruction) {
    let ea = if instr.a() == 0 {
        instr.simm() as u32
    } else {
        ctx.cpu.gpr[instr.a()].wrapping_add(instr.simm() as u32)
    };

    ctx.cpu.gpr[instr.d()] = ctx.read_u32(ea);
}

fn op_lwzu(ctx: &mut Context, instr: Instruction) {
    let ea = ctx.cpu.gpr[instr.a()].wrapping_add(instr.simm() as u32);

    ctx.cpu.gpr[instr.d()] = ctx.read_u32(ea);
    ctx.cpu.gpr[instr.a()] = ea;
}

fn op_lwzux(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_lwzux");
}

fn op_lwzx(ctx: &mut Context, instr: Instruction) {
    let ea = if instr.a() == 0 {
        ctx.cpu.gpr[instr.b()]
    } else {
        ctx.cpu.gpr[instr.a()].wrapping_add(ctx.cpu.gpr[instr.b()])
    };

    ctx.cpu.gpr[instr.d()] = ctx.read_u32(ea);
}

fn op_psq_l(ctx: &mut Context, instr: Instruction) {
    if !ctx.cpu.hid2.pse() || !ctx.cpu.hid2.lsqe() {
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

fn op_psq_lu(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_psq_lu");
}

fn op_psq_lux(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_psq_lux");
}

fn op_psq_lx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_psq_lx");
}

fn op_psq_st(ctx: &mut Context, instr: Instruction) {
    if !ctx.cpu.hid2.pse() || !ctx.cpu.hid2.lsqe() {
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

fn op_psq_stu(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_psq_stu");
}

fn op_psq_stux(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_psq_stux");
}

fn op_psq_stx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_psq_stx");
}

fn op_stb(ctx: &mut Context, instr: Instruction) {
    let ea = if instr.a() == 0 {
        instr.simm() as u32
    } else {
        ctx.cpu.gpr[instr.a()].wrapping_add(instr.simm() as u32)
    };

    ctx.write_u8(ea, ctx.cpu.gpr[instr.d()] as u8);
}

fn op_stbu(ctx: &mut Context, instr: Instruction) {
    let ea = ctx.cpu.gpr[instr.a()].wrapping_add(instr.simm() as u32);

    ctx.write_u8(ea, ctx.cpu.gpr[instr.d()] as u8);

    ctx.cpu.gpr[instr.a()] = ea;
}

fn op_stbux(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_stbux");
}

fn op_stbx(ctx: &mut Context, instr: Instruction) {
    let ea = if instr.a() == 0 {
        ctx.cpu.gpr[instr.a()]
    } else {
        ctx.cpu.gpr[instr.a()].wrapping_add(ctx.cpu.gpr[instr.b()])
    };

    ctx.write_u8(ea, ctx.cpu.gpr[instr.d()] as u8);
}

fn op_stfd(ctx: &mut Context, instr: Instruction) {
    let ea = if instr.a() == 0 {
        instr.simm() as u32
    } else {
        ctx.cpu.gpr[instr.a()].wrapping_add(instr.simm() as u32)
    };

    ctx.write_u64(ea, ctx.cpu.fpr[instr.s()]);
}

fn op_stfdu(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_stfdu");
}

fn op_stfdux(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_stfdux");
}

fn op_stfdx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_stfdx");
}

fn op_stfiwx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_stfiwx");
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

fn op_stfsux(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_stfsux");
}

fn op_stfsx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_stfsx");
}

fn op_sth(ctx: &mut Context, instr: Instruction) {
    let ea = if instr.a() == 0 {
        instr.simm() as u32
    } else {
        ctx.cpu.gpr[instr.a()].wrapping_add(instr.simm() as u32)
    };

    ctx.write_u16(ea, ctx.cpu.gpr[instr.s()] as u16);
}

fn op_sthbrx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_sthbrx");
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

fn op_sthux(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_sthux");
}

fn op_sthx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_sthx");
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

fn op_stswi(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_stswi");
}

fn op_stswx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_stswx");
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

fn op_stwbrx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_stwbrx");
}

fn op_stwcx_rc(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_stwcx_rc");
}

fn op_stwu(ctx: &mut Context, instr: Instruction) {
    if instr.a() == 0 {
        panic!("stwu: invalid instruction");
    }

    let ea = ctx.cpu.gpr[instr.a()].wrapping_add(instr.simm() as u32);

    ctx.write_u32(ea, ctx.cpu.gpr[instr.s()]);

    ctx.cpu.gpr[instr.a()] = ea; // is this conditional ???
}

fn op_stwux(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_stwux");
}

fn op_stwx(ctx: &mut Context, instr: Instruction) {
    let ea = if instr.a() == 0 {
        ctx.cpu.gpr[instr.b()]
    } else {
        ctx.cpu.gpr[instr.a()].wrapping_add(ctx.cpu.gpr[instr.b()])
    };

    ctx.write_u32(ea, ctx.cpu.gpr[instr.s()]);
}

fn op_tlbie(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_tlbie");
}
