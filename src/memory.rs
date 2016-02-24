use std::fmt;
use std::io::{Read, Write};
use memmap::{Mmap, Protection};
use byteorder::{ByteOrder, LittleEndian};

const RAM_SIZE: usize     = 0x1800000; // 24 MB
const BOOTROM_SIZE: usize = 0x0100000; // 1 MB

pub struct Memory {
    mmap: Mmap,
    bootrom: Mmap
}

// ToDo: add write/read for u8, u16, u32, u64

impl Memory {
    pub fn new() -> Memory {
        let mmap = match Mmap::anonymous(RAM_SIZE, Protection::ReadWrite) {
            Ok(v) => v,
            Err(e) => panic!("{}", e)
        };

        let bootrom = match Mmap::anonymous(BOOTROM_SIZE, Protection::ReadWrite) {
            Ok(v) => v,
            Err(e) => panic!("{}", e)
        };

        Memory {
            mmap: mmap,
            bootrom: bootrom
        }
    }

    pub fn read(&mut self, address: u32, data: &mut [u8]) {
        match address {
            0x00000000 ... 0x07FFFFFF => { // Main Memory (RAM)
                let mmap = unsafe { self.mmap.as_slice() };

                match {&mmap[address as usize ..]}.read(data) {
                    Ok(_) => {}
                    Err(e) => panic!("{}", e)
                }
            },
            0x08000000 ... 0x0BFFFFFF => { // Embedded Framebuffer (EFB)
                println!("memory address not implemented");
            },
            0x0C000000 ... 0x0C000FFF => { // Command Processor (CP)
                println!("memory address not implemented");
            },
            0x0C001000 ... 0x0C001FFF => { // Pixel Engine (PE)
                println!("memory address not implemented");
            },
            0x0C002000 ... 0x0C002FFF => { // Video Interface (VI)
                println!("memory address not implemented");
            },
            0x0C003000 ... 0x0C003FFF => { // Peripheral Interface (PI)
                println!("memory address not implemented");
            },
            0x0C004000 ... 0x0C004FFF => { // Memory Interface (MI)
                println!("memory address not implemented");
            },
            0x0C005000 ... 0x0C005FFF => { // DSP and DMA Audio Interface (AID)
                println!("memory address not implemented");
            },
            0x0C006000 ... 0x0C0063FF => { // DVD Interface (DI)
                println!("memory address not implemented");
            },
            0x0C006400 ... 0x0C0067FF => { // Serial Interface (SI)
                println!("memory address not implemented");
            },
            0x0C006800 ... 0x0C0068FF => { // External Interface (EXI)
                println!("memory address not implemented");
            },
            0x0C006C00 ... 0x0C006CFF => { // Audio Streaming Interface (AIS)
                println!("memory address not implemented");
            },
            0x0C008000 ... 0x0C008FFF => { // PI FIFO (GX)
                println!("memory address not implemented");
            },
            0xFFF00000 ... 0xFFFFFFFF => { // Bootrom
                let bootrom = unsafe { self.bootrom.as_slice() };

                match {&bootrom[(address - 0xFFF00000) as usize ..]}.read(data) {
                    Ok(_) => {}
                    Err(e) => panic!("{}", e)
                }
            },
            _ => panic!("read from invalid memory address {:#x}", address)
        }
    }

    pub fn read_u32(&mut self, address: u32) -> u32 {
        let mut data = [0u8; 4];

        self.read(address, &mut data);

        LittleEndian::read_u32(&data)
    }

    pub fn write(&mut self, address: u32, data: &[u8]) { // FixMe: probably change to size
        match address {
            0x00000000 ... 0x07FFFFFF => { // Main Memory (RAM)
                let mut mmap = unsafe { self.mmap.as_mut_slice() };

                match {&mut mmap[address as usize ..]}.write(data) {
                    Ok(_) => {}
                    Err(e) => panic!("{}", e)
                }
            },
            0x08000000 ... 0x0BFFFFFF => { // Embedded Framebuffer (EFB)
                println!("memory address not implemented");
            },
            0x0C000000 ... 0x0C000FFF => { // Command Processor (CP)
                println!("memory address not implemented");
            },
            0x0C001000 ... 0x0C001FFF => { // Pixel Engine (PE)
                println!("memory address not implemented");
            },
            0x0C002000 ... 0x0C002FFF => { // Video Interface (VI)
                println!("memory address not implemented");
            },
            0x0C003000 ... 0x0C003FFF => { // Peripheral Interface (PI)
                println!("memory address not implemented");
            },
            0x0C004000 ... 0x0C004FFF => { // Memory Interface (MI)
                println!("memory address not implemented");
            },
            0x0C005000 ... 0x0C005FFF => { // DSP and DMA Audio Interface (AID)
                println!("memory address not implemented");
            },
            0x0C006000 ... 0x0C0063FF => { // DVD Interface (DI)
                println!("memory address not implemented");
            },
            0x0C006400 ... 0x0C0067FF => { // Serial Interface (SI)
                println!("memory address not implemented");
            },
            0x0C006800 ... 0x0C0068FF => { // External Interface (EXI)
                println!("memory address not implemented");
            },
            0x0C006C00 ... 0x0C006CFF => { // Audio Streaming Interface (AIS)
                println!("memory address not implemented");
            },
            0x0C008000 ... 0x0C008FFF => { // PI FIFO (GX)
                println!("memory address not implemented");
            },
            0xFFF00000 ... 0xFFFFFFFF => { // Bootrom
                let mut bootrom = unsafe { self.bootrom.as_mut_slice() };

                match {&mut bootrom[(address - 0xFFF00000) as usize ..]}.write(data) {
                    Ok(_) => {}
                    Err(e) => panic!("{}", e)
                }
            },
            _ => panic!("write to invalid memory address {:#x}", address)
        }
    }

    pub fn write_u16(&mut self, address: u32, value: u16) {
        let mut data = [0u8; 2];

        LittleEndian::write_u16(&mut data, value);

        self.write(address, &data);
    }

    pub fn write_u32(&mut self, address: u32, value: u32) {
        let mut data = [0u8; 4];

        LittleEndian::write_u32(&mut data, value);

        self.write(address, &data);
    }
}

impl fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fixme")
    }
}
