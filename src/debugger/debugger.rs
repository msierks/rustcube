
use super::console::Console;
use super::disassembler::Disassembler;
use super::super::cpu::Cpu;

pub struct Debugger {
    console: Console,
    disassembler: Disassembler,
    active: bool,
    resume: bool,
    step: bool,
    step_count: u32,
    advance: u32,
    pub breakpoints: Vec<u32>,
    pub watchpoints: Vec<u32>
}

impl Debugger {
    pub fn new() -> Debugger {
        Debugger {
            console: Console::new(),
            disassembler: Disassembler::default(),
            active: false,
            resume: false,
            step: false,
            step_count: 0,
            advance: 0,
            breakpoints: Vec::new(),
            watchpoints: Vec::new()
        }
    }

    pub fn enable(&mut self) {
        self.active = true;
        self.console.intro();
    }

    pub fn debug(&mut self, cpu: &mut Cpu) {
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

    pub fn add_watchpoint(&mut self, addr: u32) {
        if !self.watchpoints.contains(&addr) {
            self.watchpoints.push(addr);
        }
    }

    pub fn remove_breakpoint(&mut self, addr: u32) {
        self.breakpoints.retain(|&a| a != addr);
    }

    pub fn remove_watchpoint(&mut self, addr: u32) {
        self.watchpoints.retain(|&a| a != addr);
    }

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

    pub fn set_advance(&mut self, val: u32) {
        self.advance = val;
        self.resume = true;
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

            if self.advance == 0 {
                if (self.step && self.step_count == 0) || self.breakpoints.contains(&cpu.cia) {
                    self.resume = false;
                }
            } else if self.advance == cpu.cia {
                self.advance = 0;
                self.resume = false
            }

            self.debug(cpu);

            if self.step {
                let instr = cpu.read_instruction();

                self.disassembler.disassemble(cpu, instr);

                println!("{:#010x}       {: <7} {}", cpu.cia, self.disassembler.opcode, self.disassembler.operands);
            }
        }
    }

    pub fn write_memory(&mut self, cpu: &mut Cpu, addr: u32) {
        if self.active {
            if self.watchpoints.contains(&addr) {
                println!("Write watchpoint triggered at {:#010x}", addr);

                let instr = cpu.read_instruction();

                self.disassembler.disassemble(cpu, instr);

                println!("{:#010x}       {: <7} {}", cpu.cia, self.disassembler.opcode, self.disassembler.operands);

                self.resume = false;

                self.debug(cpu);
            }
        }
    }
}
