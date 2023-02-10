fn op_bx(ctx: &mut Context, instr: Instruction) {
    if instr.aa() == 1 {
        ctx.cpu.nia = sign_ext_26(instr.li() << 2) as u32;
    } else {
        ctx.cpu.nia = ctx
            .cpu
            .cia
            .wrapping_add(sign_ext_26(instr.li() << 2) as u32);
    }

    if instr.lk() != 0 {
        ctx.cpu.spr[SPR_LR] = ctx.cpu.cia.wrapping_add(4);
    }

    ctx.tick(1);
}

fn op_bcx(ctx: &mut Context, instr: Instruction) {
    let bo = instr.bo();

    let ctr_ok = if bon(bo, 2) == 0 {
        ctx.cpu.spr[SPR_CTR] = ctx.cpu.spr[SPR_CTR].wrapping_sub(1);

        if bon(bo, 3) != 0 {
            ctx.cpu.spr[SPR_CTR] == 0
        } else {
            ctx.cpu.spr[SPR_CTR] != 0
        }
    } else {
        true
    };

    let cond_ok = if bon(bo, 0) == 0 {
        bon(bo, 1) == ctx.cpu.cr.get_bit(instr.bi())
    } else {
        true
    };

    //println!("BO: {} ctr_ok: {} cond_ok: {}", bo, ctr_ok, cond_ok);

    if ctr_ok && cond_ok {
        if instr.aa() == 1 {
            ctx.cpu.nia = sign_ext_16(instr.bd() << 2) as u32;
        } else {
            ctx.cpu.nia = ctx
                .cpu
                .cia
                .wrapping_add(sign_ext_16(instr.bd() << 2) as u32);
        }

        if instr.lk() != 0 {
            ctx.cpu.spr[SPR_LR] = ctx.cpu.cia.wrapping_add(4);
        }
    }

    ctx.tick(1);
}

fn op_bcctrx(ctx: &mut Context, instr: Instruction) {
    let bo = instr.bo();

    let cond_ok = if bon(bo, 0) == 0 {
        ctx.cpu.cr.get_bit(instr.bi()) == bon(bo, 1)
    } else {
        true
    };

    if cond_ok {
        ctx.cpu.nia = ctx.cpu.spr[SPR_CTR] & 0xFFFF_FFFC;

        if instr.lk() != 0 {
            ctx.cpu.spr[SPR_LR] = ctx.cpu.cia + 4;
        }
    }

    ctx.tick(1);
}

fn op_bclrx(ctx: &mut Context, instr: Instruction) {
    let bo = instr.bo();

    let ctr_ok = if bon(bo, 2) == 0 {
        ctx.cpu.spr[SPR_CTR] = ctx.cpu.spr[SPR_CTR].wrapping_sub(1);

        if bon(bo, 3) != 0 {
            ctx.cpu.spr[SPR_CTR] == 0
        } else {
            ctx.cpu.spr[SPR_CTR] != 0
        }
    } else {
        true
    };

    let cond_ok = if bon(bo, 0) == 0 {
        bon(bo, 1) == ctx.cpu.cr.get_bit(instr.bi())
    } else {
        true
    };

    if ctr_ok && cond_ok {
        ctx.cpu.nia = ctx.cpu.spr[SPR_LR] & 0xFFFF_FFFC;

        if instr.lk() != 0 {
            ctx.cpu.spr[SPR_LR] = ctx.cpu.cia.wrapping_add(4);
        }
    }

    ctx.tick(1);
}
