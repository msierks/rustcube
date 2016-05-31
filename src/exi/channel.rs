use super::device::Device;

const NUM_DEVICES: usize = 3;

pub struct Channel {
    // status register
    pub status: Status,

    // control register
    pub control: Control,

    pub dma_address: u32,

    pub dma_length: u32,

    pub imm_data: u32,

    // channel devices
    pub devices: [Box<Device>; NUM_DEVICES]
}

impl Channel {
    pub fn new(devices: [Box<Device>; NUM_DEVICES]) -> Channel {
        Channel {
            status: Status::default(),
            control: Control::default(),
            dma_address: 0,
            dma_length: 0,
            imm_data: 0,
            devices: devices
        }
    }

    pub fn get_device(&self, num: u8) -> &Box<Device> {
        match self.devices.get(num as usize) {
            Some(device) => device,
            None => panic!("exi device not found: {}", num)  
        }
    }

    pub fn get_device_mut(&mut self, num: u8) -> &mut Box<Device> {
        match self.devices.get_mut(num as usize) {
            Some(device) => device,
            None => panic!("exi device not found: {}", num)  
        }
    }
}

#[derive(Default, Debug)]
pub struct Status {
    connected: bool,
    ext_interrupt: bool,
    pub device_select: u8,
    exi_frequency: u8,
    tc_interupt: bool,
    exi_interrupt: bool
}

impl Status {
    pub fn as_u32(&self) -> u32 {
        let mut value = 0;

        let device:u8 = match (value >> 7) & 7 {
            0 => 1,
            1 => 2,
            2 => 4,
            _ => 1
        };

        value = value ^ ((self.connected as u32) << 13);
        value = value ^ ((self.ext_interrupt as u32) << 12);
        value = value ^ ((device as u32) << 7);
        value = value ^ ((self.exi_frequency as u32) << 4);
        value = value ^ ((self.tc_interupt as u32) << 3);
        value = value ^ ((self.exi_interrupt as u32) << 1);

        value
    }
}

impl From<u32> for Status {
    fn from(value: u32) -> Self {
        let device:u8 = match (value >> 7) & 7 {
            1 => 0,
            2 => 1,
            4 => 2,
            0 => 0, // should this really happen ???
            _ => panic!("unhandled device num: {}", (value >> 7) & 7)
        };

        Status {
            connected:     (value & (1 << 13)) != 0,
            ext_interrupt: (value & (1 << 12)) != 0,
            device_select: device,
            exi_frequency: ((value >> 4) & 7) as u8,
            tc_interupt:   (value & (1 <<  3)) != 0,
            exi_interrupt: (value & (1 <<  1)) != 0
        }
    }
}

#[derive(Debug)]
pub enum TransferMode {
    IMM,
    DMA
}

impl Default for TransferMode {
    fn default() -> Self {
        TransferMode::IMM
    }
}

#[derive(Debug)]
pub enum TransferType {
    READ,
    WRITE,
    READWRITE
}

impl Default for TransferType {
    fn default() -> Self {
        TransferType::READ
    }
}

#[derive(Default, Debug)]
pub struct Control {
    pub transfer_length: u8, // IMM transfer length for write operations
    pub transfer_type: TransferType,
    pub transfer_mode: TransferMode,
    pub transfer_start: bool // Note: When an EXI DMA\IMM operation has been completed, the EXI Enable Bit will be reset to 0.
}

impl Control {
    pub fn as_u32(&self) -> u32 {
        let mut value = 0;

        value = value ^ ((self.transfer_length as u32) << 4);

        match self.transfer_type {
            TransferType::READ => value = value ^ (0 << 2),
            TransferType::WRITE => value = value ^ (1 << 2),
            TransferType::READWRITE => value = value ^ (1 << 3)
        }

        match self.transfer_mode {
            TransferMode::IMM => value = value ^ (0 << 1),
            TransferMode::DMA => value = value ^ (1 << 1)
        };

        value = value ^ (self.transfer_start as u32);

        value
    }
}

impl From<u32> for Control {
    fn from(value: u32) -> Self {
        let transfer_type = match (value >> 2) & 3 {
            0 => TransferType::READ,
            1 => TransferType::WRITE,
            2 => TransferType::READWRITE,
            _ => panic!("Unrecognized EXI transfer type.")
        };

        let transfer_mode = match (value >> 1) & 1 {
            0 => TransferMode::IMM,
            1 => TransferMode::DMA,
            _ => unreachable!()
        };

        Control {
            transfer_length: ((value >> 4) & 3) as u8,
            transfer_type:   transfer_type,
            transfer_mode:   transfer_mode,
            transfer_start:  (value & 1) != 0
        }
    }
}
