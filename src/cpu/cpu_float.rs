fn op_fabsx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fabsx");
}

fn op_faddsx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_faddsx");
}

fn op_faddx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_faddx");
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

fn op_fctiwx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fctiwx");
}

fn op_fdivsx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fdivsx");
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

fn op_fmrx(ctx: &mut Context, instr: Instruction) {
    // This is wrong, assuming frB is is paired single every time
    ctx.cpu.fpr[instr.d()].set_ps0(ctx.cpu.fpr[instr.b()].ps0())

    ctx.tick(3);
}

fn op_fmsubsx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fmsubsx");
}

fn op_fmsubx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fmsubx");
}

fn op_fmulsx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fmulsx");
}

fn op_fmulx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fmulx");
}

fn op_fnabsx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fnabsx");
    //    self.fpr[instr.d()] = self.fpr[instr.b()] | (1 << 63);

    //    if instr.rc() {
    //        self.cr.update_cr1(self.fpr[instr.d()], &self.fpscr);
    //    }
}

fn op_fnegx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fnegx");
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

fn op_fsubsx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_fsubsx");
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
    //    self.fpscr.set_bit(instr.crbd(), true);

    //    if instr.rc() {
    //        panic!("RC: mtfsb1x");
    //    }
}

fn op_mtfsfix(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_mtfsfix");
}

fn op_mtfsfx(_ctx: &mut Context, _instr: Instruction) {
    println!("FixMe: mtfsfx");
}
