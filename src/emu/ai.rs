use crate::emu::Context;

const CONTROL_STATUS: u32 = 0x00;

#[derive(Debug, Default)]
pub struct AudioInterface {
    control_register: ControlRegister,
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    pub struct ControlRegister(u32);
    impl Debug;
    // PSTAT (Playing Status)
    pub playing_status, _ : 0;
    // AFR (Auxiliary Frequency Register)
    pub auxiliary_frequency, _ : 1;
    // AIINTMSK (Audio interface Interrupt Mask)
    pub ai_interrupt_mask, _ : 2;
    // AIINT (Audio Interface Interrupt Status and clear)
    pub ai_interrupt, _ : 3;
    // AIINTVLD (Audio Interface Interrupt Valid)
    pub ai_interrupt_valid, _ : 4;
    // SCRESET (Sample Counter Reset)
    pub sample_count_reset, _ : 5;
    // DSP (Sample Rate)
    pub dsp_sample_rate, _ : 6;
}

impl From<u32> for ControlRegister {
    fn from(v: u32) -> Self {
        ControlRegister(v)
    }
}

impl From<ControlRegister> for u32 {
    fn from(s: ControlRegister) -> u32 {
        s.0
    }
}

pub fn read_u32(ctx: &mut Context, register: u32) -> u32 {
    match register {
        CONTROL_STATUS => ctx.ai.control_register.into(),
        _ => {
            warn!("read_u32 unrecognized ai register {:#x}", register);
            0
        }
    }
}

pub fn write_u32(ctx: &mut Context, register: u32, val: u32) {
    match register {
        CONTROL_STATUS => {
            ctx.ai.control_register = val.into();
            //self.control_register.dsp_reset = false;
        }
        _ => warn!("write_u32 unrecognized ai register {:#x}", register),
    }
}
