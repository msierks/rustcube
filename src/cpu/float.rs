pub const QUANTIZE_FLOAT: u32 = 0; // Single-precision floating-point (no conversion)
pub const QUANTIZE_U8: u32 = 4; // unsigned 8 bit integer
pub const QUANTIZE_U16: u32 = 5; // unsigned 16 bit integer
pub const QUANTIZE_I8: u32 = 6; // signed 8 bit integer
pub const QUANTIZE_I16: u32 = 7; // signed 16 bit integer

// Paired-single store scale
const QUANTIZE_TABLE: [f32; 64] = [
    (1_u32 << 0) as f32,
    (1_u32 << 1) as f32,
    (1_u32 << 2) as f32,
    (1_u32 << 3) as f32,
    (1_u32 << 4) as f32,
    (1_u32 << 5) as f32,
    (1_u32 << 6) as f32,
    (1_u32 << 7) as f32,
    (1_u32 << 8) as f32,
    (1_u32 << 9) as f32,
    (1_u32 << 10) as f32,
    (1_u32 << 11) as f32,
    (1_u32 << 12) as f32,
    (1_u32 << 13) as f32,
    (1_u32 << 14) as f32,
    (1_u32 << 15) as f32,
    (1_u32 << 16) as f32,
    (1_u32 << 17) as f32,
    (1_u32 << 18) as f32,
    (1_u32 << 19) as f32,
    (1_u32 << 20) as f32,
    (1_u32 << 21) as f32,
    (1_u32 << 22) as f32,
    (1_u32 << 23) as f32,
    (1_u32 << 24) as f32,
    (1_u32 << 25) as f32,
    (1_u32 << 26) as f32,
    (1_u32 << 27) as f32,
    (1_u32 << 28) as f32,
    (1_u32 << 29) as f32,
    (1_u32 << 30) as f32,
    (1_u32 << 31) as f32,
    1.0 / (1_u64 << 32) as f32,
    1.0 / (1_u32 << 31) as f32,
    1.0 / (1_u32 << 30) as f32,
    1.0 / (1_u32 << 29) as f32,
    1.0 / (1_u32 << 28) as f32,
    1.0 / (1_u32 << 27) as f32,
    1.0 / (1_u32 << 26) as f32,
    1.0 / (1_u32 << 25) as f32,
    1.0 / (1_u32 << 24) as f32,
    1.0 / (1_u32 << 23) as f32,
    1.0 / (1_u32 << 22) as f32,
    1.0 / (1_u32 << 21) as f32,
    1.0 / (1_u32 << 20) as f32,
    1.0 / (1_u32 << 19) as f32,
    1.0 / (1_u32 << 18) as f32,
    1.0 / (1_u32 << 17) as f32,
    1.0 / (1_u32 << 16) as f32,
    1.0 / (1_u32 << 15) as f32,
    1.0 / (1_u32 << 14) as f32,
    1.0 / (1_u32 << 13) as f32,
    1.0 / (1_u32 << 12) as f32,
    1.0 / (1_u32 << 11) as f32,
    1.0 / (1_u32 << 10) as f32,
    1.0 / (1_u32 << 9) as f32,
    1.0 / (1_u32 << 8) as f32,
    1.0 / (1_u32 << 7) as f32,
    1.0 / (1_u32 << 6) as f32,
    1.0 / (1_u32 << 5) as f32,
    1.0 / (1_u32 << 4) as f32,
    1.0 / (1_u32 << 3) as f32,
    1.0 / (1_u32 << 2) as f32,
    1.0 / (1_u32 << 1) as f32,
];

// paired-single load scale
const DEQUANTIZE_TABLE: [f32; 64] = [
    1.0 / (1_u32 << 0) as f32,
    1.0 / (1_u32 << 1) as f32,
    1.0 / (1_u32 << 2) as f32,
    1.0 / (1_u32 << 3) as f32,
    1.0 / (1_u32 << 4) as f32,
    1.0 / (1_u32 << 5) as f32,
    1.0 / (1_u32 << 6) as f32,
    1.0 / (1_u32 << 7) as f32,
    1.0 / (1_u32 << 8) as f32,
    1.0 / (1_u32 << 9) as f32,
    1.0 / (1_u32 << 10) as f32,
    1.0 / (1_u32 << 11) as f32,
    1.0 / (1_u32 << 12) as f32,
    1.0 / (1_u32 << 13) as f32,
    1.0 / (1_u32 << 14) as f32,
    1.0 / (1_u32 << 15) as f32,
    1.0 / (1_u32 << 16) as f32,
    1.0 / (1_u32 << 17) as f32,
    1.0 / (1_u32 << 18) as f32,
    1.0 / (1_u32 << 19) as f32,
    1.0 / (1_u32 << 20) as f32,
    1.0 / (1_u32 << 21) as f32,
    1.0 / (1_u32 << 22) as f32,
    1.0 / (1_u32 << 23) as f32,
    1.0 / (1_u32 << 24) as f32,
    1.0 / (1_u32 << 25) as f32,
    1.0 / (1_u32 << 26) as f32,
    1.0 / (1_u32 << 27) as f32,
    1.0 / (1_u32 << 28) as f32,
    1.0 / (1_u32 << 29) as f32,
    1.0 / (1_u32 << 30) as f32,
    1.0 / (1_u32 << 31) as f32,
    (1_u64 << 32) as f32,
    (1_u32 << 31) as f32,
    (1_u32 << 30) as f32,
    (1_u32 << 29) as f32,
    (1_u32 << 28) as f32,
    (1_u32 << 27) as f32,
    (1_u32 << 26) as f32,
    (1_u32 << 25) as f32,
    (1_u32 << 24) as f32,
    (1_u32 << 23) as f32,
    (1_u32 << 22) as f32,
    (1_u32 << 21) as f32,
    (1_u32 << 20) as f32,
    (1_u32 << 19) as f32,
    (1_u32 << 18) as f32,
    (1_u32 << 17) as f32,
    (1_u32 << 16) as f32,
    (1_u32 << 15) as f32,
    (1_u32 << 14) as f32,
    (1_u32 << 13) as f32,
    (1_u32 << 12) as f32,
    (1_u32 << 11) as f32,
    (1_u32 << 10) as f32,
    (1_u32 << 9) as f32,
    (1_u32 << 8) as f32,
    (1_u32 << 7) as f32,
    (1_u32 << 6) as f32,
    (1_u32 << 5) as f32,
    (1_u32 << 4) as f32,
    (1_u32 << 3) as f32,
    (1_u32 << 2) as f32,
    (1_u32 << 1) as f32,
];

pub fn quantize(mut value: f32, st_type: u32, st_scale: u32) -> u32 {
    value *= QUANTIZE_TABLE[st_scale as usize];

    match st_type {
        QUANTIZE_FLOAT => f32::to_bits(value),
        QUANTIZE_U8 => (value.clamp(u8::MIN as f32, u8::MAX as f32) as u8) as u32,
        QUANTIZE_U16 => (value.clamp(u16::MIN as f32, u16::MAX as f32) as u16) as u32,
        QUANTIZE_I8 => ((value.clamp(i8::MIN as f32, i8::MAX as f32) as i8) as i32) as u32,
        QUANTIZE_I16 => ((value.clamp(i16::MIN as f32, i16::MAX as f32) as i16) as i32) as u32,
        _ => {
            warn!("Unrecognized quantize type {st_type}.");
            f32::to_bits(value)
        }
    }
}

pub fn dequantize(value: u32, ld_type: u32, ld_scale: u32) -> f32 {
    let result = match ld_type {
        QUANTIZE_FLOAT => f32::from_bits(value),
        QUANTIZE_U8 => (value as u8) as f32,
        QUANTIZE_U16 => (value as u16) as f32,
        QUANTIZE_I8 => (value as i8) as f32,
        QUANTIZE_I16 => (value as i16) as f32,
        _ => {
            warn!("unrecognized dequantize unknown type {ld_type}.");
            f32::from_bits(value)
        }
    };

    result * DEQUANTIZE_TABLE[ld_scale as usize]
}

pub trait Nan {
    fn is_snan(&self) -> bool;
    #[allow(dead_code)]
    fn is_qnan(&self) -> bool;
}

impl Nan for f32 {
    fn is_snan(&self) -> bool {
        let v = f32::to_bits(*self);
        v & 0x7FC0_0000 == 0x7F80_0000 && v & 0x003F_FFFF != 0
    }

    fn is_qnan(&self) -> bool {
        let v = f32::to_bits(*self);
        v & 0x7FC0_0000 == 0x7FC0_0000
    }
}

impl Nan for f64 {
    fn is_snan(&self) -> bool {
        let v = f64::to_bits(*self);
        v & 0x7FF8_0000_0000_0000 == 0x7FF0_0000_0000_0000 && v & 0x000F_FFFF_FFFF_FFFF != 0
    }

    fn is_qnan(&self) -> bool {
        let v = f64::to_bits(*self);
        v & 0x7FF8_0000_0000_0000 == 0x7FF8_0000_0000_0000
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f32_is_snan() {
        let snan = f32::from_bits(0xFF800001);

        assert!(snan.is_nan());
        assert!(snan.is_snan());
        assert!(!snan.is_qnan());

        let snan = f32::from_bits(0xFF800301);

        assert!(snan.is_nan());
        assert!(snan.is_snan());
        assert!(!snan.is_qnan());
    }

    #[test]
    fn f64_is_snan() {
        let snan = f64::from_bits(0x7FF0000000000001);

        assert!(snan.is_nan());
        assert!(snan.is_snan());
        assert!(!snan.is_qnan());

        let snan = f64::from_bits(0x7FF0000000020001);

        assert!(snan.is_nan());
        assert!(snan.is_snan());
        assert!(!snan.is_qnan());
    }

    #[test]
    fn f64_is_qnan() {
        let qnan = f64::from_bits(0x7FF8000000000001);

        assert!(qnan.is_nan());
        assert!(!qnan.is_snan());
        assert!(qnan.is_qnan());

        let qnan = f64::from_bits(0x7FF8000000020001);

        assert!(qnan.is_nan());
        assert!(!qnan.is_snan());
        assert!(qnan.is_qnan());
    }

    #[test]
    fn f632_is_qnan() {
        let qnan = f32::from_bits(0xFFC00001);

        assert!(qnan.is_nan());
        assert!(!qnan.is_snan());
        assert!(qnan.is_qnan());

        let qnan = f32::from_bits(0xFFC00301);

        assert!(qnan.is_nan());
        assert!(!qnan.is_snan());
        assert!(qnan.is_qnan());
    }
}
