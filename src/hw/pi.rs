use crate::{
    bus::Bus,
    cpu::CpuState,
    hw::{
        gp_fifo::BURST_SIZE,
        mmio::{Mmio, MmioDevice},
    },
};

const FIFO_PTR_MASK: u32 = 0xFFFF_FFE0;
const FIFO_WRAP: u32 = 0x2000_0000;
const FIFO_ADDR_MASK: u32 = 0x03FF_FFE0;

const PI_INTERRUPT_CAUSE: u32 = 0x00;
const PI_INTERRUPT_MASK: u32 = 0x04;
const PI_FIFO_BASE_START: u32 = 0x0C;
const PI_FIFO_BASE_END: u32 = 0x10;
const PI_FIFO_WRITE_POINTER: u32 = 0x14;
const PI_FIFO_RESET: u32 = 0x18;
const PI_RESET: u32 = 0x24;
const PI_REVISION: u32 = 0x2C;
const PI_UNKNOWN: u32 = 0x30;

// Flipper ID Revision C as per Dolphin Emulator
const FLIPPER_REV: u32 = 0x2465_00B1;

pub const PI_INTERRUPT_RSWST: u32 = 0x10000; // Reset Switch State (1 when pressed)
pub const PI_INTERRUPT_HSP: u32 = 0x02000; // High Speed Port
pub const PI_INTERRUPT_DEBUG: u32 = 0x01000; // Debug Hardware
pub const PI_INTERRUPT_CP: u32 = 0x0800; // Command FIFO
pub const PI_INTERRUPT_PE_FINISH: u32 = 0x0400; // GP FInished
pub const PI_INTERRUPT_PE_TOKEN: u32 = 0x0200; // GP Token
pub const PI_INTERRUPT_VI: u32 = 0x00100; // Video Interface
pub const PI_INTERRUPT_MEM: u32 = 0x0080; // Memory Interface
pub const PI_INTERRUPT_DSP: u32 = 0x0040; // DSP Interface
pub const PI_INTERRUPT_AI: u32 = 0x0020; // Audio Interface Streaming
pub const PI_INTERRUPT_EXI: u32 = 0x0010; // External Interface
pub const PI_INTERRUPT_SI: u32 = 0x0008; // Serial Interface
pub const PI_INTERRUPT_DI: u32 = 0x0004; // DVD Interface
pub const PI_INTERRUPT_RSW: u32 = 0x0002; // Reset Switch
pub const PI_INTERRUPT_ERROR: u32 = 0x0001; // GP Runtime Error

#[derive(Debug)]
pub struct ProcessorInterface {
    interrupt_cause: u32,
    interrupt_mask: u32,
    fifo_start: u32,
    fifo_end: u32,
    fifo_write_pointer: u32,
    reset: ResetRegister,
    revision: u32,
    unknown: u32,
}

impl Default for ProcessorInterface {
    fn default() -> Self {
        ProcessorInterface {
            interrupt_mask: 0,
            interrupt_cause: 0,
            fifo_start: 0,
            fifo_end: 0,
            fifo_write_pointer: 0,
            reset: Default::default(),
            revision: FLIPPER_REV,
            unknown: 0,
        }
    }
}

impl MmioDevice for ProcessorInterface {
    const BASE_ADDR: u32 = 0x0C00_3000;

    fn register_mmio(mmio: &mut Mmio) {
        mmio.register_u32(
            Self::BASE_ADDR + PI_INTERRUPT_CAUSE,
            |bus, _, _| bus.pi.interrupt_cause,
            |bus, cpu_state, _, val| {
                bus.pi.interrupt_cause &= !val;
                Self::update_interrupts(bus, cpu_state);
            },
        );
        mmio.register_u32(
            Self::BASE_ADDR + PI_INTERRUPT_MASK,
            |bus, _, _| bus.pi.interrupt_mask,
            |bus, cpu_state, _, val| {
                bus.pi.interrupt_mask = val;
                Self::update_interrupts(bus, cpu_state);
            },
        );
        mmio.register_u32(
            Self::BASE_ADDR + PI_FIFO_BASE_START,
            |bus, _, _| bus.pi.fifo_start,
            |bus, _, _, val| {
                bus.pi.fifo_start = val & FIFO_PTR_MASK;
            },
        );
        mmio.register_u32(
            Self::BASE_ADDR + PI_FIFO_BASE_END,
            |bus, _, _| bus.pi.fifo_end,
            |bus, _, _, val| {
                bus.pi.fifo_end = val & FIFO_PTR_MASK;
            },
        );
        mmio.register_u32(
            Self::BASE_ADDR + PI_FIFO_WRITE_POINTER,
            |bus, _, _| bus.pi.fifo_write_pointer,
            |bus, _, _, val| {
                bus.pi.fifo_write_pointer = val & FIFO_PTR_MASK;
            },
        );
        mmio.register_write_u32(Self::BASE_ADDR + PI_FIFO_RESET, |bus, _, _, val| {
            if val & 1 != 0 {
                bus.gp_fifo.reset();
            }
        });
        mmio.register_u32(
            Self::BASE_ADDR + PI_RESET,
            |bus, _, _| bus.pi.reset.into(),
            |bus, _, _, val| {
                bus.pi.reset = val.into();
                info!("PI_RESET_CODE {val:#010x}");
                if !bus.pi.reset.dvd() {
                    bus.di.reset_drive(true);
                }
                if !bus.pi.reset.system() {
                    warn!("PI system reset ignored");
                }
            },
        );
        mmio.register_read_u32(Self::BASE_ADDR + PI_REVISION, |bus, _, _| bus.pi.revision);
        mmio.register_write_u32(Self::BASE_ADDR + PI_UNKNOWN, |bus, _, _, val| {
            bus.pi.unknown = val;
        });
    }
}

impl ProcessorInterface {
    pub fn fifo_write_address(&self) -> u32 {
        self.fifo_write_pointer & FIFO_ADDR_MASK
    }

    pub fn advance_fifo_write_pointer(&mut self) {
        let addr = self.fifo_write_pointer & FIFO_ADDR_MASK;
        let end = self.fifo_end & FIFO_ADDR_MASK;
        let wrap = self.fifo_write_pointer & FIFO_WRAP;
        if addr == end {
            self.fifo_write_pointer = (self.fifo_start & FIFO_ADDR_MASK) | FIFO_WRAP;
        } else {
            self.fifo_write_pointer = addr.wrapping_add(BURST_SIZE as u32) | wrap;
        }
    }

    pub fn update_interrupts(bus: &mut Bus, cpu_state: &mut CpuState) {
        if bus.pi.interrupt_cause & bus.pi.interrupt_mask != 0 {
            cpu_state.external_interrupt(true);
        } else {
            cpu_state.external_interrupt(false);
        }
    }

    pub fn clear_interrupt(bus: &mut Bus, cpu_state: &mut CpuState, cause: u32) {
        if bus.pi.interrupt_cause & cause != 0 {
            debug!("Interrupt {} (clear)", Self::interrupt_name(cause));
        }

        bus.pi.interrupt_cause &= !cause;

        Self::update_interrupts(bus, cpu_state);
    }

    pub fn set_interrupt(bus: &mut Bus, cpu_state: &mut CpuState, cause: u32) {
        if bus.pi.interrupt_cause & cause == 0 {
            debug!("Interrupt {} (set)", Self::interrupt_name(cause));
        }

        bus.pi.interrupt_cause |= cause;

        Self::update_interrupts(bus, cpu_state);
    }

    fn interrupt_name(interrupt: u32) -> &'static str {
        match interrupt {
            PI_INTERRUPT_ERROR => "PI_INTERRUPT_ERROR",
            PI_INTERRUPT_RSW => "PI_INTERRUPT_RSW",
            PI_INTERRUPT_DI => "PI_INTERRUPT_DI",
            PI_INTERRUPT_SI => "PI_INTERRUPT_SI",
            PI_INTERRUPT_EXI => "PI_INTERRUPT_EXI",
            PI_INTERRUPT_AI => "PI_INTERRUPT_AI",
            PI_INTERRUPT_DSP => "PI_INTERRUPT_DSP",
            PI_INTERRUPT_MEM => "PI_INTERRUPT_MEM",
            PI_INTERRUPT_VI => "PI_INTERRUPT_VI",
            PI_INTERRUPT_PE_TOKEN => "PI_INTERRUPT_PE_TOKEN",
            PI_INTERRUPT_PE_FINISH => "PI_INTERRUPT_PE_FINISH",
            PI_INTERRUPT_CP => "PI_INTERRUPT_CP",
            PI_INTERRUPT_DEBUG => "PI_INTERRUPT_DEBUG",
            PI_INTERRUPT_HSP => "PI_INTERRUPT_HSP",
            PI_INTERRUPT_RSWST => "PI_INTERRUPT_RSWST",
            _ => "UNKNOWN",
        }
    }
}

bitfield! {
    #[derive(Copy, Clone, Default, Debug)]
    pub struct ResetRegister(u32);
    pub system, _ : 0;
    pub memory, _ : 1;
    pub dvd, _ : 2;
}

impl From<u32> for ResetRegister {
    fn from(v: u32) -> Self {
        ResetRegister(v)
    }
}

impl From<ResetRegister> for u32 {
    fn from(s: ResetRegister) -> u32 {
        s.0
    }
}
