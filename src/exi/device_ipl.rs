use std::rc::Rc;
use std::cell::RefCell;

use super::device::Device;
use super::super::memory::ram::Ram;

pub const BOOTROM_SIZE: usize = 0x0200000; // 2 MB

pub struct DeviceIpl {
    address: u32,
    command: u32,
    offset: u32,
    write: bool,
    bootrom: Rc<RefCell<Box<[u8; BOOTROM_SIZE]>>>,
    sram: Sram
}

impl Device for DeviceIpl {
    fn device_select(&mut self) {
        //println!("DeviceIpl Selected");
        self.address = 0;
        self.command = 0;
        self.offset  = 0;
        self.write   = false;
    }

    fn read_imm(&self, len: u8) -> u32 {
        //println!("ExiDeviceIpl: read_imm {}", len);
        0
    }

    fn write_imm(&mut self, value: u32, len: u8) {
        if self.command == 0 {

            self.command = (value & 0x7FFFFF00) >> 8;
            self.write   = value & 0x80000000 != 0;

            let device_name;

            match self.command {
                0x200000 => {
                    device_name = "RTC";
                },
                0x200001 => {
                    device_name = "SRAM";
                },
                0x200100 => {
                    device_name = "UART";
                },
                _ => {
                    device_name = "MaskROM";

                    self.address = value >> 6;

                    if self.address > BOOTROM_SIZE as u32  { // ipl size
                        panic!("ExiDeviceIPL: position our of range: {:#x}", self.address);
                    }
                }
            }

            let write_str = if self.write {
                "write"
            } else {
                "read"
            };

            println!("ExpansionInterface: {} {} {:#010x}", device_name, write_str, value);
        } else {

            match self.command {
                0x200000 => { // RTC
                },
                0x200001 => { // SRAM
                    if self.write {
                        self.sram.write(value, self.offset);
                    } else {
                        println!("FixMe: sram read not implemented");
                    }
                },
                0x200100 => { // UART
                },
                _ => {
                    panic!("this shouldn't happen");
                }
            }

            self.offset += len as u32;
        }
    }

    fn read_dma(&self, memory: &mut Ram, address: u32, length: u32) {
        let bootrom = **self.bootrom.borrow_mut();

        memory.write_dma(address, &bootrom[self.address as usize .. (self.address + length) as usize]);
        println!("ExiDeviceIpl: read_dma address");
    }

    fn write_dma(&self, memory: &mut Ram, address: u32, length: u32) {
        println!("ExiDeviceIpl: write_dma address");
    }
}

impl DeviceIpl {
    pub fn new(bootrom: Rc<RefCell<Box<[u8; BOOTROM_SIZE]>>>) -> DeviceIpl {
        DeviceIpl {
            address: 0,
            command: 0,
            offset: 0,
            write: false,
            bootrom: bootrom,
            sram: Sram::default()
        }
    }
}

#[derive(Default, Debug)]
struct Sram {
    checksum_1: u16,
    checksum_2: u16,
    ead_0: u32,
    ead_1: u32,
    counter_bias: u32,
    display_offset_h: u8,
    ntd: u8,
    language: u8,
    flags: u8,
    flash_id: u8,
    wireless_kbd_id: u32,
    wireless_pad_id: u32,
    last_dvd_err: u8,
    flash_id_checksum: u8
}

impl Sram {
    pub fn write(&mut self, value: u32, offset: u32) {
        match offset {
            0 => {
                self.checksum_1 = (value >> 16) as u16;
                self.checksum_2 = (value & 0xFFFF) as u16;
            },
            4 => self.ead_0 = value,
            8 => self.ead_1 = value,
            12 => self.counter_bias = value,
            16 => {
                self.display_offset_h = (value >> 20) as u8;
                self.ntd = ((value >> 16) & 0xFF) as u8;
                self.language = ((value >> 16) & 0xFF) as u8;
                self.flags = (value & 0xFF) as u8;
            }
            20 => {},
            24 => {},
            28 => {},
            32 => {},
            36 => {},
            40 => {},
            44 => {},
            48 => {},
            52 => {},
            56 => {},
            60 => {}
            _ => {
                println!("flags {:#b}", self.flags);
                panic!("{:#?}", self);
            }
        }
    }
}
