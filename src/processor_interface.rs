
const INTERRUPT_CAUSE:    u32 = 0x00;
const INTERRUPT_MASK:     u32 = 0x04;
const FIFO_BASE_START:    u32 = 0x0C;
const FIFO_BASE_END:      u32 = 0x10;
const FIFO_WRITE_POINTER: u32 = 0x14;
const RESET_CODE:         u32 = 0x24;
const REVISION:           u32 = 0x2C;
const UNKNOWN:            u32 = 0x30;

pub struct ProcessorInterface {
    interrupt_mask: u32,
    reset_code: u32,
    revision: u32
}

impl ProcessorInterface {

    pub fn new() -> ProcessorInterface {
        ProcessorInterface {
            interrupt_mask: 0,
            reset_code: 0,
            revision: 0x246500B1 // revision C
        }
    }
 
    pub fn read_u32(&self, register: u32) -> u32 {
        match register {
            RESET_CODE => self.reset_code,
            REVISION => self.revision,
            _ => panic!("unrecognized pi register {:#x}", register)
        }
    }

    pub fn write_u32(&mut self, register: u32, val: u32) {
        match register {
            INTERRUPT_CAUSE => panic!("PI: Interrupt {}", val),
            INTERRUPT_MASK => self.interrupt_mask = val,
            FIFO_BASE_START => println!("FixMe: pi write: FIFO_BASE_START {}", val),
            FIFO_BASE_END => println!("FixMe: pi write: FIFO_BASE_END {}", val),
            FIFO_WRITE_POINTER => println!("FixMe: pi write: FIFO_WRITE_POINTER {}", val),
            RESET_CODE => self.reset_code = val,
            UNKNOWN => {}, // ignore value written to unknown reg
            _ => panic!("unrecognized pi register {:#x} {:#x}", register, val)
        }
    }
}
