
pub fn bon(bo: u8, n: u8) -> u8 {
    (bo >> (4 - n)) & 1
}

// FIXME
pub fn convert_to_double(x: u32) -> u64 {
    panic!("FixMe: convert_to_double");
}

// FIXME
pub fn convert_to_single(x: u64) -> u32 {
    0
}

pub fn sign_ext_12(x: u16) -> i32 {
    if x & 0x800 != 0 {
        (x | 0xF000) as i32
    } else {
        x as i32
    }
}

// Note: A cast from a signed value widens with signed-extension
//       A cast from an unsigned value widens with zero-extension
pub fn sign_ext_16(x: u16) -> i32 {
    (x as i16) as i32
}

pub fn sign_ext_26(x: u32) -> i32 {
    if x & 0x2000000 != 0 {
        (x | 0xFC000000) as i32
    } else {
        x as i32
    }
}
