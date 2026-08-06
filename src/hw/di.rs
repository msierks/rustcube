use crate::{
    bus::Bus,
    cpu::CpuState,
    disc::Disc,
    hw::{
        mmio::{Mmio, MmioDevice},
        pi::{ProcessorInterface, PI_INTERRUPT_DI},
    },
};

const DI_STATUS: u32 = 0x00;
const DI_COVER_STATUS: u32 = 0x04;
const DI_DICMDBUF0: u32 = 0x08;
const DI_DICMDBUF1: u32 = 0x0C;
const DI_DICMDBUF2: u32 = 0x10;
const DI_DIMAR: u32 = 0x14;
const DI_DILENGTH: u32 = 0x18;
const DI_DICR: u32 = 0x1C;
//const DIIMMBUF: u32 = 0x20;
const DI_DICFG: u32 = 0x24;

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

impl MmioDevice for DvdInterface {
    const BASE_ADDR: u32 = 0x0C00_6000;

    fn register_mmio(mmio: &mut Mmio) {
        mmio.register_write_u32(Self::BASE_ADDR + DI_STATUS, |bus, cpu_state, _, val| {
            let tmp: StatusRegister = val.into();

            bus.di.status.set_device_int_mask(tmp.device_int_mask());
            bus.di.status.set_transfer_int_mask(tmp.transfer_int_mask());
            bus.di.status.set_break_int_mask(tmp.break_int_mask());
            bus.di.status.set_di_break(tmp.di_break());

            if tmp.device_int() {
                bus.di.status.set_device_int(false);
            }
            if tmp.transfer_int() {
                bus.di.status.set_transfer_int(false);
            }
            if tmp.break_int() {
                bus.di.status.set_break_int(false);
            }

            Self::update_interrupts(bus, cpu_state);
        });
        mmio.register_u32(
            Self::BASE_ADDR + DI_COVER_STATUS,
            |bus, _, _| bus.di.cover_status.into(),
            |bus, cpu_state, _, val| {
                let tmp: CoverStatusRegister = val.into();

                bus.di.cover_status.set_cover_int_mask(tmp.cover_int_mask());

                if tmp.cover_int() {
                    bus.di.cover_status.set_cover_int(false);
                }

                Self::update_interrupts(bus, cpu_state);
            },
        );
        mmio.register_write_u32(Self::BASE_ADDR + DI_DICMDBUF0, |bus, _, _, val| {
            bus.di.command_buff_0 = val;
        });
        mmio.register_write_u32(Self::BASE_ADDR + DI_DICMDBUF1, |bus, _, _, val| {
            bus.di.command_buff_1 = val;
        });
        mmio.register_write_u32(Self::BASE_ADDR + DI_DICMDBUF2, |bus, _, _, val| {
            bus.di.command_buff_2 = val;
        });
        mmio.register_write_u32(Self::BASE_ADDR + DI_DIMAR, |bus, _, _, val| {
            bus.di.dma_address = val;
        });
        mmio.register_write_u32(Self::BASE_ADDR + DI_DILENGTH, |bus, _, _, val| {
            bus.di.dma_transfer_length = val;
        });
        mmio.register_write_u32(Self::BASE_ADDR + DI_DICR, |bus, _, _, val| {
            bus.di.control = val.into();
            if bus.di.control.tstart() {
                // Execute Command
                match (bus.di.command_buff_0 >> 24) as u8 {
                    DI_COMMAND_INQUIRY => (), // Not sure what happens here
                    //DI_COMMAND_READ => (),
                    //DI_COMMAND_SEEK => (),
                    _ => warn!("Unrecognized command {:#x}", bus.di.command_buff_0),
                }
            }

            bus.di.control.set_tstart(false);
        });
        mmio.register_read_u32(Self::BASE_ADDR + DI_DICFG, |bus, _, _| bus.di.config);
    }
}

impl DvdInterface {
    pub fn set_disc(&mut self, disc: Option<Disc>) {
        self.disc = disc;
    }

    fn update_interrupts(bus: &mut Bus, cpu_state: &mut CpuState) {
        if bus.di.status.device_int() && bus.di.status.device_int_mask()
            || bus.di.status.transfer_int() && bus.di.status.transfer_int_mask()
            || bus.di.status.break_int() && bus.di.status.break_int_mask()
            || bus.di.cover_status.cover_int() && bus.di.cover_status.cover_int_mask()
        {
            ProcessorInterface::set_interrupt(bus, cpu_state, PI_INTERRUPT_DI);
        } else {
            ProcessorInterface::clear_interrupt(bus, cpu_state, PI_INTERRUPT_DI);
        }
    }
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    pub struct StatusRegister(u32);
    impl Debug;
    pub di_break, set_di_break : 0;
    pub device_int_mask, set_device_int_mask : 1;
    pub device_int, set_device_int : 2;
    pub transfer_int_mask, set_transfer_int_mask : 3;
    pub transfer_int, set_transfer_int : 4;
    pub break_int_mask, set_break_int_mask : 5;
    pub break_int, set_break_int : 6;
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
