use std::fmt;
use std::io::{Read, Write};
use memmap::{Mmap, Protection};
use byteorder::{ByteOrder, BigEndian};
use super::exi::Exi;

const RAM_SIZE: usize     = 0x1800000; // 24 MB
const BOOTROM_SIZE: usize = 0x0100000; // 1 MB

pub enum Address {
    Ram,                          // Main Memory (RAM),
    EmbeddedFramebuffer,          // Embedded Framebuffer (EFB)
    CommandProcessor,             // Command Processor (CP)
    PixelEngine,                  // Pixel Engine (PE)
    VideoInterface,               // Video Interface (VI)
    PeripheralInterface,          // Peripheral Interface (PI)
    MemoryInterface,              // Memory Interface (MI)
    AudioInterface,               // DSP and DMA Audio Interface (AID)
    DvdInterface,                 // DVD Interface (DI)
    SerialInterface,              // Serial Interface (SI)
    ExpansionInterface(u32, u32), // External Interface (EXI)
    AudioStreamingInterface,      // Audio Streaming Interface (AIS)
    PiFifo,                       // PI FIFO (GX)
    Bootrom(u32)                  // Bootrom
}

fn map_address(address: u32) -> Address {
    match address {
        0x00000000 ... 0x07FFFFFF => Address::Ram,
        0x08000000 ... 0x0BFFFFFF => Address::EmbeddedFramebuffer,
        0x0C000000 ... 0x0C000FFF => Address::CommandProcessor,
        0x0C001000 ... 0x0C001FFF => Address::PixelEngine,
        0x0C002000 ... 0x0C002FFF => Address::VideoInterface,
        0x0C003000 ... 0x0C003FFF => Address::PeripheralInterface,
        0x0C004000 ... 0x0C004FFF => Address::MemoryInterface,
        0x0C005000 ... 0x0C005FFF => Address::AudioInterface,
        0x0C006000 ... 0x0C0063FF => Address::DvdInterface,
        0x0C006400 ... 0x0C0067FF => Address::SerialInterface,
        0x0C006800 ... 0x0C0068FF => {
            let channel  = (address - 0x0C006800) / 0x14;
            let register = (address - 0x0C006800) - (channel * 0x14);
            Address::ExpansionInterface(channel, register)
        },
        0x0C006C00 ... 0x0C006CFF => Address::AudioStreamingInterface,
        0x0C008000 ... 0x0C008FFF => Address::PiFifo,
        0xFFF00000 ... 0xFFFFFFFF => {
            Address::Bootrom(address - 0xFFF00000)
        },                
        _ => panic!("Unrecognized physical address: {:#x}", address)
    }
}

pub struct Interconnect {
    exi: Exi,
    mmap: Mmap,
    bootrom: Mmap
}

impl Interconnect {
    pub fn new() -> Interconnect {
        let mmap = match Mmap::anonymous(RAM_SIZE, Protection::ReadWrite) {
            Ok(v) => v,
            Err(e) => panic!("{}", e)
        };

        let bootrom = match Mmap::anonymous(BOOTROM_SIZE, Protection::ReadWrite) {
            Ok(v) => v,
            Err(e) => panic!("{}", e)
        };

        Interconnect {
            exi: Exi::new(),
            mmap: mmap,
            bootrom: bootrom
        }
    }

    pub fn read_word(&mut self, address: u32) -> u32 {
        match map_address(address) {
            Address::Ram => { 
                let mut data = [0u8; 4];
                let mmap     = unsafe { self.mmap.as_slice() };

                match {&mmap[address as usize ..]}.read(&mut data) {
                    Ok(_) => BigEndian::read_u32(&data),
                    Err(e) => panic!("{}", e)
                }
            },
            Address::ExpansionInterface(channel, register) => {
                self.exi.read(channel, register)    
            },
            Address::Bootrom(offset) => {
                let mut data = [0u8; 4];
                let bootrom  = unsafe { self.bootrom.as_slice() };

                match {&bootrom[offset as usize ..]}.read(&mut data) {
                    Ok(_) => BigEndian::read_u32(&data),
                    Err(e) => panic!("{}", e)
                }
            },
            _ => {
                println!("interconnect read_word not implemented for address {:#x}", address);
                0 // FIXME: this is bad and I should feel bad too ;)
            }
        }
    }

    // FixMe: could remove, used for populating bootrom only
    pub fn write(&mut self, address: u32, data: &[u8]) {
        match map_address(address) {
            Address::Bootrom(offset) => {
                let mut bootrom = unsafe { self.bootrom.as_mut_slice() };

                match {&mut bootrom[offset as usize ..]}.write(data) {
                    Ok(_) => {}
                    Err(e) => panic!("{}", e)
                }
            },
            _ => println!("interconnect write not implemented for address {:#x}", address)
        }
    }

    pub fn write_word(&mut self, address: u32, value: u32) {
        match map_address(address) {
            Address::ExpansionInterface(channel, register) => {
                self.exi.write(channel, register, value);
            },
            _ => println!("interconnect write_word not implemented for address {:#x}", address)
        }
    }

    pub fn write_halfword(&mut self, address: u32, value: u16) {
        match map_address(address) {
            Address::Ram => {
                let mut data = [0u8; 2];
                let mut mmap = unsafe { self.mmap.as_mut_slice() };

                BigEndian::write_u16(&mut data, value);

                match {&mut mmap[address as usize ..]}.write(&data) {
                    Ok(_) => {}
                    Err(e) => panic!("{}", e)
                }
            }
            _ => println!("interconnect write_halfword not implemented for address {:#x}", address)
        }
    }
}

impl fmt::Debug for Interconnect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fixme")
    }
}
