
use num::FromPrimitive;

const INTERRUPT_CAUSE:    u32 = 0x00;
const INTERRUPT_MASK:     u32 = 0x04;
const FIFO_BASE_START:    u32 = 0x0C;
const FIFO_BASE_END:      u32 = 0x10;
const FIFO_WRITE_POINTER: u32 = 0x14;
const RESET_CODE:         u32 = 0x24;
const REVISION:           u32 = 0x2C;
const UNKNOWN:            u32 = 0x30;

enum_from_primitive! {
    #[derive(Debug)]
    enum Interrupt {
        UNKNOWN   = 0,
        ERROR     = 0x1,
        RSW       = 0x2,
        DI        = 0x4,
        SI        = 0x8,
        EXI       = 0x10,
        AI        = 0x20,
        DSP       = 0x40,
        MEM       = 0x80,
        VI        = 0x100,
        PE_TOKEN  = 0x200,
        PE_FINISH = 0x400,
        CP        = 0x800,
        DEBUG     = 0x1000,
        HSP       = 0x2000,
        RSWST     = 0x10000
    }
}

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
        println!("PI: read {:#x}", register);
        match register {
            RESET_CODE => self.reset_code,
            REVISION => self.revision,
            _ => panic!("unrecognized pi register {:#x}", register)
        }
    }

    pub fn write_u32(&mut self, register: u32, val: u32) {
        println!("PI: write {:#x} {:}", register, val);
        match register {
            INTERRUPT_CAUSE => println!("PI: Interrupt {} {:#?}", val, Interrupt::from_u32(val).unwrap_or_else(|| Interrupt::UNKNOWN)),
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
