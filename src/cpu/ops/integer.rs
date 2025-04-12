use crate::cpu::instruction::Instruction;
use crate::cpu::{check_overflowed, mask, Ordering, EXCEPTION_PROGRAM, SPR_SRR1};
use crate::Context;

pub fn op_addcx(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()];
    let rb = ctx.cpu.gpr[instr.b()];

    let (rd, ca) = ra.overflowing_add(rb);

    ctx.cpu.gpr[instr.d()] = rd;

    ctx.cpu.xer.set_carry(ca);

    if instr.oe() {
        ctx.cpu.set_xer_so(check_overflowed(ra, rb, rd));
    }

    if instr.rc() {
        ctx.cpu.update_cr0(rd);
    }

    ctx.tick(1);
}

pub fn op_addx(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()];
    let rb = ctx.cpu.gpr[instr.b()];
    let rd = ra.wrapping_add(rb);

    ctx.cpu.gpr[instr.d()] = rd;

    if instr.oe() {
        ctx.cpu.set_xer_so(check_overflowed(ra, rb, rd));
    }

    if instr.rc() {
        ctx.cpu.update_cr0(rd);
    }

    ctx.tick(1);
}

pub fn op_addi(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.d()] = if instr.a() == 0 {
        i32::from(instr.simm()) as u32
    } else {
        ctx.cpu.gpr[instr.a()].wrapping_add(i32::from(instr.simm()) as u32)
    };

    ctx.tick(1);
}

pub fn op_addic(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()];
    let imm = i32::from(instr.simm()) as u32;

    let (rd, ca) = ra.overflowing_add(imm);

    ctx.cpu.gpr[instr.d()] = rd;

    ctx.cpu.xer.set_carry(ca);

    ctx.tick(1);
}

pub fn op_addic_rc(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()];
    let imm = i32::from(instr.simm()) as u32;

    let (rd, ca) = ra.overflowing_add(imm);

    ctx.cpu.gpr[instr.d()] = rd;

    ctx.cpu.xer.set_carry(ca);

    ctx.cpu.update_cr0(rd);

    ctx.tick(1);
}

pub fn op_addis(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.d()] = if instr.a() == 0 {
        instr.uimm() << 16
    } else {
        ctx.cpu.gpr[instr.a()].wrapping_add(instr.uimm() << 16)
    };

    ctx.tick(1);
}

pub fn op_addmex(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_addmex");
}

pub fn op_addex(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()];
    let rb = ctx.cpu.gpr[instr.b()];

    let (rd, ca1) = ra.overflowing_add(rb);
    let (rd, ca2) = rd.overflowing_add(ctx.cpu.xer.carry() as u32);

    ctx.cpu.gpr[instr.d()] = rd;

    ctx.cpu.xer.set_carry(ca1 | ca2);

    if instr.oe() {
        ctx.cpu.set_xer_so(check_overflowed(ra, rb, rd));
    }

    if instr.rc() {
        ctx.cpu.update_cr0(rd);
    }

    ctx.tick(1);
}

pub fn op_addzex(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()];

    let (rd, ca) = ra.overflowing_add(ctx.cpu.xer.carry() as u32);

    ctx.cpu.gpr[instr.d()] = rd;

    ctx.cpu.xer.set_carry(ca);

    if instr.oe() {
        ctx.cpu.set_xer_so(check_overflowed(ra, 0, rd));
    }

    if instr.rc() {
        ctx.cpu.update_cr0(rd);
    }

    ctx.tick(1);
}

pub fn op_andcx(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.s()] & (!ctx.cpu.gpr[instr.b()]);

    ctx.cpu.gpr[instr.a()] = ra;

    if instr.rc() {
        ctx.cpu.update_cr0(ra);
    }

    ctx.tick(1);
}

pub fn op_andi_rc(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.s()] & instr.uimm();

    ctx.cpu.gpr[instr.a()] = ra;

    ctx.cpu.update_cr0(ra);

    ctx.tick(1);
}

pub fn op_andis_rc(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_andis_rc");
}

pub fn op_andx(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.s()] & ctx.cpu.gpr[instr.b()];

    ctx.cpu.gpr[instr.a()] = ra;

    if instr.rc() {
        ctx.cpu.update_cr0(ra);
    }

    ctx.tick(1);
}

pub fn op_cmp(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()] as i32;
    let rb = ctx.cpu.gpr[instr.b()] as i32;

    let mut c = match ra.cmp(&rb) {
        Ordering::Less => 0x8,
        Ordering::Greater => 0x4,
        Ordering::Equal => 0x2,
    };

    c |= ctx.cpu.xer.summary_overflow() as u32;

    ctx.cpu.cr.set_field(instr.crfd(), c);

    ctx.tick(1);
}

pub fn op_cmpi(ctx: &mut Context, instr: Instruction) {
    if instr.l() {
        panic!("cmpi: invalid instruction");
    }

    let ra = ctx.cpu.gpr[instr.a()] as i32;
    let simm = i32::from(instr.simm());

    let mut c = match ra.cmp(&simm) {
        Ordering::Less => 0x8,
        Ordering::Greater => 0x4,
        Ordering::Equal => 0x2,
    };

    c |= ctx.cpu.xer.summary_overflow() as u32;

    ctx.cpu.cr.set_field(instr.crfd(), c);

    ctx.tick(1);
}

pub fn op_cmpl(ctx: &mut Context, instr: Instruction) {
    if instr.l() {
        panic!("cmpl: invalid instruction");
    }

    let ra = ctx.cpu.gpr[instr.a()];
    let rb = ctx.cpu.gpr[instr.b()];

    let mut c = match ra.cmp(&rb) {
        Ordering::Less => 0x8,
        Ordering::Greater => 0x4,
        Ordering::Equal => 0x2,
    };

    c |= ctx.cpu.xer.summary_overflow() as u32;

    ctx.cpu.cr.set_field(instr.crfd(), c);

    ctx.tick(1);
}

pub fn op_cmpli(ctx: &mut Context, instr: Instruction) {
    if instr.l() {
        panic!("cmpli: invalid instruction");
    }

    let ra = ctx.cpu.gpr[instr.a()];
    let uimm = instr.uimm();

    let mut c = match ra.cmp(&uimm) {
        Ordering::Less => 0x8,
        Ordering::Greater => 0x4,
        Ordering::Equal => 0x2,
    };

    c |= ctx.cpu.xer.summary_overflow() as u32;

    ctx.cpu.cr.set_field(instr.crfd(), c);

    ctx.tick(1);
}

pub fn op_cntlzwx(ctx: &mut Context, instr: Instruction) {
    let n = ctx.cpu.gpr[instr.s()].leading_zeros();

    ctx.cpu.gpr[instr.a()] = n;

    if instr.rc() {
        ctx.cpu.update_cr0(n);
    }

    ctx.tick(1);
}

pub fn op_divwux(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()];
    let rb = ctx.cpu.gpr[instr.b()];
    let overflow = rb == 0;

    let rd = if overflow { 0 } else { ra / rb };

    ctx.cpu.gpr[instr.d()] = rd;

    if instr.oe() {
        ctx.cpu.set_xer_so(overflow);
    }

    if instr.rc() {
        ctx.cpu.update_cr0(rd);
    }

    ctx.tick(19);
}

// TODO: review this implementation
pub fn op_divwx(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()] as i32;
    let rb = ctx.cpu.gpr[instr.b()] as i32;
    let overflow = rb == 0 || (ra as u32 == 0x8000_0000 && rb == -1);

    let rd = if overflow {
        if ra as u32 == 0x8000_0000 && rb == 0 {
            0xFFFF_FFFF
        } else {
            0
        }
    } else {
        (ra / rb) as u32
    };

    ctx.cpu.gpr[instr.d()] = rd;

    if instr.oe() {
        ctx.cpu.set_xer_so(overflow);
    }

    if instr.rc() {
        ctx.cpu.update_cr0(rd);
    }

    ctx.tick(19);
}

pub fn op_eqvx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_eqvx");
}

pub fn op_extsbx(ctx: &mut Context, instr: Instruction) {
    let ra = ((ctx.cpu.gpr[instr.s()] as i8) as i32) as u32;

    ctx.cpu.gpr[instr.a()] = ra;

    if instr.rc() {
        ctx.cpu.update_cr0(ra);
    }

    ctx.tick(1);
}

pub fn op_extshx(ctx: &mut Context, instr: Instruction) {
    let ra = ((ctx.cpu.gpr[instr.s()] as i16) as i32) as u32;

    ctx.cpu.gpr[instr.a()] = ra;

    if instr.rc() {
        ctx.cpu.update_cr0(ra);
    }

    ctx.tick(1);
}

pub fn op_mulhwux(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()] as u64;
    let rb = ctx.cpu.gpr[instr.b()] as u64;

    let rd = ((ra * rb) >> 32) as u32;

    ctx.cpu.gpr[instr.d()] = rd;

    if instr.rc() {
        ctx.cpu.update_cr0(rd);
    }

    ctx.tick(2);
}

pub fn op_mulhwx(ctx: &mut Context, instr: Instruction) {
    let ra = (ctx.cpu.gpr[instr.a()] as i32) as i64;
    let rb = (ctx.cpu.gpr[instr.b()] as i32) as i64;

    let rd = ((ra * rb) >> 32) as u32;

    ctx.cpu.gpr[instr.d()] = rd;

    if instr.rc() {
        ctx.cpu.update_cr0(rd);
    }

    ctx.tick(2);
}

// TODO: review this implementation
pub fn op_mulli(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.d()] =
        (ctx.cpu.gpr[instr.a()] as i32).wrapping_mul(i32::from(instr.simm())) as u32;

    ctx.tick(2);
}

pub fn op_mullwx(ctx: &mut Context, instr: Instruction) {
    let ra = (ctx.cpu.gpr[instr.a()] as i32) as i64;
    let rb = (ctx.cpu.gpr[instr.b()] as i32) as i64;

    let rd = ra.wrapping_mul(rb);

    ctx.cpu.gpr[instr.d()] = rd as u32;

    if instr.oe() {
        ctx.cpu
            .set_xer_so(!(-0x8000_0000..=0x7FFF_FFFF).contains(&rd));
    }

    if instr.rc() {
        ctx.cpu.update_cr0(rd as u32);
    }

    ctx.tick(2);
}

pub fn op_nandx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_nandx");
}

pub fn op_negx(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()];
    let rd = (!ra).wrapping_add(1);

    ctx.cpu.gpr[instr.d()] = rd;

    if instr.oe() {
        ctx.cpu.set_xer_so(ra == 0x8000_0000);
    }

    if instr.rc() {
        ctx.cpu.update_cr0(rd);
    }

    ctx.tick(1);
}

pub fn op_norx(ctx: &mut Context, instr: Instruction) {
    let ra = !(ctx.cpu.gpr[instr.s()] | ctx.cpu.gpr[instr.b()]);

    ctx.cpu.gpr[instr.a()] = ra;

    if instr.rc() {
        ctx.cpu.update_cr0(ra);
    }

    ctx.tick(1);
}

pub fn op_orcx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_orcx");
}

pub fn op_ori(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.a()] = ctx.cpu.gpr[instr.s()] | instr.uimm();

    ctx.tick(1);
}

pub fn op_oris(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.a()] = ctx.cpu.gpr[instr.s()] | (instr.uimm() << 16);

    ctx.tick(1);
}

pub fn op_orx(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.s()] | ctx.cpu.gpr[instr.b()];

    ctx.cpu.gpr[instr.a()] = ra;

    if instr.rc() {
        ctx.cpu.update_cr0(ra);
    }

    ctx.tick(1);
}

pub fn op_rlwimix(ctx: &mut Context, instr: Instruction) {
    let m = mask(instr.mb(), instr.me());

    let ra = (ctx.cpu.gpr[instr.a()] & !m) | (ctx.cpu.gpr[instr.s()].rotate_left(instr.sh()) & m);

    ctx.cpu.gpr[instr.a()] = ra;

    if instr.rc() {
        ctx.cpu.update_cr0(ra);
    }

    ctx.tick(1);
}

pub fn op_rlwinmx(ctx: &mut Context, instr: Instruction) {
    let mask = mask(instr.mb(), instr.me());

    let ra = (ctx.cpu.gpr[instr.s()].rotate_left(instr.sh())) & mask;

    ctx.cpu.gpr[instr.a()] = ra;

    if instr.rc() {
        ctx.cpu.update_cr0(ra);
    }

    ctx.tick(1);
}

pub fn op_rlwnmx(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_rlwnmx");
}

pub fn op_slwx(ctx: &mut Context, instr: Instruction) {
    let rb = ctx.cpu.gpr[instr.b()];

    let ra = if rb & 0x20 != 0 {
        0
    } else {
        ctx.cpu.gpr[instr.s()] << (rb & 0x1F)
    };

    ctx.cpu.gpr[instr.a()] = ra;

    if instr.rc() {
        ctx.cpu.update_cr0(ra);
    }

    ctx.tick(1);
}

pub fn op_srawix(ctx: &mut Context, instr: Instruction) {
    let rs = ctx.cpu.gpr[instr.s()] as i32;
    let s = instr.s();

    ctx.cpu.gpr[instr.a()] = (rs >> instr.sh()) as u32;
    ctx.cpu
        .xer
        .set_carry(rs < 0 && ((rs as u32) << (32 - s)) != 0);

    ctx.tick(1);
}

// TODO: review this implementation
pub fn op_srawx(ctx: &mut Context, instr: Instruction) {
    let rb = ctx.cpu.gpr[instr.b()];

    if rb & 0x20 != 0 {
        if ctx.cpu.gpr[instr.s()] & 0x8000_0000 != 0 {
            ctx.cpu.gpr[instr.a()] = 0xFFFF_FFFF;
            ctx.cpu.xer.set_carry(true);
        } else {
            ctx.cpu.gpr[instr.a()] = 0;
            ctx.cpu.xer.set_carry(false);
        }
    } else {
        let n = rb & 0x1F;

        if n != 0 {
            let rs = ctx.cpu.gpr[instr.s()] as i32;

            ctx.cpu.gpr[instr.a()] = (rs >> n) as u32;

            ctx.cpu.xer.set_carry(rs < 0 && (rs << (32 - n) != 0));
        } else {
            ctx.cpu.gpr[instr.a()] = ctx.cpu.gpr[instr.s()];
            ctx.cpu.xer.set_carry(false);
        }
    }

    if instr.rc() {
        ctx.cpu.update_cr0(ctx.cpu.gpr[instr.a()]);
    }

    ctx.tick(1);
}

pub fn op_srwx(ctx: &mut Context, instr: Instruction) {
    let rb = ctx.cpu.gpr[instr.b()];

    let ra = if rb & 0x20 != 0 {
        0
    } else {
        ctx.cpu.gpr[instr.s()] >> (rb & 0x1F)
    };

    ctx.cpu.gpr[instr.a()] = ra;

    if instr.rc() {
        ctx.cpu.update_cr0(ra);
    }

    ctx.tick(1);
}

pub fn op_subfcx(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()];
    let rb = ctx.cpu.gpr[instr.b()];

    let (rd, ca1) = (!ra).overflowing_add(rb);
    let (rd, ca2) = rd.overflowing_add(1);

    ctx.cpu.gpr[instr.d()] = rd;

    ctx.cpu.xer.set_carry(ca1 || ca2);

    if instr.oe() {
        ctx.cpu.set_xer_so(check_overflowed(!ra, rb, rd));
    }

    if instr.rc() {
        ctx.cpu.update_cr0(rd);
    }

    ctx.tick(1);
}

pub fn op_subfex(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()];
    let rb = ctx.cpu.gpr[instr.b()];

    let (rd, ca1) = (!ra).overflowing_add(rb);
    let (rd, ca2) = rd.overflowing_add(ctx.cpu.xer.carry() as u32);

    ctx.cpu.gpr[instr.d()] = rd;

    ctx.cpu.xer.set_carry(ca1 | ca2);

    if instr.oe() {
        ctx.cpu.set_xer_so(check_overflowed(!ra, rb, rd));
    }

    if instr.rc() {
        ctx.cpu.update_cr0(rd);
    }

    ctx.tick(1);
}

pub fn op_subfic(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()];
    let simm = (instr.simm() as i32) as u32;

    let (rd, ca) = simm.overflowing_sub(ra);

    ctx.cpu.gpr[instr.d()] = rd;

    ctx.cpu.xer.set_carry(ca);

    ctx.tick(1);
}

pub fn op_subfmex(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_subfmex");
}

pub fn op_subfzex(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()];
    let ca = ctx.cpu.xer.carry() as u32;

    let rd = (!ra).wrapping_add(ca);

    ctx.cpu.gpr[instr.d()] = rd;

    ctx.cpu.xer.set_carry(ca > ra);

    if instr.rc() {
        ctx.cpu.update_cr0(rd);
    }

    if instr.oe() {
        panic!("OE: subfzex");
    }

    ctx.tick(1);
}

pub fn op_subfx(ctx: &mut Context, instr: Instruction) {
    let ra = ctx.cpu.gpr[instr.a()] as i32;
    let rb = ctx.cpu.gpr[instr.b()] as i32;

    let (rd, ov) = rb.overflowing_sub(ra);
    let rd = rd as u32;

    ctx.cpu.gpr[instr.d()] = rd;

    if instr.oe() {
        ctx.cpu.set_xer_so(ov);
    }

    if instr.rc() {
        ctx.cpu.update_cr0(rd);
    }

    ctx.tick(1);
}

pub fn op_tw(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_tw");
}

pub fn op_twi(ctx: &mut Context, instr: Instruction) {
    let a = ctx.cpu.gpr[instr.a()] as i32;
    let simm = instr.simm() as i32;
    let to = instr.to();

    if (a < simm && (to & 0x10) != 0)
        || (a > simm && (to & 0x80) != 0)
        || (a == simm && (to & 0x04) != 0)
        || ((a as u32) < simm as u32 && (to & 0x02) != 0)
        || (a as u32 > simm as u32 && (to & 0x01) != 0)
    {
        ctx.cpu.exceptions |= EXCEPTION_PROGRAM;
        // Set trap program exception flag
        ctx.cpu.spr[SPR_SRR1] = 1 << (31 - 14);
    }
}

pub fn op_xori(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.a()] = ctx.cpu.gpr[instr.s()] ^ instr.uimm();

    ctx.tick(1);
}

pub fn op_xoris(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.a()] = ctx.cpu.gpr[instr.s()] ^ (instr.uimm() << 16);

    ctx.tick(1);
}

pub fn op_xorx(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.a()] = ctx.cpu.gpr[instr.s()] ^ ctx.cpu.gpr[instr.b()];

    if instr.rc() {
        ctx.cpu.update_cr0(ctx.cpu.gpr[instr.a()]);
    }

    ctx.tick(1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_addi() {
        let mut ctx = Context::default();

        let (rd, ra, simm) = (4, 5, 0x8FF0);
        let instr = Instruction::new_addi(rd, ra, simm);

        ctx.cpu.gpr[ra] = 0x0000_0900;
        super::op_addi(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0xFFFF_98F0);
    }

    #[test]
    fn op_addic() {
        let mut ctx = Context::default();

        let (rd, ra, simm) = (6, 4, 0xFFFF);
        let instr = Instruction::new_addic(rd, ra, simm);

        ctx.cpu.gpr[ra] = 0x0000_2346;

        super::op_addic(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x0000_2345)
    }

    #[test]
    fn op_addic_rc() {
        let mut ctx = Context::default();
        let (rd, ra, simm) = (31, 3, 1);
        let instr = Instruction::new_addic_rc(rd, ra, simm);

        ctx.cpu.gpr[rd] = 0xDEAD_BEEF;
        ctx.cpu.gpr[ra] = 0xFFFF_FFFF;

        super::op_addic_rc(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x0);
        assert_eq!(ctx.cpu.gpr[ra], 0xFFFF_FFFF); // confirm gpr source remains unmodified
        assert!(ctx.cpu.xer.carry());

        ctx.cpu.gpr[ra] = 0xFFFF_FFFE;

        super::op_addic_rc(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0xFFFF_FFFF);
        assert!(!ctx.cpu.xer.carry());
    }

    #[test]
    fn op_addis() {
        let mut ctx = Context::default();

        let (rd, ra, simm) = (7, 6, 0x0011);
        let instr = Instruction::new_addis(rd, ra, simm);

        ctx.cpu.gpr[ra] = 0x0000_4000;
        super::op_addis(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x0011_4000);
    }

    #[test]
    fn op_addex() {
        let mut ctx = Context::default();

        let (rd, ra, rb) = (6, 4, 10);
        let instr = Instruction::new_addex(rd, ra, rb);

        ctx.cpu.gpr[ra] = 0x1000_0400;
        ctx.cpu.gpr[rb] = 0x1000_0400;
        ctx.cpu.xer.set_carry(true);
        super::op_addex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x2000_0801);

        ctx.cpu.gpr[ra] = 0x9000_3000;
        ctx.cpu.gpr[rb] = 0x7B41_92C0;
        ctx.cpu.xer.set_carry(false);
        super::op_addex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x0B41_C2C0);

        ctx.cpu.gpr[ra] = 0x1000_0400;
        ctx.cpu.gpr[rb] = 0xEFFF_FFFF;
        ctx.cpu.xer.set_carry(true);
        super::op_addex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x0000_0400);

        ctx.cpu.gpr[ra] = 0x9000_3000;
        ctx.cpu.gpr[rb] = 0x8000_7000;
        ctx.cpu.xer.set_carry(false);
        super::op_addex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x1000_A000);
    }

    #[test]
    fn op_addzex() {
        let mut ctx = Context::default();

        let (rd, ra) = (6, 4);
        let instr = Instruction::new_addzex(rd, ra);
        ctx.cpu.gpr[ra] = 0x7B41_92C0;
        ctx.cpu.xer.set_carry(false);
        super::op_addzex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x7B41_92C0);

        ctx.cpu.gpr[ra] = 0xEFFF_FFFF;
        ctx.cpu.xer.set_carry(true);
        super::op_addzex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0xF000_0000);

        ctx.cpu.gpr[ra] = 0x9000_3000;
        ctx.cpu.xer.set_carry(true);
        super::op_addzex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x9000_3001);

        ctx.cpu.gpr[ra] = 0xEFFF_FFFF;
        ctx.cpu.xer.set_carry(false);
        super::op_addzex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0xEFFF_FFFF);
    }

    #[test]
    fn op_addx() {
        let mut ctx = Context::default();

        let (rd, ra, rb) = (4, 6, 3);
        let instr = Instruction::new_addx(rd, ra, rb);
        ctx.cpu.gpr[ra] = 0x0004_0000;
        ctx.cpu.gpr[rb] = 0x0000_4000;
        super::op_addx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x0004_4000);
        assert!(!ctx.cpu.xer.carry());

        let instr = instr.set_rc(1);

        ctx.cpu.gpr[ra] = 0x8000_7000;
        ctx.cpu.gpr[rb] = 0x7000_8000;
        super::op_addx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0xF000_F000);
        assert!(!ctx.cpu.xer.carry());
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT

        ctx.cpu.gpr[ra] = 0xEFFF_FFFF;
        ctx.cpu.gpr[rb] = 0x8000_0000;
        super::op_addx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x6FFF_FFFF);
        // FIXME: check Summary Overflow, Overflow and carry bits are set in Fixed point
        // register

        ctx.cpu.gpr[ra] = 0xEFFF_FFFF;
        ctx.cpu.gpr[rb] = 0xEFFF_FFFF;
        super::op_addx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0xDFFF_FFFE);
        // FIXME: check check Summary Overflow, Overflow and carry bits are set in Fixed point
        // register, as well as condition register field 0 updated
    }

    #[test]
    fn op_addcx() {
        let mut ctx = Context::default();

        let (rd, ra, rb) = (6, 4, 10);
        let instr = Instruction::new_addcx(rd, ra, rb);
        ctx.cpu.gpr[ra] = 0x9000_3000;
        ctx.cpu.gpr[rb] = 0x8000_7000;
        super::op_addcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x1000_A000);

        let instr = instr.set_rc(1);

        ctx.cpu.gpr[ra] = 0x7000_3000;
        ctx.cpu.gpr[rb] = 0xFFFF_FFFF;
        super::op_addcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x7000_2FFF);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT

        let instr = instr.set_oe(1);

        ctx.cpu.gpr[ra] = 0x9000_3000;
        ctx.cpu.gpr[rb] = 0x7B41_92C0;
        super::op_addcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x0B41_C2C0);
        assert!(ctx.cpu.xer.carry());
        // FIXME: check Summary Overflow and Overflow bits are set

        ctx.cpu.gpr[ra] = 0x8000_0000;
        ctx.cpu.gpr[rb] = 0x8000_7000;
        ctx.cpu.xer.set_carry(false);
        super::op_addcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x0000_7000);
        assert!(ctx.cpu.xer.carry());
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x5); // GT, SO

        // FIXME: check Summery Overflow and Overflow bits set
    }

    #[test]
    fn op_andx() {
        let mut ctx = Context::default();

        let (ra, rs, rb) = (6, 4, 7);
        let instr = Instruction::new_andx(ra, rs, rb);

        ctx.cpu.gpr[rs] = 0xFFF2_5730;
        ctx.cpu.gpr[rb] = 0x7B41_92C0;
        super::op_andx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0x7B40_1200);

        let instr = instr.set_rc(1);

        ctx.cpu.gpr[rs] = 0xFFF2_5730;
        ctx.cpu.gpr[rb] = 0xFFFF_EFFF;
        super::op_andx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0xFFF2_4730);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT
    }

    #[test]
    fn op_andcx() {
        let mut ctx = Context::default();

        let (ra, rs, rb) = (6, 4, 5);
        let instr = Instruction::new_andcx(ra, rs, rb);

        ctx.cpu.gpr[rs] = 0x9000_3000;
        ctx.cpu.gpr[rb] = 0xFFFF_FFFF;
        super::op_andcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0x0000_0000);

        let instr = instr.set_rc(1);

        ctx.cpu.gpr[rs] = 0xB004_3000;
        ctx.cpu.gpr[rb] = 0x7676_7676;
        super::op_andcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0x8000_0000);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT
    }

    #[test]
    fn op_andi_rc() {
        let mut ctx = Context::default();

        let (ra, rs) = (6, 4);
        let uimm = 0x5730;
        let instr = Instruction::new_andi_rc(ra, rs, uimm);

        ctx.cpu.gpr[rs] = 0x7B41_92C0;
        super::op_andi_rc(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0x0000_1200);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT
    }

    #[test]
    fn op_cmp() {
        let mut ctx = Context::default();

        let (crfd, l, ra, rb) = (0, 0, 4, 6);
        let instr = Instruction::new_cmp(crfd, l, ra, rb);

        ctx.cpu.gpr[ra] = 0xFFFF_FFE7;
        ctx.cpu.gpr[rb] = 0x0000_0011;
        super::op_cmp(&mut ctx, instr);

        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT
    }

    #[test]
    fn op_cmpi() {
        let mut ctx = Context::default();

        let (crfd, l, ra, simm) = (0, 0, 4, 0x11);
        let instr = Instruction::new_cmpi(crfd, l, ra, simm);

        ctx.cpu.gpr[ra] = 0xFFFF_FFE7;
        super::op_cmpi(&mut ctx, instr);

        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT
    }

    #[test]
    fn op_cmpl() {
        let mut ctx = Context::default();

        let (crfd, l, ra, rb) = (0, 0, 4, 5);
        let instr = Instruction::new_cmpl(crfd, l, ra, rb);

        ctx.cpu.gpr[ra] = 0xFFFF_0000;
        ctx.cpu.gpr[rb] = 0x7FFF_0000;
        super::op_cmpl(&mut ctx, instr);

        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT
    }

    #[test]
    fn op_cmpli() {
        let mut ctx = Context::default();

        let (crfd, l, ra, uimm) = (0, 0, 4, 0xFF);
        let instr = Instruction::new_cmpli(crfd, l, ra, uimm);

        ctx.cpu.gpr[ra] = 0x0000_00FF;
        super::op_cmpli(&mut ctx, instr);

        assert_eq!(ctx.cpu.cr.get_cr0(), 0x2); // EQ
    }

    #[test]
    fn op_cntlzwx() {
        let mut ctx = Context::default();

        let (ra, rs) = (3, 3);
        let instr = Instruction::new_cntlzwx(ra, rs);

        ctx.cpu.gpr[ra] = 0x0061_9920;
        super::op_cntlzwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rs], 0x0000_0009);
    }

    #[test]
    fn op_divwx() {
        let mut ctx = Context::default();

        let (rd, ra, rb) = (4, 4, 6);
        let instr = Instruction::new_divwx(rd, ra, rb);

        ctx.cpu.gpr[ra] = 0x0000_0000;
        ctx.cpu.gpr[rb] = 0x0000_0002;
        super::op_divwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x0000_0000);

        let instr = instr.set_rc(1);

        ctx.cpu.gpr[ra] = 0x0000_0002;
        ctx.cpu.gpr[rb] = 0x0000_0002;
        super::op_divwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x0000_0001);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT

        ctx.cpu.gpr[ra] = 0x0000_0001;
        ctx.cpu.gpr[rb] = 0x0000_0002;
        super::op_divwx(&mut ctx, instr);

        // Undefined
        assert_eq!(ctx.cpu.gpr[rd], 0x0000_0000);

        ctx.cpu.gpr[ra] = 0x8000_0000;
        ctx.cpu.gpr[rb] = 0xFFFF_FFFF;
        super::op_divwx(&mut ctx, instr);

        // Undefined
        assert_eq!(ctx.cpu.gpr[rd], 0x0000_0000);
    }

    #[test]
    fn op_divwux() {
        let mut ctx = Context::default();

        let (rd, ra, rb) = (4, 4, 6);
        let instr = Instruction::new_divwux(rd, ra, rb);

        ctx.cpu.gpr[ra] = 0x0000_0000;
        ctx.cpu.gpr[rb] = 0x0000_0002;
        super::op_divwux(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x0000_0000);

        let instr = instr.set_rc(1);

        ctx.cpu.gpr[ra] = 0x0000_0002;
        ctx.cpu.gpr[rb] = 0x0000_0002;
        super::op_divwux(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x0000_0001);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT

        ctx.cpu.gpr[ra] = 0x0000_0001;
        ctx.cpu.gpr[rb] = 0x0000_0002;
        super::op_divwux(&mut ctx, instr);

        // Undefined
        assert_eq!(ctx.cpu.gpr[rd], 0x0000_0000);

        ctx.cpu.gpr[ra] = 0x8000_0000;
        ctx.cpu.gpr[rb] = 0xFFFF_FFFF;
        super::op_divwux(&mut ctx, instr);

        // Undefined
        assert_eq!(ctx.cpu.gpr[rd], 0x0000_0000);
    }

    #[test]
    fn op_extsbx() {
        let mut ctx = Context::default();

        let (ra, rs) = (4, 6);
        let instr = Instruction::new_extsbx(ra, rs);

        ctx.cpu.gpr[rs] = 0x5A5A_5A5A;
        super::op_extsbx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0x0000_005A);

        ctx.cpu.gpr[rs] = 0xA5A5_A5A5;
        super::op_extsbx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0xFFFF_FFA5);
    }

    #[test]
    fn op_extshx() {
        let mut ctx = Context::default();

        let (ra, rs) = (4, 6);
        let instr = Instruction::new_extshx(ra, rs);

        ctx.cpu.gpr[rs] = 0x0000_FFFF;
        super::op_extshx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0xFFFF_FFFF);

        ctx.cpu.gpr[rs] = 0x0000_2FFF;
        super::op_extshx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0x0000_2FFF);
    }

    #[test]
    fn op_mulhwux() {
        let mut ctx = Context::default();

        let (rd, ra, rb) = (6, 4, 10);
        let instr = Instruction::new_mulhwux(rd, ra, rb);

        ctx.cpu.gpr[ra] = 0x0000_0003;
        ctx.cpu.gpr[rb] = 0x0000_0002;
        super::op_mulhwux(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x0000_0000);

        let instr = instr.set_rc(1);

        ctx.cpu.gpr[ra] = 0x0000_4500;
        ctx.cpu.gpr[rb] = 0x8000_7000;
        super::op_mulhwux(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x0000_2280);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT
    }

    #[test]
    fn op_mulli() {
        let mut ctx = Context::default();

        let (rd, ra, simm) = (6, 4, 10);
        let instr = Instruction::new_mulli(rd, ra, simm);

        ctx.cpu.gpr[ra] = 0x0000_3000;
        super::op_mulli(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x0001_E000);
    }

    #[test]
    fn op_mullwx() {
        let mut ctx = Context::default();

        let (rd, ra, rb) = (6, 4, 10);
        let instr = Instruction::new_mullwx(rd, ra, rb);

        ctx.cpu.gpr[ra] = 0x0000_3000;
        ctx.cpu.gpr[rb] = 0x0000_7000;
        super::op_mullwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x1500_0000);

        let instr = instr.set_rc(1);

        ctx.cpu.gpr[ra] = 0x0000_4500;
        ctx.cpu.gpr[rb] = 0x0000_7000;
        super::op_mullwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x1E30_0000);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT

        let instr = instr.set_oe(1);

        ctx.cpu.gpr[ra] = 0x0000_4500;
        ctx.cpu.gpr[rb] = 0x0007_0000;
        super::op_mullwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0xE300_0000);
        // FIXME: check summary overflow and overflow

        ctx.cpu.gpr[ra] = 0x0000_4500;
        ctx.cpu.gpr[rb] = 0x7FFF_FFFF;
        super::op_mullwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0xFFFF_BB00);
        // FIXME: check summary overflow and overflow
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x9); // LT SO
    }

    #[test]
    fn op_negx() {
        let mut ctx = Context::default();

        let (rd, ra) = (6, 4);
        let instr = Instruction::new_negx(rd, ra);

        ctx.cpu.gpr[ra] = 0x9000_3000;
        super::op_negx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x6FFF_D000);

        let instr = instr.set_rc(1);

        ctx.cpu.gpr[ra] = 0x789A_789B;
        super::op_negx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x8765_8765);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT

        ctx.cpu.gpr[ra] = 0x9000_3000;
        super::op_negx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x6FFF_D000);
        // FIXME: check summary overflow and overflow bits

        ctx.cpu.gpr[ra] = 0x8000_0000;
        super::op_negx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x8000_0000);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT

        // FIXME: check summary overflow and overflow bits
    }

    #[test]
    fn op_norx() {
        let mut ctx = Context::default();

        let (ra, rs, rb) = (6, 4, 7);
        let instr = Instruction::new_norx(ra, rs, rb);

        ctx.cpu.gpr[rs] = 0x9000_3000;
        ctx.cpu.gpr[rb] = 0x789A_789B;
        super::op_norx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0x0765_8764);

        let instr = instr.set_rc(1);

        ctx.cpu.gpr[rs] = 0xB004_3000;
        ctx.cpu.gpr[rb] = 0x789A_789B;
        super::op_norx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0x0761_8764);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT
    }

    #[test]
    fn op_orx() {
        let mut ctx = Context::default();

        let (ra, rs, rb) = (6, 4, 7);
        let instr = Instruction::new_orx(ra, rs, rb);

        ctx.cpu.gpr[rs] = 0x9000_3000;
        ctx.cpu.gpr[rb] = 0x789A_789B;
        super::op_orx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0xF89A_789B);

        let instr = instr.set_rc(1);

        ctx.cpu.gpr[rs] = 0xB004_3000;
        ctx.cpu.gpr[rb] = 0x789A_789B;
        super::op_orx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0xF89E_789B);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT
    }

    #[test]
    fn op_ori() {
        let mut ctx = Context::default();

        let (rs, ra, uimm) = (6, 4, 0x0079);
        let instr = Instruction::new_ori(ra, rs, uimm);

        ctx.cpu.gpr[rs] = 0x9000_3000;
        super::op_ori(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0x9000_3079);
    }

    #[test]
    fn op_oris() {
        let mut ctx = Context::default();

        let (rs, ra, uimm) = (6, 4, 0x0079);
        let instr = Instruction::new_oris(ra, rs, uimm);

        ctx.cpu.gpr[rs] = 0x9000_3000;
        super::op_oris(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0x9079_3000);
    }

    #[test]
    fn op_rlwimix() {
        let mut ctx = Context::default();

        let (ra, rs, sh, mb, me) = (6, 4, 2, 0, 0x1D);
        let instr = Instruction::new_rlwimix(ra, rs, sh, mb, me);

        ctx.cpu.gpr[rs] = 0x9000_3000;
        ctx.cpu.gpr[ra] = 0x0000_0003;
        super::op_rlwimix(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0x4000_C003);

        let (mb, me) = (0, 0x1A);
        let instr = Instruction::new_rlwimix(ra, rs, sh, mb, me).set_rc(1);

        ctx.cpu.gpr[rs] = 0x789A_789B;
        ctx.cpu.gpr[ra] = 0x3000_0003;
        super::op_rlwimix(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0xE269_E263);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT
    }

    #[test]
    fn op_rlwinmx() {
        let mut ctx = Context::default();

        let (ra, rs, sh, mb, me) = (6, 4, 2, 0, 0x1D);
        let instr = Instruction::new_rlwinmx(ra, rs, sh, mb, me);

        ctx.cpu.gpr[rs] = 0x9000_3000;
        ctx.cpu.gpr[ra] = 0xFFFF_FFFF;
        super::op_rlwinmx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0x4000_C000);

        let instr = instr.set_rc(1);

        ctx.cpu.gpr[rs] = 0xB004_3000;
        ctx.cpu.gpr[ra] = 0xFFFF_FFFF;
        super::op_rlwinmx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0xC010_C000);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT
    }

    #[test]
    fn op_slwx() {
        let mut ctx = Context::default();

        let (ra, rs, rb) = (6, 4, 5);
        let instr = Instruction::new_slwx(ra, rs, rb);

        ctx.cpu.gpr[rb] = 0x0000_002F;
        ctx.cpu.gpr[rs] = 0xFFFF_FFFF;
        super::op_slwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0x0000_0000);

        let instr = instr.set_rc(1);

        ctx.cpu.gpr[rb] = 0x0000_0005;
        ctx.cpu.gpr[rs] = 0xB004_3000;
        super::op_slwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0x0086_0000);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT
    }

    #[test]
    fn op_srawx() {
        let mut ctx = Context::default();

        let (ra, rs, rb) = (6, 4, 5);
        let instr = Instruction::new_srawx(ra, rs, rb);

        ctx.cpu.gpr[rs] = 0x9000_3000;
        ctx.cpu.gpr[rb] = 0x0000_0024;
        super::op_srawx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0xFFFF_FFFF);
        assert!(ctx.cpu.xer.carry());

        let instr = instr.set_rc(1);

        ctx.cpu.gpr[rs] = 0xB004_3000;
        ctx.cpu.gpr[rb] = 0x0000_0004;
        super::op_srawx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0xFB00_4300);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT

        //assert_eq!(ctx.cpu.xer.carry(), true);
    }

    #[test]
    fn op_srawix() {
        let mut ctx = Context::default();

        let (ra, rs, sh) = (6, 4, 0x4);
        let instr = Instruction::new_srawix(ra, rs, sh);

        ctx.cpu.gpr[rs] = 0x9000_3000;
        super::op_srawix(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0xF900_0300);
        assert!(!ctx.cpu.xer.carry());

        ctx.cpu.gpr[rs] = 0xB004_3008;
        super::op_srawix(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0xFB00_4300);
        assert!(ctx.cpu.xer.carry());
    }

    #[test]
    fn op_srwx() {
        let mut ctx = Context::default();

        let (ra, rs, rb) = (6, 4, 5);
        let instr = Instruction::new_srwx(ra, rs, rb);

        ctx.cpu.gpr[rs] = 0x9000_3000;
        ctx.cpu.gpr[rb] = 0x0000_0024;
        super::op_srwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0x0000_0000);

        let instr = instr.set_rc(1);

        ctx.cpu.gpr[rs] = 0xB004_3001;
        ctx.cpu.gpr[rb] = 0x0000_0004;
        super::op_srwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0x0B00_4300);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT
    }

    #[test]
    fn op_subfx() {
        let mut ctx = Context::default();

        let (rd, ra, rb) = (6, 4, 10);
        let instr = Instruction::new_subfx(rd, ra, rb);

        ctx.cpu.gpr[ra] = 0x8000_7000;
        ctx.cpu.gpr[rb] = 0x9000_3000;
        super::op_subfx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x0FFF_C000);

        let instr = instr.set_rc(1);

        ctx.cpu.gpr[ra] = 0x0000_4500;
        ctx.cpu.gpr[rb] = 0x8000_7000;
        super::op_subfx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x8000_2B00);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT

        let instr = instr.set_oe(1);

        ctx.cpu.gpr[ra] = 0x8000_0000;
        ctx.cpu.gpr[rb] = 0x0000_4500;
        super::op_subfx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x8000_4500);
        // FIXME: check SO and O

        ctx.cpu.gpr[ra] = 0x8000_0000;
        ctx.cpu.gpr[rb] = 0x0000_7000;
        super::op_subfx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x8000_7000);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x9); // LT

        // FIXME: check SO and O
    }

    #[test]
    fn op_subfcx() {
        let mut ctx = Context::default();

        let (rd, ra, rb) = (6, 4, 10);
        let instr = Instruction::new_subfcx(rd, ra, rb);

        ctx.cpu.gpr[ra] = 0x8000_7000;
        ctx.cpu.gpr[rb] = 0x9000_3000;
        super::op_subfcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x0FFF_C000);
        assert!(ctx.cpu.xer.carry());

        let instr = instr.set_rc(1);

        ctx.cpu.gpr[ra] = 0x0000_4500;
        ctx.cpu.gpr[rb] = 0x8000_7000;
        super::op_subfcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x8000_2B00);
        assert!(ctx.cpu.xer.carry());
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT

        let instr = instr.set_oe(1);

        ctx.cpu.gpr[ra] = 0x8000_0000;
        ctx.cpu.gpr[rb] = 0x0000_4500;
        super::op_subfcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x8000_4500);
        assert!(!ctx.cpu.xer.carry());
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x9); // LT

        ctx.cpu.gpr[ra] = 0x8000_0000;
        ctx.cpu.gpr[rb] = 0x0000_7000;
        super::op_subfcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x8000_7000);
        assert!(!ctx.cpu.xer.carry());
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x9); // LT
    }

    #[test]
    fn op_subfex() {
        let mut ctx = Context::default();

        let (rd, ra, rb) = (6, 4, 10);
        let instr = Instruction::new_subfex(rd, ra, rb);

        ctx.cpu.gpr[ra] = 0x9000_3000;
        ctx.cpu.gpr[rb] = 0x8000_7000;
        ctx.cpu.xer.set_carry(true);
        super::op_subfex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0xF000_4000);
        assert!(!ctx.cpu.xer.carry());

        let instr = instr.set_rc(1);

        ctx.cpu.gpr[ra] = 0x0000_4500;
        ctx.cpu.gpr[rb] = 0x8000_7000;
        ctx.cpu.xer.set_carry(false);
        super::op_subfex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x8000_2AFF);
        assert!(ctx.cpu.xer.carry());
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT

        let instr = instr.set_oe(1);

        ctx.cpu.gpr[ra] = 0x8000_0000;
        ctx.cpu.gpr[rb] = 0xEFFF_FFFF;
        ctx.cpu.xer.set_carry(true);
        super::op_subfex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x6FFF_FFFF);
        assert!(ctx.cpu.xer.carry());
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT

        ctx.cpu.gpr[ra] = 0x8000_0000;
        ctx.cpu.gpr[rb] = 0xEFFF_FFFF;
        ctx.cpu.xer.set_carry(false);
        super::op_subfex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x6FFF_FFFE);
        assert!(ctx.cpu.xer.carry());
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT
    }

    #[test]
    fn op_subfic() {
        let mut ctx = Context::default();

        let (rd, ra, simm) = (6, 4, 0x7000);
        let instr = Instruction::new_subfic(rd, ra, simm);

        ctx.cpu.gpr[ra] = 0x9000_3000;
        super::op_subfic(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x7000_4000);
    }

    #[test]
    fn op_subfzex() {
        let mut ctx = Context::default();

        let (rd, ra) = (6, 4);
        let instr = Instruction::new_subfzex(rd, ra);

        ctx.cpu.gpr[ra] = 0x9000_3000;
        ctx.cpu.xer.set_carry(true);
        super::op_subfzex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x6FFF_D000);
        assert!(!ctx.cpu.xer.carry());

        ctx.cpu.gpr[ra] = 0xB004_3000;
        ctx.cpu.xer.set_carry(true);
        super::op_subfzex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x4FFB_D000);
        assert!(!ctx.cpu.xer.carry());

        ctx.cpu.gpr[ra] = 0xEFFF_FFFF;
        ctx.cpu.xer.set_carry(false);
        super::op_subfzex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x1000_0000);
        assert!(!ctx.cpu.xer.carry());

        ctx.cpu.gpr[ra] = 0x70FB_6500;
        ctx.cpu.xer.set_carry(false);
        super::op_subfzex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x8F04_9AFF);
        assert!(!ctx.cpu.xer.carry());
    }

    #[test]
    fn op_twi() {
        let mut ctx = Context::default();

        let a = 4;
        let instr = Instruction::new_twi(0x4, 4, 0x10);

        ctx.cpu.gpr[a] = 0x0000_0010;
        super::op_twi(&mut ctx, instr);

        assert_eq!(ctx.cpu.exceptions, EXCEPTION_PROGRAM);

        // FIXME: check cause is trap
    }

    #[test]
    fn op_xorx() {
        let mut ctx = Context::default();

        let (ra, rs, rb) = (6, 4, 3);
        let instr = Instruction::new_xorx(ra, rs, rb);

        ctx.cpu.gpr[rs] = 0x9000_3000;
        ctx.cpu.gpr[rb] = 0x789A_789B;
        super::op_xorx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0xE89A_489B);

        let instr = instr.set_rc(1);

        ctx.cpu.gpr[rs] = 0xB004_3000;
        ctx.cpu.gpr[rb] = 0x789A_789B;
        super::op_xorx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0xC89E_489B);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT
    }

    #[test]
    fn op_xoris() {
        let mut ctx = Context::default();

        let (ra, rs, uimm) = (6, 4, 0x0079);
        let instr = Instruction::new_xoris(ra, rs, uimm);

        ctx.cpu.gpr[rs] = 0x9000_3000;
        super::op_xoris(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0x9079_3000);
    }
}
