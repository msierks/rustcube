use byteorder::{BigEndian, ByteOrder};

use crate::{
    bus::{Bus, ReadWrite},
    cpu::CpuState,
};

pub(crate) const L1_CACHE_BASE: u32 = 0xE000_0000;
pub(crate) const L1_CACHE_SIZE: u32 = 0x4000; // 16 KiB locked half

pub struct L1Cache {
    data: Box<[u8]>,
}

impl Default for L1Cache {
    fn default() -> Self {
        L1Cache {
            data: vec![0; L1_CACHE_SIZE as usize].into_boxed_slice(),
        }
    }
}

impl L1Cache {
    pub fn contains(addr: u32) -> bool {
        (L1_CACHE_BASE..L1_CACHE_BASE + L1_CACHE_SIZE).contains(&addr)
    }

    fn offset(addr: u32) -> usize {
        (addr - L1_CACHE_BASE) as usize
    }

    pub fn read_u8(&self, addr: u32) -> u8 {
        self.data[Self::offset(addr)]
    }

    pub fn read_u16(&self, addr: u32) -> u16 {
        BigEndian::read_u16(&self.data[Self::offset(addr)..])
    }

    pub fn read_u32(&self, addr: u32) -> u32 {
        BigEndian::read_u32(&self.data[Self::offset(addr)..])
    }

    pub fn read_u64(&self, addr: u32) -> u64 {
        BigEndian::read_u64(&self.data[Self::offset(addr)..])
    }

    pub fn write_u8(&mut self, addr: u32, val: u8) {
        self.data[Self::offset(addr)] = val;
    }

    pub fn write_u16(&mut self, addr: u32, val: u16) {
        BigEndian::write_u16(&mut self.data[Self::offset(addr)..], val);
    }

    pub fn write_u32(&mut self, addr: u32, val: u32) {
        BigEndian::write_u32(&mut self.data[Self::offset(addr)..], val);
    }

    pub fn write_u64(&mut self, addr: u32, val: u64) {
        BigEndian::write_u64(&mut self.data[Self::offset(addr)..], val);
    }

    pub fn write_bytes(&mut self, addr: u32, buf: &[u8]) {
        let start = Self::offset(addr);
        self.data[start..start + buf.len()].copy_from_slice(buf);
    }
}

impl ReadWrite<u8> for L1Cache {
    fn read(bus: &mut Bus, _: &mut CpuState, addr: u32) -> u8 {
        bus.l1_cache.read_u8(addr)
    }

    fn write(bus: &mut Bus, _: &mut CpuState, addr: u32, val: u8) {
        bus.l1_cache.write_u8(addr, val)
    }
}

impl ReadWrite<u16> for L1Cache {
    fn read(bus: &mut Bus, _: &mut CpuState, addr: u32) -> u16 {
        bus.l1_cache.read_u16(addr)
    }

    fn write(bus: &mut Bus, _: &mut CpuState, addr: u32, val: u16) {
        bus.l1_cache.write_u16(addr, val)
    }
}

impl ReadWrite<u32> for L1Cache {
    fn read(bus: &mut Bus, _: &mut CpuState, addr: u32) -> u32 {
        bus.l1_cache.read_u32(addr)
    }

    fn write(bus: &mut Bus, _: &mut CpuState, addr: u32, val: u32) {
        bus.l1_cache.write_u32(addr, val)
    }
}

impl ReadWrite<u64> for L1Cache {
    fn read(bus: &mut Bus, _: &mut CpuState, addr: u32) -> u64 {
        bus.l1_cache.read_u64(addr)
    }

    fn write(bus: &mut Bus, _: &mut CpuState, addr: u32, val: u64) {
        bus.l1_cache.write_u64(addr, val)
    }
}
