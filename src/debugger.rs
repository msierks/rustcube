use std::io::{self, Write};

use super::cpu::Cpu;

#[derive(Debug)]
pub struct Debugger {
    enabled: bool,
    step: bool,
    resume: bool,
    breakpoints:  Vec<u32>
}

impl Debugger {

    pub fn new() -> Debugger {
        Debugger {
            enabled: false,
            step: false,
            resume: false,
            breakpoints: Vec::new()
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
        self.step = true;
        self.resume = false;
    }

    pub fn intro(&mut self) {
        println!("For help, type \"help\"");
    }

    fn read_command(&mut self, cpu: &mut Cpu) {
        print!("(rustcube) ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {},
            Err(error) => println!("error: {}", error),
        }

        let split = input.trim().split(" ");
        let command = split.collect::<Vec<&str>>();

        match command[0].as_ref() {
            "break" => self.command_break(&command),
            "clear" => self.command_clear(&command),
            "continue" => {
                self.step = false;
                self.resume = true;
            },
            "show" => self.command_show(&command, cpu),
            "help" => self.command_help(),
            "step" => {
                println!("{:#010x}", cpu.cia);
                self.resume = true
            },
            _ => println!("Unrecognized command: \"{}\". Try \"help\"", command[0])
        }
    }

    fn command_show(&mut self, command: &Vec<&str>, cpu: &mut Cpu) {
        if command.len() > 1 {
            let subcommand = command[1];

            match command[1] {
                "break" => {
                    for breakpoint in &self.breakpoints {
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
                "nia" => println!("nia: {:#010x}", cpu.nia),
                _ => println!("Unrecognized command: \"{}\". Try \"help\"", subcommand)
            }
        } else {
            self.command_help();
        }
    }

    // set breakpoint
    fn command_break(&mut self, command: &Vec<&str>) {
        if command.len() > 1 {
            match u32::from_str_radix(&command[1][2..], 16) {
                Ok(v) => self.breakpoints.push(v),
                Err(e) => println!("Error: {}", e)
            }
        } else {
            self.command_help();
        }
    }

    fn command_clear(&mut self, command: &Vec<&str>) {
        match u32::from_str_radix(&command[1][2..], 16) {
            Ok(v) => {
                match self.breakpoints.iter().position(|&x| x == v) {
                    Some(i) => {
                        self.breakpoints.remove(i);
                    },
                    None => {}
                }
            },
            Err(e) => println!("Error: {}", e)
        }
    }

    fn command_help(&mut self) {
        println!("List of available commands:\n");
        println!("break   - set a breakpoint");
        println!("clear   - delete a breakpoint");
        println!("show    - show things about program");
        println!("step    - continue running program");
    }

    pub fn set_cia(&mut self, cpu: &mut Cpu) {
        if self.enabled {
            if self.breakpoints.contains(&cpu.cia) || self.step {
                self.resume = false;
            }

            while !self.resume {
                self.read_command(cpu);
            }
        }
    }

}
