use std::fmt;
use num::FromPrimitive;

use super::spr::Spr;
use super::tbr::TBR;

pub struct Instruction(pub u32);

impl Instruction {
    #[inline(always)]
    pub fn opcode(&self) -> u8 {
        ((self.0  >> 26) & 0x3F) as u8
    }

    #[inline(always)]
    pub fn ext_opcode_a(&self) -> u8 {
        ((self.0 >> 1) & 0x1F) as u8
    }

    #[inline(always)]
    pub fn ext_opcode_x(&self) -> u16 {
        ((self.0 >> 1) & 0x3FF) as u16
    }

    #[inline(always)]
    pub fn d(&self) -> usize {
        ((self.0 >> 21) & 0x1F) as usize
    }

    #[inline(always)]
    pub fn a(&self) -> usize {
        ((self.0 >> 16) & 0x1F) as usize
    }

    #[inline(always)]
    pub fn b(&self) -> usize {
        ((self.0 >> 11) & 0x1F) as usize
    }

    #[inline(always)]
    pub fn oe(&self) -> bool {
        ((self.0 >> 10) & 1) == 1
    }

    #[inline(always)]
    pub fn rc(&self) -> bool {
        self.0 & 1 == 1
    }

    #[inline(always)]
    pub fn crfd(&self) -> usize {
        ((self.0 >> 23) & 7) as usize
    }

    // FixMe: this will always be false !!!
    #[inline(always)]
    pub fn l(&self) -> bool {
        (self.0 & 0x200000) == 1
    }

    #[inline(always)]
    pub fn simm(&self) -> i16 {
        (self.0 & 0xFFFF) as i16
    }

    #[inline(always)]
    pub fn uimm(&self) -> u32 {
        (self.0 & 0xFFFF) as u32
    }

    #[inline(always)]
    pub fn li(&self) -> u32 {
        ((self.0 >> 2) & 0xFFFFFF)
    }

    #[inline(always)]
    pub fn bo(&self) -> u8 {
        ((self.0 >> 21) & 0x1F) as u8
    }

    #[inline(always)]
    pub fn bi(&self) -> usize {
        ((self.0 >> 16) & 0x1F) as usize
    }

    #[inline(always)]
    pub fn bd(&self) -> u16 {
        ((self.0 >> 2) & 0x3FFF) as u16
    }

    #[inline(always)]
    pub fn aa(&self) -> u8 {
        ((self.0 >> 1) & 1) as u8
    }

    #[inline(always)]
    pub fn lk(&self) -> u8 {
        (self.0 & 1) as u8
    }

    #[inline(always)]
    pub fn s(&self) -> usize {
        ((self.0 >> 21) & 0x1F) as usize
    }

    #[inline(always)]
    pub fn sr(&self) -> usize {
        ((self.0 >> 16) & 0xF) as usize
    }

    #[inline(always)]
    pub fn sh(&self) -> u8 {
        ((self.0 >> 11) & 0x1F) as u8
    }

    #[inline(always)]
    pub fn mb(&self) -> u8 {
        ((self.0 >> 6) & 0x1F) as u8
    }

    #[inline(always)]
    pub fn me(&self) -> u8 {
        ((self.0 >> 1) & 0x1F) as u8
    }

    pub fn spr(&self) -> Spr {
        let n = ((self.0 >> 6) & 0x3E0) | ((self.0 >> 16) & 0x1F);

        Spr::from_u32(n).unwrap_or_else(|| Spr::UNKNOWN)
    }

    pub fn tbr(&self) -> TBR {
        let n = ((self.0 >> 6) & 0x3E0) | ((self.0 >> 16) & 0x1F);
        TBR::from_u32(n).unwrap_or_else(|| TBR::UNKNOWN)
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#b}", self.opcode())
    }
}
