use std::fs;
use std::io::Read;
use std::path::Path;

use super::cpu;
use super::interconnect;

#[derive(Debug)]
pub struct Gamecube {
    cpu: cpu::Cpu
}

impl Gamecube {
    pub fn new() -> Gamecube {
        let interconnect = interconnect::Interconnect::new();
        let cpu = cpu::Cpu::new(interconnect);

        Gamecube {
            cpu: cpu
        }
    }

    pub fn bootstrap<P: AsRef<Path>>(&mut self, path: P) {
        let mut data = read_bin(path);

        descrambler(&mut data[0x100..0x15ee30]);

        // copy ipl into bootrom
        self.cpu.interconnect.write(0xFFF00000, &data);
    }

    pub fn run_instruction(&mut self) {
        self.cpu.run_instruction();
    }
}

fn read_bin<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file = match fs::File::open(path) {
        Ok(v) => v,
        Err(e) => {
            panic!("{}", e);
        }
    };

    let mut file_buf = Vec::new();

    match file.read_to_end(&mut file_buf) {
        Ok(_) => {},
        Err(e) => {
            panic!("{}", e);
        }
    };

    file_buf
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
        x ^= ((u0 | u1)) as u8;
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