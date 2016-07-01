
const STATUS:  u32 = 0x00;
const CONTROL: u32 = 0x02;


pub struct CommandProcessor;

impl CommandProcessor {

    pub fn new() -> CommandProcessor {
        CommandProcessor
    }

    pub fn read_u16(&mut self, register: u32) -> u16 {
        println!("READ CP reg {:#x}", register);
        0
    }


    pub fn write_u16(&mut self, register: u32, val: u16) {
        println!("WRITE CP reg {:#x} {:#x}", register, val);
    }
}
