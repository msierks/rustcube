
use memmap::Mmap;

pub trait Device {
    fn read_imm(&self) -> u32;
    fn write_imm(&mut self, value: u32);
    fn read_dma(&self);
    fn write_dma(&self, memory: &mut Mmap, address: u32, length: u32);
}

#[allow(dead_code)]
pub struct DeviceDummy;

impl Device for DeviceDummy {
    fn read_imm(&self) -> u32 {
        println!("EXIDUMMY: read_imm");
        0
    }

    fn write_imm(&mut self, value: u32) {
        println!("EXIDUMMY: write_imm {:#x}", value);
    }

    fn read_dma(&self) {
        println!("EXIDUMMY: read_dma");
    }

    fn write_dma(&self, memory: &mut Mmap, address: u32, length: u32) {
        println!("EXIDUMMY: write_dma");
    }
}

#[allow(dead_code)]
impl DeviceDummy {
    pub fn new() -> DeviceDummy {
        DeviceDummy
    }
}
