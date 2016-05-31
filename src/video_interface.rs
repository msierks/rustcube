
pub struct VideoInterface;

impl VideoInterface {

    pub fn new() -> VideoInterface {
        VideoInterface
    }
 
    pub fn read_u16(&self, register: u32) -> u16 {
        println!("READ VI reg {:#x}", register);
        0
    }

    pub fn write_u16(&mut self, register: u32, val: u16) {
        println!("WRITE VI reg {:#x} {}", register, val);
    }

}
