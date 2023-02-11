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

// TODO: Potential performance improvement by placing all mask combinations in array
pub fn mask(mb: u8, me: u8) -> u32 {
    let mut mask: u32 = 0xFFFF_FFFF >> mb;

    if me >= 31 {
        mask ^= 0;
    } else {
        mask ^= 0xFFFF_FFFF >> (me + 1)
    };

    if me < mb {
        !mask
    } else {
        mask
    }
}

/// Helper to check if operation results in an overflow. This is determined by checking if both
/// operands signs bits are the same but the results sign bit is different.
///
/// Note: Overflow flag is only relavent to signed arithmetic
pub fn check_overflowed(a: u32, b: u32, result: u32) -> bool {
    ((a ^ result) & (b ^ result)) >> 31 != 0
}
