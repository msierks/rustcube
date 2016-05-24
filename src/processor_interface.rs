
pub struct ProcessorInterface;

impl ProcessorInterface {

    pub fn new() -> ProcessorInterface {
        ProcessorInterface
    }
 
    pub fn read_u32(&self, register: u32) -> u32 {
        println!("READ PI");
        0
    }

    pub fn write_u32(&mut self, register: u32, val: u32) {
        println!("WRITE PI");
    }

}
