use crate::emu::Context;

// Writing anything here should cause a reset
// BS1 reads, then writes 0x1 here
const RESET_CODE: u32 = 0x24;
// BS1 does write 0x0245248A here, purpose unknown (dolphine-emu indicates the same)
const UNKNOWN: u32 = 0x30;

#[derive(Debug, Default)]
pub struct ProcessorInterface {
    unknown: u32,
}

pub fn read_u32(_ctx: &mut Context, register: u32) -> u32 {
    match register {
        RESET_CODE => {
            info!("Read PI:RESET_CODE");
            0
        }
        _ => {
            warn!("read_u32 unrecognized register {:#x}", register);
            0
        }
    }
}

pub fn write_u32(ctx: &mut Context, register: u32, val: u32) {
    match register {
        RESET_CODE => warn!("Write PI:RESET_CODE {:#x}", val),
        UNKNOWN => ctx.pi.unknown = val,
        _ => warn!("write_u32 unrecognized register {:#x}", register),
    }
}
