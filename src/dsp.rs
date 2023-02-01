use crate::utils::Halveable;
use crate::Context;

const MAILBOX_DSP_HI: u32 = 0x00;
const MAILBOX_DSP_LO: u32 = 0x02;
const MAILBOX_CPU_HI: u32 = 0x04;
const MAILBOX_CPU_LO: u32 = 0x06;
const CONTROL_STATUS: u32 = 0x0A;
const AR_SIZE: u32 = 0x12;
const AR_MODE: u32 = 0x16;
const AR_REFRESH: u32 = 0x1A;
const AR_DMA_MMAADDR_HI: u32 = 0x20;
const AR_DMA_MMAADDR_LO: u32 = 0x22;
const AR_DMA_ARADDR_HI: u32 = 0x24;
const AR_DMA_ARADDR_LO: u32 = 0x26;
const AR_DMA_SIZE_HI: u32 = 0x28;
const AR_DMA_SIZE_LO: u32 = 0x2A;

#[derive(Debug, Default)]
pub struct DspInterface {
    mailbox_dsp: u32,
    mailbox_cpu: u32,
    control_register: ControlRegister,
    ar_size: u16,
    ar_mode: u16,
    ar_refresh: u16,
    ar_mma_addr: u32,
    ar_ar_addr: u32,
    ar_dma_size: u32,
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    pub struct ControlRegister(u16);
    impl Debug;
    pub reset, set_reset: 0;
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
        MAILBOX_CPU_HI => {
            let val = ctx.dsp.mailbox_cpu.hi();
            ctx.dsp.mailbox_cpu = ctx.dsp.mailbox_cpu.set_hi(0);
            val
        }
        MAILBOX_CPU_LO => ctx.dsp.mailbox_cpu.lo(),
        CONTROL_STATUS => ctx.dsp.control_register.into(),
        AR_SIZE => ctx.dsp.ar_size,
        AR_REFRESH => ctx.dsp.ar_refresh,
        _ => {
            warn!("read_u16 unrecognized dsp register {:#x}", register);
            0
        }
    }
}

pub fn write_u16(ctx: &mut Context, register: u32, val: u16) {
    match register {
        MAILBOX_DSP_HI => ctx.dsp.mailbox_dsp = ctx.dsp.mailbox_dsp.set_hi(val),
        MAILBOX_DSP_LO => ctx.dsp.mailbox_dsp = ctx.dsp.mailbox_dsp.set_lo(val),
        MAILBOX_CPU_HI => {
            ctx.dsp.mailbox_cpu = ctx.dsp.mailbox_cpu.set_hi(val);
            //ctx.dsp.mailbox_cpu = ctx.dsp.mailbox_cpu.set_hi(0x8000);
        }
        MAILBOX_CPU_LO => {
            ctx.dsp.mailbox_cpu = val as u32;
        }
        CONTROL_STATUS => {
            if val & 1 == 1 {
                info!("DSP reset");
            }
            ctx.dsp.control_register = val.into();
            ctx.dsp.control_register.set_reset(false);
        }
        AR_SIZE => ctx.dsp.ar_size = val,
        AR_MODE => ctx.dsp.ar_mode = val,
        AR_REFRESH => ctx.dsp.ar_refresh = val,
        AR_DMA_MMAADDR_HI => ctx.dsp.ar_mma_addr = ctx.dsp.ar_mma_addr.set_hi(val),
        AR_DMA_MMAADDR_LO => ctx.dsp.ar_mma_addr = ctx.dsp.ar_mma_addr.set_lo(val),
        AR_DMA_ARADDR_HI => ctx.dsp.ar_ar_addr = ctx.dsp.ar_ar_addr.set_hi(val),
        AR_DMA_ARADDR_LO => ctx.dsp.ar_ar_addr = ctx.dsp.ar_ar_addr.set_lo(val),
        AR_DMA_SIZE_HI => ctx.dsp.ar_dma_size = ctx.dsp.ar_dma_size.set_hi(val),
        AR_DMA_SIZE_LO => {
            ctx.dsp.mailbox_cpu = ctx.dsp.mailbox_cpu.set_hi(0x8000);
            ctx.dsp.ar_dma_size = ctx.dsp.ar_dma_size.set_lo(val)
        }
        _ => warn!("write_u16 unrecognized dsp register {:#x}", register),
    }
}
