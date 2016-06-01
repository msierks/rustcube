
use super::super::memory::ram::Ram;

pub trait Device {
    fn device_select(&mut self);
    fn read_imm(&self, len: u8) -> u32;
    fn write_imm(&mut self, value: u32, len: u8);
    fn read_dma(&self, memory: &mut Ram, address: u32, length: u32);
    fn write_dma(&self, memory: &mut Ram, address: u32, length: u32);
}

#[allow(dead_code)]
pub struct DeviceDummy;

impl Device for DeviceDummy {
    fn device_select(&mut self) {
        //println!("DeviceDummy Selected");
    }

    #[allow(unused_variables)]
    fn read_imm(&self, len: u8) -> u32 {
        println!("EXIDUMMY: read_imm");
        0
    }

    #[allow(unused_variables)]
    fn write_imm(&mut self, value: u32, len: u8) {
        println!("EXIDUMMY: write_imm {:#x}", value);
    }

    #[allow(unused_variables)]
    fn read_dma(&self, memory: &mut Ram, address: u32, length: u32) {
        println!("EXIDUMMY: read_dma");
    }

    #[allow(unused_variables)]
    fn write_dma(&self, memory: &mut Ram, address: u32, length: u32) {
        println!("EXIDUMMY: write_dma");
    }
}

#[allow(dead_code)]
impl DeviceDummy {
    pub fn new() -> DeviceDummy {
        DeviceDummy
    }
}
