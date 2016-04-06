
pub struct MemoryInterface;

impl MemoryInterface {

    pub fn new() -> MemoryInterface {
        MemoryInterface
    }

    pub fn read_u32(&self, register: u32) -> u32 {
        0
    }

    pub fn write_u32(&mut self, register: u32, val: u32) {

    }
}
