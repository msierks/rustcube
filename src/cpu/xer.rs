
// Integer Exception Register

#[derive(Debug, Default)]
pub struct Xer {
    // SO
    pub summary_overflow: bool,

    // OV
    overflow: bool,

    // CA
    pub carry: bool,

    byte_count: u8
}

impl Xer {

    pub fn as_u32(&self) -> u32 {
        let mut value = 0;

        value |= (self.summary_overflow as u32) << 31;
        value |= (self.overflow as u32)         << 30;
        value |= (self.carry as u32)            << 29;
        value |= (self.byte_count as u32) & 0x7F;

        value
    }

}
