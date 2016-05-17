use std::rc::Rc;
use std::cell::RefCell;

use super::device::Device;
use super::super::memory::ram::Ram;

pub const BOOTROM_SIZE: usize = 0x0200000; // 2 MB

pub struct DeviceIpl {
    address: u32,
    bootrom: Rc<RefCell<Box<[u8; BOOTROM_SIZE]>>>
}

impl Device for DeviceIpl {
    fn read_imm(&self, len: u8) -> u32 {
        println!("ExiDeviceIpl: read_imm {}", len);
        0
    }

    fn write_imm(&mut self, value: u32, len: u8) {
        let command = (value & 0x7FFFFF00) >> 8;
        let write   = value & 0x80000000 != 0;

        let device_name;

        match command {
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

        let write_str = if write {
            "write"
        } else {
            "read"
        };

        println!("ExpansionInterface: {} {} {:#010x}", device_name, write_str, value);
    }

    fn read_dma(&self, memory: &mut Ram, address: u32, length: u32) {
        let bootrom = **self.bootrom.borrow_mut();

        memory.write_dma(address, &bootrom[self.address as usize .. (self.address + length) as usize]);
    }

    fn write_dma(&self, memory: &mut Ram, address: u32, length: u32) {
        println!("ExiDeviceIpl: write_dma address");
    }
}

impl DeviceIpl {
    pub fn new(bootrom: Rc<RefCell<Box<[u8; BOOTROM_SIZE]>>>) -> DeviceIpl {
        DeviceIpl {
            address: 0,
            bootrom: bootrom
        }
    }
}
