use crate::cpu::instruction::Instruction;
use crate::cpu::spr::*;
use crate::cpu::{EXCEPTION_PROGRAM, EXCEPTION_SYSTEM_CALL};
use crate::Context;

pub fn op_eieio(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_eieio");
}

pub fn op_isync(ctx: &mut Context, _instr: Instruction) {
    // don't do anything

    ctx.tick(2);
}

pub fn op_mfmsr(ctx: &mut Context, instr: Instruction) {
    if ctx.cpu.msr.pr() {
        panic!("privelege level");
    }

    ctx.cpu.gpr[instr.d()] = ctx.cpu.msr.0;

    // TODO: check privilege level

    ctx.tick(1);
}

pub fn op_mfspr(ctx: &mut Context, instr: Instruction) {
    let i = instr.spr();

    ctx.cpu.gpr[instr.s()] = ctx.cpu.spr[i];

    match i {
        SPR_XER => ctx.cpu.gpr[instr.s()] = ctx.cpu.xer.into(),
        SPR_TBL => unimplemented!(),
        SPR_TBU => unimplemented!(),
        _ => (),
    }

    // TODO: check privilege level
    if (SPR_IBAT0U..=SPR_DBAT3L).contains(&i) {
        ctx.tick(3);
    } else {
        ctx.tick(1);
    }
}

pub fn op_mfsr(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_mfsr");
}

pub fn op_mfsrin(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_mfsrin");
}

pub fn op_mftb(ctx: &mut Context, instr: Instruction) {
    let timebase = ctx.timers.get_timebase();

    ctx.cpu.spr[SPR_TBL] = (timebase & 0xFFFF_FFFF) as u32;
    ctx.cpu.spr[SPR_TBU] = (timebase >> 32) as u32;

    if instr.tbr() == TBR_TBL {
        ctx.cpu.gpr[instr.d()] = ctx.cpu.spr[SPR_TBL];
    } else if instr.tbr() == TBR_TBU {
        ctx.cpu.gpr[instr.d()] = ctx.cpu.spr[SPR_TBU];
    } else {
        panic!("mftb unknown tbr {:#x}", instr.tbr());
    }

    ctx.tick(1);
}

pub fn op_mtmsr(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.msr = ctx.cpu.gpr[instr.s()].into();

    if ctx.cpu.msr.pr() {
        panic!("privelege level");
    }

    // TODO: check privilege level

    ctx.tick(1);
}

pub fn op_mtspr(ctx: &mut Context, instr: Instruction) {
    let i = instr.spr();
    let v = ctx.cpu.gpr[instr.s()];

    ctx.cpu.spr[i] = v;

    match i {
        SPR_XER => ctx.cpu.xer = v.into(),
        _ => {
            if ctx.cpu.msr.pr() {
                // TODO: properly handle this case
                ctx.cpu.exceptions |= EXCEPTION_PROGRAM;
                panic!("mtspr: user privilege level prevents setting spr {i:#?}");
            }

            match i {
                SPR_IBAT0U => ctx.cpu.mmu.write_ibatu(0, v),
                SPR_IBAT0L => ctx.cpu.mmu.write_ibatl(0, v),
                SPR_IBAT1U => ctx.cpu.mmu.write_ibatu(1, v),
                SPR_IBAT1L => ctx.cpu.mmu.write_ibatl(1, v),
                SPR_IBAT2U => ctx.cpu.mmu.write_ibatu(2, v),
                SPR_IBAT2L => ctx.cpu.mmu.write_ibatl(2, v),
                SPR_IBAT3U => ctx.cpu.mmu.write_ibatu(3, v),
                SPR_IBAT3L => ctx.cpu.mmu.write_ibatl(3, v),
                SPR_DBAT0U => ctx.cpu.mmu.write_dbatu(0, v),
                SPR_DBAT0L => ctx.cpu.mmu.write_dbatl(0, v),
                SPR_DBAT1U => ctx.cpu.mmu.write_dbatu(1, v),
                SPR_DBAT1L => ctx.cpu.mmu.write_dbatl(1, v),
                SPR_DBAT2U => ctx.cpu.mmu.write_dbatu(2, v),
                SPR_DBAT2L => ctx.cpu.mmu.write_dbatl(2, v),
                SPR_DBAT3U => ctx.cpu.mmu.write_dbatu(3, v),
                SPR_DBAT3L => ctx.cpu.mmu.write_dbatl(3, v),
                SPR_DEC => unimplemented!("Software Triggered Decrementer"),
                SPR_HID2 => ctx.cpu.hid2 = v.into(),
                SPR_TBL => ctx.timers.set_timebase_lower(v),
                SPR_TBU => ctx.timers.set_timebase_upper(v),
                SPR_WPAR => {
                    ctx.cpu.spr[i] &= !0x1F;
                    info!("WPAR set to {:#x}", ctx.cpu.spr[i]);
                    //ctx.gp_fifo.reset();
                }
                _ => {}
            }
        }
    }

    ctx.tick(2);
}

pub fn op_mtsr(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.sr[instr.sr()] = ctx.cpu.gpr[instr.s()];

    // TODO: check privilege level -> supervisor level instruction

    ctx.tick(2);
}

pub fn op_mtsrin(ctx: &mut Context, instr: Instruction) {
    let v = ctx.cpu.gpr[instr.s()];
    let i = (ctx.cpu.gpr[instr.b()] >> 28) as usize;

    ctx.cpu.sr[i] = v;

    ctx.tick(2);
}

pub fn op_rfi(ctx: &mut Context, _instr: Instruction) {
    let mask = 0x87C0_FF73;

    ctx.cpu.msr.0 = (ctx.cpu.msr.0 & !mask) | (ctx.cpu.spr[SPR_SRR1] & mask);

    ctx.cpu.msr.0 &= 0xFFFB_FFFF;

    ctx.cpu.nia = ctx.cpu.spr[SPR_SRR0] & 0xFFFF_FFFC;

    ctx.tick(2);
}

pub fn op_sc(ctx: &mut Context, _instr: Instruction) {
    ctx.cpu.exceptions |= EXCEPTION_SYSTEM_CALL;

    ctx.tick(2);
}

pub fn op_sync(ctx: &mut Context, _instr: Instruction) {
    // don't do anything

    ctx.tick(3);
}

pub fn op_tlbsync(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_tlbsync");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_eieio() {}

    #[test]
    fn op_isync() {}

    #[test]
    fn op_mfmsr() {
        let mut ctx = Context::default();

        let rd = 6;
        let instr = Instruction::new_mfmsr(rd);

        ctx.cpu.msr = 0x0D15_AA5E.into();

        super::op_mfmsr(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x0D15_AA5E);
    }

    #[test]
    fn op_mfspr() {
        let mut ctx = Context::default();

        let (rd, spr) = (6, SPR_LR as u32); // FIXME: make spr a usize
        let instr = Instruction::new_mfspr(rd, spr);

        ctx.cpu.spr[SPR_LR] = 0xDEAD_BEEF;
        super::op_mfspr(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0xDEAD_BEEF);
    }

    #[test]
    fn op_mfsr() {}

    #[test]
    fn op_mfsrin() {}

    #[test]
    fn op_mftb() {
        let mut ctx = Context::default();

        let (rd, tbr) = (6, TBR_TBL); // FIXME: make tbr usize
        let instr = Instruction::new_mftb(rd, tbr as u32);

        ctx.timers.tick(0x1784);
        super::op_mftb(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 501); // FIXME: this needs to be better
    }

    #[test]
    fn op_mtmsr() {
        let mut ctx = Context::default();

        let rs = 6;
        let instr = Instruction::new_mtmsr(rs);

        ctx.cpu.gpr[rs] = 0x0D15_AA5E;

        super::op_mtmsr(&mut ctx, instr);

        assert_eq!(ctx.cpu.msr.0, 0x0D15_AA5E);
    }

    #[test]
    fn op_mtspr() {}

    #[test]
    fn op_mtsrin() {}

    #[test]
    fn op_rfi() {}

    #[test]
    fn op_sc() {
        let mut ctx = Context::default();

        let instr = Instruction::new_sc();

        super::op_sc(&mut ctx, instr);

        assert_eq!(ctx.cpu.exceptions, EXCEPTION_SYSTEM_CALL);
    }

    #[test]
    fn op_sync() {
        let mut ctx = Context::default();

        let instr = Instruction::new_sync();

        super::op_sync(&mut ctx, instr);
    }

    #[test]
    #[should_panic]
    fn op_tlbsync() {
        let mut ctx = Context::default();

        let instr = Instruction::new_tlbsync();

        super::op_tlbsync(&mut ctx, instr);
    }
}
