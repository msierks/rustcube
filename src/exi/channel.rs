use super::device::Device;

const NUM_DEVICES: usize = 3;

pub struct Channel {
    // status register
    pub status: Status,

    // control register
    pub control: Control,

    pub dma_address: u32,

    pub dma_length: u32,

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
    pub exi_device: u8,
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
            exi_device:     device,
            exi_frequency: ((value >> 4) & 7) as u8,
            tc_interupt:   (value & (1 <<  3)) != 0,
            exi_interrupt: (value & (1 <<  1)) != 0
        }
    }
}

#[derive(Debug)]
pub enum TransferSelection {
    IMM,
    DMA
}

impl Default for TransferSelection {
    fn default() -> Self {
        TransferSelection::IMM
    }
}

impl From<u32> for TransferSelection {
    fn from(value: u32) -> Self {
        if (value & (1 << 1)) != 0 {
            TransferSelection::DMA
        } else {
            TransferSelection::IMM
        }
    }
}

#[derive(Default, Debug)]
pub struct Control {
    transfer_length: u32,
    transfer_type: u8,
    pub transfer_selection: TransferSelection,
    pub enabled: bool // Note: When an EXI DMA\IMM operation has been completed, the EXI Enable Bit will be reset to 0.
}

impl Control {
    pub fn as_u32(&self) -> u32 {
        let mut value = 0;

        value = value ^ ((self.transfer_length as u32)  << 4);
        value = value ^ ((self.transfer_type as u32)  << 2);

        match self.transfer_selection {
            TransferSelection::DMA => value = value ^ (1 << 1),
            TransferSelection::IMM => value = value ^ (0 << 1)
        };

        value = value ^ (self.enabled as u32);

        value
    }
}

impl From<u32> for Control {
    fn from(value: u32) -> Self {
        Control {
            transfer_length:    (value >> 4),
            transfer_type:      ((value >> 2) & 3) as u8,
            transfer_selection: value.into(),
            enabled:            (value & 1) != 0
        }
    }
}
