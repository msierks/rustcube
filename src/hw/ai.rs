use crate::{
    bus::Bus,
    cpu::{timers::CPU_CLOCK, CpuState},
    hw::{
        mmio::{Mmio, MmioDevice},
        pi::{ProcessorInterface, PI_INTERRUPT_AI},
    },
};

const AI_CONTROL_STATUS: u32 = 0x00;
const AI_VOLUME: u32 = 0x04;
const AI_SAMPLE_COUNTER: u32 = 0x08;
const AI_INTERRUPT_TIMING: u32 = 0x0c;

const SAMPLE_RATE_DIVIDEND: u64 = 54_000_000 * 2;

const AIS_48KHZ_DIVISOR: u64 = 1124 * 2;
const AIS_32KHZ_DIVISOR: u64 = AIS_48KHZ_DIVISOR * 3 / 2;

const CYCLES_PER_SAMPLE: [u32; 2] = [
    (CPU_CLOCK * AIS_48KHZ_DIVISOR / SAMPLE_RATE_DIVIDEND) as u32, // 48 kHz
    (CPU_CLOCK * AIS_32KHZ_DIVISOR / SAMPLE_RATE_DIVIDEND) as u32, // 32 kHz
];

#[derive(Debug)]
pub struct AudioInterface {
    control: ControlRegister,
    volume: u32,
    sample_counter: u32,
    interrupt_timing: u32,
    cycles_per_sample: u32,
    cpu_ticks: u64,
}

impl Default for AudioInterface {
    fn default() -> Self {
        let control = ControlRegister::default();
        Self {
            control,
            volume: 0,
            sample_counter: 0,
            interrupt_timing: 0,
            cycles_per_sample: CYCLES_PER_SAMPLE[control.afr() as usize],
            cpu_ticks: 0,
        }
    }
}

impl MmioDevice for AudioInterface {
    const BASE_ADDR: u32 = 0x0C00_6C00;

    fn register_mmio(mmio: &mut Mmio) {
        mmio.register_u32(
            Self::BASE_ADDR + AI_CONTROL_STATUS,
            |bus, _, _| bus.ai.control.into(),
            |bus, cpu_state, _, val| {
                let new_config = ControlRegister(val);

                if new_config.aiintmsk() != bus.ai.control.aiintmsk() {
                    bus.ai.control.set_aiintmsk(new_config.aiintmsk());
                }

                if new_config.ai_interrupt_valid() != bus.ai.control.ai_interrupt_valid() {
                    bus.ai
                        .control
                        .set_ai_interrupt_valid(new_config.ai_interrupt_valid());
                }

                if new_config.afr() != bus.ai.control.afr() {
                    bus.ai.control.set_afr(new_config.afr());
                    bus.ai.cycles_per_sample = CYCLES_PER_SAMPLE[new_config.afr() as usize];
                }

                if new_config.dsp() != bus.ai.control.dsp() {
                    bus.ai.control.set_dsp(new_config.dsp());
                }

                if new_config.aiint() {
                    bus.ai.control.set_aiint(false);
                }

                if new_config.pstat() != bus.ai.control.pstat() {
                    bus.ai.control.set_pstat(new_config.pstat());
                    bus.ai.cpu_ticks = cpu_state.timers.get_ticks();
                }

                if new_config.screset() {
                    bus.ai.sample_counter = 0;
                    bus.ai.cpu_ticks = cpu_state.timers.get_ticks();
                }

                Self::update_interrupts(bus, cpu_state);
            },
        );
        mmio.register_u32(
            Self::BASE_ADDR + AI_VOLUME,
            |bus, _, _| bus.ai.volume,
            |bus, _, _, val| {
                bus.ai.volume = val;
            },
        );
        mmio.register_u32(
            Self::BASE_ADDR + AI_SAMPLE_COUNTER,
            |bus, cpu_state, _| Self::read_sample_counter(bus, cpu_state),
            |bus, cpu_state, _, val| {
                let _ = Self::read_sample_counter(bus, cpu_state);
                bus.ai.sample_counter = val;
                bus.ai.cpu_ticks = cpu_state.timers.get_ticks();
            },
        );
        mmio.register_u32(
            Self::BASE_ADDR + AI_INTERRUPT_TIMING,
            |bus, _, _| bus.ai.interrupt_timing,
            |bus, _, _, val| {
                bus.ai.interrupt_timing = val;
            },
        );
    }
}

impl AudioInterface {
    fn read_sample_counter(bus: &mut Bus, cpu_state: &CpuState) -> u32 {
        if !bus.ai.control.pstat() {
            return bus.ai.sample_counter;
        }

        let cps = u64::from(bus.ai.cycles_per_sample.max(1));
        let ticks = cpu_state.timers.get_ticks();
        let samples = (ticks.saturating_sub(bus.ai.cpu_ticks)) / cps;
        bus.ai.sample_counter.wrapping_add(samples as u32)
    }

    fn update_interrupts(bus: &mut Bus, cpu_state: &mut CpuState) {
        if bus.ai.control.aiint() && bus.ai.control.aiintmsk() {
            ProcessorInterface::set_interrupt(bus, cpu_state, PI_INTERRUPT_AI);
        } else {
            ProcessorInterface::clear_interrupt(bus, cpu_state, PI_INTERRUPT_AI);
        }
    }

    pub fn update(bus: &mut Bus, cpu_state: &mut CpuState) {
        if !bus.ai.control.pstat() {
            return;
        }

        let cps = u64::from(bus.ai.cycles_per_sample.max(1));
        let ticks = cpu_state.timers.get_ticks();
        let diff = ticks.saturating_sub(bus.ai.cpu_ticks);
        if diff < cps {
            return;
        }

        let samples = (diff / cps) as u32;
        bus.ai.cpu_ticks += u64::from(samples) * cps;

        let old = bus.ai.sample_counter;
        bus.ai.sample_counter = old.wrapping_add(samples);

        if bus.ai.control.ai_interrupt_valid()
            && bus.ai.interrupt_timing != 0
            && old < bus.ai.interrupt_timing
            && bus.ai.sample_counter >= bus.ai.interrupt_timing
        {
            bus.ai.control.set_aiint(true);
            Self::update_interrupts(bus, cpu_state);
        }
    }
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
    pub dsp, set_dsp : 6;
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
