use byteorder::{ByteOrder, BigEndian};
use std::mem;

const GATHER_PIPE_SIZE: usize = 128;

pub struct GPFifo {
    gather_pipe: [u8; GATHER_PIPE_SIZE],
    count: usize
}

impl GPFifo {
    pub fn new() -> Self {
        GPFifo {
            gather_pipe: [0; GATHER_PIPE_SIZE],
            count: 0
        }
    }

    fn reset(&mut self) {
        self.count = 0;
    }

    fn check(&mut self) {
        if self.count >= GATHER_PIPE_SIZE - 32 {
            println!("FixMe: GPFifo check stub");
            self.count = 0;
        }
    }

    pub fn write_u8(&mut self, val: u8) {
        self.gather_pipe[self.count] = val;
        self.count += 1;
        self.check();
    }

    pub fn write_u32(&mut self, val: u32) {
        BigEndian::write_u32(&mut self.gather_pipe[self.count ..], val);
        self.count += mem::size_of::<u32>();
        self.check();
    }
}
