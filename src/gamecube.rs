use std::fs;
use std::path::Path;
use std::io::Read;

use super::cpu::Cpu;
use super::interconnect::Interconnect;
use super::debugger::Debugger;

pub struct Gamecube {
    pub cpu: Cpu,
    interconnect: Interconnect
}

impl Gamecube {
    pub fn new() -> Gamecube {
        Gamecube {
            cpu: Cpu::new(),
            interconnect: Interconnect::new()
        }
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

        match file.read_exact(&mut **bootrom) {
            Ok(_) => {},
            Err(e) => {
                panic!("{}", e);
            }
        };

        descrambler(&mut bootrom[0x100..0x15ee40]);
    }

    pub fn run(&mut self, debugger: &mut Debugger) {
        loop {
            self.cpu.run_instruction(&mut self.interconnect, debugger);
        }
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
