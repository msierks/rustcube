use byteorder::{ByteOrder, BigEndian};
use std::mem;

use super::memory::Ram;
use super::processor_interface::ProcessorInterface;
use super::command_processor::CommandProcessor;

const GATHER_PIPE_SIZE: usize = 128;
const GATHER_PIPE_BURST: usize = 32;

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

    pub fn reset(&mut self) {
        self.count = 0;
    }

    fn check(&mut self, cp: &mut CommandProcessor, pi: &mut ProcessorInterface, ram: &mut Ram) {
        if self.count >= GATHER_PIPE_BURST {
            // copy gather pipe into memory in 32 byte increments

            let size = (self.count / GATHER_PIPE_BURST) * GATHER_PIPE_BURST;

            let mut processed = 0;

            while processed < size {
                ram.write_dma(pi.fifo_write_pointer, &self.gather_pipe[processed..processed + GATHER_PIPE_BURST + 1]);

                cp.gather_pipe_burst(ram);

                pi.fifo_write_pointer += GATHER_PIPE_BURST as u32;
                processed += GATHER_PIPE_BURST;
            }

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

    pub fn write_u8(&mut self, val: u8, cp: &mut CommandProcessor, pi: &mut ProcessorInterface, ram: &mut Ram) {
        self.gather_pipe[self.count] = val;
        self.count += 1;
        self.check(cp, pi, ram);
    }

    pub fn write_u32(&mut self, val: u32, cp: &mut CommandProcessor, pi: &mut ProcessorInterface, ram: &mut Ram) {
        BigEndian::write_u32(&mut self.gather_pipe[self.count ..], val);
        self.count += mem::size_of::<u32>();
        self.check(cp, pi, ram);
    }

    pub fn write_u64(&mut self, val: u64, cp: &mut CommandProcessor, pi: &mut ProcessorInterface, ram: &mut Ram) {
        BigEndian::write_u64(&mut self.gather_pipe[self.count ..], val);
        self.count += mem::size_of::<u64>();
        self.check(cp, pi, ram);
    }
}
