fn op_bx(ctx: &mut Context, instr: Instruction) {
    if instr.aa() != 0 {
        ctx.cpu.npc = sign_ext_26(instr.li() << 2) as u32;
    } else {
        ctx.cpu.npc = ctx.cpu.pc.wrapping_add(sign_ext_26(instr.li() << 2) as u32);
    }

    if instr.lk() != 0 {
        ctx.cpu.spr[SPR_LR] = ctx.cpu.pc.wrapping_add(4);
    }
}

fn op_bcx(ctx: &mut Context, instr: Instruction) {
    let bo = instr.bo();
    if bo & 0x4 == 0 {
        ctx.cpu.spr[SPR_CTR] = ctx.cpu.spr[SPR_CTR].wrapping_sub(1);
    }

    let ctr_ok = (bo >> 2) & 1 != 0 || (((ctx.cpu.spr[SPR_CTR] != 0) as u8 ^ (bo >> 1)) & 1) != 0;
    let cond_ok = (bo >> 4) & 1 != 0 || (ctx.cpu.cr.get_bit(instr.bi()) == (bo >> 3) & 1);

    if ctr_ok && cond_ok {
        if instr.lk() != 0 {
            ctx.cpu.spr[SPR_LR] = ctx.cpu.pc + 4;
        }

        if instr.aa() != 0 {
            ctx.cpu.npc = sign_ext_16(instr.bd() << 2) as u32;
        } else {
            ctx.cpu.npc = ctx.cpu.pc.wrapping_add(sign_ext_16(instr.bd() << 2) as u32);
        }
    }
}

fn op_bcctrx(ctx: &mut Context, instr: Instruction) {
    let bo = instr.bo();

    if bo & 0x4 == 0 {
        panic!("bcctrx: Invalid instruction, BO[2] = 0");
    }

    let cond_ok = ((bo >> 4) | (ctx.cpu.cr.get_bit(instr.bi()) == ((bo >> 3) & 1)) as u8) & 1;

    if cond_ok != 0 {
        if instr.lk() != 0 {
            ctx.cpu.spr[SPR_LR] = ctx.cpu.pc.wrapping_add(4);
        }

        ctx.cpu.npc = ctx.cpu.spr[SPR_CTR] & (!3);
    }
}

fn op_bclrx(ctx: &mut Context, instr: Instruction) {
    let bo = instr.bo();

    if bo & 0x4 == 0 {
        ctx.cpu.spr[SPR_CTR] = ctx.cpu.spr[SPR_CTR].wrapping_sub(1);
    }

    let ctr_ok = ((bo >> 2) | ((ctx.cpu.spr[SPR_CTR] != 0) as u8 ^ (bo >> 1))) & 1;
    let cond_ok = ((bo >> 4) | (ctx.cpu.cr.get_bit(instr.bi()) == ((bo >> 3) & 1)) as u8) & 1;

    if ctr_ok != 0 && cond_ok != 0 {
        ctx.cpu.npc = ctx.cpu.spr[SPR_LR] & (!3);

        if instr.lk() != 0 {
            ctx.cpu.spr[SPR_LR] = ctx.cpu.pc.wrapping_add(4);
        }
    }
}
