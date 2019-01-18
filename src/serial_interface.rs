#[derive(Default)]
pub struct SerialInterface;

impl SerialInterface {
    pub fn read_u32(&self, register: u32) -> u32 {
        println!("READ SI {:#x}", register);
        0
    }

    pub fn write_u32(&mut self, register: u32, val: u32) {
        println!("WRITE SI {:#x}: {:#x}", register, val);
    }
}
