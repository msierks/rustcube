use crate::emu::Context;

const INTERRUPT_CAUSE: u32 = 0x00;
const INTERRUPT_MASK: u32 = 0x04;
const FIFO_BASE_START: u32 = 0x0C;
const FIFO_BASE_END: u32 = 0x10;
const FIFO_WRITE_POINTER: u32 = 0x14;
// Writing anything here should cause a reset
// BS1 reads, then writes 0x1 here
const RESET_CODE: u32 = 0x24;
// BS1 does write 0x0245248A here, purpose unknown (dolphine-emu indicates the same)
const UNKNOWN: u32 = 0x30;

const INT_ERROR: u32 = 0x1;
const INT_RSW: u32 = 0x2;
const INT_DI: u32 = 0x4;
const INT_SI: u32 = 0x8;
const INT_EXI: u32 = 0x10;
const INT_AI: u32 = 0x20;
const INT_DSP: u32 = 0x40;
const INT_MEM: u32 = 0x80;
const INT_VI: u32 = 0x100;
const INT_PE_TOKEN: u32 = 0x200;
const INT_PE_FINISH: u32 = 0x400;
const INT_CP: u32 = 0x800;
const INT_DEBUG: u32 = 0x1000;
const INT_HSP: u32 = 0x2000;
const INT_RSWST: u32 = 0x10000;

#[derive(Debug, Default)]
pub struct ProcessorInterface {
    interrupt_cause: u32,
    interrupt_mask: u32,
    fifo_start: u32,
    fifo_end: u32,
    fifo_write_pointer: u32,
    unknown: u32,
}

pub fn read_u32(ctx: &mut Context, register: u32) -> u32 {
    match register {
        INTERRUPT_CAUSE => ctx.pi.interrupt_cause,
        INTERRUPT_MASK => ctx.pi.interrupt_mask,
        RESET_CODE => {
            info!("Read PI:RESET_CODE");
            0
        }
        _ => {
            warn!("read_u32 unrecognized register {:#x}", register);
            0
        }
    }
}

pub fn write_u32(ctx: &mut Context, register: u32, val: u32) {
    match register {
        INTERRUPT_CAUSE => ctx.pi.interrupt_cause = val,
        INTERRUPT_MASK => ctx.pi.interrupt_mask = val,
        RESET_CODE => warn!("Write PI:RESET_CODE {:#x}", val),
        UNKNOWN => ctx.pi.unknown = val,
        _ => warn!("write_u32 unrecognized register {:#x}", register),
    }
}

pub fn set_interrupt(ctx: &mut Context, cause: u32) {
    ctx.pi.interrupt_cause |= cause;

    if ctx.pi.interrupt_cause & ctx.pi.interrupt_mask != 0 {
        // trigger cpu external interrupt exception
        ctx.cpu.external_interrupt();
    }
}
