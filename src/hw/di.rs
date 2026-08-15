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
const DI_DIIMMBUF: u32 = 0x20;
const DI_DICFG: u32 = 0x24;

const DI_CMD_INQUIRY: u8 = 0x12; // Obtain drive ID information
const DI_CMD_READ: u8 = 0xA8; // Obtain data from disk
const DI_CMD_SEEK: u8 = 0xAB; // Move optical head to position on disk
const _DI_CMD_REQUEST_ERROR: u8 = 0xE0; // Transfer error and status information from drive to host in 4 byte long format
const _DI_CMD_AUDIO_STREAMING: u8 = 0xE1; // Transfer audio streaming information to drive
const _DI_CMD_REQUEST_AUDIO_STATUS: u8 = 0xE2; // Transfer audion streaming information from drive to host including error info
const DI_CMD_STOP_MOTOR: u8 = 0xE3; // Request drive to stop its motor
const _DI_CMD_AUDIO_BUFFER_CONFIGURATION: u8 = 0xE4; // Configure the audio buffer in drive
const DI_CMD_DEBUG: u8 = 0xFE;
const DI_CMD_DEBUG_UNLOCK: u8 = 0xFF;

/// Typical retail drive inquiry response (32 bytes).
const INQUIRY_RESPONSE: [u8; 0x20] = [
    0x00, 0x00, // Product Revision Level
    0x00, 0x00, // Device Code
    0x20, 0x02, 0x04, 0x02, // release date 2002-04-02 (YYYYMMDD)
    0x61, 0x00, 0x00, 0x00, // Padding (0x61 observed in real hardware? probably ignored)
    0x00, 0x00, 0x00, 0x00, // Padding
    0x00, 0x00, 0x00, 0x00, // Padding
    0x00, 0x00, 0x00, 0x00, // Padding
    0x00, 0x00, 0x00, 0x00, // Padding
    0x00, 0x00, 0x00, 0x00, // Padding
];

#[derive(Default)]
pub struct DvdInterface {
    status: StatusRegister,
    cover_status: CoverStatusRegister,
    command_buff: [u32; 3],
    dma_address: u32,
    dma_transfer_length: u32,
    control: ControlRegister,
    immediate: u32,
    config: u32,
    disc: Option<Disc>,
}

impl MmioDevice for DvdInterface {
    const BASE_ADDR: u32 = 0x0C00_6000;

    fn register_mmio(mmio: &mut Mmio) {
        mmio.register_u32(
            Self::BASE_ADDR + DI_STATUS,
            |bus, _, _| bus.di.status.into(),
            |bus, cpu_state, _, val| {
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
            },
        );
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
        mmio.register_u32(
            Self::BASE_ADDR + DI_DICMDBUF0,
            |bus, _, _| bus.di.command_buff[0],
            |bus, _, _, val| {
                bus.di.command_buff[0] = val;
            },
        );
        mmio.register_u32(
            Self::BASE_ADDR + DI_DICMDBUF1,
            |bus, _, _| bus.di.command_buff[1],
            |bus, _, _, val| {
                bus.di.command_buff[1] = val;
            },
        );
        mmio.register_u32(
            Self::BASE_ADDR + DI_DICMDBUF2,
            |bus, _, _| bus.di.command_buff[2],
            |bus, _, _, val| {
                bus.di.command_buff[2] = val;
            },
        );
        mmio.register_u32(
            Self::BASE_ADDR + DI_DIMAR,
            |bus, _, _| bus.di.dma_address,
            |bus, _, _, val| {
                bus.di.dma_address = val & !0xFC00_001F;
            },
        );
        mmio.register_u32(
            Self::BASE_ADDR + DI_DILENGTH,
            |bus, _, _| bus.di.dma_transfer_length,
            |bus, _, _, val| {
                bus.di.dma_transfer_length = val & !0x1F;
            },
        );
        mmio.register_u32(
            Self::BASE_ADDR + DI_DICR,
            |bus, _, _| bus.di.control.into(),
            |bus, cpu_state, _, val| {
                bus.di.control = val.into();
                if bus.di.control.tstart() {
                    Self::execute_command(bus, cpu_state);
                }
                bus.di.control.set_tstart(false);
            },
        );
        mmio.register_u32(
            Self::BASE_ADDR + DI_DIIMMBUF,
            |bus, _, _| bus.di.immediate,
            |bus, _, _, val| {
                bus.di.immediate = val;
            },
        );
        mmio.register_read_u32(Self::BASE_ADDR + DI_DICFG, |bus, _, _| bus.di.config);
    }
}

impl DvdInterface {
    pub fn set_disc(&mut self, disc: Option<Disc>) {
        if disc.is_some() {
            // Disc present, cover closed.
            self.cover_status.set_cover(false);
        }
        self.disc = disc;
    }

    pub fn reset_drive(&mut self, _spinup: bool) {
        debug!("DI reset drive");
    }

    fn write_dma(bus: &mut Bus, addr: u32, data: &[u8]) {
        bus.memory.write_bytes(addr, data);
    }

    fn execute_command(bus: &mut Bus, cpu_state: &mut CpuState) {
        let cmd = (bus.di.command_buff[0] >> 24) as u8;
        match cmd {
            DI_CMD_INQUIRY => Self::do_inquiry(bus),
            DI_CMD_READ => Self::do_read(bus),
            DI_CMD_SEEK => (),
            DI_CMD_STOP_MOTOR => (),
            DI_CMD_DEBUG => (),
            DI_CMD_DEBUG_UNLOCK => (),
            _ => warn!("Unrecognized DI command {:#x}", bus.di.command_buff[0]),
        }
        Self::finish_transfer(bus, cpu_state);
    }

    fn finish_transfer(bus: &mut Bus, cpu_state: &mut CpuState) {
        bus.di.dma_transfer_length = 0;
        bus.di.status.set_transfer_int(true);
        Self::update_interrupts(bus, cpu_state);
    }

    fn do_inquiry(bus: &mut Bus) {
        let len = bus.di.dma_transfer_length.min(0x20) as usize;
        Self::write_dma(bus, bus.di.dma_address, &INQUIRY_RESPONSE[..len]);
    }

    fn do_read(bus: &mut Bus) {
        let subcmd = (bus.di.command_buff[0] & 0xFF) as u8;
        if subcmd != 0x00 {
            warn!(
                "Unimplemented DI read subcommand {:#x} (cmd {:#x})",
                subcmd, bus.di.command_buff[0]
            );
            return;
        }

        let offset = (bus.di.command_buff[1] as u64) << 2;
        let length = bus.di.command_buff[2].min(bus.di.dma_transfer_length) as usize;
        if length == 0 {
            return;
        }

        let Some(disc) = bus.di.disc.as_mut() else {
            warn!("DI read with no disc inserted");
            return;
        };

        let mut buf = vec![0u8; length];
        if let Err(err) = disc.read_at(offset, &mut buf) {
            warn!("DI read failed at {offset:#x} ({length:#x} bytes): {err}");
            return;
        }

        Self::write_dma(bus, bus.di.dma_address, &buf);
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
    pub cover, set_cover : 0;
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

impl From<ControlRegister> for u32 {
    fn from(s: ControlRegister) -> u32 {
        s.0
    }
}
