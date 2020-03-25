use std::cell::RefCell;
use std::rc::Rc;

use super::super::memory::Ram;
use super::device::Device;

pub const BOOTROM_SIZE: usize = 0x0020_0000; // 2 MB

pub struct DeviceIpl {
    address: u32,
    command: u32,
    offset: u32,
    write: bool,
    bootrom: Rc<RefCell<Vec<u8>>>,
    sram: [u8; 64],
    uart: String,
}

impl DeviceIpl {
    pub fn new(bootrom: Rc<RefCell<Vec<u8>>>) -> DeviceIpl {
        DeviceIpl {
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
        }
    }
}

impl Device for DeviceIpl {
    fn device_select(&mut self) {
        self.address = 0;
        self.command = 0;
        self.offset = 0;
        self.write = false;
    }

    fn read_imm(&self, len: u8) -> u32 {
        match self.command {
            0x0020_0000 => {
                println!("FixMe: read imm RTC");
                0
            }
            0x0020_0100 => {
                //println!("FixMe: read imm UART");
                0
            }
            _ => panic!(
                "ExiDeviceIpl: unhandled read_imm {:#x} {}",
                self.command, len
            ),
        }
    }

    fn write_imm(&mut self, value: u32, len: u8) {
        if self.command == 0 {
            self.command = (value & 0x7FFF_FF00) >> 8;
            self.write = value & 0x8000_0000 != 0;

            let device_name;

            match self.command {
                0x0020_0000 => {
                    device_name = "RTC";
                }
                0x0020_0001 => {
                    device_name = "SRAM";
                }
                0x0020_0100 => {
                    device_name = "UART";

                    self.address = value >> 6;
                }
                _ => {
                    device_name = "MaskROM";

                    self.address = value >> 6;

                    if self.address > BOOTROM_SIZE as u32 {
                        // ipl size
                        panic!("ExiDeviceIPL: position our of range: {:#x}", self.address);
                    }
                }
            }

            let write_str = if self.write { "write" } else { "read" };

            println!(
                "ExpansionInterface: {} {} {:#010x}",
                device_name, write_str, value
            );
        } else {
            match self.command {
                0x0020_0000 => {
                    // RTC
                    panic!("FixMe: rtc command");
                }
                0x0020_0001 => {
                    // SRAM
                    if self.write {
                        //self.sram.write(value, self.offset);
                    } else {
                        panic!("FixMe: sram read not implemented");
                    }
                }
                0x0020_0100 => {
                    // UART
                    if self.write {
                        let byte = ((value >> 24) as u8) as char;

                        if byte != '\0' {
                            self.uart.push(byte);
                        }

                        if byte == '\r' {
                            println!("UART: {}", self.uart);

                            self.uart.clear();
                        }
                    }
                }
                _ => {
                    panic!("this shouldn't happen");
                }
            }

            self.offset += u32::from(len);
        }
    }

    fn read_dma(&self, memory: &mut Ram, address: u32, length: u32) {
        match self.command {
            0x0020_0001 => {
                memory.write_dma(address, &self.sram[0..length as usize]);
            }
            _ => {
                let mut bootrom = &*self.bootrom.borrow_mut();

                memory.write_dma(
                    address,
                    &bootrom.as_slice()[self.address as usize..(self.address + length) as usize],
                );
            }
        }
    }

    fn write_dma(&self, _: &mut Ram, _: u32, _: u32) {
        panic!("ExiDeviceIpl: write_dma address");
    }
}
