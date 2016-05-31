
// Integer Exception Register (XER)

#[derive(Debug, Default)]
pub struct IntegerExceptionRegister {
    // SO
    pub summary_overflow: bool,

    // OV
    overflow: bool,

    // CA
    pub carry: bool,

    byte_count: u8
}
