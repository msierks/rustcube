use byteorder::{ByteOrder, BigEndian};
use std::mem;

use super::memory::Ram;
use super::processor_interface::ProcessorInterface;

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

    fn check(&mut self, pi: &mut ProcessorInterface, ram: &mut Ram) {

        if self.count >= GATHER_PIPE_SIZE - 32 {

            // copy gather pipe into memory in 32 byte increments

            let size = (self.count / 32) * 32;

            ram.write_dma(pi.fifo_write_pointer, &self.gather_pipe[0..size - 1]);

            pi.fifo_write_pointer += size as u32;

            println!("gp_fifo: {} {:#x}", self.count, pi.fifo_write_pointer);

            let mut i = 0;
            let mut j = size;
            while j < self.count {
                self.gather_pipe[i] = self.gather_pipe[j - 1];

                i += 1;
                j += 1;
            }

            self.count -= size;
        }
    }

    pub fn write_u8(&mut self, val: u8, pi: &mut ProcessorInterface, ram: &mut Ram) {
        self.gather_pipe[self.count] = val;
        self.count += 1;
        self.check(pi, ram);
    }

    pub fn write_u32(&mut self, val: u32, pi: &mut ProcessorInterface, ram: &mut Ram) {
        BigEndian::write_u32(&mut self.gather_pipe[self.count ..], val);
        self.count += mem::size_of::<u32>();
        self.check(pi, ram);
    }

    pub fn write_u64(&mut self, val: u64, pi: &mut ProcessorInterface, ram: &mut Ram) {
        BigEndian::write_u64(&mut self.gather_pipe[self.count ..], val);
        self.count += mem::size_of::<u64>();
        self.check(pi, ram);
    }
}
