use std::fmt;
use std::io::{Read, Write};
use memmap::{Mmap, Protection};
use byteorder::{ByteOrder, BigEndian};
use super::exi::Exi;

use std::rc::Rc;
use std::cell::RefCell;

use std::fs;
use std::path::Path;

const RAM_SIZE:     usize = 0x1800000; // 24 MB
const BOOTROM_SIZE: usize = 0x0200000; // 2 MB

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
        0x00000000 ... 0x017FFFFF => Address::Ram,
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
    bootrom: Rc<RefCell<Box<[u8; BOOTROM_SIZE]>>>
}

impl Interconnect {
    pub fn new() -> Interconnect {
        let mmap = match Mmap::anonymous(RAM_SIZE, Protection::ReadWrite) {
            Ok(v) => v,
            Err(e) => panic!("{}", e)
        };

        let bootrom = Rc::new(RefCell::new(Box::new([0; BOOTROM_SIZE])));

        Interconnect {
            exi: Exi::new(bootrom.clone()),
            mmap: mmap,
            bootrom: bootrom
        }
    }

    pub fn read_byte(&mut self, address: u32) -> u8 {
        match map_address(address) {
            Address::Ram => {
                let mut data = [0u8; 1];
                let mmap     = unsafe { self.mmap.as_slice() };

                match {&mmap[address as usize ..]}.read(&mut data) {
                    Ok(_) => data[0],
                    Err(e) => panic!("{}", e)
                }
            },
            _ => {
                println!("interconnect read_byte not implemented for address {:#x}", address);
                0 // FIXME: this is bad and I should feel bad too ;)
            }
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
                BigEndian::read_u32(&self.bootrom.borrow()[offset as usize ..])
            },
            _ => {
                println!("interconnect read_word not implemented for address {:#x}", address);
                0 // FIXME: this is bad and I should feel bad too ;)
            }
        }
    }

    pub fn read_halfword(&mut self, address: u32) -> u16 {
        match map_address(address) {
            Address::Ram => {
                let mut data = [0u8; 2];
                let mmap     = unsafe { self.mmap.as_slice() };

                match {&mmap[address as usize ..]}.read(&mut data) {
                    Ok(_) => BigEndian::read_u16(&data),
                    Err(e) => panic!("{}", e)
                }
            },
            _ => {
                println!("interconnect read_halfword not implemented for address {:#x}", address);
                0 // FIXME: this is bad and I should feel bad too ;)
            }
        }
    }

    pub fn read_doubleword(&mut self, address: u32) -> u64 {
        match map_address(address) {
            Address::Ram => {
                let mut data = [0u8; 8];
                let mmap     = unsafe { self.mmap.as_slice() };

                match {&mmap[address as usize ..]}.read(&mut data) {
                    Ok(_) => BigEndian::read_u64(&data),
                    Err(e) => panic!("{}", e)
                }
            },
            _ => {
                panic!("interconnect read_doubleword not implemented for address {:#x}", address);
            }
        }
    }

    pub fn write_byte(&mut self, address: u32, value: u8) {
        match map_address(address) {
            Address::Ram => {
                let data = [value];
                let mut mmap = unsafe { self.mmap.as_mut_slice() };

                match {&mut mmap[address as usize ..]}.write(&data) {
                    Ok(_) => {}
                    Err(e) => panic!("{}", e)
                }
            },
            _ => println!("interconnect write_byte not implemented for address {:#x}", address)
        }
    }

    pub fn write_word(&mut self, address: u32, value: u32) {
        match map_address(address) {
            Address::Ram => {
                let mut data = [0u8; 4];
                let mut mmap = unsafe { self.mmap.as_mut_slice() };

                BigEndian::write_u32(&mut data, value);

                match {&mut mmap[address as usize ..]}.write(&data) {
                    Ok(_) => {}
                    Err(e) => panic!("{}", e)
                }
            },
            Address::ExpansionInterface(channel, register) => {
                self.exi.write(channel, register, value, &mut self.mmap);
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

    // load ipl into bootrom and decrypt
    pub fn load_ipl<P: AsRef<Path>>(&mut self, path: P) {
        let mut file = match fs::File::open(path) {
            Ok(v) => v,
            Err(e) => {
                panic!("{}", e);
            }
        };

        let mut bootrom = self.bootrom.borrow_mut();

        match file.read_exact(&mut **bootrom) {
            Ok(_) => {},
            Err(e) => {
                panic!("{}", e);
            }
        };

        descrambler(&mut bootrom[0x100..0x15ee30]);
    }
}

impl fmt::Debug for Interconnect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fixme")
    }
}

// bootrom descrambler reversed by segher
// Copyright 2008 Segher Boessenkool <segher@kernel.crashing.org>
fn descrambler(data: &mut[u8]) {
    let size = data.len();
    let mut acc :u8 = 0;
    let mut nacc:u8 = 0;

    let mut t:u16 = 0x2953;
    let mut u:u16 = 0xd9c2;
    let mut v:u16 = 0x3ff1;

    let mut x:u8 = 1;

    let mut it = 0;

    while it < size {
        let t0 = t & 1;
        let t1 = (t >> 1) & 1;
        let u0 = u & 1;
        let u1 = (u >> 1) & 1;
        let v0 = v & 1;

        x ^= (t1 ^ v0) as u8;
        x ^= ((u0 | u1)) as u8;
        x ^= ((t0 ^ u1 ^ v0) & (t0 ^ u0)) as u8;

        if t0 == u0 {
            v >>= 1;
            if v0 != 0 {
                v ^= 0xb3d0;
            }
        }

        if t0 == 0 {
            u >>= 1;
            if u0 != 0 {
                u ^= 0xfb10;
            }
        }

        t >>= 1;
        if t0 != 0 {
            t ^= 0xa740;
        }

        nacc+=1;
        acc = (2*acc as u16 + x as u16) as u8;
        if nacc == 8 {
            data[it as usize] ^= acc;
            it+=1;
            nacc = 0;
        }
    }
}
