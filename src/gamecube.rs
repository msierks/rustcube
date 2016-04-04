use std::path::Path;

use super::cpu;
use super::memory;

#[derive(Debug)]
pub struct Gamecube {
    cpu: cpu::Cpu
}

impl Gamecube {
    pub fn new() -> Gamecube {
        let interconnect = memory::Interconnect::new();
        let cpu = cpu::Cpu::new(interconnect);

        Gamecube {
            cpu: cpu
        }
    }

    pub fn load_ipl<P: AsRef<Path>>(&mut self, path: P) {
        self.cpu.interconnect.load_ipl(path);
    }

    pub fn run_instruction(&mut self) {
        self.cpu.run_instruction();
    }
}
