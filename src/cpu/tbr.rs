
// Time Base Register (TBR)

// DUMB timer

// TODO: put some thought into this

enum_from_primitive! {
    #[derive(Debug)]
    pub enum TBR {
        TBL = 268,
        TBU = 269,
        UNKNOWN = -1
    }
}

#[derive(Default, Debug)]
pub struct Tbr {
    tbr: u64
}

impl Tbr {

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
