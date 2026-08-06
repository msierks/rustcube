use crate::{
    bus::Bus,
    cpu::CpuState,
    hw::mmio::{Mmio, MmioDevice},
    video::cp::CommandProcessor,
};

pub const BURST_SIZE: usize = 32;
const BUFFER_SIZE: usize = 128;

#[derive(Debug)]
pub struct GpFifo {
    buff: [u8; BUFFER_SIZE],
    pos: usize,
}

impl Default for GpFifo {
    fn default() -> Self {
        GpFifo {
            buff: [0; BUFFER_SIZE],
            pos: 0,
        }
    }
}

impl MmioDevice for GpFifo {
    const BASE_ADDR: u32 = 0x0C00_8000;

    fn register_mmio(mmio: &mut Mmio) {
        for offset in 0..BURST_SIZE as u32 {
            let addr = Self::BASE_ADDR + offset;

            mmio.register_write_u8(addr, |bus, cpu_state, _, val| {
                bus.gp_fifo.buff[bus.gp_fifo.pos] = val;
                bus.gp_fifo.pos += 1;
                Self::check_burst(bus, cpu_state);
            });

            if offset % 2 == 0 {
                mmio.register_write_u16(addr, |bus, cpu_state, _, val| {
                    for x in val.to_be_bytes().iter() {
                        bus.gp_fifo.buff[bus.gp_fifo.pos] = *x;
                        bus.gp_fifo.pos += 1;
                    }
                    Self::check_burst(bus, cpu_state);
                });
            }

            if offset % 4 == 0 {
                mmio.register_write_u32(addr, |bus, cpu_state, _, val| {
                    for x in val.to_be_bytes().iter() {
                        bus.gp_fifo.buff[bus.gp_fifo.pos] = *x;
                        bus.gp_fifo.pos += 1;
                    }
                    Self::check_burst(bus, cpu_state);
                });
            }
        }
    }
}

impl GpFifo {
    fn check_burst(bus: &mut Bus, cpu_state: &mut CpuState) {
        if bus.gp_fifo.pos >= BURST_SIZE {
            let mut processed = 0;

            while bus.gp_fifo.pos >= BURST_SIZE {
                bus.memory.write_bytes(
                    bus.pi.fifo_write_pointer(),
                    &bus.gp_fifo.buff[processed..processed + BURST_SIZE],
                );

                if bus.pi.fifo_write_pointer() == bus.pi.fifo_end() {
                    bus.pi.set_fifo_write_pointer(bus.pi.fifo_start());
                } else {
                    bus.pi
                        .set_fifo_write_pointer(bus.pi.fifo_write_pointer() + BURST_SIZE as u32);
                }

                processed += BURST_SIZE;
                bus.gp_fifo.pos -= BURST_SIZE;

                CommandProcessor::gather_pipe_burst(bus, cpu_state);
            }

            bus.gp_fifo.buff.rotate_left(processed);
        }
    }
}
