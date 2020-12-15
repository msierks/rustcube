fn op_faddsx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_faddsx");
}

fn op_fcmpo(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fcmpo");
}

fn op_fcmpu(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fcmpu");
}

fn op_fctiwzx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fctiwzx");
}

fn op_fdivsx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fdivsx");
}

fn op_fmrx(ctx: &mut Context, instr: Instruction) {
    // FixMe: this is wrong
    if ctx.cpu.hid2.pse() {
        ctx.cpu.fpr[instr.d()].set_ps0(ctx.cpu.fpr[instr.b()].ps0());
    } else {
        ctx.cpu.fpr[instr.d()] = ctx.cpu.fpr[instr.b()];
    }

    if instr.rc() {
        panic!("RC: fmrx");
    }
}

fn op_fmulsx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fmulsx");
}

fn op_fmulx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fmulx");
}

fn op_fnabsx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fnabsx");
}

fn op_fnegx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fnegx");
}

fn op_frspx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_frspx");
}

fn op_fsubsx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fsubsx");
}

fn op_fsubx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fsubx");
}

fn op_mtfsb1x(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_mtfsb1x");
}

fn op_mtfsfx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_mtfsfx");
}
