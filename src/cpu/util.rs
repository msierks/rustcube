pub fn bon(bo: u8, n: u8) -> u8 {
    (bo >> (4 - n)) & 1
}

// FIXME
pub fn convert_to_double(_x: u32) -> u64 {
    panic!("FixMe: convert_to_double");
}

// FIXME
pub fn convert_to_single(_x: u64) -> u32 {
    0
}

pub fn sign_ext_12(x: u16) -> i32 {
    if x & 0x800 != 0 {
        i32::from(x | 0xF000)
    } else {
        i32::from(x)
    }
}

// Note: A cast from a signed value widens with signed-extension
//       A cast from an unsigned value widens with zero-extension
pub fn sign_ext_16(x: u16) -> i32 {
    i32::from(x as i16)
}

pub fn sign_ext_26(x: u32) -> i32 {
    if x & 0x0200_0000 != 0 {
        (x | 0xFC00_0000) as i32
    } else {
        x as i32
    }
}
