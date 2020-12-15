pub trait Halveable {
    type HalfSize;

    fn hi(self) -> Self::HalfSize;
    fn lo(self) -> Self::HalfSize;
    fn set_hi(self, v: Self::HalfSize) -> Self;
    fn set_lo(self, v: Self::HalfSize) -> Self;
}

impl Halveable for u32 {
    type HalfSize = u16;

    fn hi(self) -> Self::HalfSize {
        (self >> 16) as u16
    }

    fn lo(self) -> Self::HalfSize {
        self as u16
    }

    fn set_hi(self, v: Self::HalfSize) -> Self {
        (self & 0xFFFF) | ((v as u32) << 16)
    }

    fn set_lo(self, v: Self::HalfSize) -> Self {
        (self & 0xFFFF_0000) | (v as u32)
    }
}
