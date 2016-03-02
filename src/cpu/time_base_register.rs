
// DUMB timer

// TODO: put some thought into this

#[derive(Default, Debug)]
pub struct TimeBaseRegister {
    tbr: u64
}

impl TimeBaseRegister {

    // advance that time
    pub fn advance(&mut self) {
        self.tbr += 1;
    }

    pub fn l(&self) -> u32 {
        (self.tbr & 0xFFFFFFFF) as u32
    }

    pub fn u(&self) -> u32 {
        (self.tbr >> 32) as u32
    }

}
