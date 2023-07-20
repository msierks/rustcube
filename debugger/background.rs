use crate::{
    Breakpoint, BreakpointAccessType, BreakpointType, Breakpoints, Callstack, Disassembly, Event,
    Memory, Registers,
};

use async_channel::{Receiver, Sender};
use rustcube::cpu::disassembler::Disassembler;
use rustcube::{cpu, Event as ctxEvent};

pub enum BgEvent {
    Continue,
    Step,
    Stop,
    Breakpoint(BreakpointType, u32, u32, bool, bool),
    BreakpointRemove(BreakpointType, u32),
    BreakpointClear,
    BreakpointToggle(u32),
    MemoryDump(u32),
}

pub async fn run(mut ctx: rustcube::Context, tx: Sender<Event>, rx: Receiver<BgEvent>) {
    let disassembler = Disassembler::default();

    let mut window_start = 0;
    let mut memory_dump_address = 0x8000_3000; //0x8000_0000;
    let regs = Box::new(Registers::new(&ctx));
    let disassembly = Box::new(Disassembly::new(&mut ctx, &disassembler, &mut window_start));
    let callstack = Box::new(Callstack::new(&mut ctx));
    let memory = Box::new(Memory::new(&mut ctx, memory_dump_address));
    let breakpoints = Box::new(Breakpoints::new(&ctx));
    let _ = tx.send(Event::Registers(regs)).await;
    let _ = tx.send(Event::Disassembly(disassembly)).await;
    let _ = tx.send(Event::Callstack(callstack)).await;
    let _ = tx.send(Event::Memory(memory)).await;
    let _ = tx.send(Event::Breakpoints(breakpoints)).await;

    while let Ok(event) = rx.recv().await {
        match event {
            BgEvent::Breakpoint(bp_type, start_addr, end_addr, break_on_write, break_on_read) => {
                match bp_type {
                    BreakpointType::Break => {
                        ctx.add_breakpoint(start_addr);
                    }
                    BreakpointType::Watch => {
                        ctx.add_watchpoint(start_addr, end_addr, break_on_write, break_on_read);
                    }
                }
                let breakpoints = Box::new(Breakpoints::new(&ctx));
                let disassembly =
                    Box::new(Disassembly::new(&mut ctx, &disassembler, &mut window_start));
                let _ = tx.send(Event::Breakpoints(breakpoints)).await;
                let _ = tx.send(Event::Disassembly(disassembly)).await;
            }
            BgEvent::BreakpointRemove(bp_type, num) => {
                match bp_type {
                    BreakpointType::Break => {
                        ctx.remove_breakpoint(num as usize);
                    }
                    BreakpointType::Watch => {
                        ctx.remove_watchpoint(num as usize);
                    }
                }
                let breakpoints = Box::new(Breakpoints::new(&ctx));
                let _ = tx.send(Event::Breakpoints(breakpoints)).await;
                let disassembly =
                    Box::new(Disassembly::new(&mut ctx, &disassembler, &mut window_start));
                let _ = tx.send(Event::Disassembly(disassembly)).await;
            }
            BgEvent::BreakpointClear => {
                ctx.breakpoints_clear();
                ctx.watchpoints_clear();
                let _ = tx
                    .send(Event::Breakpoints(Box::new(Breakpoints::new(&ctx))))
                    .await;
                let disassembly =
                    Box::new(Disassembly::new(&mut ctx, &disassembler, &mut window_start));
                let _ = tx.send(Event::Disassembly(disassembly)).await;
            }
            BgEvent::BreakpointToggle(addr) => {
                if ctx.breakpoints().contains(&addr) {
                    if let Some(pos) = ctx.breakpoints().iter().position(|x| *x == addr) {
                        ctx.remove_breakpoint(pos);
                    }
                } else {
                    ctx.add_breakpoint(addr);
                }
                let breakpoints = Box::new(Breakpoints::new(&ctx));
                let _ = tx.send(Event::Breakpoints(breakpoints)).await;
            }
            BgEvent::Continue => {
                while rx.is_empty() {
                    if let Some(event) = ctx.step() {
                        match event {
                            ctxEvent::Break => {
                                info!("Breakpoint {:#x}", ctx.cpu().pc());
                                break;
                            }
                            ctxEvent::WatchRead(addr) => {
                                info!("Watchpoint Read {:#x}", addr);
                                break;
                            }
                            ctxEvent::WatchWrite(addr) => {
                                info!("Watchpoint Write {:#x}", addr);
                                break;
                            }
                            ctxEvent::Halted => break,
                        }
                    }
                }

                let regs = Box::new(Registers::new(&ctx));
                let disassembly =
                    Box::new(Disassembly::new(&mut ctx, &disassembler, &mut window_start));
                let callstack = Box::new(Callstack::new(&mut ctx));
                let memory = Box::new(Memory::new(&mut ctx, memory_dump_address));
                let _ = tx.send(Event::Paused).await;
                let _ = tx.send(Event::Registers(regs)).await;
                let _ = tx.send(Event::Disassembly(disassembly)).await;
                let _ = tx.send(Event::Callstack(callstack)).await;
                let _ = tx.send(Event::Memory(memory)).await;
            }
            BgEvent::Step => {
                if let Some(event) = ctx.step() {
                    match event {
                        ctxEvent::Break => info!("Breakpoint {:#x}", ctx.cpu().pc()),
                        ctxEvent::WatchRead(addr) => info!("Watchpoint Read {:#x}", addr),
                        ctxEvent::WatchWrite(addr) => info!("Watchpoint Write {:#x}", addr),
                        ctxEvent::Halted => (),
                    }
                }

                let regs = Box::new(Registers::new(&ctx));
                let disassembly =
                    Box::new(Disassembly::new(&mut ctx, &disassembler, &mut window_start));
                let callstack = Box::new(Callstack::new(&mut ctx));
                let memory = Box::new(Memory::new(&mut ctx, memory_dump_address));
                let _ = tx.send(Event::Registers(regs)).await;
                let _ = tx.send(Event::Disassembly(disassembly)).await;
                let _ = tx.send(Event::Callstack(callstack)).await;
                let _ = tx.send(Event::Memory(memory)).await;
            }
            BgEvent::Stop => {}
            BgEvent::MemoryDump(addr) => {
                memory_dump_address = addr;
                let memory = Box::new(Memory::new(&mut ctx, memory_dump_address));
                let _ = tx.send(Event::Memory(memory)).await;
            }
        }
    }
}

impl Breakpoints {
    pub fn new(ctx: &rustcube::Context) -> Self {
        let mut breakpoints = Vec::new();

        for (num, bp) in ctx.breakpoints().iter().enumerate() {
            breakpoints.push(Breakpoint {
                type_: BreakpointType::Break,
                num: num as u32,
                start_address: *bp,
                end_address: 0,
                access_type: BreakpointAccessType::Read,
            });
        }

        for (num, wp) in ctx.watchpoints().iter().enumerate() {
            let access_type = match (wp.break_on_read, wp.break_on_write) {
                (false, false) => unreachable!(),
                (false, true) => BreakpointAccessType::Write,
                (true, false) => BreakpointAccessType::Read,
                (true, true) => BreakpointAccessType::ReadWrite,
            };

            breakpoints.push(Breakpoint {
                type_: BreakpointType::Watch,
                num: num as u32,
                start_address: wp.start_addr,
                end_address: wp.end_addr,
                access_type,
            });
        }

        Breakpoints { breakpoints }
    }
}

impl Callstack {
    pub fn new(ctx: &mut rustcube::Context) -> Self {
        // attempt to traverse callstack
        let lr = ctx.cpu().lr();
        let mut addresses = Vec::new();

        if lr != 0 {
            addresses.push(lr);

            let mut sp = ctx.cpu().gpr()[1];
            let mut count = 0;

            while sp != 0 && count < 80 {
                let address = ctx.read_u32(sp.wrapping_add(4));

                if address != 0 {
                    addresses.push(address);
                }

                sp = ctx.read_u32(sp);

                // TODO last frame ???
                if sp == 0xFFFF_FFFF {
                    break;
                }

                count += 1;
            }
        }

        Callstack { addresses }
    }
}

impl Disassembly {
    pub fn new(
        ctx: &mut rustcube::Context,
        disassem: &Disassembler,
        window_start: &mut u32,
    ) -> Self {
        let mut instructions = Vec::new();
        let mut pc = ctx.cpu().pc().wrapping_sub(100);

        let window_size = 200; // keep around 200 till scroll bug resolved: https://gitlab.gnome.org/GNOME/gtk/-/issues/2971

        if pc < *window_start || pc > *window_start + window_size {
            *window_start = pc;
        }

        pc = *window_start;

        for _ in 0..window_size {
            let addr = ctx.cpu().translate_instr_address(pc);
            let code = ctx.read_instruction(addr);
            let disinstr = disassem.decode(pc, code, true);

            instructions.push(disinstr);

            pc = pc.wrapping_add(4);
        }

        Disassembly {
            pc: ctx.cpu().pc(),
            instructions,
            breakpoints: ctx.breakpoints().clone(),
        }
    }
}

impl Memory {
    pub fn new(ctx: &mut rustcube::Context, address: u32) -> Self {
        let mut data = Vec::new();
        let start = address & 0xFFFF_FFF0;
        let end = start + 0x200;

        for ea in (start..end).step_by(16) {
            let val1 = ctx.debug_read_u32(ea);
            let val2 = ctx.debug_read_u32(ea + 4);
            let val3 = ctx.debug_read_u32(ea + 8);
            let val4 = ctx.debug_read_u32(ea + 12);
            data.push((
                ea,
                [
                    (val1 >> 24) as u8,
                    (val1 >> 16) as u8,
                    (val1 >> 8) as u8,
                    val1 as u8,
                    (val2 >> 24) as u8,
                    (val2 >> 16) as u8,
                    (val2 >> 8) as u8,
                    val2 as u8,
                    (val3 >> 24) as u8,
                    (val3 >> 16) as u8,
                    (val3 >> 8) as u8,
                    val3 as u8,
                    (val4 >> 24) as u8,
                    (val4 >> 16) as u8,
                    (val4 >> 8) as u8,
                    val4 as u8,
                ],
            ));
        }
        Memory { data }
    }
}

impl Registers {
    pub fn new(ctx: &rustcube::Context) -> Self {
        let spr = *ctx.cpu().spr();

        let spr_32 = [
            ("XER", spr[cpu::SPR_XER]),
            ("LR", spr[cpu::SPR_LR]),
            ("CTR", spr[cpu::SPR_CTR]),
            ("DSISR", spr[cpu::SPR_DSISR]),
            ("DAR", spr[cpu::SPR_DAR]),
            ("DEC", spr[cpu::SPR_DEC]),
            ("SDR1", spr[cpu::SPR_SDR1]),
            ("SRR0", spr[cpu::SPR_SRR0]),
            ("SRR1", spr[cpu::SPR_SRR1]),
            ("SPRG0", spr[cpu::SPR_SPRG0]),
            ("SPRG1", spr[cpu::SPR_SPRG0 + 1]),
            ("SPRG2", spr[cpu::SPR_SPRG0 + 2]),
            ("SPRG3", spr[cpu::SPR_SPRG0 + 3]),
            ("EAR", spr[cpu::SPR_EAR]),
            ("PVR", spr[cpu::SPR_PVR]),
            ("GQR0", spr[cpu::SPR_GQR0]),
            ("GQR1", spr[cpu::SPR_GQR0 + 1]),
            ("GQR2", spr[cpu::SPR_GQR0 + 2]),
            ("GQR3", spr[cpu::SPR_GQR0 + 3]),
            ("GQR4", spr[cpu::SPR_GQR0 + 4]),
            ("GQR5", spr[cpu::SPR_GQR0 + 5]),
            ("GQR6", spr[cpu::SPR_GQR0 + 6]),
            ("GQR7", spr[cpu::SPR_GQR0 + 7]),
            ("HID0", spr[cpu::SPR_HID0]),
            ("HID1", spr[cpu::SPR_HID1]),
            ("HID2", spr[cpu::SPR_HID2]),
            ("WPAR", spr[cpu::SPR_WPAR]),
            ("MMCR0", spr[cpu::SPR_MMCR0]),
            ("MMCR1", spr[cpu::SPR_MMCR1]),
            ("UMMCR0", spr[cpu::SPR_UMMCR0]),
            ("UMMCR1", spr[cpu::SPR_UMMCR1]),
            ("UPMC1", spr[cpu::SPR_UPMC1]),
            ("UPMC2", spr[cpu::SPR_UPMC2]),
            ("UPMC3", spr[cpu::SPR_UPMC3]),
            ("UPMC4", spr[cpu::SPR_UPMC4]),
            ("USIA", spr[cpu::SPR_USIA]),
            ("PMC1", spr[cpu::SPR_PMC1]),
            ("PMC2", spr[cpu::SPR_PMC2]),
            ("PMC3", spr[cpu::SPR_PMC3]),
            ("PMC4", spr[cpu::SPR_PMC4]),
            ("SIA", spr[cpu::SPR_SIA]),
            ("IABR", spr[cpu::SPR_IABR]),
            ("DABR", spr[cpu::SPR_DABR]),
            ("L2CR", spr[cpu::SPR_L2CR]),
            ("ICTC", spr[cpu::SPR_ICTC]),
            ("THRM1", spr[cpu::SPR_THRM1]),
            ("THRM2", spr[cpu::SPR_THRM1 + 1]),
            ("THRM3", spr[cpu::SPR_THRM1 + 2]),
        ];
        let spr_64 = [
            (
                "TB",
                (spr[cpu::SPR_TBU] as u64) << 32 | (spr[cpu::SPR_TBL] as u64),
            ),
            (
                "IBAT0",
                (spr[cpu::SPR_IBAT0U] as u64) << 32 | (spr[cpu::SPR_IBAT0L] as u64),
            ),
            (
                "IBAT1",
                (spr[cpu::SPR_IBAT1U] as u64) << 32 | (spr[cpu::SPR_IBAT1L] as u64),
            ),
            (
                "IBAT2",
                (spr[cpu::SPR_IBAT2U] as u64) << 32 | (spr[cpu::SPR_IBAT2L] as u64),
            ),
            (
                "IBAT3",
                (spr[cpu::SPR_IBAT3U] as u64) << 32 | (spr[cpu::SPR_IBAT3L] as u64),
            ),
            (
                "DBAT0",
                (spr[cpu::SPR_DBAT0U] as u64) << 32 | (spr[cpu::SPR_DBAT0L] as u64),
            ),
            (
                "DBAT1",
                (spr[cpu::SPR_DBAT1U] as u64) << 32 | (spr[cpu::SPR_DBAT1L] as u64),
            ),
            (
                "DBAT2",
                (spr[cpu::SPR_DBAT2U] as u64) << 32 | (spr[cpu::SPR_DBAT2L] as u64),
            ),
            (
                "DBAT3",
                (spr[cpu::SPR_DBAT3U] as u64) << 32 | (spr[cpu::SPR_DBAT3L] as u64),
            ),
            (
                "DMA",
                (spr[cpu::SPR_DMAU] as u64) << 32 | (spr[cpu::SPR_DMAU + 1] as u64),
            ),
        ];

        Registers {
            gpr: *ctx.cpu().gpr(),
            fpr: ctx.cpu().fpr().clone(),
            spr_32,
            spr_64,
        }
    }
}
