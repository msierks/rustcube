use byteorder::{BigEndian, ByteOrder};

use crate::{
    bus::{Bus, ReadWrite},
    cpu::CpuState,
};

/// Main Memory Size: 24MB
pub(crate) const MEMORY_SIZE: u32 = 0x180_0000;

pub struct Memory {
    data: Box<[u8]>,
}

impl Default for Memory {
    fn default() -> Self {
        Memory {
            data: vec![0; MEMORY_SIZE as usize].into_boxed_slice(),
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

    #[allow(dead_code)]
    pub fn read_u64(&self, addr: u32) -> u64 {
        BigEndian::read_u64(&self.data[addr as usize..])
    }

    pub fn read_f32(&self, addr: u32) -> f32 {
        f32::from_bits(BigEndian::read_u32(&self.data[addr as usize..]))
    }

    #[allow(dead_code)]
    pub fn read_bytes(&self, addr: u32, buf: &mut [u8]) {
        for (i, elem) in buf.iter_mut().enumerate() {
            *elem = self.data[addr as usize + i];
        }
    }

    #[allow(dead_code)]
    pub fn read_string(&self, mut addr: u32) -> String {
        let mut s = String::new();
        loop {
            let res = self.read_u8(addr);
            if res == 0 {
                break;
            }
            s.push(res as char);

            addr += 1;
        }

        s
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

    pub fn write_bytes(&mut self, addr: u32, buf: &[u8]) {
        for (i, elem) in buf.iter().enumerate() {
            self.data[addr as usize + i] = *elem;
        }
    }
}

impl ReadWrite<u8> for Memory {
    fn read(bus: &mut Bus, _: &mut CpuState, addr: u32) -> u8 {
        bus.memory.read_u8(addr)
    }

    fn write(bus: &mut Bus, _: &mut CpuState, addr: u32, val: u8) {
        bus.memory.write_u8(addr, val)
    }
}

impl ReadWrite<u16> for Memory {
    fn read(bus: &mut Bus, _: &mut CpuState, addr: u32) -> u16 {
        bus.memory.read_u16(addr)
    }

    fn write(bus: &mut Bus, _: &mut CpuState, addr: u32, val: u16) {
        bus.memory.write_u16(addr, val)
    }
}

impl ReadWrite<u32> for Memory {
    fn read(bus: &mut Bus, _: &mut CpuState, addr: u32) -> u32 {
        bus.memory.read_u32(addr)
    }

    fn write(bus: &mut Bus, _: &mut CpuState, addr: u32, val: u32) {
        bus.memory.write_u32(addr, val)
    }
}

impl ReadWrite<u64> for Memory {
    fn read(bus: &mut Bus, _: &mut CpuState, addr: u32) -> u64 {
        bus.memory.read_u64(addr)
    }

    fn write(bus: &mut Bus, _: &mut CpuState, addr: u32, val: u64) {
        bus.memory.write_u64(addr, val)
    }
}
