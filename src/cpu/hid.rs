#[derive(Debug, Default)]
pub struct Hid2 {
    // LSQE
    pub load_stored_quantized: bool,

    // WPE
    write_pipe: bool,

    // PSE
    pub paired_single: bool,

    // LCE
    locked_cache: bool,

    // DMAQL
    dma_queue_length: u8, // DCHERR
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

        value |= u32::from(self.load_stored_quantized) << 31;
        value |= u32::from(self.write_pipe) << 30;
        value |= u32::from(self.paired_single) << 29;
        value |= u32::from(self.locked_cache) << 28;
        value |= u32::from(self.dma_queue_length) << 24;

        value
    }
}

impl From<u32> for Hid2 {
    fn from(value: u32) -> Self {
        Hid2 {
            load_stored_quantized: (value & (1 << 31)) != 0,
            write_pipe: (value & (1 << 30)) != 0,
            paired_single: (value & (1 << 29)) != 0,
            locked_cache: (value & (1 << 28)) != 0,
            dma_queue_length: ((value >> 24) & 15) as u8,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Hid2;

    #[test]
    fn u32_conversion() {
        let hid2: Hid2 = 0xA9000000.into();

        assert!(hid2.load_stored_quantized);
        assert!(!hid2.write_pipe);
        assert!(hid2.paired_single);
        assert!(!hid2.write_pipe);
        assert_eq!(9, hid2.dma_queue_length);

        assert_eq!(0xA9000000, hid2.as_u32());
    }
}
