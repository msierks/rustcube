use crate::pi::{clear_interrupt, set_interrupt, PI_INTERRUPT_DI};
use crate::Context;
use crate::Disc;

const STATUS: u32 = 0x00;
const COVER_STATUS: u32 = 0x04;
const DICMDBUF0: u32 = 0x08;
const DICMDBUF1: u32 = 0x0C;
const DICMDBUF2: u32 = 0x10;
const DIMAR: u32 = 0x14;
const DILENGTH: u32 = 0x18;
const DICR: u32 = 0x1C;
//const DIIMMBUF: u32 = 0x20;
const DICFG: u32 = 0x24;

const DI_COMMAND_INQUIRY: u8 = 0x12;
//const DI_COMMAND_READ: u8 = 0xA8;
//const DI_COMMAND_SEEK: u8 = 0xAB;

#[derive(Default)]
pub struct DvdInterface {
    status: StatusRegister,
    cover_status: CoverStatusRegister,
    command_buff_0: u32,
    command_buff_1: u32,
    command_buff_2: u32,
    dma_address: u32,
    dma_transfer_length: u32,
    control: ControlRegister,
    config: u32,
    disc: Option<Disc>,
}

impl DvdInterface {
    pub fn set_disc(&mut self, disc: Option<Disc>) {
        self.disc = disc;
    }
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

bitfield! {
    #[derive(Copy, Clone, Default)]
    pub struct ControlRegister(u32);
    impl Debug;
    pub tstart, set_tstart : 0;
    pub dma, _ : 1;
    pub rw, _ : 2;
}

impl From<u32> for ControlRegister {
    fn from(v: u32) -> Self {
        ControlRegister(v)
    }
}

pub fn read_u32(ctx: &mut Context, register: u32) -> u32 {
    match register {
        COVER_STATUS => ctx.di.cover_status.into(),
        DICFG => ctx.di.config,
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
        DICMDBUF0 => ctx.di.command_buff_0 = val,
        DICMDBUF1 => ctx.di.command_buff_1 = val,
        DICMDBUF2 => ctx.di.command_buff_2 = val,
        DIMAR => ctx.di.dma_address = val,
        DILENGTH => ctx.di.dma_transfer_length = val,
        DICR => {
            ctx.di.control = val.into();
            if ctx.di.control.tstart() {
                // Execute Command
                match (ctx.di.command_buff_0 >> 24) as u8 {
                    DI_COMMAND_INQUIRY => (), // Not sure what happens here
                    //DI_COMMAND_READ => (),
                    //DI_COMMAND_SEEK => (),
                    _ => warn!("Unrecognized command {:#x}", ctx.di.command_buff_0),
                }
            }

            ctx.di.control.set_tstart(false);
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
