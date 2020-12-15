mod ai;
mod cpu;
mod di;
mod dol;
mod dsp;
mod exi;
#[cfg(feature = "gdb")]
mod gdb;
mod mem;
mod pi;
mod si;
mod util;

use byteorder::{BigEndian, ByteOrder};
use std::cell::RefCell;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::rc::Rc;

use self::ai::AudioInterface;
use self::cpu::Cpu;
use self::di::DvdInterface;
use self::dol::Dol;
use self::dsp::DspInterface;
use self::exi::ExternalInterface;
use self::mem::Memory;
use self::pi::ProcessorInterface;
use self::si::SerialInterface;

const BOOTROM_SIZE: usize = 0x0020_0000; // 2 MB

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Event {
    Halted,
    Break,
    WatchWrite(u32),
    WatchRead(u32),
}

#[derive(Debug, Copy, Clone)]
enum AccessKind {
    Read,
    Write,
}

#[derive(Debug, Copy, Clone)]
struct Access {
    pub kind: AccessKind,
    pub addr: u32,
}

pub struct Context {
    ai: AudioInterface,
    cpu: Cpu,
    bootrom: Rc<RefCell<Vec<u8>>>,
    mem: Memory,
    di: DvdInterface,
    dsp: DspInterface,
    exi: ExternalInterface,
    pi: ProcessorInterface,
    si: SerialInterface,
    // ToDo: Command Processor
    // ToDo: DVD Interface
    // ToDo: GPFIFO
    // ToDo: Everything Else
    watchpoints: Vec<u32>,
    breakpoints: Vec<u32>,
    hit_watchpoint: Option<Access>,
}

impl Default for Context {
    fn default() -> Self {
        let bootrom = Rc::new(RefCell::new(vec![0; BOOTROM_SIZE]));
        let exi = ExternalInterface::new(bootrom.clone());

        Context {
            ai: Default::default(),
            cpu: Default::default(),
            bootrom,
            mem: Default::default(),
            di: Default::default(),
            dsp: Default::default(),
            exi,
            pi: Default::default(),
            si: Default::default(),
            watchpoints: Default::default(),
            breakpoints: Default::default(),
            hit_watchpoint: None,
        }
    }
}

impl Context {
    pub fn load_dol<P: AsRef<Path>>(&mut self, path: P) {
        let dol = Dol::open(path).unwrap();

        //self.cpu.emulate_bs2();

        dol.load(self);

        self.cpu.set_pc(dol.get_entry_point());
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
            Ok(_) => {}
            Err(e) => {
                panic!("{}", e);
            }
        };

        descrambler(&mut bootrom[0x100..0x0015_ee40]);
    }

    pub fn step(&mut self) -> Option<Event> {
        cpu::step(self);

        #[cfg(feature = "gdb")]
        if let Some(access) = self.hit_watchpoint {
            self.hit_watchpoint = None;

            return Some(match access.kind {
                AccessKind::Read => Event::WatchRead(access.addr),
                AccessKind::Write => Event::WatchWrite(access.addr),
            });
        }

        #[cfg(feature = "gdb")]
        if self.breakpoints.contains(&self.cpu.get_pc()) {
            return Some(Event::Break);
        }

        None
    }

    pub fn read_instruction(&mut self, addr: u32) -> u32 {
        use Address::*;

        let val = match map(addr) {
            Memory => mem::read_u32(self, addr),
            Bootrom(offset) => BigEndian::read_u32(&self.bootrom.borrow()[offset as usize..]),
            _ => panic!(
                "read_instruction not implemented for {:#?} address {:#x}",
                map(addr),
                addr
            ),
        };

        #[cfg(feature = "gdb")]
        if self.watchpoints.contains(&addr) {
            self.hit_watchpoint = Some(Access {
                kind: AccessKind::Read,
                addr: addr,
            })
        }

        val
    }

    pub fn read_u8(&mut self, ea: u32) -> u8 {
        let addr = self.cpu.translate_data_address(ea);

        use Address::*;

        let ret = match map(addr) {
            Memory => self.mem.read_u8(addr),
            Bootrom(offset) => self.bootrom.borrow()[offset as usize],
            _ => panic!(
                "read_u8 not implemented for {:#?} address {:#x}",
                map(addr),
                addr
            ),
        };

        #[cfg(feature = "gdb")]
        if self.watchpoints.contains(&addr) {
            self.hit_watchpoint = Some(Access {
                kind: AccessKind::Read,
                addr: addr,
            })
        }

        ret
    }

    pub fn read_u16(&mut self, ea: u32) -> u16 {
        let addr = self.cpu.translate_data_address(ea);

        use Address::*;

        let ret = match map(addr) {
            Memory => mem::read_u16(self, addr),
            DspInterface(reg) => dsp::read_u16(self, reg),
            _ => panic!(
                "read_u16 not implemented for {:#?} address {:#x}",
                map(addr),
                addr
            ),
        };

        #[cfg(feature = "gdb")]
        if self.watchpoints.contains(&addr) {
            self.hit_watchpoint = Some(Access {
                kind: AccessKind::Read,
                addr: addr,
            })
        }

        ret
    }

    pub fn read_u32(&mut self, ea: u32) -> u32 {
        let addr = self.cpu.translate_data_address(ea);

        use Address::*;

        let ret = match map(addr) {
            Memory => mem::read_u32(self, addr),
            ExternalInterface(chan, reg) => exi::read_u32(self, chan, reg),
            ProcessorInterface(reg) => pi::read_u32(self, reg),
            SerialInterface(reg) => si::read_u32(self, reg),
            AudioInterface(reg) => ai::read_u32(self, reg),
            _ => panic!(
                "read_u32 not implemented for {:#?} address {:#x}",
                map(addr),
                addr
            ),
        };

        #[cfg(feature = "gdb")]
        if self.watchpoints.contains(&addr) {
            self.hit_watchpoint = Some(Access {
                kind: AccessKind::Read,
                addr: addr,
            })
        }

        ret
    }

    pub fn read_u64(&mut self, ea: u32) -> u64 {
        let addr = self.cpu.translate_data_address(ea);

        use Address::*;

        let ret = match map(addr) {
            Memory => mem::read_u64(self, addr),
            _ => panic!(
                "read_u64 not implemented for {:#?} address {:#x}",
                map(addr),
                addr
            ),
        };

        #[cfg(feature = "gdb")]
        if self.watchpoints.contains(&addr) {
            self.hit_watchpoint = Some(Access {
                kind: AccessKind::Read,
                addr: addr,
            })
        }

        ret
    }

    pub fn write(&mut self, ea: u32, data: &[u8]) {
        let addr = self.cpu.translate_data_address(ea);

        use Address::*;

        match map(addr) {
            Memory => mem::write(self, addr, data),
            _ => unimplemented!(
                "write not implemented for {:#?} address {:#x}",
                map(addr),
                addr
            ),
        };

        #[cfg(feature = "gdb")]
        if self.watchpoints.contains(&addr) {
            self.hit_watchpoint = Some(Access {
                kind: AccessKind::Write,
                addr: addr,
            })
        }
    }

    pub fn write_u8(&mut self, ea: u32, val: u8) {
        let addr = self.cpu.translate_data_address(ea);

        use Address::*;

        match map(addr) {
            Memory => self.mem.write_u8(addr, val),
            _ => panic!(
                "write_u16 not implemented for {:#?} address {:#x}",
                map(addr),
                addr
            ),
        }

        #[cfg(feature = "gdb")]
        if self.watchpoints.contains(&addr) {
            self.hit_watchpoint = Some(Access {
                kind: AccessKind::Write,
                addr: addr,
            })
        }
    }

    pub fn write_u16(&mut self, ea: u32, val: u16) {
        let addr = self.cpu.translate_data_address(ea);

        use Address::*;

        match map(addr) {
            Memory => mem::write_u16(self, addr, val),
            DspInterface(reg) => dsp::write_u16(self, reg, val),
            MemoryInterface(_) => {} //ignore
            _ => panic!(
                "write_u16 not implemented for {:#?} address {:#x}",
                map(addr),
                addr
            ),
        }

        #[cfg(feature = "gdb")]
        if self.watchpoints.contains(&addr) {
            self.hit_watchpoint = Some(Access {
                kind: AccessKind::Write,
                addr: addr,
            })
        }
    }

    pub fn write_u32(&mut self, ea: u32, val: u32) {
        let addr = self.cpu.translate_data_address(ea);

        use Address::*;

        match map(addr) {
            Memory => mem::write_u32(self, addr, val),
            DspInterface(reg) => {
                dsp::write_u16(self, reg, (val >> 16) as u16);
                dsp::write_u16(self, reg + 2, val as u16);
            }
            DvdInterface(reg) => di::write_u32(self, reg, val),
            ExternalInterface(chan, reg) => exi::write_u32(self, chan, reg, val),
            ProcessorInterface(reg) => pi::write_u32(self, reg, val),
            SerialInterface(reg) => si::write_u32(self, reg, val),
            AudioInterface(reg) => ai::write_u32(self, reg, val),
            _ => panic!(
                "write_u32 not implemented for {:#?} address {:#x}",
                map(addr),
                addr
            ),
        }

        #[cfg(feature = "gdb")]
        if self.watchpoints.contains(&addr) {
            self.hit_watchpoint = Some(Access {
                kind: AccessKind::Write,
                addr: addr,
            })
        }
    }
}

#[derive(Debug)]
pub enum Address {
    Memory,
    EmbeddedFramebuffer,
    CommandProcessor(u32),
    PixelEngine(u32),
    VideoInterface(u32),
    ProcessorInterface(u32),
    MemoryInterface(u32),
    DspInterface(u32),
    DvdInterface(u32),
    SerialInterface(u32),
    ExternalInterface(u32, u32),
    AudioInterface(u32),
    GPFifo,
    Bootrom(u32),
}

fn map(address: u32) -> Address {
    use Address::*;

    match address {
        0x0000_0000..=0x017F_FFFF => Memory,
        0x0800_0000..=0x0BFF_FFFF => EmbeddedFramebuffer,
        0x0C00_0000..=0x0C00_0FFF => CommandProcessor(address - 0x0C00_0000),
        0x0C00_1000..=0x0C00_1FFF => PixelEngine(address - 0x0C00_1000),
        0x0C00_2000..=0x0C00_2FFF => VideoInterface(address - 0x0C00_2000),
        0x0C00_3000..=0x0C00_3FFF => ProcessorInterface(address - 0x0C00_3000),
        0x0C00_4000..=0x0C00_4FFF => MemoryInterface(address - 0x0C00_4000),
        0x0C00_5000..=0x0C00_5200 => DspInterface(address - 0x0C00_5000),
        0x0C00_6000..=0x0C00_63FF => DvdInterface(address - 0x0C00_6000),
        0x0C00_6400..=0x0C00_67FF => SerialInterface(address - 0x0C00_6400),
        0x0C00_6800..=0x0C00_6938 => {
            let channel = (address - 0x0C00_6800) / 0x14;
            let register = (address - 0x0C00_6800) % 0x14;
            ExternalInterface(channel, register)
        }
        0x0C00_6C00..=0x0C00_6C20 => AudioInterface(address - 0x0C00_6C00),
        0x0C00_8000 => GPFifo,
        0xFFF0_0000..=0xFFFF_FFFF => Bootrom(address - 0xFFF0_0000),
        _ => panic!("Unrecognized physical address: {:#x}", address),
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
            data[it as usize] ^= acc;
            it += 1;
            nacc = 0;
        }
    }
}
