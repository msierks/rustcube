use byteorder::{ByteOrder, BigEndian};
use memmap::{Mmap, Protection};
use std::io::{Read, Write};

const RAM_SIZE: usize = 0x1800000; // 24 MB

pub struct Ram {
    data: Mmap
}

impl Ram {

    pub fn new() -> Ram {
        let data = match Mmap::anonymous(RAM_SIZE, Protection::ReadWrite) {
            Ok(v) => v,
            Err(e) => panic!("{}", e)
        };

        Ram {
            data: data
        }
    }

    pub fn read_u8(&self, addr: u32) -> u8 {
        let mut buf = [0u8; 1];
        let data = unsafe { self.data.as_slice() };

        match {&data[addr as usize ..]}.read(&mut buf) {
            Ok(_) => data[0],
            Err(e) => panic!("{}", e)
        }
    }

    pub fn read_u16(&self, addr: u32) -> u16 {
        let mut buf = [0u8; 2];
        let data = unsafe { self.data.as_slice() };

        match {&data[addr as usize ..]}.read(&mut buf) {
            Ok(_) => BigEndian::read_u16(&data),
            Err(e) => panic!("{}", e)
        }
    }

    pub fn read_u32(&self, addr: u32) -> u32 {
        let mut buf = [0u8; 4];
        let data = unsafe { self.data.as_slice() };

        match {&data[addr as usize ..]}.read(&mut buf) {
            Ok(_) => BigEndian::read_u32(&data),
            Err(e) => panic!("{}", e)
        }
    }

    pub fn read_u64(&self, addr: u32) -> u64 {
        let mut buf = [0u8; 8];
        let data = unsafe { self.data.as_slice() };

        match {&data[addr as usize ..]}.read(&mut buf) {
            Ok(_) => BigEndian::read_u64(&data),
            Err(e) => panic!("{}", e)
        }
    }

    pub fn write_u8(&mut self, addr: u32, val: u8) {
        let buf = [val];
        let mut data = unsafe { self.data.as_mut_slice() };

        match {&mut data[addr as usize ..]}.write(&buf) {
            Ok(_) => {}
            Err(e) => panic!("{}", e)
        }
    }

    pub fn write_u16(&mut self, addr: u32, val: u16) {
        let mut buf  = [0u8; 2];
        let mut data = unsafe { self.data.as_mut_slice() };

        BigEndian::write_u16(&mut buf, val);

        match {&mut data[addr as usize ..]}.write(&buf) {
            Ok(_) => {}
            Err(e) => panic!("{}", e)
        }
    }

    pub fn write_u32(&mut self, addr: u32, val: u32) {
        let mut buf  = [0u8; 4];
        let mut data = unsafe { self.data.as_mut_slice() };

        BigEndian::write_u32(&mut buf, val);

        match {&mut data[addr as usize ..]}.write(&buf) {
            Ok(_) => {}
            Err(e) => panic!("{}", e)
        }
    }

    pub fn write_u64(&mut self, addr: u32, val: u64) {
        let mut buf  = [0u8; 8];
        let mut data = unsafe { self.data.as_mut_slice() };

        BigEndian::write_u64(&mut buf, val);

        match {&mut data[addr as usize ..]}.write(&buf) {
            Ok(_) => {}
            Err(e) => panic!("{}", e)
        }
    }

}
