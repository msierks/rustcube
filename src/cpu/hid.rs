
#[derive(Debug, Default)]
pub struct Hid2 {

    // LSQE
    load_stored_quantized: bool,

    // WPE
    write_pipe: bool,

    // PSE
    pub paired_single: bool,

    // LCE
    locked_cache: bool,

    // DMAQL
    dma_queue_length: u8

    // DCHERR
    // DNCERR
    // DCMERR
    // DQOERR
    // DCHEE
    // DNCEE
    // DCMEE
    // DQOEE
}

impl Hid2 {
    pub fn as_u32(&self) -> u32 {
        let mut value = 0;

        value = value ^ ((self.load_stored_quantized as u32) << 31);
        value = value ^ ((self.write_pipe as u32)            << 30);
        value = value ^ ((self.paired_single as u32)         << 29);
        value = value ^ ((self.locked_cache as u32)          << 28);
        value = value ^ ((self.dma_queue_length as u32)      << 24);

        value
    }
}

impl From<u32> for Hid2 {
    fn from(value: u32) -> Self {
        Hid2 {
            load_stored_quantized: (value & (1 << 31)) != 0,
            write_pipe:            (value & (1 << 30)) != 0,
            paired_single:         (value & (1 << 29)) != 0,
            locked_cache:          (value & (1 << 28)) != 0,
            dma_queue_length:      (value & (15 << 24)) as u8,
        }
    }
}
