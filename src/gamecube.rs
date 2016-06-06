use std::path::Path;

use super::cpu::Cpu;
use super::memory::Interconnect;

pub struct Gamecube {
    pub cpu: Cpu
}

impl Gamecube {
    pub fn new() -> Gamecube {
        let interconnect = Interconnect::new();
        let cpu = Cpu::new(interconnect);

        Gamecube {
            cpu: cpu
        }
    }

    pub fn load_ipl<P: AsRef<Path>>(&mut self, path: P) {
        self.cpu.interconnect.load_ipl(path);
    }

    pub fn run(&mut self) {
        loop {
            self.cpu.run_instruction();
        }
    }
}
