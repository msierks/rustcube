// Integer Exception Register

#[derive(Debug, Default)]
pub struct Xer {
    // SO
    pub summary_overflow: bool,

    // OV
    overflow: bool,

    // CA
    pub carry: bool,

    byte_count: u8,
}

impl Xer {
    pub fn as_u32(&self) -> u32 {
        let mut value = 0;

        value |= u32::from(self.summary_overflow) << 31;
        value |= u32::from(self.overflow) << 30;
        value |= u32::from(self.carry) << 29;
        value |= u32::from(self.byte_count) & 0x7F;

        value
    }
}
