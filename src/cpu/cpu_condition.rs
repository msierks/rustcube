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
