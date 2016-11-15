use std::path::Path;

use super::cpu::Cpu;
use super::memory::Interconnect;
use super::debugger::Debugger;

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

    pub fn run(&mut self, debugger: &mut Debugger) {
        loop {
            self.cpu.run_instruction(debugger);
        }
    }
}
