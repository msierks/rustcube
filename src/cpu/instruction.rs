#[derive(Copy, Clone)]
pub struct Instruction(pub u32);

impl Instruction {
    /// Primary opcode field
    #[inline(always)]
    pub fn opcd(self) -> usize {
        ((self.0 >> 26) & 0x3F) as usize
    }

    /// Extended opcode A-Form instructions
    #[inline(always)]
    pub fn xo_a(self) -> usize {
        ((self.0 >> 1) & 0x1F) as usize
    }

    /// Extended opcode (X,XL,XFX,XFL)-Form instructions
    #[inline(always)]
    pub fn xo_x(self) -> usize {
        ((self.0 >> 1) & 0x3FF) as usize
    }

    /// GPR destination
    #[inline(always)]
    pub fn d(self) -> usize {
        ((self.0 >> 21) & 0x1F) as usize
    }

    /// GPR source or destination
    #[inline(always)]
    pub fn a(self) -> usize {
        ((self.0 >> 16) & 0x1F) as usize
    }

    /// GPR source
    #[inline(always)]
    pub fn b(self) -> usize {
        ((self.0 >> 11) & 0x1F) as usize
    }

    #[inline(always)]
    pub fn c(self) -> usize {
        ((self.0 >> 6) & 0x1F) as usize
    }

    #[inline(always)]
    pub fn oe(self) -> bool {
        ((self.0 >> 10) & 1) != 0
    }

    /// Record bit
    #[inline(always)]
    pub fn rc(self) -> bool {
        self.0 & 1 != 0
    }

    #[inline(always)]
    pub fn crbd(self) -> u8 {
        ((self.0 >> 21) & 0x1F) as u8
    }

    #[inline(always)]
    pub fn crfd(self) -> usize {
        ((self.0 >> 23) & 7) as usize
    }

    #[inline(always)]
    pub fn crfs(self) -> usize {
        ((self.0 >> 18) & 7) as usize
    }

    #[inline(always)]
    pub fn l(self) -> bool {
        (self.0 & 0x20_0000) != 0
    }

    /// Immidiate field as 16-bit signed integer
    #[inline(always)]
    pub fn simm(self) -> i16 {
        (self.0 & 0xFFFF) as i16
    }

    /// Immidiate field as 16-bit unsigned integer
    #[inline(always)]
    pub fn uimm(self) -> u32 {
        self.0 & 0xFFFF
    }

    #[inline(always)]
    pub fn uimm_1(self) -> u16 {
        (self.0 & 0xFFF) as u16
    }

    pub fn i(self) -> usize {
        ((self.0 >> 12) & 0x7) as usize
    }

    pub fn w(self) -> bool {
        ((self.0 >> 15) & 1) != 0
    }

    #[inline(always)]
    pub fn li(self) -> u32 {
        (self.0 >> 2) & 0xFF_FFFF
    }

    #[inline(always)]
    pub fn bo(self) -> u8 {
        ((self.0 >> 21) & 0x1F) as u8
    }

    #[inline(always)]
    pub fn bi(self) -> usize {
        ((self.0 >> 16) & 0x1F) as usize
    }

    #[inline(always)]
    pub fn bd(self) -> u16 {
        ((self.0 >> 2) & 0x3FFF) as u16
    }

    /// Absolute address bit
    #[inline(always)]
    pub fn aa(self) -> bool {
        ((self.0 >> 1) & 1) != 0
    }

    #[inline(always)]
    pub fn lk(self) -> bool {
        (self.0 & 1) != 0
    }

    /// GPR source
    #[inline(always)]
    pub fn s(self) -> usize {
        ((self.0 >> 21) & 0x1F) as usize
    }

    #[inline(always)]
    pub fn sr(self) -> usize {
        ((self.0 >> 16) & 0xF) as usize
    }

    /// Shift amount
    #[inline(always)]
    pub fn sh(self) -> u32 {
        (self.0 >> 11) & 0x1F
    }

    #[inline(always)]
    pub fn mb(self) -> u8 {
        ((self.0 >> 6) & 0x1F) as u8
    }

    #[inline(always)]
    pub fn me(self) -> u8 {
        ((self.0 >> 1) & 0x1F) as u8
    }

    pub fn spr(self) -> usize {
        let spr = (self.0 >> 11) & 0x3FF;

        (((spr & 0x1F) << 5) + ((spr >> 5) & 0x1F)) as usize
    }

    #[inline(always)]
    pub fn crm(self) -> usize {
        ((self.0 >> 12) & 0xFF) as usize
    }

    pub fn tbr(self) -> usize {
        (((self.0 >> 6) & 0x3E0) | ((self.0 >> 16) & 0x1F)) as usize
    }

    #[inline(always)]
    pub fn fm(self) -> u8 {
        ((self.0 >> 17) & 0xFF) as u8
    }

    #[inline(always)]
    pub fn to(self) -> u8 {
        ((self.0 >> 21) & 0x1F) as u8
    }

    #[inline(always)]
    pub fn nb(self) -> u8 {
        ((self.0 >> 11) & 0x1F) as u8
    }

    #[inline(always)]
    pub fn imm(self) -> u8 {
        ((self.0 >> 12) & 0xF) as u8
    }
}
