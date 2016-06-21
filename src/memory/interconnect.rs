use byteorder::{ByteOrder, BigEndian};
use std::cell::RefCell;
use std::fs;
use std::path::Path;
use std::io::Read;
use std::rc::Rc;

use super::ram::Ram;
use super::super::audio_interface::AudioInterface;
use super::super::dsp_interface::DspInterface;
use super::super::dvd_interface::DvdInterface;
use super::super::exi::Exi;
use super::super::cpu::instruction::Instruction;
use super::super::cpu::mmu::Mmu;
use super::super::cpu::machine_status::MachineStatus;
use super::super::memory_interface::MemoryInterface;
use super::super::processor_interface::ProcessorInterface;
use super::super::serial_interface::SerialInterface;
use super::super::video_interface::VideoInterface;

const BOOTROM_SIZE: usize = 0x0200000; // 2 MB

#[derive(Debug)]
pub enum Address {
    Ram,
    EmbeddedFramebuffer,
    CommandProcessor,
    PixelEngine,
    VideoInterface(u32),
    ProcessorInterface(u32),
    MemoryInterface,
    DspInterface(u32),
    DvdInterface(u32),
    SerialInterface,
    ExpansionInterface(u32, u32),
    AudioInterface(u32),
    PiFifo,
    Bootrom(u32)
}

fn map(address: u32) -> Address {
    match address {
        0x00000000 ... 0x017FFFFF => Address::Ram,
        0x08000000 ... 0x0BFFFFFF => Address::EmbeddedFramebuffer,
        0x0C000000 ... 0x0C000FFF => Address::CommandProcessor,
        0x0C001000 ... 0x0C001FFF => Address::PixelEngine,
        0x0C002000 ... 0x0C002FFF => Address::VideoInterface(address - 0x0C002000),
        0x0C003000 ... 0x0C003FFF => Address::ProcessorInterface(address - 0x0C003000),
        0x0C004000 ... 0x0C004FFF => Address::MemoryInterface,
        0x0C005000 ... 0x0C005200 => Address::DspInterface(address - 0x0C005000),
        0x0C006000 ... 0x0C0063FF => Address::DvdInterface(address - 0x0C006000),
        0x0C006400 ... 0x0C0067FF => Address::SerialInterface,
        0x0C006800 ... 0x0C0068FF => {
            let channel  = (address - 0x0C006800) / 0x14;
            let register = (address - 0x0C006800) % 0x14;
            Address::ExpansionInterface(channel, register)
        },
        0x0C006C00 ... 0x0C006C20 => Address::AudioInterface(address - 0x0C006C00),
        0x0C008000 ... 0x0C008FFF => Address::PiFifo,
        0xFFF00000 ... 0xFFFFFFFF => Address::Bootrom(address - 0xFFF00000),
        _ => panic!("Unrecognized physical address: {:#x}", address)
    }
}

pub struct Interconnect {
    ai: AudioInterface,
    bootrom: Rc<RefCell<Box<[u8; BOOTROM_SIZE]>>>,
    dsp: DspInterface,
    dvd: DvdInterface,
    exi: Exi,
    pub mmu: Mmu,
    mi: MemoryInterface,
    pi: ProcessorInterface,
    ram: Ram,
    si: SerialInterface,
    vi: VideoInterface
}

impl Interconnect {

    pub fn new() -> Interconnect {
        let bootrom = Rc::new(RefCell::new(Box::new([0; BOOTROM_SIZE])));

        Interconnect {
            ai: AudioInterface::default(),
            dsp: DspInterface::default(),
            dvd: DvdInterface::default(),
            exi: Exi::new(bootrom.clone()),
            bootrom: bootrom,
            mmu: Mmu::new(),
            mi: MemoryInterface::new(),
            pi: ProcessorInterface::new(),
            ram: Ram::new(),
            si: SerialInterface::new(),
            vi: VideoInterface::new()
        }
    }

    pub fn read_instruction(&self, msr: &MachineStatus, cia: u32) -> Instruction {
        let addr = self.mmu.translate_instr_address(msr, cia);

        let val = match map(addr) {
            Address::Ram => self.ram.read_u32(addr),
            Address::Bootrom(offset) => BigEndian::read_u32(&self.bootrom.borrow()[offset as usize ..]),
            _ => panic!("read_instruction not implemented for {:#?} address {:#x}", map(addr), addr)
        };

        Instruction(val)
    }

    pub fn read_u8(&self, msr: &MachineStatus, addr: u32) -> u8 {
        let addr = self.mmu.translate_data_address(msr, addr);

        match map(addr) {
            Address::Ram => self.ram.read_u8(addr),
            Address::Bootrom(offset) => self.bootrom.borrow()[offset as usize],
            _ => panic!("read_u8 not implemented for {:#?} address {:#x}", map(addr), addr)
        }
    }

    pub fn read_u16(&mut self, msr: &MachineStatus, addr: u32) -> u16 {
        let addr = self.mmu.translate_data_address(msr, addr);

        match map(addr) {
            Address::Ram => self.ram.read_u16(addr),
            Address::VideoInterface(offset) => self.vi.read_u16(offset),
            Address::DspInterface(offset) => self.dsp.read_u16(offset),
            Address::Bootrom(offset) => BigEndian::read_u16(&self.bootrom.borrow()[offset as usize ..]),
            _ => panic!("read_u16 not implemented for {:#?} address {:#x}", map(addr), addr)
        }
    }

    pub fn read_u32(&self, msr: &MachineStatus, addr: u32) -> u32 {
        let addr = self.mmu.translate_data_address(msr, addr);

        match map(addr) {
            Address::Ram => self.ram.read_u32(addr),
            Address::ProcessorInterface(offset) => self.pi.read_u32(offset),
            Address::SerialInterface => self.si.read_u32(addr),
            Address::ExpansionInterface(channel, register) => self.exi.read(channel, register),
            Address::Bootrom(offset) => BigEndian::read_u32(&self.bootrom.borrow()[offset as usize ..]),
            Address::AudioInterface(offset) => self.ai.read_u32(offset),
            _ => panic!("read_u32 not implemented for {:#?} address {:#x}", map(addr), addr)
        }
    }

    pub fn read_u64(&self, msr: &MachineStatus, addr: u32) -> u64 {
        let addr = self.mmu.translate_data_address(msr, addr);

        match map(addr) {
            Address::Ram => self.ram.read_u64(addr),
            Address::Bootrom(offset) => BigEndian::read_u64(&self.bootrom.borrow()[offset as usize ..]),
            _ => panic!("read_u64 not implemented for {:#?} address {:#x}", map(addr), addr)
        }
    }

    pub fn write_u8(&mut self, msr: &MachineStatus, addr: u32, val: u8) {
        let addr = self.mmu.translate_data_address(msr, addr);

        match map(addr) {
            Address::Ram => self.ram.write_u8(addr, val),
            _ => panic!("write_u8 not implemented for {:#?} address {:#x}", map(addr), addr)
        }
    }

    pub fn write_u16(&mut self, msr: &MachineStatus, addr: u32, val: u16) {
        let addr = self.mmu.translate_data_address(msr, addr);

        match map(addr) {
            Address::Ram => self.ram.write_u16(addr, val),
            Address::VideoInterface(offset) => self.vi.write_u16(offset, val),
            Address::MemoryInterface => println!("FixMe: memory interface"),
            Address::DspInterface(offset) => self.dsp.write_u16(offset, val),
            _ => panic!("write_u16 not implemented for {:#?} address {:#x}", map(addr), addr)
        }
    }

    pub fn write_u32(&mut self, msr: &MachineStatus, addr: u32, val: u32) {
        let addr = self.mmu.translate_data_address(msr, addr);

        match map(addr) {
            Address::Ram => self.ram.write_u32(addr, val),
            Address::ProcessorInterface(offset) => self.pi.write_u32(offset, val),
            Address::DspInterface(offset) => self.dsp.write_u32(offset, val),
            Address::DvdInterface(offset) => self.dvd.write_u32(offset, val),
            Address::SerialInterface => self.si.write_u32(addr, val),
            Address::ExpansionInterface(channel, register) => self.exi.write(channel, register, val, &mut self.ram),
            Address::AudioInterface(offset) => self.ai.write_u32(offset, val),
            _ => panic!("write_u32 not implemented for {:#?} address {:#x}", map(addr), addr)
        }
    }

    pub fn write_u64(&mut self, msr: &MachineStatus, addr: u32, val: u64) {
        let addr = self.mmu.translate_data_address(msr, addr);

        match map(addr) {
            Address::Ram => self.ram.write_u64(addr, val),
            _ => panic!("write_u64 not implemented for {:#?} address {:#x}", map(addr), addr)
        }
    }

    // FixMe: would be great to move this into exi device ipl
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
            Ok(_) => {},
            Err(e) => {
                panic!("{}", e);
            }
        };

        descrambler(&mut bootrom[0x100..0x15ee40]);
    }
}

// bootrom descrambler reversed by segher
// Copyright 2008 Segher Boessenkool <segher@kernel.crashing.org>
fn descrambler(data: &mut[u8]) {
    let size = data.len();
    let mut acc :u8 = 0;
    let mut nacc:u8 = 0;

    let mut t:u16 = 0x2953;
    let mut u:u16 = 0xd9c2;
    let mut v:u16 = 0x3ff1;

    let mut x:u8 = 1;

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

        nacc+=1;
        acc = (2*acc as u16 + x as u16) as u8;
        if nacc == 8 {
            data[it as usize] ^= acc;
            it+=1;
            nacc = 0;
        }
    }
}
