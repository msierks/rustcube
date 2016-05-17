use std::rc::Rc;
use std::cell::RefCell;

use super::channel::{Channel, TransferSelection};
use super::device::DeviceDummy;
use super::device_ipl::{DeviceIpl, BOOTROM_SIZE};
use super::device_ad16::DeviceAd16;
use super::super::memory::ram::Ram;

const NUM_CHANNELS: usize = 3;

pub struct Exi {
    channels: [Channel; NUM_CHANNELS]
}

impl Exi {
    pub fn new(bootrom: Rc<RefCell<Box<[u8; BOOTROM_SIZE]>>>) -> Exi {
        let channel0 = Channel::new([
            Box::new(DeviceDummy::new()),
            Box::new(DeviceIpl::new(bootrom)),
            Box::new(DeviceDummy::new())
        ]);
        let channel1 = Channel::new([
            Box::new(DeviceDummy::new()),
            Box::new(DeviceDummy::new()),
            Box::new(DeviceDummy::new())
        ]);
        let channel2 = Channel::new([
            Box::new(DeviceAd16::new()),
            Box::new(DeviceDummy::new()),
            Box::new(DeviceDummy::new())
        ]);

        Exi {
            channels: [channel0, channel1, channel2]
        }
    }

    pub fn read(&self, channel_num: u32, register: u32) -> u32 {
        let channel = match self.channels.get(channel_num as usize) {
            Some(channel) => channel,
            None => panic!("exi channel out of range: {}", channel_num)
        };

        match register {
            0x00 => channel.status.as_u32(),  // Status
            0x04 => channel.dma_address,      // DMA Addr
            0x08 => channel.dma_length,       // DMA Length
            0x0C => channel.control.as_u32(), // DMA Control
            0x10 => channel.get_device(channel.status.exi_device).read_imm(), // IMM Data
            _ => panic!("exi register out of range {:#x}", register)
        }
    }

    pub fn write(&mut self, channel_num: u32, register: u32, value: u32, memory: &mut Ram) {
        let channel = match self.channels.get_mut(channel_num as usize) {
            Some(channel) => channel,
            None => panic!("exi channel out of range: {}", channel_num)
        };

        match register {
            0x00 => channel.status = value.into(), // Status
            0x04 => channel.dma_address = value,   // DMA Addr
            0x08 => channel.dma_length = value,    // DMA Length
            0x0C => {
                channel.control = value.into();

                //println!("EXI {:#b} {:#?}", value, channel.control);

                match channel.control.transfer_selection {
                    TransferSelection::DMA => {
                        channel.get_device(channel.status.exi_device).write_dma(memory, channel.dma_address, channel.dma_length)
                    },
                    _ => {}
                }

                channel.control.enabled = false; // finish transfer immediately
            }, // DMA Control
            0x10 => {
                let device_num = channel.status.exi_device;

                channel.get_device_mut(device_num).write_imm(value);
            }, // IMM Data
            _ => panic!("exi register out of range {:#x}", register)
        }
    }

}