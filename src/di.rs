use crate::pi::{clear_interrupt, set_interrupt, PI_INTERRUPT_DI};
use crate::Context;

const STATUS: u32 = 0x00;
const COVER_STATUS: u32 = 0x04;

#[derive(Debug, Default)]
pub struct DvdInterface {
    status: StatusRegister,
    cover_status: CoverStatusRegister,
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    pub struct StatusRegister(u32);
    impl Debug;
    pub di_break, _ : 0;
    pub device_int_mask, _ : 1;
    pub device_int, _ : 2;
    pub transfer_int_mask, _ : 3;
    pub transfer_int, _ : 4;
    pub break_int_mask, _ : 5;
    pub break_int, _ : 6;
}

impl From<u32> for StatusRegister {
    fn from(v: u32) -> Self {
        StatusRegister(v)
    }
}

impl From<StatusRegister> for u32 {
    fn from(s: StatusRegister) -> u32 {
        s.0
    }
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    pub struct CoverStatusRegister(u32);
    impl Debug;
    pub cover, _ : 0;
    pub cover_int_mask, set_cover_int_mask : 1;
    pub cover_int, set_cover_int : 2;
}

impl From<u32> for CoverStatusRegister {
    fn from(v: u32) -> Self {
        CoverStatusRegister(v)
    }
}

impl From<CoverStatusRegister> for u32 {
    fn from(s: CoverStatusRegister) -> u32 {
        s.0
    }
}

pub fn read_u32(ctx: &mut Context, register: u32) -> u32 {
    match register {
        COVER_STATUS => ctx.di.cover_status.into(),
        _ => {
            warn!("read_u32 unrecognized di register {:#x}", register);
            0
        }
    }
}

pub fn write_u32(ctx: &mut Context, register: u32, val: u32) {
    match register {
        STATUS => {
            ctx.di.status = val.into();
            update_interrupts(ctx);
        }
        COVER_STATUS => {
            let tmp: CoverStatusRegister = val.into();

            ctx.di.cover_status.set_cover_int_mask(tmp.cover_int_mask());

            if tmp.cover_int() {
                ctx.di.cover_status.set_cover_int(false);
            }

            ctx.di.cover_status = val.into();
            update_interrupts(ctx);
        }
        _ => warn!("write_u32 unrecognized di register {:#x}", register),
    }
}

fn update_interrupts(ctx: &mut Context) {
    if ctx.di.status.device_int() && ctx.di.status.device_int_mask()
        || ctx.di.status.transfer_int() && ctx.di.status.transfer_int_mask()
        || ctx.di.status.break_int() && ctx.di.status.break_int_mask()
        || ctx.di.cover_status.cover_int() && ctx.di.cover_status.cover_int_mask()
    {
        set_interrupt(ctx, PI_INTERRUPT_DI);
    } else {
        clear_interrupt(ctx, PI_INTERRUPT_DI);
    }
}
