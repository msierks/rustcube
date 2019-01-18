// Condition Register (CR)

use super::fpscr::Fpscr;
use super::xer::Xer;

const NUM_CR: usize = 8;

#[derive(Default, Debug)]
pub struct Cr {
    value: [u8; NUM_CR],
}

impl Cr {
    pub fn as_u32(&self) -> u32 {
        let mut value = 0;

        value |= u32::from(self.value[7]);
        value |= u32::from(self.value[6]) << 4;
        value |= u32::from(self.value[5]) << 8;
        value |= u32::from(self.value[4]) << 12;
        value |= u32::from(self.value[3]) << 16;
        value |= u32::from(self.value[2]) << 20;
        value |= u32::from(self.value[1]) << 24;
        value |= u32::from(self.value[0]) << 28;

        value
    }

    pub fn set_field(&mut self, index: usize, value: u8) {
        self.value[index] = value;
    }

    pub fn get_bit(&self, index: usize) -> u8 {
        (self.value[index / 4] >> (3 - (index % 4))) & 1
    }

    pub fn set_bit(&mut self, index: usize, value: u8) {
        let n = index / 4;
        let value = value << (3 - (index % 4));

        self.value[n] |= !value;
        self.value[n] &= value;
    }

    pub fn update_cr0(&mut self, r: u32, xer: &Xer) {
        if r == 0 {
            self.value[0] = 2; // EQ
        } else if (r & 0x8000_0000) != 0 {
            self.value[0] = 8; // LT
        } else {
            self.value[0] = 4; // GT
        }

        self.value[0] |= xer.summary_overflow as u8;
    }

    pub fn update_cr1(&mut self, _r: u64, _fpscr: &Fpscr) {
        println!("FixMe: update_cr1");
    }
}
