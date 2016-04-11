use std::path::Path;

use super::cpu::Cpu;
use super::debugger::Debugger;
use super::memory::Interconnect;

#[derive(Debug)]
pub struct Gamecube {
    cpu: Cpu,
    debugger: Debugger
}

impl Gamecube {
    pub fn new() -> Gamecube {
        let debugger = Debugger::new();
        let interconnect = Interconnect::new();
        let cpu = Cpu::new(interconnect);

        Gamecube {
            cpu: cpu,
            debugger: debugger
        }
    }

    pub fn enable_debugger(&mut self) {
        self.debugger.enable();
        self.debugger.intro();
    }

    pub fn load_ipl<P: AsRef<Path>>(&mut self, path: P) {
        self.cpu.interconnect.load_ipl(path);
    }

    pub fn run_instruction(&mut self) {
        self.cpu.run_instruction(&mut self.debugger);
    }
}
