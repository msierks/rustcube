use std::fs;
use std::io::Read;
use std::path::Path;

use super::cpu::Cpu;
use super::debugger::Debugger;
use super::interconnect::Interconnect;
use crate::dol::Dol;

pub struct Gamecube {
    pub cpu: Cpu,
    interconnect: Interconnect,
}

impl Default for Gamecube {
    fn default() -> Self {
        Gamecube {
            cpu: Cpu::default(),
            interconnect: Interconnect::default(),
        }
    }
}

impl Gamecube {
    pub fn load_dol<P: AsRef<Path>>(&mut self, path: P) {
        let dol = Dol::open(path).unwrap();

        self.emulate_bs2();

        dol.load(&mut self.cpu, &mut self.interconnect);

        self.cpu.cia = dol.get_entry_point();
    }

    // load ipl into bootrom and decrypt
    pub fn load_ipl<P: AsRef<Path>>(&mut self, path: P) {
        let mut file = match fs::File::open(path) {
            Ok(v) => v,
            Err(e) => {
                panic!("{}", e);
            }
        };

        let mut bootrom = self.interconnect.bootrom.borrow_mut();

        match file.read_exact(&mut *bootrom) {
            Ok(_) => {}
            Err(e) => {
                panic!("{}", e);
            }
        };

        descrambler(&mut bootrom[0x100..0x0015_ee40]);
    }

    pub fn run(&mut self, debugger: &mut Debugger) {
        loop {
            self.cpu.run_instruction(&mut self.interconnect, debugger);
        }
    }

    pub fn emulate_bs2(&mut self) {
        self.cpu.msr = 0x0000_2030.into();

        self.interconnect.mmu.write_ibatu(0, 0x8000_1fff); // Spr::IBAT0U
        self.interconnect.mmu.write_ibatl(0, 0x0000_0002); // Spr::IBAT0L
        self.interconnect.mmu.write_ibatu(3, 0xfff0_001f); // Spr::IBAT3U
        self.interconnect.mmu.write_ibatl(3, 0xfff0_0001); // Spr::IBAT3L
        self.interconnect.mmu.write_dbatu(0, 0x8000_1fff); // Spr::DBAT0U
        self.interconnect.mmu.write_dbatl(0, 0x0000_0002); // Spr::DBAT0L
        self.interconnect.mmu.write_dbatu(1, 0xc000_1fff); // Spr::DBAT1U
        self.interconnect.mmu.write_dbatl(1, 0x0000_002a); // Spr::DBAT1L
        self.interconnect.mmu.write_dbatu(3, 0xfff0_001f); // Spr::DBAT3U
        self.interconnect.mmu.write_dbatl(3, 0xfff0_0001); // Spr::DBAT3L

        self.interconnect
            .write_u32(&self.cpu.msr, 0x8000_0034, 0x817F_E8C0); // ArenaHi

        self.interconnect
            .write_u16(&self.cpu.msr, 0xCC00_2002, 0x0001); // VI - Display Config
    }
}

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
