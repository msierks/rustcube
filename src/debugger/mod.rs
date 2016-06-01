mod console;
mod disassembler;

use self::console::Console;
use self::disassembler::Disassembler;
use super::gamecube::Gamecube;

pub struct Debugger {
    console: Console,
    pub gamecube: Gamecube,
    resume: bool,
    step: bool,
    step_count: u32,
    advance: u32,
    pub breakpoints: Vec<u32>,
    pub watchpoints: Vec<u32>
}

impl Debugger {
    pub fn new(gamecube: Gamecube) -> Debugger {
        Debugger {
            console: Console::new(),
            gamecube: gamecube,
            resume: false,
            step: false,
            step_count: 0,
            advance: 0,
            breakpoints: Vec::new(),
            watchpoints: Vec::new()
        }
    }

    // loop over cia and get next instruction
    pub fn run(&mut self) {
        self.console.intro();

        loop {
            while !self.resume {
                let command = self.console.read();

                command.execute(self);
            }

            self.set_cia();

            self.gamecube.cpu.run_instruction();
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

    pub fn set_cia(&mut self) {
        let cia = self.gamecube.cpu.cia;

        if self.step {
            if self.step_count > 0 {
                self.step_count -= 1;
            } else {
                self.step_count = 0;
                self.step = false;
            }
        }

        if self.advance == 0 {
            if (self.step && self.step_count == 0) || self.breakpoints.contains(&cia) {
                self.resume = false;
            }
        } else if self.advance == cia {
            self.advance = 0;
            self.resume = false
        }

        if self.step {
        //if cia > 0x81300000 && cia < 0xFFF00000 {
            let instr = self.gamecube.cpu.read_instruction();

            let mut disassembler = Disassembler::default();

            disassembler.disassemble(self, instr);

            println!("{:#010x}       {: <7} {}", cia, disassembler.opcode, disassembler.operands);
        }
    }

    pub fn write_memory(&mut self, addr: u32) {
        if self.watchpoints.contains(&addr) {
            println!("Write watchpoint triggered at {:#010x}", addr);

            self.resume = false;
        }
    }

    /*
    pub fn write_memory(&mut self, cpu: &mut Cpu, addr: u32) {
        if self.watchpoints.contains(&addr) {
            println!("Write watchpoint triggered at {:#010x}", addr);

            let instr = cpu.read_instruction();

            self.disassembler.disassemble(cpu, instr);

            println!("{:#010x}       {: <7} {}", cpu.cia, self.disassembler.opcode, self.disassembler.operands);

            self.resume = false;

            self.debug(cpu);
        }
    }
    */
}
