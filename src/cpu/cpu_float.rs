fn op_fabsx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fabsx");
}

fn op_faddsx(ctx: &mut Context, instr: Instruction) {
    if !ctx.cpu.msr.fp() {
        ctx.cpu.exceptions |= EXCEPTION_FPU_UNAVAILABLE;
        return;
    }

    let fra = ctx.cpu.fpr[instr.b()].ps0_as_f64();
    let frb = ctx.cpu.fpr[instr.b()].ps0_as_f64();

    let result = fra + frb;

    ctx.cpu.fpr[instr.d()].set_ps0_f64(result);

    if ctx.cpu.hid2.pse() {
        ctx.cpu.fpr[instr.d()].set_ps1_f64(result);
    }

    if instr.rc() {
        ctx.cpu.update_cr1();
    }

    ctx.tick(1);
}

fn op_faddx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_faddx");
}

fn op_fcmpo(ctx: &mut Context, instr: Instruction) {
    if !ctx.cpu.msr.fp() {
        ctx.cpu.exceptions |= EXCEPTION_FPU_UNAVAILABLE;
        return;
    }

    let fra = ctx.cpu.fpr[instr.a()].ps0_as_f64();
    let frb = ctx.cpu.fpr[instr.b()].ps0_as_f64();

    let c = if fra.is_nan() || frb.is_nan() {
        if fra.is_snan() || frb.is_snan() {
            ctx.cpu.fpscr.set_vxsnan(true);
            if !ctx.cpu.fpscr.ve() {
                ctx.cpu.fpscr.set_vxvc(true);
            }
        } else {
            ctx.cpu.fpscr.set_vxsnan(true);
        }
        0b1 // ?
    } else if fra < frb {
        0x8 // <
    } else if fra > frb {
        0x4 // >
    } else {
        0x2 // =
    };

    ctx.cpu.fpscr.set_fpcc(c);

    ctx.cpu.cr.set_field(instr.crfd(), c);

    ctx.tick(1);
}

fn op_fcmpu(ctx: &mut Context, instr: Instruction) {
    if !ctx.cpu.msr.fp() {
        ctx.cpu.exceptions |= EXCEPTION_FPU_UNAVAILABLE;
        return;
    }

    let fra = ctx.cpu.fpr[instr.a()].ps0_as_f64();
    let frb = ctx.cpu.fpr[instr.b()].ps0_as_f64();

    let c = if fra.is_nan() || frb.is_nan() {
        if fra.is_snan() || frb.is_snan() {
            ctx.cpu.fpscr.set_vxsnan(true);
        }
        0b1 // ?
    } else if fra < frb {
        0x8 // <
    } else if fra > frb {
        0x4 // >
    } else {
        0x2 // =
    };

    ctx.cpu.fpscr.set_fpcc(c);

    ctx.cpu.cr.set_field(instr.crfd(), c);

    ctx.tick(1);
}

fn op_fctiwzx(ctx: &mut Context, instr: Instruction) {
    let frb = ctx.cpu.fpr[instr.b()].ps0_as_f64();

    // TODO: implement more accurate conversion
    let result = ((frb as i32) as u32) as u64;

    ctx.cpu.fpr[instr.d()].set_ps0(result);

    if instr.rc() {
        ctx.cpu.update_cr1();
    }

    ctx.tick(1);
}

fn op_fctiwx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fctiwx");
}

fn op_fdivsx(ctx: &mut Context, instr: Instruction) {
    let fra = ctx.cpu.fpr[instr.a()].ps0_as_f64();
    let frb = ctx.cpu.fpr[instr.b()].ps0_as_f64();

    let result = fra / frb;

    if frb.is_nan() {
        panic!();
    }

    ctx.cpu.fpr[instr.d()].set_ps0_f64(result);

    if ctx.cpu.hid2.pse() {
        ctx.cpu.fpr[instr.d()].set_ps1_f64(result);
    }

    if instr.rc() {
        ctx.cpu.update_cr1();
    }

    ctx.tick(17);
}

fn op_fdivx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fdivx");
}

fn op_fmaddsx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fmaddsx");
}

fn op_fmaddx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fmaddx");
}

// FIXME: Verify paired single functionality with HID2[PSE] value
fn op_fmrx(ctx: &mut Context, instr: Instruction) {
    if !ctx.cpu.msr.fp() {
        ctx.cpu.exceptions |= EXCEPTION_FPU_UNAVAILABLE;
        return;
    }

    let frb = ctx.cpu.fpr[instr.b()].ps0();

    ctx.cpu.fpr[instr.d()].set_ps0(frb);

    if ctx.cpu.hid2.pse() {
        ctx.cpu.fpr[instr.d()].set_ps1(frb);
    }

    if instr.rc() {
        ctx.cpu.update_cr1();
    }

    ctx.tick(1);
}

fn op_fmsubsx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fmsubsx");
}

fn op_fmsubx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fmsubx");
}

fn op_fmulsx(ctx: &mut Context, instr: Instruction) {
    if !ctx.cpu.msr.fp() {
        ctx.cpu.exceptions |= EXCEPTION_FPU_UNAVAILABLE;
        return;
    }

    let result = ctx.cpu.fpr[instr.a()].ps0_as_f64() * ctx.cpu.fpr[instr.c()].ps0_as_f64();

    ctx.cpu.fpr[instr.d()].set_ps0_f64(result);

    if ctx.cpu.hid2.pse() {
        ctx.cpu.fpr[instr.d()].set_ps1_f64(result);
    }

    if instr.rc() {
        ctx.cpu.update_cr1();
    }

    ctx.tick(1);
}

fn op_fmulx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fmulx");
}

fn op_fnabsx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fnabsx");
}

fn op_fnegx(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.fpr[instr.d()].set_ps0(ctx.cpu.fpr[instr.b()].ps0() | 1_u64 << 63);

    ctx.tick(1);
}

fn op_fnmaddsx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fnmaddsx");
}

fn op_fnmaddx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fnmaddx");
}

fn op_fnmsubsx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fnsubsx");
}

fn op_fnmsubx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fnmsubx");
}

fn op_fresx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fresx");
}

fn op_frspx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_frspx");
}

fn op_frsqrtex(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_frsqrtex");
}

fn op_fselx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fselx");
}

fn op_fsubsx(ctx: &mut Context, instr: Instruction) {
    if !ctx.cpu.msr.fp() {
        ctx.cpu.exceptions |= EXCEPTION_FPU_UNAVAILABLE;
        return;
    }

    let result = ctx.cpu.fpr[instr.a()].ps0_as_f64() - ctx.cpu.fpr[instr.b()].ps0_as_f64();

    ctx.cpu.fpr[instr.d()].set_ps0_f64(result);

    if ctx.cpu.hid2.pse() {
        ctx.cpu.fpr[instr.d()].set_ps1_f64(result);
    }

    if instr.rc() {
        ctx.cpu.update_cr1();
    }

    ctx.tick(1);
}

fn op_ps_absx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_absx");
}

fn op_ps_addx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_addx");
}

fn op_ps_cmpo0(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_cmpo0");
}

fn op_ps_cmpo1(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_cmpo1");
}

fn op_ps_cmpu0(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_cmpu0");
}

fn op_ps_cmpu1(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_cmpu1");
}

fn op_ps_divx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_divx");
}

fn op_ps_maddx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_maddx");
}

fn op_ps_madds0x(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_madds0x");
}

fn op_ps_madds1x(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_madds1x");
}

fn op_ps_merge00x(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_merge00x");
}

fn op_ps_merge01x(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_merge01x");
}

fn op_ps_merge10x(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_merge10x");
}

fn op_ps_merge11x(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_merge11x");
}

fn op_ps_mrx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_mrx");
}

fn op_ps_msubx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_msubx");
}

fn op_ps_mulx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fsel");
}

fn op_ps_muls0x(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_muls0x");
}

fn op_ps_muls1x(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_muls1x");
}

fn op_ps_nabsx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_nabsx");
}

fn op_ps_negx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_negx");
}

fn op_ps_nmaddx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_nmaddx");
}

fn op_ps_nmsubx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_nmsubx");
}

fn op_ps_resx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_resx");
}

fn op_ps_rsqrtex(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_rsqrtex");
}

fn op_ps_selx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_selx");
}

fn op_ps_subx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_subx");
}

fn op_ps_sum0x(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_sum0x");
}

fn op_ps_sum1x(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_sum1x");
}

fn op_fsubx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fsubx");
}

fn op_mcrfs(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_mcrfs");
}

fn op_mffsx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_mffsx");
}

fn op_mtfsb0x(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_mtfsb0x");
}

fn op_mtfsb1x(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_mtfsb1x");
}

fn op_mtfsfix(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_mtfsfix");
}

fn op_mtfsfx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_mtfsfx");
}
