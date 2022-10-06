use crate::{Callstack, Disassembly, Event, Memory, Registers};

use async_channel::{Receiver, Sender};
use rustcube::cpu::disassembler::Disassembler;

pub enum BgEvent {
    Continue,
    Step,
    Stop,
    BreakpointToggle(u32),
}

pub async fn run(mut ctx: rustcube::Context, tx: Sender<Event>, rx: Receiver<BgEvent>) {
    let disassembler = Disassembler::default();

    let mut window_start = 0;
    let regs = Box::new(Registers::new(&ctx));
    let disassembly = Box::new(Disassembly::new(&mut ctx, &disassembler, &mut window_start));
    let callstack = Box::new(Callstack::new(&mut ctx));
    let memory = Box::new(Memory::new(&mut ctx));
    let _ = tx.send(Event::Registers(regs)).await;
    let _ = tx.send(Event::Disassembly(disassembly)).await;
    let _ = tx.send(Event::Callstack(callstack)).await;
    let _ = tx.send(Event::Memory(memory)).await;

    while let Ok(event) = rx.recv().await {
        match event {
            BgEvent::BreakpointToggle(addr) => {
                if ctx.breakpoints().contains(&addr) {
                    ctx.remove_breakpoint(addr);
                } else {
                    ctx.add_breakpoint(addr);
                }
            }
            BgEvent::Continue => {
                while rx.is_empty() {
                    if ctx.step() != None {
                        break;
                    }
                }

                let regs = Box::new(Registers::new(&ctx));
                let disassembly =
                    Box::new(Disassembly::new(&mut ctx, &disassembler, &mut window_start));
                let callstack = Box::new(Callstack::new(&mut ctx));
                let memory = Box::new(Memory::new(&mut ctx));
                let _ = tx.send(Event::Registers(regs)).await;
                let _ = tx.send(Event::Disassembly(disassembly)).await;
                let _ = tx.send(Event::Callstack(callstack)).await;
                let _ = tx.send(Event::Memory(memory)).await;
            }
            BgEvent::Step => {
                ctx.step();

                let regs = Box::new(Registers::new(&ctx));
                let disassembly =
                    Box::new(Disassembly::new(&mut ctx, &disassembler, &mut window_start));
                let callstack = Box::new(Callstack::new(&mut ctx));
                let memory = Box::new(Memory::new(&mut ctx));
                let _ = tx.send(Event::Registers(regs)).await;
                let _ = tx.send(Event::Disassembly(disassembly)).await;
                let _ = tx.send(Event::Callstack(callstack)).await;
                let _ = tx.send(Event::Memory(memory)).await;
            }
            BgEvent::Stop => {}
        }
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

            while sp != 0 {
                let address = ctx.read_u32(sp + 4);
                sp = ctx.read_u32(sp);

                if address != 0 {
                    addresses.push(address);
                }
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
        let mut pc = ctx.cpu().pc() - 100;

        let window_size = 1000;

        if pc < *window_start || pc > *window_start + window_size {
            *window_start = pc;
        }

        pc = *window_start;

        for _ in 0..window_size {
            let addr = ctx.cpu().translate_instr_address(pc);
            let code = ctx.read_instruction(addr);
            let disinstr = disassem.decode(pc, code);

            instructions.push(disinstr);

            pc += 4;
        }

        Disassembly {
            pc: ctx.cpu().pc(),
            instructions,
            breakpoints: ctx.breakpoints().clone(),
        }
    }
}

impl Memory {
    pub fn new(ctx: &mut rustcube::Context) -> Self {
        let mut data = Vec::new();

        let window_start = 0x8000_3000; //0x8000_0000;
        let window_size = 896;

        for ea in (window_start..(window_start + window_size)).step_by(16) {
            let val1 = ctx.read_u32(ea);
            let val2 = ctx.read_u32(ea + 4);
            let val3 = ctx.read_u32(ea + 8);
            let val4 = ctx.read_u32(ea + 12);
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
        Registers {
            gpr: *ctx.cpu().gpr(),
            fpr: *ctx.cpu().fpr(),
            spr: *ctx.cpu().spr(),
        }
    }
}
