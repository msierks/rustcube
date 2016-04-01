
// Integer Exception Register (XER)

#[derive(Debug, Default)]
pub struct IntegerExceptionRegister {
    // SO
    summary_overflow: bool,

    // OV
    overflow: bool,

    // CA
    carry: bool,

    byte_count: u8
}
