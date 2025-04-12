use crate::cpu::instruction::Instruction;
use crate::cpu::util::*;
use crate::cpu::{SPR_CTR, SPR_LR};
use crate::Context;

const BO_DONT_DECREMENT: u8 = 0x4;

pub fn op_bx(ctx: &mut Context, instr: Instruction) {
    let address = sign_ext_26(instr.li() << 2) as u32;

    if instr.aa() {
        ctx.cpu.nia = address;
    } else {
        ctx.cpu.nia = ctx.cpu.cia.wrapping_add(address);
    }

    if instr.lk() {
        ctx.cpu.spr[SPR_LR] = ctx.cpu.cia.wrapping_add(4);
    }

    ctx.tick(1);
}

pub fn op_bcx(ctx: &mut Context, instr: Instruction) {
    let bo = instr.bo();

    if bo & BO_DONT_DECREMENT == 0 {
        ctx.cpu.spr[SPR_CTR] = ctx.cpu.spr[SPR_CTR].wrapping_sub(1);
    }

    let ctr_ok = (bo >> 2) & 1 != 0 || (((ctx.cpu.spr[SPR_CTR] != 0) as u8 ^ (bo >> 1)) & 1) != 0;
    let cond_ok = (bo >> 4) & 1 != 0 || (ctx.cpu.cr.get_bit(instr.bi()) == (bo >> 3) & 1);

    if ctr_ok && cond_ok {
        let address = sign_ext_16(instr.bd() << 2) as u32;

        if instr.aa() {
            ctx.cpu.nia = address;
        } else {
            ctx.cpu.nia = ctx.cpu.cia.wrapping_add(address);
        }

        if instr.lk() {
            ctx.cpu.spr[SPR_LR] = ctx.cpu.cia.wrapping_add(4);
        }
    }

    ctx.tick(1);
}

pub fn op_bcctrx(ctx: &mut Context, instr: Instruction) {
    let bo = instr.bo();

    // FIXME: check this logic
    if bo & BO_DONT_DECREMENT == 0 {
        panic!("bcctrx: Invalid instruction, BO[2] = 0");
    }

    let cond_ok = ((bo >> 4) | (ctx.cpu.cr.get_bit(instr.bi()) == ((bo >> 3) & 1)) as u8) & 1;

    if cond_ok != 0 {
        ctx.cpu.nia = ctx.cpu.spr[SPR_CTR] & (!3);

        if instr.lk() {
            ctx.cpu.spr[SPR_LR] = ctx.cpu.cia.wrapping_add(4);
        }
    }

    ctx.tick(1);
}

pub fn op_bclrx(ctx: &mut Context, instr: Instruction) {
    let bo = instr.bo();

    if bo & BO_DONT_DECREMENT == 0 {
        ctx.cpu.spr[SPR_CTR] = ctx.cpu.spr[SPR_CTR].wrapping_sub(1);
    }

    let ctr_ok = ((bo >> 2) | ((ctx.cpu.spr[SPR_CTR] != 0) as u8 ^ (bo >> 1))) & 1;
    let cond_ok = ((bo >> 4) | (ctx.cpu.cr.get_bit(instr.bi()) == ((bo >> 3) & 1)) as u8) & 1;

    if ctr_ok != 0 && cond_ok != 0 {
        ctx.cpu.nia = ctx.cpu.spr[SPR_LR] & (!3);

        if instr.lk() {
            ctx.cpu.spr[SPR_LR] = ctx.cpu.cia.wrapping_add(4);
        }
    }

    ctx.tick(1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::ops::integer::{op_addi, op_addic_rc};
    use crate::cpu::ops::system::op_mtspr;

    #[test]
    fn op_bcx() {
        let mut ctx = Context::default();

        // addi 8,0,3
        let (rd, ra, simm) = (8, 0, 0x3);
        let instr = Instruction::new_addi(rd, ra, simm);

        op_addi(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x0000_0003);

        // mtctr 8
        let instr = Instruction::new_mtspr(0x9, 0x8);
        op_mtspr(&mut ctx, instr);

        // check counter register is set to 0x3
        assert_eq!(ctx.cpu.spr[SPR_CTR], 0x0000_0003);

        // addic. 9,8,0x1
        let (rd, ra, simm) = (9, 8, 0x1);
        let instr = Instruction::new_addic_rc(rd, ra, simm);

        op_addic_rc(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x0000_0004);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x0000_0004);

        // bc 0xC,1,0x456
        let (bo, bi, bd) = (0xC, 1, 0x456);
        let instr = Instruction::new_bcx(bo, bi, bd);

        super::op_bcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.nia, 0xFFF0_1258);

        // bcl 0x8,1,0x456
        let (bo, bi, bd, lk) = (0x8, 1, 0x456, 1);
        let instr = Instruction::new_bcx(bo, bi, bd).set_lk(lk);

        super::op_bcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.spr[SPR_CTR], 0x2);
        assert_eq!(ctx.cpu.spr[SPR_LR], 0xFFF0_0104);
    }
}
