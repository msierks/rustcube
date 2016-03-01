use std::fmt;

pub struct Instruction(pub u32);

impl Instruction {
    #[inline(always)]
    pub fn opcode(&self) -> u8 {
        ((self.0  >> 26) & 0b11_1111) as u8
    }

    #[inline(always)]
    pub fn subopcode(&self) -> u16 {
        ((self.0 >> 1) & 0b11_1111_1111) as u16
    }

    #[inline(always)]
    pub fn d(&self) -> usize {
        ((self.0 >> 21) & 0b1_1111) as usize
    }

    #[inline(always)]
    pub fn a(&self) -> usize {
        ((self.0 >> 16) & 0b1_1111) as usize
    }

    #[inline(always)]
    pub fn b(&self) -> usize {
        ((self.0 >> 11) & 0b1_1111) as usize
    }

    #[inline(always)]
    pub fn crfd(&self) -> usize {
        ((self.0 >> 16) & 0b1_1111) as usize
    }

    #[inline(always)]
    pub fn simm(&self) -> u32 { // FIXME: should probably be signed
        (self.0 & 0b1111_1111_1111_1111) as u32
    }

    #[inline(always)]
    pub fn uimm(&self) -> u32 {
        (self.0 & 0b1111_1111_1111_1111) as u32
    }

    #[inline(always)]
    pub fn li(&self) -> u32 {
        ((self.0 >> 2) & 0b1111_1111_1111_1111_1111_1111)
    }

    #[inline(always)]
    pub fn bo(&self) -> u8 {
        ((self.0 >> 21) & 0b1_1111) as u8
    }

    #[inline(always)]
    pub fn bi(&self) -> usize {
        ((self.0 >> 16) & 0b1_1111) as usize
    }

    #[inline(always)]
    pub fn bd(&self) -> u32 {
        ((self.0 >> 2) & 0b1111_1111_1111_11)
    }

    #[inline(always)]
    pub fn aa(&self) -> u8 {
        ((self.0 >> 1) & 0b1) as u8
    }

    #[inline(always)]
    pub fn lk(&self) -> u8 {
        (self.0 & 0b1) as u8
    }

    #[inline(always)]
    pub fn s(&self) -> usize {
        ((self.0 >> 21) & 0b1_1111) as usize
    }

    #[inline(always)]
    pub fn sr(&self) -> usize {
        ((self.0 >> 16) & 0b1111) as usize
    }

    #[inline(always)]
    pub fn sh(&self) -> u8 {
        ((self.0 >> 11) & 0b1_1111) as u8
    }

    #[inline(always)]
    pub fn mb(&self) -> u8 {
        ((self.0 >> 6) & 0b1_1111) as u8
    }

    #[inline(always)]
    pub fn me(&self) -> u8 {
        ((self.0 >> 1) & 0b1_1111) as u8
    }

    #[inline(always)]
    //pub fn spr(&self) -> u32 {
    //    (self.0 >> 11) & 0b11_1111_1111
    //}

    #[inline(always)]
    pub fn spr_upper(&self) -> u32 {
        (self.0 >> 11) & 0b1_1111
    }

    #[inline(always)]
    pub fn spr_lower(&self) -> u32 {
        (self.0 >> 16) & 0b1_1111
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#b}", self.opcode())
    }
}
