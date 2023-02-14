//use crate::video::cp;

use crate::Context;

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

//impl GpFifo {
//    pub fn reset(&mut self) {
//        self.pos = 0;
//        info!("GPFifo buffer reset");
//    }
//}

fn check_burst(ctx: &mut Context) {
    if ctx.gp_fifo.pos >= BURST_SIZE {
        let mut processed = 0;

        while ctx.gp_fifo.pos >= BURST_SIZE {
            ctx.mem.write(
                ctx.pi.fifo_write_pointer(),
                &ctx.gp_fifo.buff[processed..processed + BURST_SIZE],
            );

            if ctx.pi.fifo_write_pointer() == ctx.pi.fifo_end() {
                ctx.pi.set_fifo_write_pointer(ctx.pi.fifo_start());
            } else {
                ctx.pi
                    .set_fifo_write_pointer(ctx.pi.fifo_write_pointer() + BURST_SIZE as u32);
            }

            processed += BURST_SIZE;
            ctx.gp_fifo.pos -= BURST_SIZE;

            //cp::gather_pipe_burst(ctx);
        }

        if ctx.gp_fifo.pos > 0 {
            ctx.gp_fifo.buff.rotate_left(processed);
        }
    }
}

pub fn write_u8(ctx: &mut Context, val: u8) {
    ctx.gp_fifo.buff[ctx.gp_fifo.pos] = val;
    ctx.gp_fifo.pos += 1;

    check_burst(ctx);
}

pub fn write_u16(ctx: &mut Context, val: u16) {
    for x in val.to_be_bytes().iter() {
        ctx.gp_fifo.buff[ctx.gp_fifo.pos] = *x;
        ctx.gp_fifo.pos += 1;
    }

    check_burst(ctx);
}

pub fn write_u32(ctx: &mut Context, val: u32) {
    for x in val.to_be_bytes().iter() {
        ctx.gp_fifo.buff[ctx.gp_fifo.pos] = *x;
        ctx.gp_fifo.pos += 1;
    }

    check_burst(ctx);
}

pub fn write_u64(ctx: &mut Context, val: u64) {
    for x in val.to_be_bytes().iter() {
        ctx.gp_fifo.buff[ctx.gp_fifo.pos] = *x;
        ctx.gp_fifo.pos += 1;
    }

    check_burst(ctx);
}
