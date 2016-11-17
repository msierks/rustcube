mod console;
mod disassembler;

#[cfg(unix)] use std::sync;
#[cfg(unix)] use std::sync::atomic;
#[cfg(unix)] use nix::sys::signal;

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

        install_sigint_handler();

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

    // check if a SIGINT signal has been received
    #[cfg(unix)]
    pub fn sigint(&self) -> bool {
        SIGINT.compare_and_swap(true, false, atomic::Ordering::SeqCst)
    }

    // This function is only included when foo is not defined
    #[cfg(not(unix))]
    pub fn sigint(&self) -> bool {
        false
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
        if self.sigint() {
            self.resume = false;
        }

        if self.breakpoints.contains(&cpu.cia) {
            self.resume = false;
            println!("breakpoint {:#010x}", cpu.cia);
        }

        if self.advance == cpu.cia {
            self.advance = 0;
            self.resume = false;
            println!("advanced {:#010x}", cpu.cia);
        }

        self.debug(cpu);
    }

    fn memory_write(&mut self, cpu: &mut Cpu, addr: u32) {
        if self.watchpoints.contains(&addr) {
            self.resume = false;
            println!("watchpoint {:#010x}", addr);
        }

        self.debug(cpu);
    }
}

#[cfg(unix)]
static SIGINT_ONCE: sync::Once = sync::ONCE_INIT;
#[cfg(unix)]
static SIGINT: atomic::AtomicBool = atomic::ATOMIC_BOOL_INIT;

#[cfg(unix)]
fn install_sigint_handler() {
    SIGINT_ONCE.call_once(|| unsafe {
        let sigint = signal::SigAction::new(signal::SigHandler::Handler(sigint_handler),
                                            signal::SaFlag::empty(),
                                            signal::SigSet::empty());
        let _ = signal::sigaction(signal::SIGINT, &sigint);
    });
}

#[cfg(not(unix))]
fn install_sigint_handler() {
}

#[cfg(unix)]
extern "C" fn sigint_handler(_: signal::SigNum) {
    SIGINT.store(true, atomic::Ordering::SeqCst);
}
