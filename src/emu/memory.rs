use byteorder::{BigEndian, ByteOrder};

/// Main Memory Size: 24MB
const MEMORY_SIZE: usize = 0x180_0000;

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

    pub fn read_u16(&self, addr: u32) -> u16 {
        BigEndian::read_u16(&self.data[addr as usize..])
    }

    pub fn read_u32(&self, addr: u32) -> u32 {
        BigEndian::read_u32(&self.data[addr as usize..])
    }

    pub fn read_u64(&self, addr: u32) -> u64 {
        BigEndian::read_u64(&self.data[addr as usize..])
    }

    pub fn read_f32(&self, addr: u32) -> f32 {
        BigEndian::read_f32(&self.data[addr as usize..])
    }

    pub fn read_dma(&mut self, addr: u32, buf: &mut [u8]) {
        for (i, elem) in buf.iter_mut().enumerate() {
            *elem = self.data[addr as usize + i];
        }
    }

    pub fn write_u8(&mut self, addr: u32, val: u8) {
        self.data[addr as usize] = val;
    }

    pub fn write_u16(&mut self, addr: u32, val: u16) {
        BigEndian::write_u16(&mut self.data[addr as usize..], val);
    }

    pub fn write_u32(&mut self, addr: u32, val: u32) {
        BigEndian::write_u32(&mut self.data[addr as usize..], val);
    }

    pub fn write_u64(&mut self, addr: u32, val: u64) {
        BigEndian::write_u64(&mut self.data[addr as usize..], val);
    }

    pub fn write_dma(&mut self, addr: u32, buf: &[u8]) {
        for (i, elem) in buf.iter().enumerate() {
            self.data[addr as usize + i] = *elem;
        }
    }
}
