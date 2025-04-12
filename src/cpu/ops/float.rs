use crate::cpu::float::Nan;
use crate::cpu::instruction::Instruction;
use crate::cpu::EXCEPTION_FPU_UNAVAILABLE;
use crate::Context;

pub fn op_fabsx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fabsx");
}

pub fn op_faddsx(ctx: &mut Context, instr: Instruction) {
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

pub fn op_faddx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_faddx");
}

pub fn op_fcmpo(ctx: &mut Context, instr: Instruction) {
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

pub fn op_fcmpu(ctx: &mut Context, instr: Instruction) {
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

pub fn op_fctiwzx(ctx: &mut Context, instr: Instruction) {
    let frb = ctx.cpu.fpr[instr.b()].ps0_as_f64();

    // TODO: implement more accurate conversion
    let result = ((frb as i32) as u32) as u64;

    ctx.cpu.fpr[instr.d()].set_ps0(result);

    if instr.rc() {
        ctx.cpu.update_cr1();
    }

    ctx.tick(1);
}

pub fn op_fctiwx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fctiwx");
}

pub fn op_fdivsx(ctx: &mut Context, instr: Instruction) {
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

pub fn op_fdivx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fdivx");
}

pub fn op_fmaddsx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fmaddsx");
}

pub fn op_fmaddx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fmaddx");
}

// FIXME: Verify paired single functionality with HID2[PSE] value
pub fn op_fmrx(ctx: &mut Context, instr: Instruction) {
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

pub fn op_fmsubsx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fmsubsx");
}

pub fn op_fmsubx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fmsubx");
}

pub fn op_fmulsx(ctx: &mut Context, instr: Instruction) {
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

pub fn op_fmulx(ctx: &mut Context, instr: Instruction) {
    if !ctx.cpu.msr.fp() {
        ctx.cpu.exceptions |= EXCEPTION_FPU_UNAVAILABLE;
        return;
    }

    let fra = ctx.cpu.fpr[instr.a()].ps0_as_f64();
    let frc = ctx.cpu.fpr[instr.c()].ps0_as_f64();

    let result = fra * frc;

    ctx.cpu.fpr[instr.d()].set_ps0_f64(result);

    if instr.rc() {
        ctx.cpu.update_cr1();
    }
}

pub fn op_fnabsx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fnabsx");
}

pub fn op_fnegx(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.fpr[instr.d()].set_ps0(ctx.cpu.fpr[instr.b()].ps0() | (1_u64 << 63));

    ctx.tick(1);
}

pub fn op_fnmaddsx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fnmaddsx");
}

pub fn op_fnmaddx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fnmaddx");
}

pub fn op_fnmsubsx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fnsubsx");
}

pub fn op_fnmsubx(ctx: &mut Context, instr: Instruction) {
    if !ctx.cpu.msr.fp() {
        ctx.cpu.exceptions |= EXCEPTION_FPU_UNAVAILABLE;
        return;
    }

    let fra = ctx.cpu.fpr[instr.a()].ps0_as_f64();
    let frb = ctx.cpu.fpr[instr.b()].ps0_as_f64();
    let frc = ctx.cpu.fpr[instr.c()].ps0_as_f64();

    let result = fra.mul_add(frc, -frb);

    ctx.cpu.fpr[instr.d()].set_ps0_f64(result);

    if instr.rc() {
        ctx.cpu.update_cr1();
    }

    ctx.tick(2);
}

pub fn op_fresx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fresx");
}

pub fn op_frspx(ctx: &mut Context, instr: Instruction) {
    if !ctx.cpu.msr.fp() {
        ctx.cpu.exceptions |= EXCEPTION_FPU_UNAVAILABLE;
        return;
    }

    let frb = ctx.cpu.fpr[instr.b()].ps0_as_f64();

    if frb.is_nan() {
        unimplemented!();
    }

    ctx.cpu.fpr[instr.d()].set_ps0_f64(frb);

    if ctx.cpu.hid2.pse() {
        ctx.cpu.fpr[instr.d()].set_ps1_f64(frb);
    }

    if instr.rc() {
        ctx.cpu.update_cr1();
    }
}

pub fn op_frsqrtex(ctx: &mut Context, instr: Instruction) {
    let frb = ctx.cpu.fpr[instr.b()].ps0_as_f64();

    ctx.cpu.fpr[instr.d()].set_ps0_f64(1.0 / frb.sqrt());

    if instr.rc() {
        ctx.cpu.update_cr1();
    }

    ctx.tick(2);
}

pub fn op_fselx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fselx");
}

pub fn op_fsubsx(ctx: &mut Context, instr: Instruction) {
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

pub fn op_ps_absx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_absx");
}

pub fn op_ps_addx(ctx: &mut Context, instr: Instruction) {
    let fra = ctx.cpu.fpr[instr.a()].ps0_as_f64();
    let frb = ctx.cpu.fpr[instr.b()].ps0_as_f64();

    ctx.cpu.fpr[instr.d()].set_ps0_f64(fra + frb);

    let fra = ctx.cpu.fpr[instr.a()].ps1_as_f64();
    let frb = ctx.cpu.fpr[instr.b()].ps1_as_f64();

    ctx.cpu.fpr[instr.d()].set_ps1_f64(fra + frb);

    if instr.rc() {
        ctx.cpu.update_cr1();
    }

    ctx.tick(1);
}

pub fn op_ps_cmpo0(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_cmpo0");
}

pub fn op_ps_cmpo1(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_cmpo1");
}

pub fn op_ps_cmpu0(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_cmpu0");
}

pub fn op_ps_cmpu1(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_cmpu1");
}

pub fn op_ps_divx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_divx");
}

pub fn op_ps_maddx(ctx: &mut Context, instr: Instruction) {
    let fra = ctx.cpu.fpr[instr.a()].ps0_as_f64();
    let frb = ctx.cpu.fpr[instr.b()].ps0_as_f64();
    let frc = ctx.cpu.fpr[instr.c()].ps0_as_f64();

    ctx.cpu.fpr[instr.d()].set_ps0_f64(fra.mul_add(frc, frb));

    let fra = ctx.cpu.fpr[instr.a()].ps1_as_f64();
    let frb = ctx.cpu.fpr[instr.b()].ps1_as_f64();
    let frc = ctx.cpu.fpr[instr.c()].ps1_as_f64();

    ctx.cpu.fpr[instr.d()].set_ps1_f64(fra.mul_add(frc, frb));

    if instr.rc() {
        ctx.cpu.update_cr1();
    }

    ctx.tick(1);
}

pub fn op_ps_madds0x(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_madds0x");
}

pub fn op_ps_madds1x(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_madds1x");
}

pub fn op_ps_merge00x(ctx: &mut Context, instr: Instruction) {
    if !ctx.cpu.msr.fp() {
        ctx.cpu.exceptions |= EXCEPTION_FPU_UNAVAILABLE;
        return;
    }

    let fra = ctx.cpu.fpr[instr.a()].ps0();
    let frb = ctx.cpu.fpr[instr.b()].ps0();

    ctx.cpu.fpr[instr.d()].set_ps0(fra);
    ctx.cpu.fpr[instr.d()].set_ps1(frb);

    if instr.rc() {
        ctx.cpu.update_cr1();
    }

    ctx.tick(1);
}

pub fn op_ps_merge01x(ctx: &mut Context, instr: Instruction) {
    if !ctx.cpu.msr.fp() {
        ctx.cpu.exceptions |= EXCEPTION_FPU_UNAVAILABLE;
        return;
    }

    let fra = ctx.cpu.fpr[instr.a()].ps0();
    let frb = ctx.cpu.fpr[instr.b()].ps1();

    ctx.cpu.fpr[instr.d()].set_ps0(fra);
    ctx.cpu.fpr[instr.d()].set_ps1(frb);

    if instr.rc() {
        ctx.cpu.update_cr1();
    }

    ctx.tick(1);
}

pub fn op_ps_merge10x(ctx: &mut Context, instr: Instruction) {
    if !ctx.cpu.msr.fp() {
        ctx.cpu.exceptions |= EXCEPTION_FPU_UNAVAILABLE;
        return;
    }

    let fra = ctx.cpu.fpr[instr.a()].ps1();
    let frb = ctx.cpu.fpr[instr.b()].ps0();

    ctx.cpu.fpr[instr.d()].set_ps0(fra);
    ctx.cpu.fpr[instr.d()].set_ps1(frb);

    if instr.rc() {
        ctx.cpu.update_cr1();
    }

    ctx.tick(1);
}

pub fn op_ps_merge11x(ctx: &mut Context, instr: Instruction) {
    if !ctx.cpu.msr.fp() {
        ctx.cpu.exceptions |= EXCEPTION_FPU_UNAVAILABLE;
        return;
    }

    let fra = ctx.cpu.fpr[instr.a()].ps1();
    let frb = ctx.cpu.fpr[instr.b()].ps1();

    ctx.cpu.fpr[instr.d()].set_ps0(fra);
    ctx.cpu.fpr[instr.d()].set_ps1(frb);

    if instr.rc() {
        ctx.cpu.update_cr1();
    }

    ctx.tick(1);
}

pub fn op_ps_mrx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_mrx");
}

pub fn op_ps_msubx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_msubx");
}

pub fn op_ps_mulx(ctx: &mut Context, instr: Instruction) {
    let fra = ctx.cpu.fpr[instr.a()].ps0_as_f64();
    let frc = ctx.cpu.fpr[instr.c()].ps0_as_f64();

    let result = fra * frc;

    ctx.cpu.fpr[instr.d()].set_ps0_f64(result);

    if instr.rc() {
        ctx.cpu.update_cr1();
    }

    ctx.tick(2);
}

pub fn op_ps_muls0x(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_muls0x");
}

pub fn op_ps_muls1x(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_muls1x");
}

pub fn op_ps_nabsx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_nabsx");
}

pub fn op_ps_negx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_negx");
}

pub fn op_ps_nmaddx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_nmaddx");
}

pub fn op_ps_nmsubx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_nmsubx");
}

pub fn op_ps_resx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_resx");
}

pub fn op_ps_rsqrtex(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_rsqrtex");
}

pub fn op_ps_selx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_selx");
}

pub fn op_ps_subx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_subx");
}

pub fn op_ps_sum0x(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_sum0x");
}

pub fn op_ps_sum1x(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_ps_sum1x");
}

pub fn op_fsubx(ctx: &mut Context, instr: Instruction) {
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

pub fn op_mcrfs(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_mcrfs");
}

pub fn op_mffsx(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.fpr[instr.d()].set_ps0(ctx.cpu.fpscr.0 as u64);

    if instr.rc() {
        ctx.cpu.update_cr1();
    }

    ctx.tick(1);
}

// TODO: test this implementation
pub fn op_mtfsb0x(ctx: &mut Context, instr: Instruction) {
    let b = 0x8000_0000_u32 >> instr.crbd();

    ctx.cpu.fpscr.0 &= !b;

    if instr.rc() {
        ctx.cpu.update_cr1();
    }

    ctx.tick(3);
}

// TODO: test this implementation
pub fn op_mtfsb1x(ctx: &mut Context, instr: Instruction) {
    let b = 0x8000_0000_u32 >> instr.crbd();

    ctx.cpu.fpscr.0 |= b;

    if instr.rc() {
        ctx.cpu.update_cr1();
    }

    ctx.tick(3);
}

pub fn op_mtfsfix(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_mtfsfix");
}

// TODO: test this implementation
pub fn op_mtfsfx(ctx: &mut Context, instr: Instruction) {
    let (mut m, mut i) = (0, 7);
    let fm = instr.fm();

    while i >= 0 {
        if (fm >> i) & 1 != 0 {
            m |= 0xF;
        }
        m <<= 4;
        i -= 1;
    }

    ctx.cpu.fpscr.0 = (ctx.cpu.fpscr.0 & !m) | (ctx.cpu.fpr[instr.b()].ps0() as u32 & m);

    if instr.rc() {
        ctx.cpu.update_cr1();
    }
}
