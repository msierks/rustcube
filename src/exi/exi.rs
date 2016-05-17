use std::rc::Rc;
use std::cell::RefCell;

use super::channel::{Channel, TransferMode, TransferType};
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
            0x10 => channel.imm_data,         // IMM Data
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
            0x0C => {                              // DMA Control
                channel.control = value.into();

                //println!("EXI {:#b} {:#?}", value, channel.control);

                if channel.control.transfer_start {

                    match channel.control.transfer_mode {
                        TransferMode::IMM => {
                            match channel.control.transfer_type {
                                TransferType::READ => channel.imm_data = channel.get_device(channel.status.exi_device).read_imm(channel.control.transfer_length + 1),
                                TransferType::WRITE => {
                                    let exi_device = channel.status.exi_device;
                                    let imm_data = channel.imm_data;
                                    let transfer_len = channel.control.transfer_length + 1;
                                    channel.get_device_mut(exi_device).write_imm(imm_data, transfer_len);
                                },
                                _ => panic!("EXI IMM invalid transfer type")
                            }
                        },
                        TransferMode::DMA => {
                            match channel.control.transfer_type {
                                TransferType::READ => channel.get_device(channel.status.exi_device).read_dma(memory, channel.dma_address, channel.dma_length),
                                TransferType::WRITE => channel.get_device(channel.status.exi_device).write_dma(memory, channel.dma_address, channel.dma_length),
                                _ => panic!("EXI DMA invalid transfer type")
                            }
                        }
                    }

                    channel.control.transfer_start = false;
                }
            },
            0x10 => channel.imm_data = value, // IMM Data
            _ => panic!("exi register out of range {:#x}", register)
        }
    }

}