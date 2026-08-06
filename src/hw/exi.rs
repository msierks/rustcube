use std::{cell::RefCell, rc::Rc};

use crate::hw::{
    bootrom::IPL_MEM_SIZE,
    memory::Memory,
    mmio::{Mmio, MmioDevice},
};

const EXI_STATUS: u32 = 0x00;
const EXI_DMA_ADDRESS: u32 = 0x04;
const EXI_DMA_LENGTH: u32 = 0x08;
const EXI_DMA_CONTROL: u32 = 0x0C;
const EXI_IMM_DATA: u32 = 0x10;
const NUM_CHANNELS: usize = 3;
const NUM_DEVICES: usize = 3;

const TRANSFER_TYPE_READ: u32 = 0;
const TRANSFER_TYPE_WRITE: u32 = 1;
//const TRANSFER_TYPE_RW: u32 = 2;

const AD16_ID: u32 = 0x04120000;

const AD16_COMMAND_INIT: u8 = 0x00;
const AD16_COMMAND_READ: u8 = 0xa2;
const AD16_COMMAND_WRITE: u8 = 0xa0;

pub struct ExternalInterface {
    status: [StatusRegister; NUM_CHANNELS],
    control: [ControlRegister; NUM_CHANNELS],
    dma_address: [u32; NUM_CHANNELS],
    dma_length: [u32; NUM_CHANNELS],
    imm_data: [u32; NUM_CHANNELS],
    devices: [Option<Box<dyn Device>>; NUM_CHANNELS * NUM_DEVICES],
}

impl ExternalInterface {
    pub fn new(bootrom: Rc<RefCell<Vec<u8>>>) -> Self {
        let mut exi = ExternalInterface {
            status: Default::default(),
            control: Default::default(),
            dma_address: Default::default(),
            dma_length: Default::default(),
            imm_data: Default::default(),
            devices: Default::default(),
        };

        let device_ad16 = DeviceAd16::default();

        exi.devices[1] = Some(Box::new(DeviceIpl::new(bootrom)));
        exi.devices[2 * NUM_CHANNELS] = Some(Box::new(device_ad16));

        exi
    }

    fn get_channel(addr: u32) -> usize {
        ((addr - Self::BASE_ADDR) / 0x14) as usize
    }
}

impl MmioDevice for ExternalInterface {
    const BASE_ADDR: u32 = 0x0C00_6800;

    fn register_mmio(mmio: &mut Mmio) {
        for channel in 0..NUM_CHANNELS as u32 {
            mmio.register_u32(
                Self::BASE_ADDR + channel * 0x14 + EXI_STATUS,
                |bus, _, addr| {
                    let c = ExternalInterface::get_channel(addr);

                    bus.exi.status[c].into()
                },
                |bus, _, addr, val| {
                    let c = ExternalInterface::get_channel(addr);
                    let mut status = bus.exi.status[c];
                    let new_status = StatusRegister(val);

                    status.set_exi_interrupt_mask(new_status.exi_interrupt_mask());
                    status.set_tc_interrupt_mask(new_status.tc_interrupt_mask());
                    status.set_clock_frequency(new_status.clock_frequency());

                    if c == 0 && !status.rom_descramble() {
                        status.set_rom_descramble(new_status.rom_descramble());
                    }

                    status.set_device_select(new_status.device_select());

                    let device_index = c * NUM_CHANNELS + status.get_selected_device() as usize;

                    bus.exi.status[c] = status;

                    if let Some(device) = bus.exi.devices[device_index].as_mut() {
                        device.device_select();
                    }
                },
            );
            mmio.register_u32(
                Self::BASE_ADDR + channel * 0x14 + EXI_DMA_ADDRESS,
                |bus, _, addr| {
                    let c = ExternalInterface::get_channel(addr);

                    bus.exi.dma_address[c]
                },
                |bus, _, addr, val| {
                    let c = ExternalInterface::get_channel(addr);

                    bus.exi.dma_address[c] = val;
                },
            );
            mmio.register_u32(
                Self::BASE_ADDR + channel * 0x14 + EXI_DMA_LENGTH,
                |bus, _, addr| {
                    let c = ExternalInterface::get_channel(addr);

                    bus.exi.dma_length[c]
                },
                |bus, _, addr, val| {
                    let c = ExternalInterface::get_channel(addr);

                    bus.exi.dma_length[c] = val;
                },
            );
            mmio.register_u32(
                Self::BASE_ADDR + channel * 0x14 + EXI_DMA_CONTROL,
                |bus, _, addr| {
                    let c = ExternalInterface::get_channel(addr);

                    bus.exi.control[c].into()
                },
                |bus, _, addr, val| {
                    let c = ExternalInterface::get_channel(addr);

                    let mut control = ControlRegister(val);

                    if control.transfer_start() {
                        let device_index =
                            c * NUM_CHANNELS + bus.exi.status[c].get_selected_device() as usize;

                        match bus.exi.devices[device_index].as_mut() {
                            Some(device) => {
                                if control.transfer_mode() {
                                    // DMA Mode
                                    let dma_address = bus.exi.dma_address[c];
                                    let dma_length = bus.exi.dma_length[c];

                                    if control.transfer_type() == TRANSFER_TYPE_READ {
                                        device.dma_read(&mut bus.memory, dma_address, dma_length);
                                    } else if control.transfer_type() == TRANSFER_TYPE_WRITE {
                                        device.dma_write(&mut bus.memory, dma_address, dma_length);
                                    }
                                } else {
                                    // Immediate Mode
                                    let transfer_len = control.transfer_len() + 1;

                                    if control.transfer_type() == TRANSFER_TYPE_READ {
                                        bus.exi.imm_data[c] = device.imm_read(transfer_len as u8);
                                    } else if control.transfer_type() == TRANSFER_TYPE_WRITE {
                                        device.imm_write(bus.exi.imm_data[c], transfer_len as u8);
                                    }
                                }
                            }
                            None => warn!(
                                "no device on this channel frequency {}:{}",
                                c,
                                bus.exi.status[c].get_selected_device(),
                            ),
                        }

                        control.set_transfer_start(false);
                    }

                    bus.exi.control[c] = control;
                },
            );
            mmio.register_u32(
                Self::BASE_ADDR + channel * 0x14 + EXI_IMM_DATA,
                |bus, _, addr| {
                    let c = ExternalInterface::get_channel(addr);

                    bus.exi.imm_data[c]
                },
                |bus, _, addr, val| {
                    let c = ExternalInterface::get_channel(addr);

                    bus.exi.imm_data[c] = val;
                },
            );
        }
    }
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    pub struct StatusRegister(u32);
    impl Debug;
    pub exi_interrupt_mask, set_exi_interrupt_mask : 0;
    pub exi_interrupt_status, _ : 1;
    pub tc_interrupt_mask, set_tc_interrupt_mask : 2;
    pub tc_interrupt, _ : 3;
    pub clock_frequency, set_clock_frequency : 6, 4;
    pub device_select, set_device_select : 9, 7;
    pub ext_interrupt_mask, _ : 10;
    pub ext_insertion_interrupt_status, _ : 11;
    pub device_connected, _ : 12;
    pub rom_descramble, set_rom_descramble : 13;
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

impl StatusRegister {
    fn get_selected_device(&self) -> u8 {
        match self.device_select() {
            1 => 0,
            2 => 1,
            4 => 2,
            _ => 0, // FixMe: handle this case properly instead of default to 0
        }
    }
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    pub struct ControlRegister(u32);
    impl Debug;
    pub transfer_start, set_transfer_start : 0;
    pub transfer_mode, _ : 1;
    pub transfer_type, _ : 3, 2;
    pub transfer_len, _ : 5, 4;
}

impl From<ControlRegister> for u32 {
    fn from(s: ControlRegister) -> u32 {
        s.0
    }
}

pub trait Device {
    fn device_select(&mut self);

    fn transfer_byte(&mut self, _byte: &mut u8) {}

    fn imm_read(&mut self, mut len: u8) -> u32 {
        let mut result: u32 = 0;
        let mut position = 0;

        while len > 0 {
            len -= 1;
            let mut byte: u8 = 0;
            self.transfer_byte(&mut byte);
            result |= (byte as u32) << (24 - (position * 8));
            position += 1;
        }

        result
    }

    fn imm_write(&mut self, mut value: u32, mut len: u8) {
        while len > 0 {
            len -= 1;
            let mut byte = (value >> 24) as u8;
            self.transfer_byte(&mut byte);
            value <<= 8;
        }
    }

    fn dma_read(&mut self, mem: &mut Memory, mut address: u32, mut len: u32) {
        while len > 0 {
            len -= 1;
            let mut byte = 0;
            self.transfer_byte(&mut byte);
            mem.write_u8(address, byte);
            address += 1;
        }
    }

    fn dma_write(&mut self, mem: &mut Memory, mut address: u32, mut len: u32) {
        while len > 0 {
            len -= 1;
            let mut byte = mem.read_u8(address);
            self.transfer_byte(&mut byte);
            address += 1;
        }
    }
}

#[derive(Default)]
pub struct DeviceAd16 {
    position: usize,
    command: u8,
    register: u32,
}

impl Device for DeviceAd16 {
    fn device_select(&mut self) {
        self.position = 0;
        self.command = 0;
    }

    fn transfer_byte(&mut self, byte: &mut u8) {
        if self.position == 0 {
            self.command = *byte;
        } else {
            match self.command {
                AD16_COMMAND_INIT => {
                    self.register = AD16_ID;

                    if self.position > 1 && self.position < 6 {
                        let pos = self.position - 2;
                        *byte = (self.register >> (24 - (pos * 8))) as u8;
                    }
                }
                AD16_COMMAND_READ => {
                    if self.position < 4 {
                        let pos = self.position - 1;
                        *byte = (self.register >> (24 - (pos * 8))) as u8;
                    }
                }
                AD16_COMMAND_WRITE => {
                    if self.position < 4 {
                        self.register |= *byte as u32;
                        self.register <<= 8
                    }
                    if self.position == 3 {
                        let msg = match self.register {
                            0x0100_0000 => "Init",
                            0x0200_0000 => "Cache line 0x3e0 prefetched", // ???
                            0x0300_0000 => "rest of cache line 0x3e0 prefetched", // ???
                            0x0400_0000 => "Memory test passed",
                            0x0500_0000 | 0x0600_0000 | 0x0700_0000 => "Memory test failed",
                            0x0800_0000 => "IPL and OS Init called",
                            0x0900_0000 => "DVD Init",
                            0x0A00_0000 => "Card Init",
                            0x0B00_0000 => "VI Init",
                            0x0C00_0000 => "PAD Init",
                            _ => "unknown",
                        };

                        info!("AD16: {:#010x} {:}", self.register, msg);
                    }
                }
                _ => (),
            }
        }

        self.position += 1;
    }
}

// Dolphin CEXIIPL layout inside the shared IPL_MEM array (indexed by decoded address).
const IPL_SRAM_BASE: usize = 0x80_0000;
const IPL_UART_BASE: usize = 0x80_0400;

pub struct DeviceIpl {
    position: u32,
    /// Decoded device address: `(raw >> 6) & 0x1ffffff`
    address: u32,
    offset: usize,
    write: bool,
    /// Shared with Bootrom: MaskROM at 0, SRAM at 0x800000, UART at 0x800400.
    mem: Rc<RefCell<Vec<u8>>>,
    /// Accumulator for OSReport lines written to the UART FIFO.
    uart_line: Vec<u8>,
}

impl DeviceIpl {
    pub fn new(mem: Rc<RefCell<Vec<u8>>>) -> DeviceIpl {
        debug_assert!(mem.borrow().len() >= IPL_MEM_SIZE);

        {
            let mut data = mem.borrow_mut();
            // RTC (first 4 bytes of SRAM region)
            data[IPL_SRAM_BASE..IPL_SRAM_BASE + 4].copy_from_slice(&[0x38, 0x62, 0x43, 0x80]);
            // SRAM settings / flash id (Dolphin SRAM_SIZE 0x44 total with RTC)
            let sram = [
                0xFF, 0x6B, // checksum 1
                0x00, 0x91, // checksum 2
                0x00, 0x00, 0x00, 0x00, // ead 0
                0x00, 0x00, 0x00, 0x00, // ead 1
                0x00, 0x00, 0x00, 0x00, // counter bias
                0x00, // display offset h
                0x00, // ntd
                0x00, // language
                0x2C, // flags
                0x44, 0x4F, 0x4C, 0x50, 0x48, 0x49, 0x4E, 0x53, 0x4C, 0x4F, 0x54,
                0x41, // flash id
                0x44, 0x4F, 0x4C, 0x50, 0x48, 0x49, 0x4E, 0x53, 0x4C, 0x4F, 0x54,
                0x42, // flash id
                0x00, 0x00, 0x00, 0x00, // wireless keyboard id
                0x00, 0x00, // wireless pad id
                0x00, 0x00, // wireless pad id
                0x00, 0x00, // wireless pad id
                0x00, 0x00, // wireless pad id
                0x00, // last dvd error code
                0x00, // padding
                0x6E, 0x6D, // flash id checksum
                0x00, 0x00, // flash id checksum
                0x00, 0x00, // padding
            ];
            data[IPL_SRAM_BASE + 4..IPL_SRAM_BASE + 4 + sram.len()].copy_from_slice(&sram);
        }

        DeviceIpl {
            position: 0,
            address: 0,
            offset: 0,
            write: false,
            mem,
            uart_line: Vec::new(),
        }
    }
}

impl Device for DeviceIpl {
    fn device_select(&mut self) {
        self.position = 0;
        self.address = 0;
        self.offset = 0;
        self.write = false;
    }

    fn transfer_byte(&mut self, byte: &mut u8) {
        if self.position < 4 {
            self.address <<= 8;
            self.address |= u32::from(*byte);
            *byte = 0xff;

            if self.position == 3 {
                self.write = self.address & 0x8000_0000 != 0;
                self.address = (self.address >> 6) & 0x01FF_FFFF;
                self.offset = 0;

                let write_str = if self.write { "write" } else { "read" };
                debug!(
                    "ExpansionInterface: IPL {} {:#010x}",
                    write_str, self.address
                );
            }
        } else {
            let addr = self.address as usize + self.offset;
            if addr < IPL_MEM_SIZE {
                if self.write {
                    self.mem.borrow_mut()[addr] = *byte;
                } else if self.address as usize != IPL_UART_BASE {
                    // UART FIFO reads return queue length 0 (instant), not mem.
                    *byte = self.mem.borrow()[addr];
                } else {
                    *byte = 0;
                }
            } else {
                warn!("ExpansionInterface: IPL access out of range {:#010x}", addr);
            }

            // OSReport is keyed off the latched command address (Dolphin), not the
            // cursor — every data byte of a UART FIFO transfer must be logged.
            if self.write && self.address as usize == IPL_UART_BASE {
                if *byte != 0 {
                    self.uart_line.push(*byte);
                }
                if *byte == b'\r' {
                    let (text, _, _) = encoding_rs::SHIFT_JIS.decode(&self.uart_line);
                    info!("UART: {text}");
                    self.uart_line.clear();
                }
            }

            self.offset += 1;
        }

        self.position += 1;
    }
}
