use std::io::{self, Write};

use super::debugger::Debugger;
use super::super::cpu::Cpu;

#[derive(Default)]
pub struct Console;

impl Console {
    // FixMe: handle arrow key control chars, command history, etc

    pub fn read(&mut self) -> Command {
        let mut input = String::new();

        print!("(rustcube) ");

        io::stdout().flush().unwrap();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {},
            Err(error) => println!("error: {}", error),
        }

        Command::new(input)
    }

    pub fn intro(&mut self) {
        println!("For help, type \"help\"");
    }
}

pub struct Command {
    data: String
}

impl Command {
    pub fn new(data: String) -> Command {
        Command {
            data: data
        }
    }

    pub fn execute(&self, debugger: &mut Debugger, cpu: &mut Cpu) {
        let args = self.data.trim().split(" ").collect::<Vec<&str>>();

        if args.len() == 0 {
            self.help(&args);
        } else {

            match args[0] {
                "advance" => self.advance(&args, debugger),
                "break" | "b" => self.break_(&args, debugger),
                "clear" => self.clear(&args, debugger),
                "continue" | "c" => self.continue_(debugger),
                "help" => self.help(&args),
                "show" => self.show(&args, debugger, cpu),
                "step" => self.step(&args, debugger),
                _ => self.help(&args)
            }

        }
    }

    fn advance(&self, args: &Vec<&str>, debugger: &mut Debugger) {
        println!("FixMe: continue running till given location.");
    }

    fn break_(&self, args: &Vec<&str>, debugger: &mut Debugger) {
        if args.len() > 1 {
            match u32::from_str_radix(&args[1][2..], 16) {
                Ok(v) => debugger.add_breakpoint(v),
                Err(e) => println!("Error: {}", e)
            }
        } else {
            println!("Missing required argument.");
        }
    }

    fn clear(&self, args: &Vec<&str>, debugger: &mut Debugger) {
        if args.len() > 1 {
            match u32::from_str_radix(&args[1][2..], 16) {
                Ok(v) => debugger.remove_breakpoint(v),
                Err(e) => println!("Error: {}", e)
            }
        } else {
            println!("Missing required argument.");
        }
    }

    fn continue_(&self, debugger: &mut Debugger) {
        debugger.continue_();
    }

    fn help(&self, args: &Vec<&str>) {
        if args.len() < 2 {
            println!("List of available commands:\n");
            println!("break    - set a breakpoint");
            println!("clear    - delete a breakpoint");
            println!("continue - continue running program");
            println!("show     - show things about program");
            println!("step     - step a single instruction");
        } else {
            println!("Unrecognized help command: \"{}\". Try \"help\"", args[1])
        }
    }

    fn show(&self, args: &Vec<&str>, debugger: &mut Debugger, cpu: &mut Cpu) {
        if args.len() > 1 {

            match args[1] {
                "break" | "b" => {
                    for breakpoint in &debugger.breakpoints {
                        println!("break: {:#010x}", breakpoint);
                    }
                },
                "cia" => println!("cia: {:#010x}", cpu.cia),
                "gpr" => {
                    for i in 0..cpu.gpr.len() {
                        if cpu.gpr[i] != 0 {
                            println!("r{:<10} {:#010x}    {}", i, cpu.gpr[i], cpu.gpr[i]);
                        }
                    }
                },
                _ => println!("Unrecognized show command: \"{}\". Try \"help show\"", args[1])
            }

        } else {
            println!("Missing required argument.");
        }
    }

    fn step(&self, args: &Vec<&str>, debugger: &mut Debugger) {
        if args.len() > 1 {
            match u32::from_str_radix(&args[1], 10) {
                Ok(v) => debugger.set_step(v),
                Err(e) => println!("Error: {}", e)
            }
        } else {
            debugger.set_step(1);
        }
    }
}
