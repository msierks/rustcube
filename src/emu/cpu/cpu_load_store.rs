fn get_ea(ctx: &Context, instr: Instruction) -> u32 {
    ctx.cpu.gpr[instr.a()].wrapping_add(i32::from(instr.simm()) as u32)
}

fn get_ea_u(ctx: &Context, instr: Instruction) -> u32 {
    ctx.cpu.gpr[instr.a()].wrapping_add(i32::from(instr.simm()) as u32)
}

fn op_dcbf(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_dcbf");
}

fn op_dcbi(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_dcbi");
}

fn op_icbi(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_icbi");
}

fn op_lbz(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_lbz");
}

fn op_lbzu(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_lbzu");
}

fn op_lbzx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_lbzx");
}

fn op_lfd(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_lfd");
}

fn op_lfs(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_lfs");
}

fn op_lha(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_lha");
}

fn op_lhz(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_lhz");
}

fn op_lhzu(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_lhzu");
}

fn op_lmw(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_lmw");
}

fn op_lwz(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.d()] = ctx.read_u32(get_ea(ctx, instr));
}

fn op_lwzx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_lwzx");
}

fn op_lwzu(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_lwzu");
}

fn op_psq_l(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_psq_l");
}

fn op_psq_st(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_psq_st");
}

fn op_stb(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_stb");
}

fn op_stbx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_stbx");
}

fn op_stbu(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_stbu");
}

fn op_stfd(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_stfd");
}

fn op_stfs(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_stfs");
}

fn op_stfsu(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_stfsu");
}

fn op_sth(ctx: &mut Context, instr: Instruction) {
    ctx.write_u16(get_ea(ctx, instr), ctx.cpu.gpr[instr.s()] as u16);
}

fn op_sthu(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_sthu");
}

fn op_stmw(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_stmw");
}

fn op_stw(ctx: &mut Context, instr: Instruction) {
    //if ctx.cpu.pc == 0x8130_04c4 {
    //    ctx.write_u32(get_ea(ctx, instr), 0x1000_0006);
    //} else {
    ctx.write_u32(get_ea(ctx, instr), ctx.cpu.gpr[instr.s()]);
    //}
}

fn op_stwx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_stwx");
}

fn op_stwu(ctx: &mut Context, instr: Instruction) {
    if instr.a() == 0 {
        panic!("stwu: invalid instruction");
    }

    let ea = get_ea_u(ctx, instr);

    ctx.write_u32(ea, ctx.cpu.gpr[instr.s()]);

    ctx.cpu.gpr[instr.a()] = ea;
}
