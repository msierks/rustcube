use crate::{
    bus::Bus,
    cpu::CpuState,
    hw::{
        mmio::{Mmio, MmioDevice},
        pi::{ProcessorInterface, PI_INTERRUPT_PE_FINISH, PI_INTERRUPT_PE_TOKEN},
    },
};

const PE_Z_CONFIG: u32 = 0x00;
const PE_ALPHA_CONFIG: u32 = 0x02;
const PE_DESTINATION_ALPHA: u32 = 0x04;
const PE_ALPHA_MODE: u32 = 0x06;
const PE_ALPHA_READ: u32 = 0x08;
const PE_CONTROL: u32 = 0x0A;
const PE_TOKEN: u32 = 0x0E;

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

impl MmioDevice for PixelEngine {
    const BASE_ADDR: u32 = 0x0C00_1000;

    fn register_mmio(mmio: &mut Mmio) {
        mmio.register_u16(
            Self::BASE_ADDR + PE_Z_CONFIG,
            |bus, _, _| bus.pe.z_config,
            |bus, _, _, val| bus.pe.z_config = val,
        );
        mmio.register_u16(
            Self::BASE_ADDR + PE_ALPHA_CONFIG,
            |bus, _, _| bus.pe.alpha_config,
            |bus, _, _, val| bus.pe.alpha_config = val,
        );
        mmio.register_u16(
            Self::BASE_ADDR + PE_DESTINATION_ALPHA,
            |bus, _, _| bus.pe.destination_alpha,
            |bus, _, _, val| bus.pe.destination_alpha = val,
        );
        mmio.register_u16(
            Self::BASE_ADDR + PE_ALPHA_MODE,
            |bus, _, _| bus.pe.alpha_mode,
            |bus, _, _, val| bus.pe.alpha_mode = val,
        );
        mmio.register_u16(
            Self::BASE_ADDR + PE_ALPHA_READ,
            |bus, _, _| bus.pe.alpha_read,
            |bus, _, _, val| bus.pe.alpha_read = val,
        );
        mmio.register_u16(
            Self::BASE_ADDR + PE_CONTROL,
            |bus, _, _| bus.pe.control.into(),
            |bus, cpu_state, _, val| {
                let control: ControlRegister = val.into();

                if control.pe_token() {
                    bus.pe.signal_token_interrupt = false;
                }

                if control.pe_finish() {
                    bus.pe.signal_finish_interrupt = false;
                }

                bus.pe.control = control;

                bus.pe.control.set_pe_token(false);
                bus.pe.control.set_pe_finish(false);

                Self::update_interrupts(bus, cpu_state);
            },
        );
        mmio.register_u16(
            Self::BASE_ADDR + PE_TOKEN,
            |bus, _, _| bus.pe.token,
            |bus, _, _, val| bus.pe.token = val,
        );
    }
}

impl PixelEngine {
    fn update_interrupts(bus: &mut Bus, cpu_state: &mut CpuState) {
        if bus.pe.signal_token_interrupt && bus.pe.control.pe_token_enable() {
            ProcessorInterface::set_interrupt(bus, cpu_state, PI_INTERRUPT_PE_TOKEN);
        } else {
            ProcessorInterface::clear_interrupt(bus, cpu_state, PI_INTERRUPT_PE_TOKEN);
        }

        if bus.pe.signal_finish_interrupt && bus.pe.control.pe_finish_enable() {
            ProcessorInterface::set_interrupt(bus, cpu_state, PI_INTERRUPT_PE_FINISH);
        } else {
            ProcessorInterface::clear_interrupt(bus, cpu_state, PI_INTERRUPT_PE_FINISH);
        }
    }
}

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
