
const STATUS: u32 = 0x00;
const COVER:  u32 = 0x04;

#[derive(Debug, Default)]
pub struct DvdInterface {
    status_register: StatusRegister,
    cover_register: CoverRegister
}

impl DvdInterface {

    pub fn write_u32(&mut self, register: u32, val: u32) {
        println!("write dvd interface");
        match register {
            STATUS => self.status_register = val.into(),
            COVER => self.cover_register = val.into(),
            _ => panic!("unrecognized dvd interface register {:#x}", register)
        }
    }
}

#[derive(Debug, Default)]
struct StatusRegister {
    break_interrupt: bool,

    break_interrupt_mask: bool,

    transfer_interrupt: bool,

    transfer_interrupt_mask: bool,

    device_error_interrupt: bool,

    device_error_interrup_mask: bool,

    di_break: bool
}

impl From<u32> for StatusRegister {
    fn from(value: u32) -> Self {
        StatusRegister {
            break_interrupt:            (value & (1 <<  6)) != 0,
            break_interrupt_mask:       (value & (1 <<  5)) != 0,
            transfer_interrupt:         (value & (1 <<  4)) != 0,
            transfer_interrupt_mask:    (value & (1 <<  3)) != 0,
            device_error_interrupt:     (value & (1 <<  2)) != 0,
            device_error_interrup_mask: (value & (1 <<  1)) != 0,
            di_break:                   (value & 1) != 0
        }
    }
}

#[derive(Debug, Default)]
struct CoverRegister {
    interrupt_status: bool,

    interrupt_mask: bool,

    signal_state: bool
}

impl From<u32> for CoverRegister {
    fn from(value: u32) -> Self {
        CoverRegister {
            interrupt_status: (value & (1 <<  2)) != 0,
            interrupt_mask:   (value & (1 <<  1)) != 0,
            signal_state:     (value & 1) != 0
        }
    }
}
