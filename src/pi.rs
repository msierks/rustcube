use crate::Context;

const PI_INTERRUPT_CAUSE: u32 = 0x00;
const PI_INTERRUPT_MASK: u32 = 0x04;
const PI_FIFO_BASE_START: u32 = 0x0C;
const PI_FIFO_BASE_END: u32 = 0x10;
const PI_FIFO_WRITE_POINTER: u32 = 0x14;
//const PI_FIFO_RESET: u32 = 0x18;
const PI_CONFIG: u32 = 0x24;
const PI_REVISION: u32 = 0x2C;
const PI_UNKNOWN: u32 = 0x30;

// Flipper ID Revision C as per Dolphin Emulator
const FLIPPER_REV: u32 = 0x2465_00B1;

pub const PI_INTERRUPT_RSWST: u32 = 0x10000; // Reset Switch State (1 when pressed)
pub const PI_INTERRUPT_HSP: u32 = 0x02000; // High Speed Port
pub const PI_INTERRUPT_DEBUG: u32 = 0x01000; // Debug Hardware
pub const PI_INTERRUPT_CP: u32 = 0x0800; // Command FIFO
pub const PI_INTERRUPT_PE_FINISH: u32 = 0x0400; // GP FInished
pub const PI_INTERRUPT_PE_TOKEN: u32 = 0x0200; // GP Token
pub const PI_INTERRUPT_VI: u32 = 0x00100; // Video Interface
pub const PI_INTERRUPT_MEM: u32 = 0x0080; // Memory Interface
pub const PI_INTERRUPT_DSP: u32 = 0x0040; // DSP Interface
pub const PI_INTERRUPT_AI: u32 = 0x0020; // Audio Interface Streaming
pub const PI_INTERRUPT_EXI: u32 = 0x0010; // External Interface
pub const PI_INTERRUPT_SI: u32 = 0x0008; // Serial Interface
pub const PI_INTERRUPT_DI: u32 = 0x0004; // DVD Interface
pub const PI_INTERRUPT_RSW: u32 = 0x0002; // Reset Switch
pub const PI_INTERRUPT_ERROR: u32 = 0x0001; // GP Runtime Error

#[derive(Debug)]
pub struct ProcessorInterface {
    interrupt_cause: u32,
    interrupt_mask: u32,
    fifo_start: u32,
    fifo_end: u32,
    fifo_write_pointer: u32,
    config: u32,
    revision: u32,
    unknown: u32,
}

impl Default for ProcessorInterface {
    fn default() -> Self {
        ProcessorInterface {
            interrupt_mask: 0,
            interrupt_cause: 0,
            fifo_start: 0,
            fifo_end: 0,
            fifo_write_pointer: 0,
            config: 0,
            revision: FLIPPER_REV,
            unknown: 0,
        }
    }
}

impl ProcessorInterface {
    pub fn fifo_start(&self) -> u32 {
        self.fifo_start
    }

    pub fn fifo_end(&self) -> u32 {
        self.fifo_end
    }

    pub fn fifo_write_pointer(&self) -> u32 {
        self.fifo_write_pointer
    }

    pub fn set_fifo_write_pointer(&mut self, val: u32) {
        self.fifo_write_pointer = val;
    }
}

pub fn read_u32(ctx: &mut Context, register: u32) -> u32 {
    match register {
        PI_INTERRUPT_CAUSE => ctx.pi.interrupt_cause,
        PI_INTERRUPT_MASK => ctx.pi.interrupt_mask,
        PI_CONFIG => ctx.pi.config,
        PI_REVISION => ctx.pi.revision,
        _ => {
            warn!("read_u32 unrecognized register {register:#x}");
            0
        }
    }
}

pub fn write_u32(ctx: &mut Context, register: u32, val: u32) {
    match register {
        PI_INTERRUPT_CAUSE => {
            ctx.pi.interrupt_cause &= !val;
            update_exception(ctx);
        }
        PI_INTERRUPT_MASK => {
            ctx.pi.interrupt_mask = val;
            update_exception(ctx);
        }
        PI_FIFO_BASE_START => ctx.pi.fifo_start = val,
        PI_FIFO_BASE_END => ctx.pi.fifo_end = val,
        PI_FIFO_WRITE_POINTER => ctx.pi.fifo_write_pointer = val,
        PI_CONFIG => {
            // TODO: dig into the purpose of this register
            // not sure why the DVDlowReset writes 0x3, waits some amount of time, then writes 0x7
            ctx.pi.config = val;
        }
        PI_UNKNOWN => ctx.pi.unknown = val,
        _ => warn!("write_u32 unrecognized register {register:#x}"),
    }
}

pub fn clear_interrupt(ctx: &mut Context, cause: u32) {
    if ctx.pi.interrupt_cause & cause != 0 {
        info!("Interrupt {} (clear)", interrupt_name(cause));
    }

    ctx.pi.interrupt_cause &= !cause;

    update_exception(ctx);
}

pub fn set_interrupt(ctx: &mut Context, cause: u32) {
    if ctx.pi.interrupt_cause & cause == 0 {
        info!("Interrupt {} (set)", interrupt_name(cause));
    }

    ctx.pi.interrupt_cause |= cause;

    update_exception(ctx);
}

pub fn update_exception(ctx: &mut Context) {
    if ctx.pi.interrupt_cause & ctx.pi.interrupt_mask != 0 {
        ctx.cpu.external_interrupt(true);
    } else {
        ctx.cpu.external_interrupt(false);
    }
}

fn interrupt_name(interrupt: u32) -> &'static str {
    match interrupt {
        PI_INTERRUPT_ERROR => "PI_INTERRUPT_ERROR",
        PI_INTERRUPT_RSW => "PI_INTERRUPT_RSW",
        PI_INTERRUPT_DI => "PI_INTERRUPT_DI",
        PI_INTERRUPT_SI => "PI_INTERRUPT_SI",
        PI_INTERRUPT_EXI => "PI_INTERRUPT_EXI",
        PI_INTERRUPT_AI => "PI_INTERRUPT_AI",
        PI_INTERRUPT_DSP => "PI_INTERRUPT_DSP",
        PI_INTERRUPT_MEM => "PI_INTERRUPT_MEM",
        PI_INTERRUPT_VI => "PI_INTERRUPPT_VI",
        PI_INTERRUPT_PE_TOKEN => "PI_INTERRUPT_PE_TOKEN",
        PI_INTERRUPT_PE_FINISH => "PI_INTERUPT_PE_FINISH",
        PI_INTERRUPT_CP => "PI_INTERRUPT_CP",
        PI_INTERRUPT_DEBUG => "PI_INTERRUPT_DEBUG",
        PI_INTERRUPT_HSP => "PI_INTERRUPT_HSP",
        PI_INTERRUPT_RSWST => "PI_INTERRUPT_RSWST",
        _ => "UNKNOWN",
    }
}
