
use super::memory::Ram;

//const STATUS:  u32 = 0x00;
//const CONTROL: u32 = 0x02;

#[derive(Default)]
pub struct CommandProcessor;

impl CommandProcessor {

    pub fn new() -> CommandProcessor {
        CommandProcessor
    }

    //pub fn read_u16(&mut self, register: u32) -> u16 {
    //    println!("READ CP reg {:#x}", register);
    //    0
    //}

    pub fn write_u16(&mut self, register: u32, val: u16) {
        println!("WRITE CP reg {:#x} {:#x}", register, val);
    }

    pub fn write_u32(&mut self, register: u32, val: u32) {
        println!("WRITE CP reg {:#x} {:#x}", register, val);
    }

    pub fn gather_pipe_burst(&mut self, ram: &mut Ram) {

    }
}
