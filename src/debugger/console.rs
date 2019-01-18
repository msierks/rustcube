use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::num::ParseIntError;
use std::process;

use super::super::cpu::Cpu;
use super::super::interconnect::Interconnect;
use crate::debugger::ConsoleDebugger;

pub struct Console {
    rl: Editor<()>,
}

impl Console {
    pub fn new() -> Console {
        Console { rl: Editor::new() }
    }

    pub fn read(&mut self) -> Command {
        loop {
            let readline = self.rl.readline("(rustcube) ");

            match readline {
                Ok(line) => {
                    self.rl.add_history_entry(&line);
                    return Command::new(line);
                }
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    process::exit(0);
                }
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    process::exit(0);
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    process::exit(1);
                }
            }
        }
    }

    pub fn intro(&mut self) {
        println!("For help, type \"help\"");
    }
}

pub struct Command {
    data: String,
}

impl Command {
    pub fn new(data: String) -> Command {
        Command { data }
    }

    pub fn execute(
        &self,
        debugger: &mut ConsoleDebugger,
        cpu: &mut Cpu,
        interconnect: &mut Interconnect,
    ) {
        let args = self.data.trim().split(' ').collect::<Vec<&str>>();

        if args.is_empty() {
            self.help(&args);
        } else {
            match args[0] {
                "advance" => self.advance(&args, debugger),
                "break" | "b" => self.break_(&args, debugger),
                "clear" => self.clear(&args, debugger),
                "continue" | "c" => debugger.continue_(),
                "examine" | "x" => self.examine(&args, cpu, interconnect),
                "show" => self.show(&args, debugger, cpu, interconnect),
                "step" | "" => self.step(&args, debugger),
                "watch" | "w" => self.watch_(&args, debugger),
                "help" | _ => self.help(&args),
            }
        }
    }

    fn advance(&self, args: &[&str], debugger: &mut ConsoleDebugger) {
        if args.len() > 1 {
            match parse_hex_str(args[1]) {
                Ok(v) => debugger.set_advance(v),
                Err(e) => println!("Error: {}", e),
            }
        } else {
            println!("Missing required argument.");
        }
        println!("FixMe: continue running till given location.");
    }

    fn break_(&self, args: &[&str], debugger: &mut ConsoleDebugger) {
        if args.len() > 1 {
            match parse_hex_str(args[1]) {
                Ok(v) => debugger.add_breakpoint(v),
                Err(e) => println!("Error: {}", e),
            }
        } else {
            println!("Missing required argument.");
        }
    }

    fn examine(&self, args: &[&str], cpu: &mut Cpu, interconnect: &mut Interconnect) {
        if args.len() > 1 {
            match parse_hex_str(args[1]) {
                Ok(v) => println!("{:#010x}: {:#010x}", v, interconnect.read_u16(&cpu.msr, v)),
                Err(e) => println!("Error: {}", e),
            }
        } else {
            println!("Missing required argument.");
        }
    }

    fn watch_(&self, args: &[&str], debugger: &mut ConsoleDebugger) {
        if args.len() > 1 {
            match parse_hex_str(args[1]) {
                Ok(v) => debugger.add_watchpoint(v),
                Err(e) => println!("Error: {}", e),
            }
        } else {
            println!("Missing required argument.");
        }
    }

    fn clear(&self, args: &[&str], debugger: &mut ConsoleDebugger) {
        if args.len() > 1 {
            match parse_hex_str(args[1]) {
                Ok(v) => {
                    debugger.remove_breakpoint(v);
                    debugger.remove_watchpoint(v);
                }
                Err(e) => println!("Error: {}", e),
            }
        } else {
            println!("Missing required argument.");
        }
    }

    fn help(&self, args: &[&str]) {
        if args.len() < 2 {
            println!("List of available commands:\n");
            println!("advance  - continue running to given location");
            println!("break    - set a breakpoint");
            println!("clear    - delete a breakpoint");
            println!("continue - continue running program");
            println!("examine  - show memory value");
            println!("show     - show things about program");
            println!("step     - step a single instruction");
            println!("watch    - set a watchpoint for written value\n");
            println!("Note: the ipl starts at address 0x81300000")
        } else {
            println!("Unrecognized help command: \"{}\". Try \"help\"", args[1])
        }
    }

    fn show(
        &self,
        args: &[&str],
        debugger: &mut ConsoleDebugger,
        cpu: &mut Cpu,
        interconnect: &mut Interconnect,
    ) {
        if args.len() > 1 {
            match args[1] {
                "breakpoints" | "b" => {
                    for breakpoint in &debugger.breakpoints {
                        println!("break: {:#010x}", breakpoint);
                    }
                }
                "ci" => debugger.print_instruction(cpu, interconnect),
                "cia" => println!("cia: {:#010x}", cpu.cia),
                "gpr" => {
                    for i in 0..cpu.gpr.len() {
                        if cpu.gpr[i] != 0 {
                            println!("r{:<10} {:#010x}    {}", i, cpu.gpr[i], cpu.gpr[i]);
                        }
                    }
                }
                "lr" => println!("lr: {:#010x}", cpu.lr),
                "watchpoints" | "w" => {
                    for watchpoint in &debugger.watchpoints {
                        println!("watch: {:#010x}", watchpoint);
                    }
                }
                _ => println!(
                    "Unrecognized show command: \"{}\". Try \"help show\"",
                    args[1]
                ),
            }
        } else {
            println!("Missing required argument.");
        }
    }

    fn step(&self, args: &[&str], debugger: &mut ConsoleDebugger) {
        if args.len() > 1 {
            match u32::from_str_radix(args[1], 10) {
                Ok(v) => debugger.set_step(v),
                Err(e) => println!("Error: {}", e),
            }
        } else {
            debugger.set_step(1);
        }
    }
}

fn parse_hex_str(val: &str) -> Result<u32, ParseIntError> {
    if val.starts_with("0x") {
        u32::from_str_radix(&val[2..], 16)
    } else {
        u32::from_str_radix(val, 16)
    }
}
