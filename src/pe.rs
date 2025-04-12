use crate::pi::{clear_interrupt, set_interrupt, PI_INTERRUPT_PE_FINISH, PI_INTERRUPT_PE_TOKEN};
use crate::Context;

const PE_Z_CONFIG: u32 = 0x00;
const PE_ALPHA_CONFIG: u32 = 0x02;
const PE_DESTINATION_ALPHA: u32 = 0x04;
const PE_ALPHA_MODE: u32 = 0x06;
const PE_ALPHA_READ: u32 = 0x08;
const PE_CONTROL: u32 = 0x0A;
const PE_TOKEN: u32 = 0x0E;

bitfield! {
    #[derive(Copy, Clone, Default)]
    pub struct ControlRegister(u16);
    impl Debug;
    // PE_TOKEN_ENABLE
    pub pe_token_enable, _ : 0;
    // PE_FINISH_ENABLE
    pub pe_finish_enable, _ : 1;
    // PE_TOKEN
    pub pe_token, set_pe_token : 2;
    // PE_FINISH
    pub pe_finish, set_pe_finish : 3;
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

#[derive(Default)]
pub struct PixelEngine {
    z_config: u16,
    alpha_config: u16,
    destination_alpha: u16,
    alpha_mode: u16,
    alpha_read: u16,
    control: ControlRegister,
    token: u16,
    signal_token_interrupt: bool,
    signal_finish_interrupt: bool,
}

pub fn read_u16(ctx: &mut Context, register: u32) -> u16 {
    match register {
        PE_Z_CONFIG => ctx.pe.z_config,
        PE_ALPHA_CONFIG => ctx.pe.alpha_config,
        PE_DESTINATION_ALPHA => ctx.pe.destination_alpha,
        PE_ALPHA_MODE => ctx.pe.alpha_mode,
        PE_ALPHA_READ => ctx.pe.alpha_read,
        PE_CONTROL => ctx.pe.control.into(),
        PE_TOKEN => ctx.pe.token,
        _ => {
            warn!("read_u16 unrecoognized pe register {:#x}", register);
            0
        }
    }
}

pub fn write_u16(ctx: &mut Context, register: u32, val: u16) {
    match register {
        PE_Z_CONFIG => ctx.pe.z_config = val,
        PE_ALPHA_CONFIG => ctx.pe.alpha_config = val,
        PE_DESTINATION_ALPHA => ctx.pe.destination_alpha = val,
        PE_ALPHA_MODE => ctx.pe.alpha_mode = val,
        PE_ALPHA_READ => ctx.pe.alpha_read = val,
        PE_CONTROL => {
            let control: ControlRegister = val.into();

            if control.pe_token() {
                ctx.pe.signal_token_interrupt = false;
            }

            if control.pe_finish() {
                ctx.pe.signal_finish_interrupt = false;
            }

            ctx.pe.control = control;

            ctx.pe.control.set_pe_token(false);
            ctx.pe.control.set_pe_finish(false);

            update_interrupts(ctx);
        }
        PE_TOKEN => ctx.pe.token = val,
        _ => warn!("write_u16 unrecoognized pe register {:#x}", register),
    }
}

fn update_interrupts(ctx: &mut Context) {
    if ctx.pe.signal_token_interrupt && ctx.pe.control.pe_token_enable() {
        set_interrupt(ctx, PI_INTERRUPT_PE_TOKEN);
    } else {
        clear_interrupt(ctx, PI_INTERRUPT_PE_TOKEN);
    }

    if ctx.pe.signal_finish_interrupt && ctx.pe.control.pe_finish_enable() {
        set_interrupt(ctx, PI_INTERRUPT_PE_FINISH);
    } else {
        clear_interrupt(ctx, PI_INTERRUPT_PE_FINISH);
    }
}
