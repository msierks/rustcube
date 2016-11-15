mod console;
mod disassembler;

use self::console::Console;
use self::disassembler::Disassembler;
use super::cpu::Cpu;

pub trait Debugger {
    fn nia_change(&mut self, cpu: &mut Cpu);
    fn memory_write(&mut self, cpu: &mut Cpu, addr: u32);
}

pub struct DummyDebugger;

impl DummyDebugger {
    pub fn new() -> DummyDebugger {
        DummyDebugger
    }
}

impl Debugger for DummyDebugger {
    fn nia_change(&mut self, _: &mut Cpu) {}
    fn memory_write(&mut self, _: &mut Cpu, _: u32) {}
}

pub struct ConsoleDebugger {
    console: Console,
    resume: bool,
    step: bool,
    step_count: u32,
    advance: u32,
    pub breakpoints: Vec<u32>,
    pub watchpoints: Vec<u32>,
}

impl ConsoleDebugger {
    pub fn new() -> ConsoleDebugger {
        let mut console = Console::new();

        console.intro();

        ConsoleDebugger {
            console: console,
            resume: false,
            step: false,
            step_count: 0,
            advance: 0,
            breakpoints: Vec::new(),
            watchpoints: Vec::new(),
        }
    }

    pub fn debug(&mut self, cpu: &mut Cpu) {
        if self.step {
            if self.step_count > 0 {
                self.step_count -= 1;
                self.print_instruction(cpu);
            } else {
                self.step_count = 0;
                self.step = false;
                self.resume = false;
            }
        }

        while !self.resume {
            self.console.read().execute(self, cpu);
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

    pub fn print_instruction(&mut self, cpu: &mut Cpu) {
        let instr = cpu.read_instruction();

        let mut disassembler = Disassembler::default();

        disassembler.disassemble(cpu, instr);

        println!("{:#010x}       {: <7} {}", cpu.cia, disassembler.opcode, disassembler.operands);
    }
}

impl Debugger for ConsoleDebugger {
    fn nia_change(&mut self, cpu: &mut Cpu) {
        if self.breakpoints.contains(&cpu.cia) {
            self.resume = false;
            println!("break {:#010x}", cpu.cia);
        }

        if self.advance == cpu.cia {
            self.advance = 0;
            self.resume = false;
            println!("advance {:#010x}", cpu.cia);
        }

        self.debug(cpu);
    }

    fn memory_write(&mut self, cpu: &mut Cpu, addr: u32) {
        if self.watchpoints.contains(&addr) {
            self.resume = false;
            println!("watch {:#010x}", addr);
        }

        self.debug(cpu);
    }
}
