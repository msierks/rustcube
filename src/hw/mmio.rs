use crate::{
    bus::{Bus, ReadWrite},
    cpu::CpuState,
};

const BLOCK_SIZE: usize = 0x1_0000;

type MmioReadFn<T> = fn(&mut Bus, &mut CpuState, u32) -> T;
type MmioWriteFn<T> = fn(&mut Bus, &mut CpuState, u32, T);

pub(crate) trait MmioDevice {
    const BASE_ADDR: u32;

    fn register_mmio(mmio: &mut Mmio);
}

pub struct Mmio {
    read_u8_fns: Box<[MmioReadFn<u8>; BLOCK_SIZE]>,
    read_u16_fns: Box<[MmioReadFn<u16>; BLOCK_SIZE / size_of::<u16>()]>,
    read_u32_fns: Box<[MmioReadFn<u32>; BLOCK_SIZE / size_of::<u32>()]>,
    write_u8_fns: Box<[MmioWriteFn<u8>; BLOCK_SIZE]>,
    write_u16_fns: Box<[MmioWriteFn<u16>; BLOCK_SIZE / size_of::<u16>()]>,
    write_u32_fns: Box<[MmioWriteFn<u32>; BLOCK_SIZE / size_of::<u32>()]>,
}

impl Default for Mmio {
    fn default() -> Self {
        Self {
            read_u8_fns: Box::new([default_read_u8_handler; BLOCK_SIZE]),
            read_u16_fns: Box::new([default_read_u16_fn; BLOCK_SIZE / size_of::<u16>()]),
            read_u32_fns: Box::new([default_read_u32_fn; BLOCK_SIZE / size_of::<u32>()]),
            write_u8_fns: Box::new([default_write_u8_fn; BLOCK_SIZE]),
            write_u16_fns: Box::new([default_write_u16_fn; BLOCK_SIZE / size_of::<u16>()]),
            write_u32_fns: Box::new([default_write_u32_fn; BLOCK_SIZE / size_of::<u32>()]),
        }
    }
}

impl Mmio {
    /// Register all handlers for a device.
    pub(crate) fn register_device<D: MmioDevice>(&mut self) {
        D::register_mmio(self);
    }

    /// Table index from the low 16 bits of `addr` only.
    fn unique_id<T>(addr: u32) -> usize {
        (addr & 0xFFFF) as usize / size_of::<T>()
    }

    //pub fn register_u8(&mut self, addr: u32, read_fn: MmioReadFn<u8>, write_fn: MmioWriteFn<u8>) {
    //    self.register_read_u8(addr, read_fn);
    //    self.register_write_u8(addr, write_fn);
    //}

    pub fn register_u16(
        &mut self,
        addr: u32,
        read_fn: MmioReadFn<u16>,
        write_fn: MmioWriteFn<u16>,
    ) {
        self.register_read_u16(addr, read_fn);
        self.register_write_u16(addr, write_fn);
    }

    pub fn register_u32(
        &mut self,
        addr: u32,
        read_fn: MmioReadFn<u32>,
        write_fn: MmioWriteFn<u32>,
    ) {
        self.register_read_u32(addr, read_fn);
        self.register_write_u32(addr, write_fn);
    }

    pub fn register_read_u8(&mut self, addr: u32, read_handler: MmioReadFn<u8>) {
        self.read_u8_fns[Self::unique_id::<u8>(addr)] = read_handler;
    }

    pub fn register_read_u16(&mut self, addr: u32, read_handler: MmioReadFn<u16>) {
        self.read_u16_fns[Self::unique_id::<u16>(addr)] = read_handler;
    }

    pub fn register_read_u32(&mut self, addr: u32, read_handler: MmioReadFn<u32>) {
        self.read_u32_fns[Self::unique_id::<u32>(addr)] = read_handler;
    }

    fn get_read_u8(&self, addr: u32) -> MmioReadFn<u8> {
        self.read_u8_fns[Self::unique_id::<u8>(addr)]
    }

    fn get_read_u16(&self, addr: u32) -> MmioReadFn<u16> {
        self.read_u16_fns[Self::unique_id::<u16>(addr)]
    }

    fn get_read_u32(&self, addr: u32) -> MmioReadFn<u32> {
        self.read_u32_fns[Self::unique_id::<u32>(addr)]
    }

    pub fn register_write_u8(&mut self, addr: u32, handler: MmioWriteFn<u8>) {
        self.write_u8_fns[Self::unique_id::<u8>(addr)] = handler;
    }

    pub fn register_write_u16(&mut self, addr: u32, handler: MmioWriteFn<u16>) {
        self.write_u16_fns[Self::unique_id::<u16>(addr)] = handler;
    }

    pub fn register_write_u32(&mut self, addr: u32, handler: MmioWriteFn<u32>) {
        self.write_u32_fns[Self::unique_id::<u32>(addr)] = handler;
    }

    fn get_write_u8(&self, addr: u32) -> MmioWriteFn<u8> {
        self.write_u8_fns[Self::unique_id::<u8>(addr)]
    }

    fn get_write_u16(&self, addr: u32) -> MmioWriteFn<u16> {
        self.write_u16_fns[Self::unique_id::<u16>(addr)]
    }

    fn get_write_u32(&self, addr: u32) -> MmioWriteFn<u32> {
        self.write_u32_fns[Self::unique_id::<u32>(addr)]
    }
}

fn default_read_u8_handler(_: &mut Bus, _: &mut CpuState, addr: u32) -> u8 {
    warn!("Unhandled 8 bit read from address: {:#010X}", addr);
    0
}

fn default_read_u16_fn(_: &mut Bus, _: &mut CpuState, addr: u32) -> u16 {
    warn!("Unhandled 8 bit read from address: {:#010X}", addr);
    0
}

fn default_read_u32_fn(_: &mut Bus, _: &mut CpuState, addr: u32) -> u32 {
    warn!("Unhandled 32 bit read from address: {:#010X}", addr);
    0
}

fn default_write_u8_fn(_: &mut Bus, _: &mut CpuState, addr: u32, val: u8) {
    warn!("Unhandled 8 bit write to address: {:#010X} {:}", addr, val);
}

fn default_write_u16_fn(_: &mut Bus, _: &mut CpuState, addr: u32, val: u16) {
    warn!("Unhandled 16 bit write to address: {:#010X} {:}", addr, val);
}

fn default_write_u32_fn(_: &mut Bus, _: &mut CpuState, addr: u32, val: u32) {
    warn!("Unhandled 32 bit write to address: {:#010X} {:}", addr, val);
}

impl ReadWrite<u8> for Mmio {
    fn read(bus: &mut Bus, cpu_state: &mut CpuState, addr: u32) -> u8 {
        bus.mmio.get_read_u8(addr)(bus, cpu_state, addr)
    }

    fn write(bus: &mut Bus, cpu_state: &mut CpuState, addr: u32, val: u8) {
        bus.mmio.get_write_u8(addr)(bus, cpu_state, addr, val)
    }
}

impl ReadWrite<u16> for Mmio {
    fn read(bus: &mut Bus, cpu_state: &mut CpuState, addr: u32) -> u16 {
        bus.mmio.get_read_u16(addr)(bus, cpu_state, addr)
    }
    fn write(bus: &mut Bus, cpu_state: &mut CpuState, addr: u32, val: u16) {
        bus.mmio.get_write_u16(addr)(bus, cpu_state, addr, val)
    }
}

impl ReadWrite<u32> for Mmio {
    fn read(bus: &mut Bus, cpu_state: &mut CpuState, addr: u32) -> u32 {
        bus.mmio.get_read_u32(addr)(bus, cpu_state, addr)
    }

    fn write(bus: &mut Bus, cpu_state: &mut CpuState, addr: u32, val: u32) {
        bus.mmio.get_write_u32(addr)(bus, cpu_state, addr, val)
    }
}

impl ReadWrite<u64> for Mmio {
    fn read(_bus: &mut Bus, _: &mut CpuState, _addr: u32) -> u64 {
        panic!("Mmio doesn't support read::<u64>");
    }

    fn write(_bus: &mut Bus, _: &mut CpuState, _addr: u32, _val: u64) {
        panic!("Mmio doesn't support write::<u64>");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_unique_id() {
        for (addr, expect) in [
            (0x0C000000, 0x0000_0000),
            (0x0C008000, 0x0000_8000),
            (0x0D008002, 0x0000_8002),
        ] {
            let result = Mmio::unique_id::<u8>(addr);
            assert_eq!(
                result, expect,
                "Result {:#x} Expected: {:#x}",
                result, expect
            );
        }

        for (addr, expect) in [
            (0x0C000000, 0x0000_0000),
            (0x0C008000, 0x0000_4000),
            (0x0D008004, 0x0000_4002),
        ] {
            let result = Mmio::unique_id::<u16>(addr);
            assert_eq!(
                result, expect,
                "Result {:#x} Expected: {:#x}",
                result, expect
            );
        }

        for (addr, expect) in [
            (0x0C000000, 0x0000_0000),
            (0x0C008000, 0x0000_2000),
            (0x0D008004, 0x0000_2001),
        ] {
            let result = Mmio::unique_id::<u32>(addr);
            assert_eq!(
                result, expect,
                "Result {:#x} Expected: {:#x}",
                result, expect
            );
        }
    }
}
