use memmap::Mmap;
use std::rc::Rc;
use std::cell::RefCell;
use std::io::Write;

use super::device::Device;

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
            0x20000000 => panic!("ExiDeviceIpl: command RTC unhandled"),  // RTC
            0x20000100 => panic!("ExiDeviceIpl: command SRAM unhandled"), // SRAM
            0x20010000 => panic!("ExiDeviceIpl: command UART unhandled"), // UART
            _ => { // Mask ROM
                self.address = value >> 6;

                //println!("ExiDeviceIpl: write_imm -> position {:#x}", self.address);
                if self.address > BOOTROM_SIZE as u32  { // ipl size
                    panic!("ExiDeviceIPL: position our of range: {:#x}", self.address);
                }
            }
        }
    }

    fn read_dma(&self) {
        println!("ExiDeviceIpl: read_dma address");
    }

    fn write_dma(&self, memory: &mut Mmap, address: u32, length: u32) {
        let bootrom = **self.bootrom.borrow_mut();
        let mut mmap = unsafe { memory.as_mut_slice() };

        match {&mut mmap[address as usize ..]}.write(&bootrom[self.address as usize .. (self.address + length) as usize]) {
            Ok(_) => {},
            Err(e) => panic!("{}", e)
        }
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
