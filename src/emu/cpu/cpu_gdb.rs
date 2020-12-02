use crate::emu::gdb::{PowerPc750Regs, PowerPcCoreRegId};
use crate::emu::Event;

use core::convert::TryInto;
use gdbstub::target::ext::base::singlethread::{ResumeAction, SingleThreadOps, StopReason};
use gdbstub::target::ext::breakpoints::WatchKind;
use gdbstub::target::{TargetError, TargetResult};

impl SingleThreadOps for Context {
    fn resume(
        &mut self,
        action: ResumeAction,
        check_gdb_interrupt: &mut dyn FnMut() -> bool,
    ) -> Result<StopReason<u32>, Self::Error> {
        let event = match action {
            ResumeAction::Step => match self.step() {
                Some(e) => e,
                None => return Ok(StopReason::DoneStep),
            },
            ResumeAction::Continue => {
                let mut cycles = 0;
                loop {
                    if let Some(event) = self.step() {
                        break event;
                    };

                    // check for GDB interrupt every 1024 instructions
                    cycles += 1;
                    if cycles % 1024 == 0 && check_gdb_interrupt() {
                        return Ok(StopReason::GdbInterrupt);
                    }
                }
            }
        };

        Ok(match event {
            Event::Halted => StopReason::Halted,
            Event::Break => StopReason::HwBreak,
            Event::WatchWrite(addr) => StopReason::Watch {
                kind: WatchKind::Write,
                addr,
            },
            Event::WatchRead(addr) => StopReason::Watch {
                kind: WatchKind::Read,
                addr,
            },
        })
    }

    fn read_registers(&mut self, regs: &mut PowerPc750Regs) -> TargetResult<(), Self> {
        regs.gpr.copy_from_slice(&self.cpu.gpr);

        //regs.fpr.copy_from_slice(&self.cpu.fpr);
        regs.pc = self.cpu.pc;
        regs.msr = self.cpu.msr.0;
        //regs.cr = self.
        //regs.lr = self.
        regs.ctr = self.cpu.spr[SPR_CTR];
        regs.xer = self.cpu.spr[SPR_XER];
        //regs.fpscr = self.cpu.
        regs.sr.copy_from_slice(&self.cpu.sr);
        regs.pvr = self.cpu.spr[SPR_PVR];
        regs.bat
            .copy_from_slice(&self.cpu.spr[SPR_IBAT0U..SPR_IBAT0U + 16]);
        regs.sdr1 = self.cpu.spr[SPR_SDR1];
        //regs.asr = self.cpu.spr[SPR_ASR];
        regs.dar = self.cpu.spr[SPR_DAR];
        regs.dsisr = self.cpu.spr[SPR_DSISR];
        regs.sprg
            .copy_from_slice(&self.cpu.spr[SPR_SPRG0..SPR_SPRG0 + 4]);
        regs.srr0 = self.cpu.spr[SPR_SRR0];
        regs.srr1 = self.cpu.spr[SPR_SRR0 + 1];
        //regs.tbl = self.cpu.
        //regs.tbu = self.cpu.
        regs.dec = self.cpu.spr[SPR_DEC];
        regs.dabr = self.cpu.spr[SPR_DABR];
        regs.ear = self.cpu.spr[SPR_EAR];

        Ok(())
    }

    fn write_registers(&mut self, _regs: &PowerPc750Regs) -> TargetResult<(), Self> {
        unimplemented!();
    }
    /*
        fn read_register(
            &mut self,
            reg_id: PowerPcCoreRegId,
            dst: &mut [u8],
        ) -> TargetResult<(), Self> {
            if let Some(i) = cpu_reg_id(reg_id) {
                let w = self.cpu.reg_get(self.cpu.mode(), i);
                dst.copy_from_slice(&w.to_le_bytes());
                Ok(())
            } else {
                Err(().into())
            }
        }
    */
    fn write_register(&mut self, reg_id: PowerPcCoreRegId, val: &[u8]) -> TargetResult<(), Self> {
        use PowerPcCoreRegId::*;

        if let Fpr(_i) = reg_id {
            let _w = u64::from_be_bytes(
                val.try_into()
                    .map_err(|_| TargetError::Fatal("invalid data"))?,
            );

        //self.cpu.fpr[i] = w;
        } else if let Asr = reg_id {
            // ignore
        } else {
            let w = u32::from_be_bytes(
                val.try_into()
                    .map_err(|_| TargetError::Fatal("invalid data"))?,
            );

            match reg_id {
                Gpr(i) => self.cpu.gpr[i] = w, // FixMe: write using register id
                Pc => self.cpu.pc = w,
                Msr => self.cpu.msr = w.into(),
                Cr => (), //self.cpu.cr = w,
                Lr => self.cpu.lr = w,
                Ctr => self.cpu.ctr = w,
                Xer => (),
                Fpscr => (),
                Sr(i) => self.cpu.sr[i] = w,
                Pvr => self.cpu.spr[SPR_PVR] = w,
                Ibat0u => self.cpu.spr[SPR_IBAT0U] = w,
                Ibat0l => self.cpu.spr[SPR_IBAT0L] = w,
                Ibat1u => self.cpu.spr[SPR_IBAT1U] = w,
                Ibat1l => self.cpu.spr[SPR_IBAT1L] = w,
                Ibat2u => self.cpu.spr[SPR_IBAT2U] = w,
                Ibat2l => self.cpu.spr[SPR_IBAT2L] = w,
                Ibat3u => self.cpu.spr[SPR_IBAT3U] = w,
                Ibat3l => self.cpu.spr[SPR_IBAT3L] = w,
                Dbat0u => self.cpu.spr[SPR_DBAT0U] = w,
                Dbat0l => self.cpu.spr[SPR_DBAT0L] = w,
                Dbat1u => self.cpu.spr[SPR_DBAT1U] = w,
                Dbat1l => self.cpu.spr[SPR_DBAT1L] = w,
                Dbat2u => self.cpu.spr[SPR_DBAT2U] = w,
                Dbat2l => self.cpu.spr[SPR_DBAT2L] = w,
                Dbat3u => self.cpu.spr[SPR_DBAT3U] = w,
                Dbat3l => self.cpu.spr[SPR_DBAT3L] = w,
                Sdr1 => self.cpu.spr[SPR_SDR1] = w,
                Dar => self.cpu.spr[SPR_DAR] = w,
                Dsisr => self.cpu.spr[SPR_DSISR] = w,
                Sprg(i) => self.cpu.spr[SPR_SPRG0 + i] = w,
                Srr0 => self.cpu.spr[SPR_SRR0] = w,
                Srr1 => self.cpu.spr[SPR_SRR1] = w,
                Tbl => self.cpu.spr[SPR_TBL] = w,
                Tbu => self.cpu.spr[SPR_TBL + 1] = w,
                Dec => self.cpu.spr[SPR_DEC] = w,
                Dabr => self.cpu.spr[SPR_DABR] = w,
                Ear => self.cpu.spr[SPR_EAR] = w,
                Hid0 => self.cpu.spr[SPR_HID0] = w,
                Hid1 => self.cpu.spr[SPR_HID1] = w,
                Iabr => self.cpu.spr[SPR_IABR] = w,
                Ummcr0 => self.cpu.spr[SPR_UMMCR0] = w,
                Upmc1 => self.cpu.spr[SPR_UPMC1] = w,
                Upmc2 => self.cpu.spr[SPR_UPMC1 + 1] = w,
                Usia => self.cpu.spr[SPR_USIA] = w,
                Ummcr1 => self.cpu.spr[SPR_UMMCR1] = w,
                Upmc3 => self.cpu.spr[SPR_UPMC3] = w,
                Upmc4 => self.cpu.spr[SPR_UPMC4] = w,
                Mmcr0 => self.cpu.spr[SPR_MMCR0] = w,
                Pmc1 => self.cpu.spr[SPR_PMC1] = w,
                Pmc2 => self.cpu.spr[SPR_PMC2] = w,
                Sia => self.cpu.spr[SPR_SIA] = w,
                Mmcr1 => self.cpu.spr[SPR_MMCR1] = w,
                Pmc3 => self.cpu.spr[SPR_PMC3] = w,
                Pmc4 => self.cpu.spr[SPR_PMC4] = w,
                L2cr => self.cpu.spr[SPR_L2CR] = w,
                Ictc => self.cpu.spr[SPR_ICTC] = w,
                Thrm1 => self.cpu.spr[SPR_THRM1] = w,
                Thrm2 => self.cpu.spr[SPR_THRM1 + 1] = w,
                Thrm3 => self.cpu.spr[SPR_THRM1 + 1] = w,
                _ => unreachable!(),
            }
        }

        Ok(())
    }

    fn read_addrs(&mut self, start_addr: u32, data: &mut [u8]) -> TargetResult<(), Self> {
        for (addr, val) in (start_addr..).zip(data.iter_mut()) {
            *val = self.read_u8(addr);
        }

        Ok(())
    }

    fn write_addrs(&mut self, _start_addr: u32, _data: &[u8]) -> TargetResult<(), Self> {
        unimplemented!();
    }
}
