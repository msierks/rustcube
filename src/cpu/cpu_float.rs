fn op_faddsx(_ctx: &mut Context, _instr: Instruction) {
    println!("FixMe: faddsx");
}

fn op_fcmpo(_ctx: &mut Context, _instr: Instruction) {
    println!("FixMe: fcmpo");
}

fn op_fcmpu(_ctx: &mut Context, _instr: Instruction) {
    println!("FixMe: fcmpu");
}

fn op_fctiwzx(_ctx: &mut Context, _instr: Instruction) {
    println!("FixMe: fctiwzx");
}

fn op_fdivsx(_ctx: &mut Context, _instr: Instruction) {
    println!("FixMe: fdivsx");
}

fn op_fmrx(_ctx: &mut Context, _instr: Instruction) {
    //    self.fpr[instr.d()] = self.fpr[instr.b()];

    //    if instr.rc() {
    //        self.cr.update_cr1(self.fpr[instr.d()], &self.fpscr);
    //    }
}

fn op_fmulsx(_ctx: &mut Context, _instr: Instruction) {
    println!("FixMe: fmulsx");
}

fn op_fmulx(_ctx: &mut Context, _instr: Instruction) {
    println!("FixMe: fmulx");
}

fn op_fnabsx(_ctx: &mut Context, _instr: Instruction) {
    //    self.fpr[instr.d()] = self.fpr[instr.b()] | (1 << 63);

    //    if instr.rc() {
    //        self.cr.update_cr1(self.fpr[instr.d()], &self.fpscr);
    //    }
}

fn op_fnegx(_ctx: &mut Context, _instr: Instruction) {
    println!("FixMe: fnegx");
}

fn op_frspx(_ctx: &mut Context, _instr: Instruction) {
    println!("FixMe: frspx");
}

fn op_fsubsx(_ctx: &mut Context, _instr: Instruction) {
    println!("FixMe: fsubsx");
}

fn op_fsubx(_ctx: &mut Context, instr: Instruction) {
    println!("FixMe: fsubx");

    if instr.rc() {
        panic!("RC: fsubx");
    }
}

fn op_mtfsb1x(_ctx: &mut Context, _instr: Instruction) {
    //    self.fpscr.set_bit(instr.crbd(), true);

    //    if instr.rc() {
    //        panic!("RC: mtfsb1x");
    //    }
}

fn op_mtfsfx(_ctx: &mut Context, _instr: Instruction) {
    println!("FixMe: mtfsfx");
}
