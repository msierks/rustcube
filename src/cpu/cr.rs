
// Condition Register (CR)

use super::fpscr::Fpscr;
use super::xer::Xer;

const NUM_CR : usize = 8;

#[derive(Default, Debug)]
pub struct Cr {
    value: [u8; NUM_CR]
}

impl Cr {

    pub fn as_u32(&self) -> u32 {
        let mut value = 0;

        value |=  self.value[7] as u32;
        value |= (self.value[6] as u32) << 4;
        value |= (self.value[5] as u32) << 8;
        value |= (self.value[4] as u32) << 12;
        value |= (self.value[3] as u32) << 16;
        value |= (self.value[2] as u32) << 20;
        value |= (self.value[1] as u32) << 24;
        value |= (self.value[0] as u32) << 28;

        value
    }

    pub fn set_field(&mut self, index: usize, value: u8) {
        self.value[index] = value;
    }

    pub fn get_bit(&self, index: usize) -> u8 {
        (self.value[index / 4] >> (3 - (index % 4))) & 1
    }

    pub fn set_bit(&mut self, index: usize, value: u8) {
        let n     = index / 4;
        let value = value << (3 - (index % 4));

        self.value[n] |= !value;
        self.value[n] &= value;
    }

    pub fn update_cr0(&mut self, r: u32, xer: &Xer) {
        if r == 0 {
            self.value[0] = 2; // EQ
        } else if (r & 0x80000000) != 0 {
            self.value[0] = 8; // LT
        } else {
            self.value[0] = 4; // GT
        }

        self.value[0] |= xer.summary_overflow as u8;
    }

    pub fn update_cr1(&mut self, r: u64, fpscr: &Fpscr) {
        println!("FixMe: update_cr1");
    }

}
