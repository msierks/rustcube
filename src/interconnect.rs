use byteorder::{BigEndian, ByteOrder};
use std::cell::RefCell;
use std::rc::Rc;

use crate::audio_interface::AudioInterface;
use crate::command_processor::CommandProcessor;
use crate::cpu::instruction::Instruction;
use crate::cpu::mmu::Mmu;
use crate::cpu::msr::Msr;
use crate::dsp_interface::DspInterface;
use crate::dvd_interface::DvdInterface;
use crate::exi::Exi;
use crate::gp_fifo::GPFifo;
use crate::memory::Ram;
//use crate::memory_interface::MemoryInterface;
use crate::pixel_engine::PixelEngine;
use crate::processor_interface::ProcessorInterface;
use crate::serial_interface::SerialInterface;
use crate::video_interface::VideoInterface;

const BOOTROM_SIZE: usize = 0x0020_0000; // 2 MB

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
    Bootrom(u32),
}

fn map(address: u32) -> Address {
    match address {
        0x0000_0000...0x017F_FFFF => Address::Ram,
        0x0800_0000...0x0BFF_FFFF => Address::EmbeddedFramebuffer,
        0x0C00_0000...0x0C00_0FFF => Address::CommandProcessor(address - 0x0C00_0000),
        0x0C00_1000...0x0C00_1FFF => Address::PixelEngine(address - 0x0C00_1000),
        0x0C00_2000...0x0C00_2FFF => Address::VideoInterface(address - 0x0C00_2000),
        0x0C00_3000...0x0C00_3FFF => Address::ProcessorInterface(address - 0x0C00_3000),
        0x0C00_4000...0x0C00_4FFF => Address::MemoryInterface(address - 0x0C00_4000),
        0x0C00_5000...0x0C00_5200 => Address::DspInterface(address - 0x0C00_5000),
        0x0C00_6000...0x0C00_63FF => Address::DvdInterface(address - 0x0C00_6000),
        0x0C00_6400...0x0C00_67FF => Address::SerialInterface(address - 0x0C00_6400),
        0x0C00_6800...0x0C00_6938 => {
            let channel = (address - 0x0C00_6800) / 0x14;
            let register = (address - 0x0C00_6800) % 0x14;
            Address::ExpansionInterface(channel, register)
        }
        0x0C00_6C00...0x0C00_6C20 => Address::AudioInterface(address - 0x0C00_6C00),
        0x0C00_8000 => Address::GPFifo,
        0xFFF0_0000...0xFFFF_FFFF => Address::Bootrom(address - 0xFFF0_0000),
        _ => panic!("Unrecognized physical address: {:#x}", address),
    }
}

pub struct Interconnect {
    ai: AudioInterface,
    pub bootrom: Rc<RefCell<Vec<u8>>>,
    cp: CommandProcessor,
    dsp: DspInterface,
    dvd: DvdInterface,
    exi: Exi,
    pub gp: GPFifo,
    pub mmu: Mmu,
    //mi: MemoryInterface,
    pe: PixelEngine,
    pi: ProcessorInterface,
    ram: Ram,
    si: SerialInterface,
    vi: VideoInterface,
}

impl Default for Interconnect {
    fn default() -> Self {
        let bootrom = Rc::new(RefCell::new(Vec::with_capacity(BOOTROM_SIZE)));

        Interconnect {
            ai: AudioInterface::default(),
            cp: CommandProcessor::new(),
            dsp: DspInterface::default(),
            dvd: DvdInterface::default(),
            exi: Exi::new(bootrom.clone()),
            bootrom,
            gp: GPFifo::default(),
            mmu: Mmu::default(),
            //mi: MemoryInterface::new(),
            pe: PixelEngine::default(),
            pi: ProcessorInterface::default(),
            ram: Ram::default(),
            si: SerialInterface::default(),
            vi: VideoInterface::default(),
        }
    }
}

impl Interconnect {
    pub fn read_instruction(&self, msr: &Msr, cia: u32) -> Instruction {
        let addr = self.mmu.translate_instr_address(msr, cia);

        let val = match map(addr) {
            Address::Ram => self.ram.read_u32(addr),
            Address::Bootrom(offset) => {
                BigEndian::read_u32(&self.bootrom.borrow()[offset as usize..])
            }
            _ => panic!(
                "read_instruction not implemented for {:#?} address {:#x}",
                map(addr),
                addr
            ),
        };

        Instruction(val)
    }

    pub fn read_u8(&self, msr: &Msr, addr: u32) -> u8 {
        let addr = self.mmu.translate_data_address(msr, addr);

        match map(addr) {
            Address::Ram => self.ram.read_u8(addr),
            Address::Bootrom(offset) => self.bootrom.borrow()[offset as usize],
            _ => panic!(
                "read_u8 not implemented for {:#?} address {:#x}",
                map(addr),
                addr
            ),
        }
    }

    pub fn read_u16(&mut self, msr: &Msr, addr: u32) -> u16 {
        let addr = self.mmu.translate_data_address(msr, addr);

        match map(addr) {
            Address::Ram => self.ram.read_u16(addr),
            Address::VideoInterface(offset) => self.vi.read_u16(offset, &self.ram),
            Address::DspInterface(offset) => self.dsp.read_u16(offset),
            Address::Bootrom(offset) => {
                BigEndian::read_u16(&self.bootrom.borrow()[offset as usize..])
            }
            Address::PixelEngine(offset) => self.pe.read_u16(offset),
            _ => panic!(
                "read_u16 not implemented for {:#?} address {:#x}",
                map(addr),
                addr
            ),
        }
    }

    pub fn read_u32(&self, msr: &Msr, addr: u32) -> u32 {
        let addr = self.mmu.translate_data_address(msr, addr);

        match map(addr) {
            Address::Ram => self.ram.read_u32(addr),
            Address::ProcessorInterface(offset) => self.pi.read_u32(offset),
            Address::SerialInterface(offset) => self.si.read_u32(offset),
            Address::ExpansionInterface(channel, register) => self.exi.read(channel, register),
            Address::Bootrom(offset) => {
                BigEndian::read_u32(&self.bootrom.borrow()[offset as usize..])
            }
            Address::AudioInterface(offset) => self.ai.read_u32(offset),
            _ => panic!(
                "read_u32 not implemented for {:#?} address {:#x}",
                map(addr),
                addr
            ),
        }
    }

    pub fn read_u64(&self, msr: &Msr, addr: u32) -> u64 {
        let addr = self.mmu.translate_data_address(msr, addr);

        match map(addr) {
            Address::Ram => self.ram.read_u64(addr),
            Address::Bootrom(offset) => {
                BigEndian::read_u64(&self.bootrom.borrow()[offset as usize..])
            }
            _ => panic!(
                "read_u64 not implemented for {:#?} address {:#x}",
                map(addr),
                addr
            ),
        }
    }

    pub fn write_u8(&mut self, msr: &Msr, addr: u32, val: u8) {
        let addr = self.mmu.translate_data_address(msr, addr);

        match map(addr) {
            Address::Ram => self.ram.write_u8(addr, val),
            Address::GPFifo => self
                .gp
                .write_u8(val, &mut self.cp, &mut self.pi, &mut self.ram),
            _ => panic!(
                "write_u8 not implemented for {:#?} address {:#x}",
                map(addr),
                addr
            ),
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
            _ => panic!(
                "write_u16 not implemented for {:#?} address {:#x}",
                map(addr),
                addr
            ),
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
            }
            Address::CommandProcessor(offset) => self.cp.write_u32(offset, val),
            Address::DvdInterface(offset) => self.dvd.write_u32(offset, val),
            Address::SerialInterface(offset) => self.si.write_u32(offset, val),
            Address::ExpansionInterface(channel, register) => {
                self.exi.write(channel, register, val, &mut self.ram)
            }
            Address::AudioInterface(offset) => self.ai.write_u32(offset, val),
            Address::GPFifo => self
                .gp
                .write_u32(val, &mut self.cp, &mut self.pi, &mut self.ram),
            _ => panic!(
                "write_u32 not implemented for {:#?} address {:#x}",
                map(addr),
                addr
            ),
        }
    }

    pub fn write_u64(&mut self, msr: &Msr, addr: u32, val: u64) {
        let addr = self.mmu.translate_data_address(msr, addr);

        match map(addr) {
            Address::Ram => self.ram.write_u64(addr, val),
            Address::GPFifo => self
                .gp
                .write_u64(val, &mut self.cp, &mut self.pi, &mut self.ram),
            _ => panic!(
                "write_u64 not implemented for {:#?} address {:#x}",
                map(addr),
                addr
            ),
        }
    }

    pub fn write(&mut self, msr: &Msr, addr: u32, data: &[u8]) {
        let addr = self.mmu.translate_data_address(msr, addr);

        match map(addr) {
            Address::Ram => self.ram.write_dma(addr, data),
            _ => panic!(
                "write_u64 not implemented for {:#?} address {:#x}",
                map(addr),
                addr
            ),
        }
    }
}
