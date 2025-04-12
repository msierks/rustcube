use crate::pi::{clear_interrupt, set_interrupt, PI_INTERRUPT_AI};
use crate::Context;

const AI_CONTROL_STATUS: u32 = 0x00;
const AI_VOLUME: u32 = 0x04;
const AI_SAMPLE_COUNTER: u32 = 0x08;
const AI_INTERRUPT_TIMING: u32 = 0x0c;

// 0 - 48 kHz
// 1 - 32 kHz
static SAMPLE_RATE: [u32; 2] = [48000, 32000];

#[derive(Debug, Default)]
pub struct AudioInterface {
    control: ControlRegister,
    volume: u32,
    sample_counter: u32,
    interrupt_timing: u32,
    sample_rate: u32,
    cycles_per_sample: u32,
    cpu_ticks: u64,
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    pub struct ControlRegister(u32);
    impl Debug;
    // PSTAT (Playing Status)
    pub pstat, set_pstat : 0;
    // AFR (Auxiliary Frequency Register)
    pub afr, set_afr : 1;
    // AIINTMSK (Audio interface Interrupt Mask)
    pub aiintmsk, set_aiintmsk : 2;
    // AIINT (Audio Interface Interrupt Status and clear)
    pub aiint, set_aiint : 3;
    // AIINTVLD (Audio Interface Interrupt Valid)
    pub ai_interrupt_valid, set_ai_interrupt_valid : 4;
    // SCRESET (Sample Counter Reset)
    pub screset, set_screset : 5;
    // DSP (Sample Rate)
    pub dsp, _ : 6;
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
        AI_CONTROL_STATUS => ctx.ai.control.into(),
        AI_SAMPLE_COUNTER => ctx.ai.sample_counter,
        AI_VOLUME => ctx.ai.volume,
        _ => {
            warn!("read_u32 unrecognized ai register {:#x}", register);
            0
        }
    }
}

pub fn write_u32(ctx: &mut Context, register: u32, val: u32) {
    match register {
        AI_CONTROL_STATUS => {
            let new_config = ControlRegister(val);
            let config = &mut ctx.ai.control;

            if new_config.aiintmsk() != config.aiintmsk() {
                info!("Change ai_interrupt_mask to {}", new_config.aiintmsk());
                config.set_aiintmsk(new_config.aiintmsk());
            }

            if new_config.ai_interrupt_valid() != config.ai_interrupt_valid() {
                info!(
                    "Change ai_interrupt_valid to {}",
                    new_config.ai_interrupt_valid()
                );
                config.set_ai_interrupt_valid(new_config.ai_interrupt_valid());
            }

            if new_config.afr() != config.afr() {
                config.set_afr(new_config.afr());
                ctx.ai.sample_rate = SAMPLE_RATE[config.afr() as usize];
                ctx.ai.cycles_per_sample = 486000000 / ctx.ai.sample_rate;
            }

            if new_config.aiint() {
                info!("CLEAR INTERRUPT");
                config.set_aiint(false);
            }

            if new_config.pstat() != config.pstat() {
                config.set_pstat(new_config.pstat());

                if new_config.pstat() {
                    info!("start streaming audio");
                } else {
                    info!("stop streaming audio");
                }

                ctx.ai.cpu_ticks = ctx.timers.get_ticks();
            }

            if new_config.screset() {
                ctx.ai.sample_counter = 0;

                ctx.ai.cpu_ticks = ctx.timers.get_ticks();
            }

            if config.aiint() && config.aiintmsk() {
                panic!("interrupt");
            }

            update_interrupts(ctx);
        }
        AI_VOLUME => ctx.ai.volume = val,
        AI_INTERRUPT_TIMING => ctx.ai.interrupt_timing = val,
        _ => panic!("write_u32 unrecognized ai register {register:#x}"),
    }
}

fn update_interrupts(ctx: &mut Context) {
    if ctx.ai.control.aiint() && ctx.ai.control.aiintmsk() {
        set_interrupt(ctx, PI_INTERRUPT_AI);
    } else {
        clear_interrupt(ctx, PI_INTERRUPT_AI);
    }
}

pub fn update(ctx: &mut Context) {
    let ticks = ctx.get_ticks();
    if ticks - ctx.ai.cpu_ticks > 600 {
        ctx.ai.cpu_ticks = ticks;
    } else {
        return;
    }

    if !ctx.ai.control.pstat() {
        return;
    }

    if ctx.ai.sample_counter > ctx.ai.interrupt_timing {
        ctx.ai.control.set_aiint(true);

        update_interrupts(ctx);
    }

    ctx.ai.sample_counter += 1;
}
