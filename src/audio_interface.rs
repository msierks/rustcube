
const CONTROL_STATUS: u32 = 0x00;

#[derive(Debug, Default)]
pub struct AudioInterface {
    control_register: ControlRegister
}

impl AudioInterface {

    pub fn read_u32(&self, register: u32) -> u32 {
        match register {
            CONTROL_STATUS => self.control_register.as_u32(),
            _ => panic!("unrecognized ai register {:#x}", register)
        }
    }

    pub fn write_u32(&mut self, register: u32, val: u32) {
        match register {
            CONTROL_STATUS => self.control_register = val.into(),
            _ => panic!("unrecognized ai register {:#x}", register)
        }
    }

}

#[derive(Debug, Default)]
struct ControlRegister {
    dsp_sample_rate: bool,

    sample_count_reset: bool,

    ai_interrupt_valid: bool,

    ai_interrupt: bool,

    ai_interrupt_mask: bool,

    auxiliary_frequency: bool,

    playing_status: bool
}

impl ControlRegister {

    pub fn as_u32(&self) -> u32 {
        let mut value = 0;

        value ^= (self.dsp_sample_rate as u32)     <<  6;
        value ^= (self.sample_count_reset as u32)  <<  5;
        value ^= (self.ai_interrupt_valid as u32)  <<  4;
        value ^= (self.ai_interrupt as u32)        <<  3;
        value ^= (self.ai_interrupt_mask as u32)   <<  2;
        value ^= (self.auxiliary_frequency as u32) <<  1;
        value ^=  self.playing_status as u32;

        value
    }

}

impl From<u32> for ControlRegister {
    fn from(value: u32) -> Self {
        ControlRegister {
            dsp_sample_rate:     (value & (1 <<  6)) != 0,
            sample_count_reset:  (value & (1 <<  5)) != 0,
            ai_interrupt_valid:  (value & (1 <<  4)) != 0,
            ai_interrupt:        (value & (1 <<  3)) != 0,
            ai_interrupt_mask:   (value & (1 <<  2)) != 0,
            auxiliary_frequency: (value & (1 <<  1)) != 0,
            playing_status:      (value & 1) != 0
        }
    }
}
