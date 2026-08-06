use std::{cell::RefCell, fs::File, io::Read, path::Path, rc::Rc};

use byteorder::{BigEndian, ByteOrder};

use crate::{
    bus::{Bus, ReadWrite},
    cpu::CpuState,
};

pub(crate) const BOOTROM_SIZE: usize = 0x20_0000; // 2 MB MaskROM
pub(crate) const IPL_MEM_SIZE: usize = 0x80_0450;

pub struct Bootrom {
    data: Rc<RefCell<Vec<u8>>>,
}

impl Default for Bootrom {
    fn default() -> Self {
        Self {
            data: Rc::new(RefCell::new(vec![0; IPL_MEM_SIZE])),
        }
    }
}

impl Bootrom {
    pub(crate) const BASE_ADDR: u32 = 0xFFF0_0000;

    pub(crate) fn new(data: Rc<RefCell<Vec<u8>>>) -> Self {
        Self { data }
    }

    // load ipl into bootrom and decrypt
    pub(crate) fn load_ipl<P: AsRef<Path>>(&mut self, path: P) {
        let mut file = match File::open(path) {
            Ok(v) => v,
            Err(e) => {
                panic!("{}", e);
            }
        };

        let mut data = self.data.borrow_mut();

        match file.read_exact(&mut data[..BOOTROM_SIZE]) {
            Ok(_) => {}
            Err(e) => {
                panic!("{}", e);
            }
        };

        descrambler(&mut data[0x100..0x1AFF00]);
    }

    fn offset(addr: u32) -> usize {
        (addr - Self::BASE_ADDR) as usize
    }

    fn read_u8(&self, addr: u32) -> u8 {
        self.data.borrow()[Self::offset(addr)]
    }

    fn read_u16(&self, addr: u32) -> u16 {
        BigEndian::read_u16(&self.data.borrow()[Self::offset(addr)..])
    }

    fn read_u32(&self, addr: u32) -> u32 {
        BigEndian::read_u32(&self.data.borrow()[Self::offset(addr)..])
    }

    fn read_u64(&self, addr: u32) -> u64 {
        BigEndian::read_u64(&self.data.borrow()[Self::offset(addr)..])
    }
}

impl ReadWrite<u8> for Bootrom {
    fn read(bus: &mut Bus, _: &mut CpuState, addr: u32) -> u8 {
        bus.bootrom.read_u8(addr)
    }

    fn write(_bus: &mut Bus, _: &mut CpuState, _addr: u32, _val: u8) {
        panic!("Bootrom is not writeable");
    }
}

impl ReadWrite<u16> for Bootrom {
    fn read(bus: &mut Bus, _: &mut CpuState, addr: u32) -> u16 {
        bus.bootrom.read_u16(addr)
    }

    fn write(_bus: &mut Bus, _: &mut CpuState, _addr: u32, _val: u16) {
        panic!("Bootrom is not writeable");
    }
}

impl ReadWrite<u32> for Bootrom {
    fn read(bus: &mut Bus, _: &mut CpuState, addr: u32) -> u32 {
        bus.bootrom.read_u32(addr)
    }

    fn write(_bus: &mut Bus, _: &mut CpuState, _addr: u32, _val: u32) {
        panic!("Bootrom is not writeable");
    }
}

impl ReadWrite<u64> for Bootrom {
    fn read(bus: &mut Bus, _: &mut CpuState, addr: u32) -> u64 {
        bus.bootrom.read_u64(addr)
    }

    fn write(_bus: &mut Bus, _: &mut CpuState, _addr: u32, _val: u64) {
        panic!("Bootrom is not writeable");
    }
}

// Rust port of descrambler from Dolphin Emulater source code
// https://github.com/dolphin-emu/dolphin/blob/master/Source/Core/Core/HW/EXI/EXI_DeviceIPL.cpp#L49
//
// bootrom descrambler reversed by segher
// Copyright 2008 Segher Boessenkool <segher@kernel.crashing.org>
fn descrambler(data: &mut [u8]) {
    let size = data.len();
    let mut acc: u8 = 0;
    let mut nacc: u8 = 0;

    let mut t: u16 = 0x2953;
    let mut u: u16 = 0xd9c2;
    let mut v: u16 = 0x3ff1;

    let mut x: u8 = 1;

    let mut it = 0;

    while it < size {
        let t0 = t & 1;
        let t1 = (t >> 1) & 1;
        let u0 = u & 1;
        let u1 = (u >> 1) & 1;
        let v0 = v & 1;

        x ^= (t1 ^ v0) as u8;
        x ^= (u0 | u1) as u8;
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

        nacc += 1;
        acc = (2 * u16::from(acc) + u16::from(x)) as u8;
        if nacc == 8 {
            data[it] ^= acc;
            it += 1;
            nacc = 0;
        }
    }
}
