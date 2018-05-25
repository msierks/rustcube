
use byteorder::{ByteOrder, BigEndian};

/// Main RAM Size: 24MB
const RAM_SIZE: usize = 0x180_0000;

pub struct Ram {
    data: Box<[u8]>,
}

impl Default for Ram {
    fn default() -> Self {
        Ram {
            data: vec![0; RAM_SIZE].into_boxed_slice(),
        }
    }
}

impl Ram {
    pub fn read_u8(&self, addr: u32) -> u8 {
        self.data[addr as usize]
    }

    pub fn read_u16(&self, addr: u32) -> u16 {
        BigEndian::read_u16(&self.data[addr as usize ..])
    }

    pub fn read_u32(&self, addr: u32) -> u32 {
        BigEndian::read_u32(&self.data[addr as usize ..])
    }

    pub fn read_u64(&self, addr: u32) -> u64 {
        BigEndian::read_u64(&self.data[addr as usize ..])
    }

    pub fn read_dma(&mut self, addr: u32, buf: &mut [u8]) {
        for i in 0..buf.len() {
            buf[i] = self.data[addr as usize + i];
        }
    }

    pub fn write_u8(&mut self, addr: u32, val: u8) {
        self.data[addr as usize] = val;
    }

    pub fn write_u16(&mut self, addr: u32, val: u16) {
        BigEndian::write_u16(&mut self.data[addr as usize ..], val);
    }

    pub fn write_u32(&mut self, addr: u32, val: u32) {
        BigEndian::write_u32(&mut self.data[addr as usize ..], val);
    }

    pub fn write_u64(&mut self, addr: u32, val: u64) {
        BigEndian::write_u64(&mut self.data[addr as usize ..], val);
    }

    pub fn write_dma(&mut self, addr: u32, buf: &[u8]) {
        for i in 0..buf.len() {
            self.data[addr as usize + i] = buf[i];
        }
    }

}
