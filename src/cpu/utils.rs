pub fn convert_to_double(v: u32) -> u64 {
    let x = v as u64;
    let mut exp = (x >> 23) & 0xFF;
    let mut frac = x & 0x007F_FFFF;

    // Normalize Operand
    if exp > 0 && exp < 255 {
        let y = (exp >> 7) ^ 0x1;
        let z = (y << 61) | (y << 60) | (y << 59);
        ((x & 0xC000_0000) << 32) | z | ((x & 0x3FFF_FFFF) << 29)
    // Denormalize Operand
    } else if exp == 0 && frac != 0 {
        exp = 1023 - 126;
        while (frac & 0x0080_0000) == 0 {
            frac <<= 1;
            exp -= 1;
        }

        ((x & 0x8000_0000) << 32) | (exp << 52) | ((frac & 0x007F_FFFF) << 29)
    // Infinity / QNaN / SNaN / Zero
    } else {
        let y = exp >> 7;
        let z = (y << 61) | (y << 60) | (y << 59);
        ((x & 0xC000_0000) << 32) | z | ((x & 0x3FFF_FFFF) << 29)
    }
}

pub fn convert_to_single(x: u64) -> u32 {
    let exp64 = ((x >> 52) & 0x7FF) as u32;

    // No Denormalization (includes Zero/ Infinity / NaN)
    if exp64 > 896 || x & 0x7FFF_FFFF == 0 {
        (((x >> 32) as u32) & 0xC000_0000) | (((x >> 29) as u32) & 0x3FFF_FFFF)
    // Denormalization
    } else if exp64 >= 874 {
        // TODO: simplify ???
        let mut exp = (exp64 as i16) - 1023;
        let mut frac = 0x8000_0000_0000_0000 | (x << 12);
        while exp < -126 {
            frac >>= 1;
            exp += 1;
        }
        (((x >> 32) & 0x8000_0000) | (frac >> 40)) as u32
    // Undefined
    } else {
        // According to dolphin, determined through hardware tests
        (((x >> 32) & 0xC000_0000) | ((x >> 29) & 0x3FFF_FFFF)) as u32
    }
}

pub fn sign_ext_12(x: u16) -> i32 {
    ((x as i32) << 20) >> 20
}

// Note: A cast from a signed value widens with signed-extension
//       A cast from an unsigned value widens with zero-extension
pub fn sign_ext_16(x: u16) -> i32 {
    i32::from(x as i16)
}

pub fn sign_ext_26(x: u32) -> i32 {
    ((x as i32) << 6) >> 6
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

#[cfg(test)]
mod tests {

    // TODO: expand on these test cases
    #[test]
    fn convert_to_double() {
        let test_values: [(f32, f64); 3] = [
            (0.0, 0.0),
            (1.0, 1.0),
            (1.1754942e-38, 1.1754942106924411e-38),
        ];

        for t in test_values.iter() {
            let result = f64::from_bits(super::convert_to_double(f32::to_bits(t.0)));

            assert_eq!(result, t.1);
        }
    }

    // TODO: expand on these test cases
    #[test]
    fn convert_to_single() {
        let test_values: [(f64, f32); 4] = [
            (0.0, 0.0),
            (1.0, 1.0),
            (4.484155085839414e-44, 4.3e-44),
            (1.4693679385492415e-39, 1.469368e-39),
        ];

        for t in test_values.iter() {
            let result = f32::from_bits(super::convert_to_single(f64::to_bits(t.0)));

            assert_eq!(result, t.1);
        }
    }

    #[test]
    fn sign_ext_12() {
        assert_eq!(super::sign_ext_12(0), 0);
        assert_eq!(super::sign_ext_12(1), 1);
        assert_eq!(super::sign_ext_12(0x7FF), 0x0000_07FF_i32);
        assert_eq!(super::sign_ext_12(0x800), 0xFFFF_F800_u32 as i32);
        assert_eq!(super::sign_ext_12(0xFFF), 0xFFFF_FFFF_u32 as i32);
        assert_eq!(super::sign_ext_12(0xFF0), 0xFFFF_FFF0_u32 as i32);
    }

    #[test]
    fn sign_ext_16() {
        assert_eq!(super::sign_ext_16(0), 0);
        assert_eq!(super::sign_ext_16(1), 1);
        assert_eq!(super::sign_ext_16(0x7FFF), 0x0000_7FFF_i32);
        assert_eq!(super::sign_ext_16(0x8000), 0xFFFF_8000_u32 as i32);
        assert_eq!(super::sign_ext_16(0xFFFF), 0xFFFF_FFFF_u32 as i32);
        assert_eq!(super::sign_ext_16(0xFFF0), 0xFFFF_FFF0_u32 as i32);
    }

    #[test]
    fn sign_ext_26() {
        assert_eq!(super::sign_ext_26(0), 0);
        assert_eq!(super::sign_ext_26(1), 1);
        assert_eq!(super::sign_ext_26(0x01FF_FFFF), 0x01FF_FFFF_i32);
        assert_eq!(super::sign_ext_26(0x0200_0000), 0xFE00_0000_u32 as i32);
        assert_eq!(super::sign_ext_26(0x03FF_FFFF), 0xFFFF_FFFF_u32 as i32);
        assert_eq!(super::sign_ext_26(0x03FF_FFF0), 0xFFFF_FFF0_u32 as i32);
    }
}
