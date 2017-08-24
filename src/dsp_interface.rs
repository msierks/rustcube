
const MAILBOX_IN_HIGH: u32     = 0x00;
const MAILBOX_IN_LOW: u32      = 0x02;
const MAILBOX_OUT_HIGH: u32    = 0x04;
const MAILBOX_OUT_LOW: u32     = 0x06;
const CONTROL_STATUS: u32      = 0x0A;
const AR_SIZE: u32             = 0x12;
const AR_MODE: u32             = 0x16;
const AR_REFRESH: u32          = 0x1A;
const AR_DMA_MMAADDR_HIGH: u32 = 0x20;
const AR_DMA_MMAADDR_LOW: u32  = 0x22;
const AR_DMA_ARADDR_HIGH: u32  = 0x24;
const AR_DMA_ARADDR_LOW: u32   = 0x26;
const AR_DMA_SIZE_HIGH: u32    = 0x28;
const AR_DMA_SIZE_LOW: u32     = 0x2A;

#[derive(Debug, Default)]
pub struct DspInterface {
    mailbox_high: u16,
    mailbox_low: u16,
    control_register: ControlRegister,
    ar_size: u16,
    ar_refresh: u16,
    ar_mma_addr_high: u16,
    ar_mma_addr_low: u16,
    ar_ar_addr_high: u16,
    ar_ar_addr_low: u16,
    ar_dma_size_high: u16,
    ar_dma_size_low: u16
}

impl DspInterface {

    pub fn read_u16(&mut self, register: u32) -> u16 {
        println!("dsp read register {}", register);

        match register {
            MAILBOX_OUT_HIGH => {
                let val = self.mailbox_high;
                self.mailbox_high = 0;
                val
            },
            MAILBOX_OUT_LOW => self.mailbox_low,
            CONTROL_STATUS => self.control_register.as_u16(),
            AR_SIZE => self.ar_size,
            AR_REFRESH => self.ar_refresh,
            _ => {
                println!("Warn: unrecognized dsp register {:#x}", register);
                0
            }
        }
    }

    pub fn write_u16(&mut self, register: u32, val: u16) {
        match register {
            MAILBOX_IN_HIGH => {
                self.mailbox_high = 0x8000
            },
            MAILBOX_IN_LOW => {
                self.mailbox_high = 0x0000; // check if this is necessary
                self.mailbox_low = val
            },
            CONTROL_STATUS => {
                self.control_register = val.into();
                self.control_register.dsp_reset = false;
            },
            AR_SIZE => self.ar_size = val,
            AR_MODE => panic!("FixMe: write_u16 dsp register {:#x}", register),
            AR_REFRESH => self.ar_refresh = val,
            AR_DMA_MMAADDR_HIGH => self.ar_mma_addr_high = val,
            AR_DMA_MMAADDR_LOW => self.ar_mma_addr_low = val,
            AR_DMA_ARADDR_HIGH => self.ar_ar_addr_high = val,
            AR_DMA_ARADDR_LOW => self.ar_ar_addr_low = val,
            AR_DMA_SIZE_HIGH => self.ar_dma_size_high = val,
            AR_DMA_SIZE_LOW => {
                self.mailbox_high = 0x8000;
                self.ar_dma_size_low = val;
            },
            _ => println!("Warn: unrecognized dsp register {:#x}", register)
        }
    }
}

#[derive(Debug, Default)]
struct ControlRegister {
    dsp_init: bool,

    init_code: bool,

    dma_state: bool,

    dsp_interrupt_mask: bool,

    dsp_interrupt: bool,

    aram_interrupt_mask: bool,

    aram_interrupt: bool,

    ai_interrupt_mask: bool,

    ai_interrupt: bool,

    dsp_halt: bool,

    dsp_interrupt_assert: bool,
 
    dsp_reset: bool
}

impl ControlRegister {

    pub fn as_u16(&self) -> u16 {
        let mut value = 0;

        value |= (self.dsp_init as u16)             << 11;
        value |= (self.init_code as u16)            << 10;
        value |= (self.dma_state as u16)            <<  9;
        value |= (self.dsp_interrupt_mask as u16)   <<  8;
        value |= (self.dsp_interrupt as u16)        <<  7;
        value |= (self.aram_interrupt_mask as u16)  <<  6;
        value |= (self.aram_interrupt as u16)       <<  5;
        value |= (self.ai_interrupt_mask as u16)    <<  4;
        value |= (self.ai_interrupt as u16)         <<  3;
        value |= (self.dsp_halt as u16)             <<  2;
        value |= (self.dsp_interrupt_assert as u16) <<  1;
        value |=  self.dsp_reset as u16;

        value
    }

}

impl From<u16> for ControlRegister {
    fn from(value: u16) -> Self {
        ControlRegister {
            dsp_init:             (value & (1 << 11)) != 0,
            init_code:            (value & (1 << 10)) != 0,
            dma_state:            (value & (1 <<  9)) != 0,
            dsp_interrupt_mask:   (value & (1 <<  8)) != 0,
            dsp_interrupt:        (value & (1 <<  7)) != 0,
            aram_interrupt_mask:  (value & (1 <<  6)) != 0,
            aram_interrupt:       (value & (1 <<  5)) != 0,
            ai_interrupt_mask:    (value & (1 <<  4)) != 0,
            ai_interrupt:         (value & (1 <<  3)) != 0,
            dsp_halt:             (value & (1 <<  2)) != 0,
            dsp_interrupt_assert: (value & (1 <<  1)) != 0,
            dsp_reset:            (value & 1) != 0
        }
    }
}
