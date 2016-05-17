
use super::device::Device;
use super::super::memory::ram::Ram;

// AD16

pub struct DeviceAd16;

impl Device for DeviceAd16 {
    fn read_imm(&self, len: u8) -> u32 {
        println!("ExiDeviceAd16: read_imm {}", len);
        0x04120000 // FixMe: always returns AD16 EXI ID
    }

    fn write_imm(&mut self, value: u32, len: u8) {
        match value {
            0x00000000 => println!("AD16: get ID command"),
            0x01000000 => println!("AD16: init"),
            0x02000000 => println!("AD16: ???"),
            0x03000000 => println!("AD16: ???"),
            0x04000000 => println!("AD16: memory test passed"),
            0x05000000 => panic!("AD16: memory test failed {:#x}", value),
            0x06000000 => panic!("AD16: memory test failed {:#x}", value),
            0x07000000 => panic!("AD16: memory test failed {:#x}", value),
            _ => println!("AD16: unhandled value {:#x}", value)
        }
    }

    fn read_dma(&self, memory: &mut Ram, address: u32, length: u32) {
        println!("ExiDeviceAd16: read_dma");
    }

    fn write_dma(&self, memory: &mut Ram, address: u32, length: u32) {
        println!("ExiDeviceAd16: write_dma");
    }
}

impl DeviceAd16 {
    pub fn new() -> DeviceAd16 {
        DeviceAd16
    }
}
