#[derive(Default)]
pub struct PixelEngine;

impl PixelEngine {
    pub fn read_u16(&mut self, register: u32) -> u16 {
        println!("READ PE reg {:#x}", register);
        0
    }

    pub fn write_u16(&mut self, register: u32, val: u16) {
        println!("WRITE PE reg {:#x} {:#x}", register, val);
    }
}
