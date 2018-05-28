
use byteorder::{ByteOrder, BigEndian};
use std::cell::RefCell;
use std::rc::Rc;

use audio_interface::AudioInterface;
use command_processor::CommandProcessor;
use dsp_interface::DspInterface;
use dvd_interface::DvdInterface;
use exi::Exi;
use cpu::instruction::Instruction;
use cpu::mmu::Mmu;
use cpu::msr::Msr;
use gp_fifo::GPFifo;
use memory::Ram;
//use memory_interface::MemoryInterface;
use pixel_engine::PixelEngine;
use processor_interface::ProcessorInterface;
use serial_interface::SerialInterface;
use video_interface::VideoInterface;

const BOOTROM_SIZE: usize = 0x0200000; // 2 MB

#[derive(Debug)]
pub enum Address {
    Ram,
    EmbeddedFramebuffer,
    CommandProcessor(u32),
    PixelEngine(u32),
    VideoInterface(u32),
    ProcessorInterface(u32),
    MemoryInterface(u32),
    DspInterface(u32),
    DvdInterface(u32),
    SerialInterface(u32),
    ExpansionInterface(u32, u32),
    AudioInterface(u32),
    GPFifo,
    Bootrom(u32)
}

fn map(address: u32) -> Address {
    match address {
        0x00000000 ... 0x017FFFFF => Address::Ram,
        0x08000000 ... 0x0BFFFFFF => Address::EmbeddedFramebuffer,
        0x0C000000 ... 0x0C000FFF => Address::CommandProcessor(address - 0x0C000000),
        0x0C001000 ... 0x0C001FFF => Address::PixelEngine(address - 0x0C001000),
        0x0C002000 ... 0x0C002FFF => Address::VideoInterface(address - 0x0C002000),
        0x0C003000 ... 0x0C003FFF => Address::ProcessorInterface(address - 0x0C003000),
        0x0C004000 ... 0x0C004FFF => Address::MemoryInterface(address - 0x0C004000),
        0x0C005000 ... 0x0C005200 => Address::DspInterface(address - 0x0C005000),
        0x0C006000 ... 0x0C0063FF => Address::DvdInterface(address - 0x0C006000),
        0x0C006400 ... 0x0C0067FF => Address::SerialInterface(address - 0x0C006400),
        0x0C006800 ... 0x0C006938 => {
            let channel  = (address - 0x0C006800) / 0x14;
            let register = (address - 0x0C006800) % 0x14;
            Address::ExpansionInterface(channel, register)
        },
        0x0C006C00 ... 0x0C006C20 => Address::AudioInterface(address - 0x0C006C00),
        0x0C008000 => Address::GPFifo,
        0xFFF00000 ... 0xFFFFFFFF => Address::Bootrom(address - 0xFFF00000),
        _ => panic!("Unrecognized physical address: {:#x}", address)
    }
}

pub struct Interconnect {
    ai: AudioInterface,
    pub bootrom: Rc<RefCell<Box<[u8; BOOTROM_SIZE]>>>,
    cp: CommandProcessor,
    dsp: DspInterface,
    dvd: DvdInterface,
    exi: Exi,
    gp: GPFifo,
    pub mmu: Mmu,
    //mi: MemoryInterface,
    pe: PixelEngine,
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
            cp: CommandProcessor::new(),
            dsp: DspInterface::default(),
            dvd: DvdInterface::default(),
            exi: Exi::new(bootrom.clone()),
            bootrom: bootrom,
            gp: GPFifo::new(),
            mmu: Mmu::new(),
            //mi: MemoryInterface::new(),
            pe: PixelEngine::new(),
            pi: ProcessorInterface::new(),
            ram: Ram::default(),
            si: SerialInterface::new(),
            vi: VideoInterface::new()
        }
    }

    pub fn read_instruction(&self, msr: &Msr, cia: u32) -> Instruction {
        let addr = self.mmu.translate_instr_address(msr, cia);

        let val = match map(addr) {
            Address::Ram => self.ram.read_u32(addr),
            Address::Bootrom(offset) => BigEndian::read_u32(&self.bootrom.borrow()[offset as usize ..]),
            _ => panic!("read_instruction not implemented for {:#?} address {:#x}", map(addr), addr)
        };

        Instruction(val)
    }

    pub fn read_u8(&self, msr: &Msr, addr: u32) -> u8 {
        let addr = self.mmu.translate_data_address(msr, addr);

        match map(addr) {
            Address::Ram => self.ram.read_u8(addr),
            Address::Bootrom(offset) => self.bootrom.borrow()[offset as usize],
            _ => panic!("read_u8 not implemented for {:#?} address {:#x}", map(addr), addr)
        }
    }

    pub fn read_u16(&mut self, msr: &Msr, addr: u32) -> u16 {
        let addr = self.mmu.translate_data_address(msr, addr);

        match map(addr) {
            Address::Ram => self.ram.read_u16(addr),
            Address::VideoInterface(offset) => self.vi.read_u16(offset, &self.ram),
            Address::DspInterface(offset) => self.dsp.read_u16(offset),
            Address::Bootrom(offset) => BigEndian::read_u16(&self.bootrom.borrow()[offset as usize ..]),
            Address::PixelEngine(offset) => self.pe.read_u16(offset),
            _ => panic!("read_u16 not implemented for {:#?} address {:#x}", map(addr), addr)
        }
    }

    pub fn read_u32(&self, msr: &Msr, addr: u32) -> u32 {
        let addr = self.mmu.translate_data_address(msr, addr);

        match map(addr) {
            Address::Ram => self.ram.read_u32(addr),
            Address::ProcessorInterface(offset) => self.pi.read_u32(offset),
            Address::SerialInterface(offset) => self.si.read_u32(offset),
            Address::ExpansionInterface(channel, register) => self.exi.read(channel, register),
            Address::Bootrom(offset) => BigEndian::read_u32(&self.bootrom.borrow()[offset as usize ..]),
            Address::AudioInterface(offset) => self.ai.read_u32(offset),
            _ => panic!("read_u32 not implemented for {:#?} address {:#x}", map(addr), addr)
        }
    }

    pub fn read_u64(&self, msr: &Msr, addr: u32) -> u64 {
        let addr = self.mmu.translate_data_address(msr, addr);

        match map(addr) {
            Address::Ram => self.ram.read_u64(addr),
            Address::Bootrom(offset) => BigEndian::read_u64(&self.bootrom.borrow()[offset as usize ..]),
            _ => panic!("read_u64 not implemented for {:#?} address {:#x}", map(addr), addr)
        }
    }

    pub fn write_u8(&mut self, msr: &Msr, addr: u32, val: u8) {
        let addr = self.mmu.translate_data_address(msr, addr);

        match map(addr) {
            Address::Ram => self.ram.write_u8(addr, val),
            Address::GPFifo => self.gp.write_u8(val, &mut self.cp, &mut self.pi, &mut self.ram),
            _ => panic!("write_u8 not implemented for {:#?} address {:#x}", map(addr), addr)
        }
    }

    pub fn write_u16(&mut self, msr: &Msr, addr: u32, val: u16) {
        let addr = self.mmu.translate_data_address(msr, addr);

        match map(addr) {
            Address::Ram => self.ram.write_u16(addr, val),
            Address::VideoInterface(offset) => self.vi.write_u16(offset, val),
            Address::MemoryInterface(_) => println!("FixMe: memory interface"),
            Address::DspInterface(offset) => self.dsp.write_u16(offset, val),
            Address::CommandProcessor(offset) => self.cp.write_u16(offset, val),
            Address::PixelEngine(offset) => self.pe.write_u16(offset, val),
            _ => panic!("write_u16 not implemented for {:#?} address {:#x}", map(addr), addr)
        }
    }

    pub fn write_u32(&mut self, msr: &Msr, addr: u32, val: u32) {
        let addr = self.mmu.translate_data_address(msr, addr);

        match map(addr) {
            Address::Ram => self.ram.write_u32(addr, val),
            Address::VideoInterface(offset) => self.vi.write_u32(offset, val),
            Address::ProcessorInterface(offset) => self.pi.write_u32(offset, val),
            Address::DspInterface(offset) => {
                self.dsp.write_u16(offset, (val >> 16) as u16);
                self.dsp.write_u16(offset + 2, val as u16);
            },
            Address::CommandProcessor(offset) => self.cp.write_u32(offset, val),
            Address::DvdInterface(offset) => self.dvd.write_u32(offset, val),
            Address::SerialInterface(offset) => self.si.write_u32(offset, val),
            Address::ExpansionInterface(channel, register) => self.exi.write(channel, register, val, &mut self.ram),
            Address::AudioInterface(offset) => self.ai.write_u32(offset, val),
            Address::GPFifo => self.gp.write_u32(val, &mut self.cp, &mut self.pi, &mut self.ram),
            _ => panic!("write_u32 not implemented for {:#?} address {:#x}", map(addr), addr)
        }
    }

    pub fn write_u64(&mut self, msr: &Msr, addr: u32, val: u64) {
        let addr = self.mmu.translate_data_address(msr, addr);

        match map(addr) {
            Address::Ram => self.ram.write_u64(addr, val),
            Address::GPFifo => self.gp.write_u64(val, &mut self.cp, &mut self.pi, &mut self.ram),
            _ => panic!("write_u64 not implemented for {:#?} address {:#x}", map(addr), addr)
        }
    }

    pub fn write(&mut self, msr: &Msr, addr: u32, data: &[u8]) {
        let addr = self.mmu.translate_data_address(msr, addr);

        match map(addr) {
            Address::Ram => self.ram.write_dma(addr, data),
            _ => panic!("write_u64 not implemented for {:#?} address {:#x}", map(addr), addr)
        }
    }
}
