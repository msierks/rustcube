use crate::emu::Context;

const MAILBOX_IN_HIGH: u32 = 0x00;
const MAILBOX_IN_LOW: u32 = 0x02;
const MAILBOX_OUT_HIGH: u32 = 0x04;
const MAILBOX_OUT_LOW: u32 = 0x06;
const CONTROL_STATUS: u32 = 0x0A;
const AR_SIZE: u32 = 0x12;
const AR_MODE: u32 = 0x16;
const AR_REFRESH: u32 = 0x1A;
const AR_DMA_MMAADDR_HIGH: u32 = 0x20;
const AR_DMA_MMAADDR_LOW: u32 = 0x22;
const AR_DMA_ARADDR_HIGH: u32 = 0x24;
const AR_DMA_ARADDR_LOW: u32 = 0x26;
const AR_DMA_SIZE_HIGH: u32 = 0x28;
const AR_DMA_SIZE_LOW: u32 = 0x2A;

#[derive(Debug, Default)]
pub struct DspInterface {
    mailbox_high: u16,
    mailbox_low: u16,
    control_register: ControlRegister,
    ar_size: u16,
    ar_mode: u16,
    ar_refresh: u16,
    ar_mma_addr_high: u16,
    ar_mma_addr_low: u16,
    ar_ar_addr_high: u16,
    ar_ar_addr_low: u16,
    ar_dma_size_high: u16,
    ar_dma_size_low: u16,
}

bitfield! {
    #[derive(Copy, Clone)]
    pub struct ControlRegister(u16);
    impl Debug;
    pub reset, _: 0;
    pub interrupt, _ : 1;
    pub halt, _ : 2;
    pub ai_interrupt, _ : 3;
    pub ai_interrupt_mask, _ : 4;
    pub aram_interrupt, _ : 5;
    pub aram_interrupt_mask, _ : 6;
    pub dsp_interrupt, _ : 7;
    pub dsp_interrupt_mask, _ : 8;
    pub dma_state, _ : 9;
    pub init_code, _ : 10;
    pub dsp_init, _ : 11;
}

impl Default for ControlRegister {
    fn default() -> Self {
        ControlRegister(0)
    }
}

impl From<u16> for ControlRegister {
    fn from(v: u16) -> Self {
        ControlRegister(v)
    }
}

impl From<ControlRegister> for u16 {
    fn from(s: ControlRegister) -> u16 {
        s.0
    }
}

pub fn read_u16(ctx: &mut Context, register: u32) -> u16 {
    match register {
        MAILBOX_OUT_HIGH => {
            let val = ctx.dsp.mailbox_high;
            ctx.dsp.mailbox_high = 0;
            val
        }
        MAILBOX_OUT_LOW => ctx.dsp.mailbox_low,
        CONTROL_STATUS => ctx.dsp.control_register.into(),
        AR_SIZE => ctx.dsp.ar_size,
        AR_REFRESH => ctx.dsp.ar_refresh,
        _ => {
            warn!("Warn: unrecognized dsp register {:#x}", register);
            0
        }
    }
}

pub fn write_u16(ctx: &mut Context, register: u32, val: u16) {
    match register {
        MAILBOX_IN_HIGH => ctx.dsp.mailbox_high = 0x8000,
        MAILBOX_IN_LOW => {
            ctx.dsp.mailbox_high = 0x0000; // check if this is necessary
            ctx.dsp.mailbox_low = val
        }
        CONTROL_STATUS => {
            ctx.dsp.control_register = val.into();
            //self.control_register.dsp_reset = false;
        }
        AR_SIZE => ctx.dsp.ar_size = val,
        AR_MODE => ctx.dsp.ar_mode = val,
        AR_REFRESH => ctx.dsp.ar_refresh = val,
        AR_DMA_MMAADDR_HIGH => ctx.dsp.ar_mma_addr_high = val,
        AR_DMA_MMAADDR_LOW => ctx.dsp.ar_mma_addr_low = val,
        AR_DMA_ARADDR_HIGH => ctx.dsp.ar_ar_addr_high = val,
        AR_DMA_ARADDR_LOW => ctx.dsp.ar_ar_addr_low = val,
        AR_DMA_SIZE_HIGH => ctx.dsp.ar_dma_size_high = val,
        AR_DMA_SIZE_LOW => {
            ctx.dsp.mailbox_high = 0x8000;
            ctx.dsp.ar_dma_size_low = val;
        }
        _ => warn!("Warn: unrecognized dsp register {:#x}", register),
    }
}
