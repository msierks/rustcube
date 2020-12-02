fn op_addi(ctx: &mut Context, instr: Instruction) {
    if instr.a() == 0 {
        ctx.cpu.gpr[instr.d()] = i32::from(instr.simm()) as u32;
    } else {
        ctx.cpu.gpr[instr.d()] =
            ctx.cpu.gpr[instr.a()].wrapping_add(i32::from(instr.simm()) as u32);
    }
}

fn op_addic(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_addic");
}

fn op_addic_rc(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_addic_rc");
}

fn op_addis(ctx: &mut Context, instr: Instruction) {
    if instr.a() == 0 {
        ctx.cpu.gpr[instr.d()] = instr.uimm() << 16;
    } else {
        ctx.cpu.gpr[instr.d()] = ctx.cpu.gpr[instr.a()].wrapping_add(instr.uimm() << 16);
    }
}

fn op_addex(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_addex");
}

fn op_addzex(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_addzex");
}

fn op_addx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_addx");
}

fn op_addcx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_addcx");
}

fn op_andx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_andx");
}

fn op_andcx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_andcx");
}

fn op_andi_rc(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_andi_rc");
}

fn op_cmp(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_cmp");
}

fn op_cmpi(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_cmpi");
}

fn op_cmpl(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_cmpl");
}

fn op_cmpli(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_cmpli");
}

fn op_cntlzwx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_cntlzwx");
}

fn op_divwx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_divwx");
}

fn op_divwux(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_divwux");
}

fn op_extsbx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_extsbx");
}

fn op_extshx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_extshx");
}

fn op_mulhwux(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_mulhwux");
}

fn op_mulli(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_mulli");
}

fn op_mullwx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_mullwx");
}

fn op_negx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_negx");
}

fn op_norx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_norx");
}

fn op_orx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_orx");
}

fn op_ori(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.a()] = ctx.cpu.gpr[instr.s()] | instr.uimm();
}

fn op_oris(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_oris");
}

fn op_rlwimix(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_rlwimix");
}

fn op_rlwinmx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_rlwinmx");
}

fn op_slwx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_slwx");
}

fn op_srawx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_srawx");
}

fn op_srawix(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_srawix");
}

fn op_srwx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_srwx");
}

fn op_subfx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_subfx");
}

fn op_subfcx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_subfcx");
}

fn op_subfex(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_subfex");
}

fn op_subfic(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_subfic");
}

fn op_subfzex(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_subfzex");
}

fn op_xorx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_xorx");
}

fn op_xoris(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_xoris");
}
