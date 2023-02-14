fn get_ea(ctx: &Context, instr: Instruction) -> u32 {
    if instr.a() == 0 {
        (instr.simm() as i32) as u32
    } else {
        ctx.cpu.gpr[instr.a()].wrapping_add((instr.simm() as i32) as u32)
    }
}

fn get_ea_u(ctx: &Context, instr: Instruction) -> u32 {
    ctx.cpu.gpr[instr.a()].wrapping_add((instr.simm() as i32) as u32)
}

fn get_ea_x(ctx: &Context, instr: Instruction) -> u32 {
    if instr.a() == 0 {
        ctx.cpu.gpr[instr.b()]
    } else {
        ctx.cpu.gpr[instr.a()].wrapping_add(ctx.cpu.gpr[instr.b()])
    }
}

fn op_dcbf(ctx: &mut Context, _instr: Instruction) {
    // don't do anything

    ctx.tick(3);
}

fn op_dcbi(ctx: &mut Context, _instr: Instruction) {
    // don't do anything

    ctx.tick(3);
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

fn op_icbi(ctx: &mut Context, _instr: Instruction) {
    // don't do anything

    ctx.tick(3);
}

fn op_lbz(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.d()] = u32::from(ctx.read_u8(get_ea(ctx, instr)));

    ctx.tick(2);
}

fn op_lbzu(ctx: &mut Context, instr: Instruction) {
    if instr.a() == 0 || instr.a() == instr.d() {
        panic!("lbzu: invalid instruction");
    }

    let ea = ctx.cpu.gpr[instr.a()].wrapping_add(instr.simm() as u32);

    ctx.cpu.gpr[instr.d()] = u32::from(ctx.read_u8(ea));
    ctx.cpu.gpr[instr.a()] = ea;

    ctx.tick(2);
}

fn op_lbzux(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_lbzux");
}

fn op_lbzx(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.d()] = u32::from(ctx.read_u8(get_ea_x(ctx, instr)));

    ctx.tick(2);
}

fn op_lfd(ctx: &mut Context, instr: Instruction) {
    let ea = get_ea(ctx, instr);

    // FIXME: check for DSI exception ???
    let val = ctx.read_u64(ea);

    ctx.cpu.fpr[instr.d()].set_ps0(val);

    ctx.tick(2);
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
    let ea = get_ea(ctx, instr);

    let val = convert_to_double(ctx.read_u32(ea));

    ctx.cpu.fpr[instr.d()].set_ps0(val);

    if ctx.cpu.hid2.pse() {
        ctx.cpu.fpr[instr.d()].set_ps1(val);
    }

    ctx.tick(2);
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
    let ea = get_ea(ctx, instr);

    ctx.cpu.gpr[instr.d()] = ((ctx.read_u16(ea) as i16) as i32) as u32;

    ctx.tick(2);
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
    ctx.cpu.gpr[instr.d()] = u32::from(ctx.read_u16(get_ea(ctx, instr)));

    ctx.tick(2);
}

fn op_lhzu(ctx: &mut Context, instr: Instruction) {
    let ea = get_ea_u(ctx, instr);

    ctx.cpu.gpr[instr.d()] = u32::from(ctx.read_u16(ea));
    ctx.cpu.gpr[instr.a()] = ea;

    ctx.tick(2);
}

fn op_lhzux(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_lhzux");
}

fn op_lhzx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_lhzx");
}

fn op_lmw(ctx: &mut Context, instr: Instruction) {
    let mut ea = get_ea(ctx, instr);
    let mut r = instr.d();
    let n = (32 - r) as u32;

    while r < 32 {
        ctx.cpu.gpr[r] = ctx.read_u32(ea);

        r += 1;
        ea += 4;
    }

    ctx.tick(2 + n);
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
    ctx.cpu.gpr[instr.d()] = ctx.read_u32(get_ea(ctx, instr));

    ctx.tick(2);
}

fn op_lwzu(ctx: &mut Context, instr: Instruction) {
    let ea = get_ea_u(ctx, instr);

    ctx.cpu.gpr[instr.d()] = ctx.read_u32(ea);
    ctx.cpu.gpr[instr.a()] = ea;

    ctx.tick(2);
}

fn op_lwzux(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_lwzux");
}

fn op_lwzx(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.d()] = ctx.read_u32(get_ea_x(ctx, instr));

    ctx.tick(2);
}

fn op_psq_l(ctx: &mut Context, instr: Instruction) {
    if !ctx.cpu.hid2.pse() {
        ctx.cpu.exceptions |= EXCEPTION_PROGRAM;
        return;
    }

    if !ctx.cpu.msr.fp() {
        ctx.cpu.exceptions |= EXCEPTION_FPU_UNAVAILABLE;
        return;
    }

    let ea = if instr.a() == 0 {
        sign_ext_12(instr.uimm_1()) as u32
    } else {
        ctx.cpu.gpr[instr.a()].wrapping_add(sign_ext_12(instr.uimm_1()) as u32)
    };

    let gqr = Gqr(ctx.cpu.spr[SPR_GQR0 + instr.i()]);
    let ld_type = gqr.lt();
    let ld_scale = gqr.ls();

    if instr.w() {
        let val = match ld_type {
            QUANTIZE_U8 | QUANTIZE_I8 => ctx.read_u8(ea) as u32,
            QUANTIZE_U16 | QUANTIZE_I16 => ctx.read_u16(ea) as u32,
            _ => ctx.read_u32(ea),
        };

        ctx.cpu.fpr[instr.d()].set_ps0_f64(dequantize(val, ld_type, ld_scale) as f64);
        ctx.cpu.fpr[instr.d()].set_ps1_f64(1.0);
    } else {
        let (val1, val2) = match ld_type {
            QUANTIZE_U8 | QUANTIZE_I8 => (ctx.read_u8(ea) as u32, ctx.read_u8(ea + 1) as u32),
            QUANTIZE_U16 | QUANTIZE_I16 => (ctx.read_u16(ea) as u32, ctx.read_u16(ea + 2) as u32),
            _ => (ctx.read_u32(ea), ctx.read_u32(ea + 3)),
        };
        ctx.cpu.fpr[instr.d()].set_ps0_f64(dequantize(val1, ld_type, ld_scale) as f64);
        ctx.cpu.fpr[instr.d()].set_ps1_f64(dequantize(val2, ld_type, ld_scale) as f64);
    }

    ctx.tick(3);
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
    if !ctx.cpu.hid2.pse() {
        ctx.cpu.exceptions |= EXCEPTION_PROGRAM;
        return;
    }

    if !ctx.cpu.msr.fp() {
        ctx.cpu.exceptions |= EXCEPTION_FPU_UNAVAILABLE;
        return;
    }

    let ea = if instr.a() == 0 {
        sign_ext_12(instr.uimm_1()) as u32
    } else {
        ctx.cpu.gpr[instr.a()].wrapping_add(sign_ext_12(instr.uimm_1()) as u32)
    };

    let gqr = Gqr(ctx.cpu.spr[SPR_GQR0 + instr.i()]);
    let st_type = gqr.st();
    let st_scale = gqr.ls();

    let ps0 = ctx.cpu.fpr[instr.s()].ps0() as f32;
    let ps1 = ctx.cpu.fpr[instr.s()].ps1() as f32;

    if instr.w() {
        match st_type {
            QUANTIZE_U8 | QUANTIZE_I8 => {
                ctx.write_u8(ea, quantize(ps0, st_type, st_scale) as u8);
            }
            QUANTIZE_U16 | QUANTIZE_I16 => {
                ctx.write_u16(ea, quantize(ps0, st_type, st_scale) as u16);
            }
            _ => ctx.write_u32(ea, quantize(ps0, st_type, st_scale)),
        }
    } else {
        // TODO: complete in one write, not two
        match st_type {
            QUANTIZE_U8 | QUANTIZE_I8 => {
                ctx.write_u8(ea, quantize(ps0, st_type, st_scale) as u8);
                ctx.write_u8(ea + 1, quantize(ps1, st_type, st_scale) as u8);
            }
            QUANTIZE_U16 | QUANTIZE_I16 => {
                ctx.write_u16(ea, quantize(ps0, st_type, st_scale) as u16);
                ctx.write_u16(ea + 2, quantize(ps1, st_type, st_scale) as u16);
            }
            _ => {
                ctx.write_u64(
                    ea,
                    (quantize(ps0, st_type, st_scale) as u64) << 32
                        | quantize(ps1, st_type, st_scale) as u64,
                );
            }
        }
    }

    ctx.tick(2);
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
    ctx.write_u8(get_ea(ctx, instr), ctx.cpu.gpr[instr.s()] as u8);

    ctx.tick(2);
}

fn op_stbu(ctx: &mut Context, instr: Instruction) {
    let ea = get_ea_u(ctx, instr);

    ctx.write_u8(ea, ctx.cpu.gpr[instr.s()] as u8);

    ctx.cpu.gpr[instr.a()] = ea;

    ctx.tick(2);
}

fn op_stbux(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_stbux");
}

fn op_stbx(ctx: &mut Context, instr: Instruction) {
    ctx.write_u8(get_ea_x(ctx, instr), ctx.cpu.gpr[instr.s()] as u8);

    ctx.tick(2);
}

fn op_stfd(ctx: &mut Context, instr: Instruction) {
    let ea = get_ea(ctx, instr);

    ctx.write_u64(ea, ctx.cpu.fpr[instr.s()].ps0());

    ctx.tick(2);
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
    let ea = get_ea(ctx, instr);

    let val = ctx.cpu.fpr[instr.s()].ps0();

    ctx.write_u32(ea, convert_to_single(val));

    ctx.tick(2);
}

fn op_stfsu(ctx: &mut Context, instr: Instruction) {
    let ea = get_ea_u(ctx, instr);

    let val = ctx.cpu.fpr[instr.s()].ps0();

    ctx.write_u32(ea, convert_to_single(val));

    ctx.cpu.gpr[instr.a()] = ea;

    ctx.tick(2);
}

fn op_stfsux(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_stfsux");
}

fn op_stfsx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_stfsx");
}

fn op_sth(ctx: &mut Context, instr: Instruction) {
    ctx.write_u16(get_ea(ctx, instr), ctx.cpu.gpr[instr.s()] as u16);

    ctx.tick(2);
}

fn op_sthbrx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_sthbrx");
}

fn op_sthu(ctx: &mut Context, instr: Instruction) {
    let ea = get_ea_u(ctx, instr);

    ctx.write_u16(ea, ctx.cpu.gpr[instr.s()] as u16);

    ctx.cpu.gpr[instr.a()] = ea;

    ctx.tick(2);
}

fn op_sthux(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_sthux");
}

fn op_sthx(ctx: &mut Context, instr: Instruction) {
    ctx.write_u16(get_ea_x(ctx, instr), ctx.cpu.gpr[instr.s()] as u16);

    ctx.tick(2);
}

// FIXME: handle alignment interrupt if ea is not multiple of 4
fn op_stmw(ctx: &mut Context, instr: Instruction) {
    let mut ea = get_ea(ctx, instr);
    let mut r = instr.s();
    let n = (32 - r) as u32;

    while r < 32 {
        ctx.write_u32(ea, ctx.cpu.gpr[r]);

        r += 1;
        ea += 4;
    }

    ctx.tick(2 + n);
}

fn op_stswi(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_stswi");
}

fn op_stswx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_stswx");
}

fn op_stw(ctx: &mut Context, instr: Instruction) {
    //if ctx.cpu.cia == 0x8130_04c4 {
    // TODO: remove this at some point
    // set console type to latest Devkit HW
    //ctx.write_u32(get_ea(ctx, instr), 0x1000_0006);
    //} else {
    ctx.write_u32(get_ea(ctx, instr), ctx.cpu.gpr[instr.s()]);
    //}

    ctx.tick(2);
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

    let ea = get_ea_u(ctx, instr);

    ctx.write_u32(ea, ctx.cpu.gpr[instr.s()]);

    ctx.cpu.gpr[instr.a()] = ea;

    ctx.tick(2);
}

fn op_stwux(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_stwux");
}

fn op_stwx(ctx: &mut Context, instr: Instruction) {
    let ea = get_ea_x(ctx, instr);

    ctx.write_u32(ea, ctx.cpu.gpr[instr.s()]);

    ctx.tick(2);
}

fn op_tlbie(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_tlbie");
}
