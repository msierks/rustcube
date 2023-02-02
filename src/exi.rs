use std::cell::RefCell;
use std::rc::Rc;

use crate::{Context, Memory, BOOTROM_SIZE};

const STATUS: u32 = 0x00;
const DMA_ADDRESS: u32 = 0x04;
const DMA_LENGTH: u32 = 0x08;
const DMA_CONTROL: u32 = 0x0C;
const IMM_DATA: u32 = 0x10;
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

pub fn read_u32(ctx: &mut Context, channel: u32, register: u32) -> u32 {
    let c = channel as usize;

    match register {
        STATUS => ctx.exi.status[c].into(),
        DMA_ADDRESS => ctx.exi.dma_address[c],
        DMA_LENGTH => ctx.exi.dma_length[c],
        DMA_CONTROL => ctx.exi.control[c].into(),
        IMM_DATA => ctx.exi.imm_data[c],
        _ => {
            panic!("read_u32 unrecognized register {register:#x}");
        }
    }
}

pub fn write_u32(ctx: &mut Context, channel: u32, register: u32, val: u32) {
    let c = channel as usize;

    match register {
        STATUS => {
            let mut status = ctx.exi.status[c];
            let new_status = StatusRegister(val);

            status.set_exi_interrupt_mask(new_status.exi_interrupt_mask());
            status.set_tc_interrupt_mask(new_status.tc_interrupt_mask());
            status.set_clock_frequency(new_status.clock_frequency());

            if c == 0 && !status.rom_descramble() {
                status.set_rom_descramble(new_status.rom_descramble());
            }

            status.set_device_select(new_status.device_select());

            let device_index = c * NUM_CHANNELS + status.get_selected_device() as usize;

            ctx.exi.status[c] = status;

            if let Some(device) = ctx.exi.devices[device_index].as_mut() {
                device.device_select();
            }
        }
        DMA_ADDRESS => ctx.exi.dma_address[c] = val,
        DMA_LENGTH => ctx.exi.dma_length[c] = val,
        DMA_CONTROL => {
            let mut control = ControlRegister(val);

            if control.transfer_start() {
                let device_index =
                    c * NUM_CHANNELS + ctx.exi.status[c].get_selected_device() as usize;

                match ctx.exi.devices[device_index].as_mut() {
                    Some(device) => {
                        if control.transfer_mode() {
                            // DMA Mode
                            let dma_address = ctx.exi.dma_address[c];
                            let dma_length = ctx.exi.dma_length[c];

                            if control.transfer_type() == TRANSFER_TYPE_READ {
                                device.dma_read(&mut ctx.mem, dma_address, dma_length);
                            } else if control.transfer_type() == TRANSFER_TYPE_WRITE {
                                device.dma_write(&mut ctx.mem, dma_address, dma_length);
                            }
                        } else {
                            // Immediate Mode
                            let transfer_len = control.transfer_len() + 1;

                            if control.transfer_type() == TRANSFER_TYPE_READ {
                                ctx.exi.imm_data[c] = device.imm_read(transfer_len as u8);
                            } else if control.transfer_type() == TRANSFER_TYPE_WRITE {
                                device.imm_write(ctx.exi.imm_data[c], transfer_len as u8);
                            }
                        }
                    }
                    None => warn!(
                        "no device on this channel frequency {}:{}",
                        c,
                        ctx.exi.status[c].get_selected_device(),
                    ),
                }

                control.set_transfer_start(false);
            }

            ctx.exi.control[c] = control;
        }
        IMM_DATA => ctx.exi.imm_data[c] = val,
        _ => panic!("write_u32 unrecognized register {register:#x}:{val}"),
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

pub struct DeviceIpl {
    position: u32,
    address: u32,
    command: u32,
    offset: usize,
    write: bool,
    bootrom: Rc<RefCell<Vec<u8>>>,
    sram: [u8; 64],
    uart: String,
    rtc: [u8; 4],
}

impl DeviceIpl {
    pub fn new(bootrom: Rc<RefCell<Vec<u8>>>) -> DeviceIpl {
        DeviceIpl {
            position: 0,
            address: 0,
            command: 0,
            offset: 0,
            write: false,
            bootrom,
            sram: [
                0xFF, 0x6B, // checksum 1
                0x00, 0x91, // checksum 2
                0x00, 0x00, 0x00, 0x00, // ead 0
                0x00, 0x00, 0x00, 0x00, // ead 1
                0xFF, 0xFF, 0xFF, 0x40, // counter bias
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
            ],
            uart: String::new(),
            rtc: [0x38, 0x62, 0x43, 0x80],
        }
    }
}
impl Device for DeviceIpl {
    fn device_select(&mut self) {
        self.position = 0;
        self.address = 0;
        self.command = 0;
        self.offset = 0;
        self.write = false;
    }

    fn transfer_byte(&mut self, byte: &mut u8) {
        // First 4 bytes are the address
        if self.position < 4 {
            self.address <<= 8;
            self.address |= *byte as u32;
            self.offset = 0;

            // check if command is complete
            if self.position == 3 {
                let device_name;

                self.command = self.address & 0x7FFF_FF00;
                self.write = self.address & 0x8000_0000 != 0;

                match self.command {
                    0x2000_0000 => {
                        device_name = "RTC";
                    }
                    0x2000_0100 => {
                        device_name = "SRAM";
                    }
                    0x2001_0000 => {
                        device_name = "UART";

                        self.address >>= 6;
                    }
                    _ => {
                        device_name = "MaskROM";

                        self.address >>= 6;

                        if self.address > BOOTROM_SIZE as u32 {
                            panic!("Exi DeviceIPL: position out of range: {:#x}", self.address);
                        }
                    }
                }

                let write_str = if self.write { "write" } else { "read" };

                debug!(
                    "ExpansionInterface: {} {} {:#010x}",
                    device_name, write_str, self.address
                );
            }
        } else {
            match self.command {
                0x2000_0000 => {
                    // RTC
                    if self.write {
                        self.rtc[(self.address & 0x03) as usize + self.offset] = *byte;
                    } else {
                        *byte = self.rtc[(self.address & 0x03) as usize + self.offset];
                    }
                }
                0x2000_0100 => {
                    // SRAM
                    if self.write {
                        self.sram[(self.address & 0x3F) as usize + self.offset] = *byte;
                    } else {
                        *byte = self.sram[(self.address & 0x3F) as usize + self.offset];
                    }
                }
                0x2001_0000 => {
                    // UART
                    if self.write {
                        let byte_char = *byte as char;

                        if byte_char != '\0' {
                            self.uart.push(byte_char);
                        }

                        if byte_char == '\r' {
                            info!("UART: {}", self.uart);
                            self.uart.clear();
                        }
                    } else {
                        *byte = 0x01;
                    }
                }
                _ => {
                    // MASKROM
                    if !self.write {
                        *byte = self.bootrom.borrow()[self.address as usize + self.offset];
                    }
                }
            }
            self.offset += 1;
        }

        self.position += 1;
    }
}
