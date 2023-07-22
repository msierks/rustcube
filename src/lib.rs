#[macro_use]
extern crate bitfield;

#[macro_use]
extern crate log;

//mod ai;
pub mod cpu;
mod di;
mod disc;
mod dol;
mod dsp;
mod exi;
mod gp_fifo;
mod mem;
//mod pe;
mod pi;
mod si;
mod timers;
mod utils;
mod vi;
//mod video;

//use self::ai::AudioInterface;
use self::cpu::Cpu;
use self::di::DvdInterface;
use self::disc::Disc;
use self::dol::Dol;
use self::dsp::DspInterface;
use self::exi::ExternalInterface;
use self::gp_fifo::GpFifo;
use self::mem::{Memory, MEMORY_SIZE};
//use self::pe::PixelEngine;
use self::pi::ProcessorInterface;
use self::si::SerialInterface;
use self::timers::{Timers, BUS_CLOCK, CPU_CLOCK};
use self::vi::VideoInterface;
//use self::video::cp;
//use self::video::cp::CommandProcessor;
use crate::utils::Halveable;

use byteorder::{BigEndian, ByteOrder};
use std::cell::RefCell;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::rc::Rc;

const BOOTROM_SIZE: usize = 0x0020_0000; // 2 MB
const OP_RFI: u32 = 0x4C00_0064;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Event {
    Halted,
    Break,
    WatchWrite(u32),
    WatchRead(u32),
}

#[derive(Debug, Copy, Clone)]
struct Access {
    pub addr: u32,
    pub write: bool,
}

pub struct Watchpoint {
    pub start_addr: u32,
    pub end_addr: u32,
    pub break_on_write: bool,
    pub break_on_read: bool,
}

pub struct Context {
    //ai: AudioInterface,
    cpu: Cpu,
    bootrom: Rc<RefCell<Vec<u8>>>,
    mem: Memory,
    //cp: CommandProcessor,
    di: DvdInterface,
    dsp: DspInterface,
    exi: ExternalInterface,
    gp_fifo: GpFifo,
    //pe: PixelEngine,
    pi: ProcessorInterface,
    si: SerialInterface,
    vi: VideoInterface,
    timers: Timers,
    watchpoints: Vec<Watchpoint>,
    breakpoints: Vec<u32>,
    hit_watchpoint: Option<Access>,
}

impl Default for Context {
    fn default() -> Self {
        let bootrom = Rc::new(RefCell::new(vec![0; BOOTROM_SIZE]));
        let exi = ExternalInterface::new(bootrom.clone());

        Context {
            //ai: Default::default(),
            cpu: Default::default(),
            bootrom,
            mem: Default::default(),
            //cp: Default::default(),
            di: Default::default(),
            dsp: Default::default(),
            exi,
            gp_fifo: Default::default(),
            //pe: Default::default(),
            pi: Default::default(),
            si: Default::default(),
            vi: Default::default(),
            timers: Default::default(),
            watchpoints: Default::default(),
            breakpoints: Default::default(),
            hit_watchpoint: None,
        }
    }
}

impl Context {
    pub fn load_dol<P: AsRef<Path>>(&mut self, path: P) {
        let dol = Dol::open(path).unwrap();

        self.emulate_bs2();

        dol.load(self);

        self.cpu.set_pc(dol.get_entry_point());
    }

    pub fn load_iso<P: AsRef<Path>>(&mut self, path: P) {
        let mut disc = Disc::open(path).unwrap();

        self.emulate_bs2();

        disc.load(self).unwrap(); // fix this and don't be lazy

        self.di.set_disc(Some(disc));
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

        match file.read_exact(&mut bootrom) {
            Ok(_) => {}
            Err(e) => {
                panic!("{}", e);
            }
        };

        descrambler(&mut bootrom[0x100..0x1AFF00]);
    }

    pub fn emulate_bs2(&mut self) {
        self.cpu.emulate_bs2();

        // VI - Display Config
        self.write_u16(0xCC00_2002, 0x0001);

        // Magic Word - Normal Boot
        self.write_u32(0x8000_0020, 0x0D15_EA5E);
        // Version
        self.write_u32(0x8000_0024, 0x0000_0001);
        // Physical Memory Size
        self.write_u32(0x8000_0028, MEMORY_SIZE as u32);
        // Console Type - Latest production board
        self.write_u32(0x8000_002C, 0x0000_0003);
        // ArenaLo
        self.write_u32(0x8000_0030, 0x0000_0000);
        // ArenaHi
        self.write_u32(0x8000_0034, 0x817F_E8C0);
        // Bus Clock Speed
        self.write_u32(0x8000_00F8, BUS_CLOCK as u32);
        // CPU Clock Speed
        self.write_u32(0x8000_00FC, CPU_CLOCK as u32);

        // Exception Handlers
        for x in [
            0x8000_0100,
            0x8000_0200,
            0x8000_0300,
            0x8000_0400,
            0x8000_0500,
            0x8000_0600,
            0x8000_0700,
            0x8000_0800,
            0x8000_0900,
            0x8000_0C00,
            0x8000_0d00,
            0x8000_0f00,
            0x8000_1300,
            0x8000_1400,
            0x8000_1700,
        ]
        .iter()
        {
            self.write_u32(*x, OP_RFI);
        }
    }

    pub fn step(&mut self) -> Option<Event> {
        cpu::step(self);

        vi::update(self);

        if let Some(access) = self.hit_watchpoint {
            self.hit_watchpoint = None;

            return Some(match access.write {
                false => Event::WatchRead(access.addr),
                true => Event::WatchWrite(access.addr),
            });
        }

        if self.breakpoints.contains(&self.cpu.pc()) {
            return Some(Event::Break);
        }

        None
    }

    pub fn cpu(&self) -> &Cpu {
        &self.cpu
    }

    pub fn tick(&mut self, cycles: u32) {
        self.timers.tick(cycles);
    }

    pub fn get_ticks(&self) -> u64 {
        self.timers.get_ticks()
    }

    fn check_watchpoints(&mut self, addr: u32, size: usize, write: bool) {
        if self.watchpoints.is_empty() {
            return;
        }

        for wp in self.watchpoints.iter() {
            if addr + (size as u32) > wp.start_addr
                && addr <= wp.end_addr
                && (wp.break_on_read != write || wp.break_on_write == write)
            {
                self.hit_watchpoint = Some(Access { addr, write });
                break;
            }
        }
    }

    pub fn read_instruction(&mut self, addr: u32) -> u32 {
        use Address::*;

        let val = match map(addr) {
            Memory => mem::read_u32(self, addr),
            Bootrom(offset) => BigEndian::read_u32(&self.bootrom.borrow()[offset as usize..]),
            _ => panic!(
                "read_instruction not implemented for {:?} address {:#x}",
                map(addr),
                addr
            ),
        };

        val
    }

    pub fn read_u8(&mut self, ea: u32) -> u8 {
        let addr = self.cpu.translate_data_address(ea);

        use Address::*;

        let ret = match map(addr) {
            Memory => self.mem.read_u8(addr),
            Bootrom(offset) => self.bootrom.borrow()[offset as usize],
            _ => {
                warn!(
                    "read_u8 not implemented for {:?} address {:#x}",
                    map(addr),
                    addr
                );
                0
            }
        };

        self.check_watchpoints(addr, 1, false);

        ret
    }

    pub fn read_u16(&mut self, ea: u32) -> u16 {
        let addr = self.cpu.translate_data_address(ea);

        use Address::*;

        let ret = match map(addr) {
            Memory => mem::read_u16(self, addr),
            //PixelEngine(reg) => pe::read_u16(self, reg),
            VideoInterface(reg) => vi::read_u16(self, reg),
            DspInterface(reg) => dsp::read_u16(self, reg),
            _ => {
                warn!(
                    "read_u16 not implemented for {:?} address {:#x}",
                    map(addr),
                    addr
                );
                0
            }
        };

        self.check_watchpoints(addr, 2, false);

        ret
    }

    pub fn read_u32(&mut self, ea: u32) -> u32 {
        let addr = self.cpu.translate_data_address(ea);

        use Address::*;

        let ret = match map(addr) {
            Memory => mem::read_u32(self, addr),
            ProcessorInterface(reg) => pi::read_u32(self, reg),
            ExternalInterface(chan, reg) => exi::read_u32(self, chan, reg),
            DvdInterface(reg) => di::read_u32(self, reg),
            SerialInterface(reg) => si::read_u32(self, reg),
            //AudioInterface(reg) => ai::read_u32(self, reg),
            Bootrom(offset) => BigEndian::read_u32(&self.bootrom.borrow()[offset as usize..]),
            _ => {
                warn!(
                    "read_u32 not implemented for {:?} address {:#x}",
                    map(addr),
                    addr
                );
                0
            }
        };

        self.check_watchpoints(addr, 4, false);

        ret
    }

    pub fn debug_read_u32(&mut self, ea: u32) -> u32 {
        let addr = self.cpu.translate_data_address(ea);

        use Address::*;

        let ret = match map(addr) {
            Memory => mem::read_u32(self, addr),
            ProcessorInterface(reg) => pi::read_u32(self, reg),
            DvdInterface(reg) => di::read_u32(self, reg),
            SerialInterface(reg) => si::read_u32(self, reg),
            ExternalInterface(chan, reg) => exi::read_u32(self, chan, reg),
            //AudioInterface(reg) => ai::read_u32(self, reg),
            Bootrom(offset) => BigEndian::read_u32(&self.bootrom.borrow()[offset as usize..]),
            _ => 0,
        };

        ret
    }

    pub fn read_u64(&mut self, ea: u32) -> u64 {
        let addr = self.cpu.translate_data_address(ea);

        use Address::*;

        let ret = match map(addr) {
            Memory => mem::read_u64(self, addr),
            _ => {
                warn!(
                    "read_u64 not implemented for {:?} address {:#x}",
                    map(addr),
                    addr
                );
                0
            }
        };

        self.check_watchpoints(addr, 8, false);

        ret
    }

    pub fn write(&mut self, ea: u32, data: &[u8]) {
        let addr = self.cpu.translate_data_address(ea);

        use Address::*;

        match map(addr) {
            Memory => mem::write(self, addr, data),
            _ => warn!(
                "write not implemented for {:?} address {:#x}",
                map(addr),
                addr
            ),
        };

        self.check_watchpoints(addr, data.len(), true);
    }

    pub fn write_u8(&mut self, ea: u32, val: u8) {
        let addr = self.cpu.translate_data_address(ea);

        use Address::*;

        match map(addr) {
            Memory => self.mem.write_u8(addr, val),
            GpFifo => gp_fifo::write_u8(self, val),
            _ => warn!(
                "write_u8 not implemented for {:?} address {:#x}",
                map(addr),
                addr
            ),
        }

        self.check_watchpoints(addr, 1, true);
    }

    pub fn write_u16(&mut self, ea: u32, val: u16) {
        let addr = self.cpu.translate_data_address(ea);

        use Address::*;

        match map(addr) {
            Memory => mem::write_u16(self, addr, val),
            //CommandProcessor(reg) => cp::write_u16(self, reg, val),
            //PixelEngine(reg) => pe::write_u16(self, reg, val),
            VideoInterface(reg) => vi::write_u16(self, reg, val),
            DspInterface(reg) => dsp::write_u16(self, reg, val),
            //MemoryInterface(_) => {} //ignore
            GpFifo => gp_fifo::write_u16(self, val),
            _ => warn!(
                "write_u16 not implemented for {:?} address {:#x}",
                map(addr),
                addr
            ),
        }

        self.check_watchpoints(addr, 2, true);
    }

    pub fn write_u32(&mut self, ea: u32, val: u32) {
        let addr = self.cpu.translate_data_address(ea);

        use Address::*;

        match map(addr) {
            Memory => mem::write_u32(self, addr, val),
            VideoInterface(reg) => {
                vi::write_u16(self, reg, val.hi());
                vi::write_u16(self, reg + 2, val.lo());
            }
            ProcessorInterface(reg) => pi::write_u32(self, reg, val),
            DspInterface(reg) => {
                dsp::write_u16(self, reg, val.hi());
                dsp::write_u16(self, reg + 2, val.lo());
            }
            DvdInterface(reg) => di::write_u32(self, reg, val),
            SerialInterface(reg) => si::write_u32(self, reg, val),
            ExternalInterface(chan, reg) => exi::write_u32(self, chan, reg, val),
            //AudioInterface(reg) => ai::write_u32(self, reg, val),
            GpFifo => gp_fifo::write_u32(self, val),
            _ => warn!(
                "write_u32 not implemented for {:?} address {:#x}",
                map(addr),
                addr
            ),
        }

        self.check_watchpoints(addr, 4, true);
    }

    pub fn write_u64(&mut self, ea: u32, val: u64) {
        let addr = self.cpu.translate_data_address(ea);

        use Address::*;

        match map(addr) {
            Memory => mem::write_u64(self, addr, val),
            GpFifo => gp_fifo::write_u64(self, val),
            _ => warn!(
                "write_u64 not implemented for {:?} address {:#x}",
                map(addr),
                addr
            ),
        }

        self.check_watchpoints(ea, 8, true);
    }

    pub fn breakpoints(&self) -> &Vec<u32> {
        &self.breakpoints
    }

    pub fn breakpoints_clear(&mut self) {
        self.breakpoints.clear();
    }

    pub fn add_breakpoint(&mut self, addr: u32) {
        self.breakpoints.push(addr);
    }

    pub fn remove_breakpoint(&mut self, index: usize) {
        self.breakpoints.remove(index);
    }

    pub fn watchpoints(&self) -> &Vec<Watchpoint> {
        &self.watchpoints
    }

    pub fn watchpoints_clear(&mut self) {
        self.watchpoints.clear();
    }

    pub fn add_watchpoint(
        &mut self,
        start_addr: u32,
        end_addr: u32,
        break_on_write: bool,
        break_on_read: bool,
    ) {
        self.watchpoints.push(Watchpoint {
            start_addr,
            end_addr,
            break_on_write,
            break_on_read,
        });
    }

    pub fn remove_watchpoint(&mut self, index: usize) {
        self.watchpoints.remove(index);
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
    GpFifo,
    Bootrom(u32),
    Unknown(u32),
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
        0x0C00_6800..=0x0C00_6838 => {
            let channel = (address - 0x0C00_6800) / 0x14;
            let register = (address - 0x0C00_6800) % 0x14;
            ExternalInterface(channel, register)
        }
        0x0C00_6C00..=0x0C00_6C20 => AudioInterface(address - 0x0C00_6C00),
        0x0C00_8000 => GpFifo,
        0xFFF0_0000..=0xFFFF_FFFF => Bootrom(address - 0xFFF0_0000),
        _ => Unknown(address),
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
