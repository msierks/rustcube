#[derive(Default, Debug, Copy, Clone)]
pub struct Gqr(pub u32);

impl Gqr {
    //pub fn set(&mut self, value: u32) {
    //    panic!("FixMe:");
    //}

    //pub fn as_u32(&self) -> u32 {
    //    self.0
    //}

    //#[inline(always)]
    //pub fn ld_scale(&self) -> u8 {
    //    ((self.0 >> 24) & 0x3F) as u8
    //}

    #[inline(always)]
    pub fn ld_type(self) -> u8 {
        ((self.0 >> 16) & 0x7) as u8
    }

    //#[inline(always)]
    //pub fn st_scale(&self) -> u8 {
    //    ((self.0 >> 8) & 0x3F) as u8
    //}

    #[inline(always)]
    pub fn st_type(self) -> u8 {
        (self.0 & 0x7) as u8
    }
}
