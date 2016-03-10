use std::collections::HashMap;

const NUM_CHANNELS: usize = 3;
const NUM_DEVICES : usize = 3;
const BASE_ADDRESS: u32 = 0x0C006800;

pub struct Exi {
    channels: [ExiChannel; NUM_CHANNELS]
}

impl Exi {
    pub fn new() -> Exi {
        let mut channel = ExiChannel::new();

        channel.add_device(0, Box::new(ExiAd16::new()));

        Exi {
            channels: [ExiChannel::new(), ExiChannel::new(), channel]
        }
    }

    pub fn read(&self, channel: u32, register: u32) -> u32 {
        println!("Read Channel: {} Register: {:#x}", channel, register);

        let channel = match self.channels.get(channel as usize) {
            Some(channel) => channel,
            None => panic!("exi channel out of range: {}", channel)
        };

        match register {
            0x00 => channel.status.as_u32(), // STATUS
            //0x04 => // DMA Addr
            //0x08 => // DMA Length
            0x0C => {
                println!("CONTROL REG: {:#?}", channel.control);
                channel.control.as_u32()
            }, // DMA Control
            0x10 => channel.get_device(channel.status.exi_device).read_imm(), // IMM Data
            _ => panic!("exi register out of range {:#x}", register)
        }
    }

    pub fn write(&mut self, channel: u32, register: u32, value: u32) {
        println!("Write Channel: {} Register: {:#x} value: {}", channel, register, value);

        let channel = match self.channels.get_mut(channel as usize) {
            Some(channel) => channel,
            None => panic!("exi channel out of range: {}", channel)
        };

        match register {
            0x00 => {
                channel.status = value.into()
            }, // STATUS
            //0x04 => // DMA Addr
            //0x08 => // DMA Length
            0x0C => {
                channel.control = value.into();
                channel.control.transfer_length = 0; // finish transfer immediately
                channel.control.enabled = false; // finish transfer immediately
            }, // DMA Control
            0x10 => channel.get_device(channel.status.exi_device).write_imm(value), // IMM Data
            _ => panic!("exi register out of range {:#x}", register)
        }
    }
}

struct ExiChannel {
    // status register
    status: ExiStatus,

    // control register
    control: ExiControl,

    // channel devices
    devices: HashMap<u8, Box<ExiDevice>>
}

impl ExiChannel {
    pub fn new() -> ExiChannel {
        ExiChannel {
            status: ExiStatus::default(),
            control: ExiControl::default(),
            devices: HashMap::new()
        }
    }

    pub fn add_device(&mut self, num: u8, device: Box<ExiDevice>) {
        self.devices.insert(num, device);
    }

    pub fn get_device(&self, num: u8) -> &Box<ExiDevice> {
        // FixMe: use status register to select device
        self.devices.get(&num).unwrap()
    }
}

#[derive(Default, Debug)]
struct ExiStatus {
    connected: bool,
    ext_interrupt: bool,
    exi_device: u8,
    exi_channel: u8,
    tc_interupt: bool,
    exi_interrupt: bool
}

impl ExiStatus {
    pub fn as_u32(&self) -> u32 {
        let mut value = 0;

        value = value ^ ((self.connected as u32)  << 13);
        value = value ^ ((self.ext_interrupt as u32)  << 12);
        value = value ^ ((self.exi_device as u32)  << 7);
        value = value ^ ((self.exi_channel as u32)  << 4);
        value = value ^ ((self.tc_interupt as u32)  << 3);
        value = value ^ ((self.exi_interrupt as u32)  << 1);

        value
    }
}

impl From<u32> for ExiStatus {
    fn from(value: u32) -> Self {
        ExiStatus {
            connected:     (value & (1 << 13)) != 0,
            ext_interrupt: (value & (1 << 12)) != 0,
            exi_device:    ((value << 7) & 7) as u8,
            exi_channel:   ((value << 4) & 7) as u8,
            tc_interupt:   (value & (1 <<  3)) != 0,
            exi_interrupt: (value & (1 <<  1)) != 0
        }
    }
}

#[derive(Default, Debug)]
struct ExiControl {
    transfer_length: u32,
    transfer_type: u8,
    transfer_selection: bool,
    enabled: bool               // Note: When an EXI DMA\IMM operation has been completed, the EXI Enable Bit will be reset to 0.
}

impl ExiControl {
    pub fn as_u32(&self) -> u32 {
        let mut value = 0;

        value = value ^ ((self.transfer_length as u32)  << 4);
        value = value ^ ((self.transfer_type as u32)  << 2);
        value = value ^ ((self.transfer_selection as u32)  << 1);
        value = value ^ (self.enabled as u32);

        value
    }
}

impl From<u32> for ExiControl {
    fn from(value: u32) -> Self {
        ExiControl {
            transfer_length:    (value << 4),
            transfer_type:      ((value << 2) & 3) as u8,
            transfer_selection: (value & (1 << 1)) != 0,
            enabled:            (value & 1) != 0
        }
    }
}

trait ExiDevice {
    fn read_imm(&self) -> u32;
    fn write_imm(&self, value: u32);
    fn read_dma(&self);
    fn write_dma(&self);
}

// AD16

struct ExiAd16;

impl ExiDevice for ExiAd16 {
    fn read_imm(&self) -> u32 {
        0x04120000
    }
    fn write_imm(&self, value: u32) {
        println!("AD16 Command: {}", value);
    }
    fn read_dma(&self) {

    }
    fn write_dma(&self) {

    }
}

impl ExiAd16 {
    pub fn new() -> ExiAd16 {
        ExiAd16
    }
}
