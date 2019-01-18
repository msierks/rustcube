use super::super::memory::Ram;
use super::device::Device;

// AD16

pub struct DeviceAd16;

impl Device for DeviceAd16 {
    fn device_select(&mut self) {
        println!("DeviceAd16 Selected");
    }

    fn read_imm(&self, len: u8) -> u32 {
        println!("ExiDeviceAd16: read_imm {}", len);
        0x0412_0000 // FixMe: always returns AD16 EXI ID
    }

    fn write_imm(&mut self, value: u32, _: u8) {
        match value {
            0x0A00_0000 => println!("AD16: CardInit {:#x}", value),
            0x0B00_0000 => println!("AD16: VIInit {:#x}", value),
            0x0C00_0000 => println!("AD16: PADInit {:#x}", value),
            0x0000_0000 => println!("AD16: get ID command"),
            0x0100_0000 => println!("AD16: init"),
            0x0200_0000 => println!("AD16: ???"),
            0x0300_0000 => println!("AD16: ???"),
            0x0400_0000 => println!("AD16: memory test passed"),
            0x0500_0000 => panic!("AD16: memory test failed {:#x}", value),
            0x0600_0000 => panic!("AD16: memory test failed {:#x}", value),
            0x0700_0000 => panic!("AD16: memory test failed {:#x}", value),
            0x0800_0000 => println!("AD16: AD16Init {:#x}", value),
            0x0900_0000 => println!("AD16: DVDInit {:#x}", value),
            0xA000_0000 => println!("AD16: WRITE/CODE/PADDING??? {:#x}", value),
            _ => panic!("AD16: unhandled value {:#x}", value),
        }
    }

    fn read_dma(&self, _: &mut Ram, _: u32, _: u32) {
        println!("ExiDeviceAd16: read_dma");
    }

    fn write_dma(&self, _: &mut Ram, _: u32, _: u32) {
        println!("ExiDeviceAd16: write_dma");
    }
}

impl DeviceAd16 {
    pub fn new() -> DeviceAd16 {
        DeviceAd16
    }
}
