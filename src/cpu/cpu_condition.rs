fn op_crand(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_crand");
}

fn op_crandc(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_crandc");
}

fn op_creqv(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_creqv");
}

fn op_crnand(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_crnand");
}

fn op_crnor(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_crnor");
}

fn op_cror(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_cror");
}

fn op_crorc(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_crorc");
}

fn op_crxor(ctx: &mut Context, instr: Instruction) {
    let d = ctx.cpu.cr.get_bit(instr.a()) ^ ctx.cpu.cr.get_bit(instr.b());

    ctx.cpu.cr.set_bit(instr.d(), d);
}

fn op_mcrf(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_mcrf");
}

fn op_mcrxr(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_mcrxr");
}

fn op_mfcr(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.d()] = ctx.cpu.cr.as_u32();
}

fn op_mtcrf(ctx: &mut Context, instr: Instruction) {
    let crm = instr.crm();

    if crm == 0xFF {
        ctx.cpu.cr.set(ctx.cpu.gpr[instr.s()]);
    } else {
        unimplemented!();
    }
}
