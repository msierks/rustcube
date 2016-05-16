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
    fn read_imm(&self) -> u32 {
        println!("ExiDeviceIpl: read_imm");
        0
    }

    fn write_imm(&mut self, value: u32) {
        let command = value & 0x3FFFFF00;

        match command {
            0x20000000 => println!("ExiDeviceIpl: command RTC unhandled {:#x}", value),  // RTC
            0x20000100 => println!("ExiDeviceIpl: command SRAM unhandled {:#x}", value), // SRAM
            0x20010000 => panic!("ExiDeviceIpl: command UART unhandled"), // UART
            _ => { // Mask ROM
                self.address = value >> 6;

                if self.address > BOOTROM_SIZE as u32  { // ipl size
                    panic!("ExiDeviceIPL: position our of range: {:#x}", self.address);
                }
            }
        }
    }

    fn read_dma(&self) {
        println!("ExiDeviceIpl: read_dma address");
    }

    fn write_dma(&self, memory: &mut Ram, address: u32, length: u32) {
        let bootrom = **self.bootrom.borrow_mut();

        memory.write_dma(address, &bootrom[self.address as usize .. (self.address + length) as usize]);
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
