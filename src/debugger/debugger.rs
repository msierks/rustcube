
use super::console::Console;
use super::super::cpu::Cpu;

pub struct Debugger {
    console: Console,
    active: bool,
    resume: bool,
    step: bool,
    step_count: u32,
    pub breakpoints: Vec<u32>,
}

impl Debugger {
    pub fn new() -> Debugger {
        Debugger {
            console: Console::default(),
            active: false,
            resume: false,
            step: false,
            step_count: 0,
            breakpoints: Vec::new()
        }
    }

    pub fn enable(&mut self) {
        self.active = true;
        self.console.intro();
    }

    pub fn debug(&mut self, cpu: &mut Cpu) {
        if (self.step && self.step_count == 0) || self.breakpoints.contains(&cpu.cia) {
            self.resume = false;
        }

        while !self.resume {
            let command = self.console.read();

            command.execute(self, cpu);
        }
    }

    pub fn add_breakpoint(&mut self, addr: u32) {
        if !self.breakpoints.contains(&addr) {
            self.breakpoints.push(addr);
        }
    }

    pub fn remove_breakpoint(&mut self, addr: u32) {
        self.breakpoints.retain(|&a| a != addr);
    }

    // FixMe: ignore breakpoints at current location
    pub fn continue_(&mut self) {
        self.step = false;
        self.resume = true;
    }

    pub fn set_step(&mut self, val: u32) {
        if val != 0 {
            self.step = true;
            self.step_count = val;
            self.resume = true;
        }
    }

    pub fn set_cia(&mut self, cpu: &mut Cpu) {
        if self.active {
            if self.step {
                if self.step_count > 0 {
                    self.step_count -= 1;
                } else {
                    self.step_count = 0;
                    self.step = false;
                }
            }

            self.debug(cpu);
        }
    }
}
