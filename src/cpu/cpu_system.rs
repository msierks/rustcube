fn op_eieio(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_eieio");
}

fn op_isync(ctx: &mut Context, _instr: Instruction) {
    // don't do anything

    ctx.tick(2);
}

fn op_mfmsr(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.gpr[instr.d()] = ctx.cpu.msr.0;

    // TODO: check privilege level

    ctx.tick(1);
}

fn op_mfspr(ctx: &mut Context, instr: Instruction) {
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

fn op_mfsr(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_mfsr");
}

fn op_mfsrin(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_mfsrin");
}

fn op_mftb(ctx: &mut Context, instr: Instruction) {
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

fn op_mtmsr(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.msr = ctx.cpu.gpr[instr.s()].into();

    // TODO: check privilege level

    ctx.tick(1);
}

fn op_mtspr(ctx: &mut Context, instr: Instruction) {
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

fn op_mtsr(ctx: &mut Context, instr: Instruction) {
    ctx.cpu.sr[instr.sr()] = ctx.cpu.gpr[instr.s()];

    // TODO: check privilege level -> supervisor level instruction

    ctx.tick(2);
}

fn op_mtsrin(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_mtsrin");
}

fn op_rfi(ctx: &mut Context, _instr: Instruction) {
    let mask = 0x87C0_FF73;

    ctx.cpu.msr.0 = (ctx.cpu.msr.0 & !mask) | (ctx.cpu.spr[SPR_SRR1] & mask);

    ctx.cpu.msr.0 &= 0xFFFB_FFFF;

    ctx.cpu.nia = ctx.cpu.spr[SPR_SRR0] & 0xFFFF_FFFC;

    ctx.tick(2);
}

fn op_sc(ctx: &mut Context, _instr: Instruction) {
    ctx.cpu.exceptions |= EXCEPTION_SYSTEM_CALL;

    ctx.tick(2);
}

fn op_sync(ctx: &mut Context, _instr: Instruction) {
    // don't do anything

    ctx.tick(3);
}

fn op_tlbsync(_ctx: &mut Context, _instr: Instruction) {
    unimplemented!("op_tlbsync");
}
