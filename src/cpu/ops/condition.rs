use crate::cpu::instruction::Instruction;
use crate::Context;

pub fn op_crand(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_crand");
}

pub fn op_crandc(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_crandc");
}

pub fn op_creqv(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_creqv");
}

pub fn op_crnand(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_crnand");
}

pub fn op_crnor(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_crnor");
}

pub fn op_cror(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_cror");
}

pub fn op_crorc(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_crorc");
}

pub fn op_crxor(ctx: &mut Context, instr: Instruction) {
    let d = ctx.cpu.cr.get_bit(instr.a()) ^ ctx.cpu.cr.get_bit(instr.b());

    ctx.cpu.cr.set_bit(instr.d(), d);

    ctx.tick(1);
}

pub fn op_mcrf(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_mcrf");
}

pub fn op_mcrxr(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_mcrxr");
}

pub fn op_mfcr(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.d()] = ctx.cpu.cr.as_u32();

    ctx.tick(1);
}

pub fn op_mtcrf(ctx: &mut Context, instr: Instruction) {
    let crm = instr.crm();

    if crm == 0xFF {
        ctx.cpu.cr.set(ctx.cpu.gpr[instr.s()]);
    } else {
        unimplemented!("op_mtcrf crm != 0xFF");
    }

    ctx.tick(1);
}
