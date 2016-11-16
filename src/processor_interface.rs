
const INTERRUPT_CAUSE:    u32 = 0x00;
const INTERRUPT_MASK:     u32 = 0x04;
const FIFO_BASE_START:    u32 = 0x0C;
const FIFO_BASE_END:      u32 = 0x10;
const FIFO_WRITE_POINTER: u32 = 0x14;
const RESET_CODE:         u32 = 0x24;
const REVISION:           u32 = 0x2C;
const UNKNOWN:            u32 = 0x30;

#[derive(Debug)]
pub struct ProcessorInterface {
    interrupt_mask: u32,
    revision: u32,
    pub fifo_base_start: u32,
    pub fifo_base_end: u32,
    pub fifo_write_pointer: u32,
    reset_code: u32
}

impl ProcessorInterface {

    pub fn new() -> ProcessorInterface {
        ProcessorInterface {
            interrupt_mask: 0,
            revision: 0x246500B1, // revision C
            fifo_base_start: 0,
            fifo_base_end: 0,
            fifo_write_pointer: 0,
            reset_code: 0
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
            FIFO_BASE_START => self.fifo_base_start = val,
            FIFO_BASE_END => self.fifo_base_end = val,
            FIFO_WRITE_POINTER => self.fifo_write_pointer = val,
            RESET_CODE => self.reset_code = val,
            UNKNOWN => {}, // ignore value written to unknown reg
            _ => panic!("unrecognized pi register {:#x} {:#x}", register, val)
        }
    }
}
