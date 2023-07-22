use byteorder::{BigEndian, ByteOrder};

use super::Context;

/// Main Memory Size: 24MB
pub const MEMORY_SIZE: usize = 0x180_0000;

pub struct Memory {
    data: Box<[u8]>,
}

impl Default for Memory {
    fn default() -> Self {
        Memory {
            data: vec![0; MEMORY_SIZE].into_boxed_slice(),
        }
    }
}

impl Memory {
    pub fn read_u8(&self, addr: u32) -> u8 {
        self.data[addr as usize]
    }

    pub fn write_u8(&mut self, addr: u32, val: u8) {
        self.data[addr as usize] = val;
    }

    pub fn write(&mut self, addr: u32, buf: &[u8]) {
        for (i, elem) in buf.iter().enumerate() {
            self.data[addr as usize + i] = *elem;
        }
    }
}

//pub fn read(ctx: &mut Context, addr: u32, buf: &mut [u8]) {
//    for (i, elem) in buf.iter_mut().enumerate() {
//        *elem = ctx.mem.data[addr as usize + i];
//    }
//}

pub fn read_u16(ctx: &mut Context, addr: u32) -> u16 {
    BigEndian::read_u16(&ctx.mem.data[addr as usize..])
}

pub fn read_u32(ctx: &mut Context, addr: u32) -> u32 {
    BigEndian::read_u32(&ctx.mem.data[addr as usize..])
}

pub fn read_u64(ctx: &mut Context, addr: u32) -> u64 {
    BigEndian::read_u64(&ctx.mem.data[addr as usize..])
}

pub fn write(ctx: &mut Context, addr: u32, buf: &[u8]) {
    for (i, elem) in buf.iter().enumerate() {
        ctx.mem.data[addr as usize + i] = *elem;
    }
}

pub fn write_u16(ctx: &mut Context, addr: u32, val: u16) {
    BigEndian::write_u16(&mut ctx.mem.data[addr as usize..], val);
}

pub fn write_u32(ctx: &mut Context, addr: u32, val: u32) {
    BigEndian::write_u32(&mut ctx.mem.data[addr as usize..], val);
}

pub fn write_u64(ctx: &mut Context, addr: u32, val: u64) {
    BigEndian::write_u64(&mut ctx.mem.data[addr as usize..], val);
}
