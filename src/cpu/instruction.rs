#[cfg(test)]
use super::optable::*;

#[derive(Copy, Clone)]
pub struct Instruction(pub u32);

impl Instruction {
    /// Primary opcode field
    pub fn opcd(self) -> usize {
        ((self.0 >> 26) & 0x3F) as usize
    }

    /// Extended opcode A-Form instructions
    pub fn xo_a(self) -> usize {
        ((self.0 >> 1) & 0x1F) as usize
    }

    /// Extended opcode (X,XL,XFX,XFL)-Form instructions
    pub fn xo_x(self) -> usize {
        ((self.0 >> 1) & 0x3FF) as usize
    }

    /// GPR destination
    pub fn d(self) -> usize {
        ((self.0 >> 21) & 0x1F) as usize
    }

    /// GPR source or destination
    pub fn a(self) -> usize {
        ((self.0 >> 16) & 0x1F) as usize
    }

    /// GPR source
    pub fn b(self) -> usize {
        ((self.0 >> 11) & 0x1F) as usize
    }

    pub fn c(self) -> usize {
        ((self.0 >> 6) & 0x1F) as usize
    }

    pub fn oe(self) -> bool {
        ((self.0 >> 10) & 1) != 0
    }

    /// Record bit
    pub fn rc(self) -> bool {
        self.0 & 1 != 0
    }

    pub fn crbd(self) -> u8 {
        ((self.0 >> 21) & 0x1F) as u8
    }

    pub fn crfd(self) -> usize {
        ((self.0 >> 23) & 7) as usize
    }

    pub fn crfs(self) -> usize {
        ((self.0 >> 18) & 7) as usize
    }

    pub fn l(self) -> bool {
        (self.0 & 0x20_0000) != 0
    }

    /// Immediate field as 16-bit signed integer
    pub fn simm(self) -> i16 {
        (self.0 & 0xFFFF) as i16
    }

    /// Immediate field as 16-bit unsigned integer
    pub fn uimm(self) -> u32 {
        self.0 & 0xFFFF
    }

    pub fn uimm_1(self) -> u16 {
        (self.0 & 0xFFF) as u16
    }

    pub fn i(self) -> usize {
        ((self.0 >> 12) & 0x7) as usize
    }

    pub fn w(self) -> bool {
        ((self.0 >> 15) & 1) != 0
    }

    pub fn li(self) -> u32 {
        (self.0 >> 2) & 0xFF_FFFF
    }

    pub fn bo(self) -> u8 {
        ((self.0 >> 21) & 0x1F) as u8
    }

    pub fn bi(self) -> usize {
        ((self.0 >> 16) & 0x1F) as usize
    }

    pub fn bd(self) -> u16 {
        ((self.0 >> 2) & 0x3FFF) as u16
    }

    /// Absolute address bit
    pub fn aa(self) -> bool {
        ((self.0 >> 1) & 1) != 0
    }

    pub fn lk(self) -> bool {
        (self.0 & 1) != 0
    }

    /// GPR source
    pub fn s(self) -> usize {
        ((self.0 >> 21) & 0x1F) as usize
    }

    pub fn sr(self) -> usize {
        ((self.0 >> 16) & 0xF) as usize
    }

    /// Shift amount
    pub fn sh(self) -> u32 {
        (self.0 >> 11) & 0x1F
    }

    pub fn mb(self) -> u8 {
        ((self.0 >> 6) & 0x1F) as u8
    }

    pub fn me(self) -> u8 {
        ((self.0 >> 1) & 0x1F) as u8
    }

    pub fn spr(self) -> usize {
        let spr = (self.0 >> 11) & 0x3FF;

        (((spr & 0x1F) << 5) + ((spr >> 5) & 0x1F)) as usize
    }

    pub fn crm(self) -> usize {
        ((self.0 >> 12) & 0xFF) as usize
    }

    pub fn tbr(self) -> usize {
        (((self.0 >> 6) & 0x3E0) | ((self.0 >> 16) & 0x1F)) as usize
    }

    pub fn fm(self) -> u8 {
        ((self.0 >> 17) & 0xFF) as u8
    }

    pub fn to(self) -> u8 {
        ((self.0 >> 21) & 0x1F) as u8
    }

    pub fn nb(self) -> u8 {
        ((self.0 >> 11) & 0x1F) as u8
    }

    pub fn imm(self) -> u8 {
        ((self.0 >> 12) & 0xF) as u8
    }

    /// Create new Instruction and set primary opcode
    #[cfg(test)]
    pub fn new(opcd: u32) -> Self {
        Self(opcd << 26)
    }

    /// Set extended opcode in A-Form instructions
    #[cfg(test)]
    pub fn set_xo_a(self, val: u32) -> Self {
        Self((self.0 & !(0x1F << 1)) | ((val & 0x1F) << 1))
    }

    /// Set extended opcode in (X,XL,XFX,XFL)-Form instructions
    #[cfg(test)]
    pub fn set_xo_x(self, val: u32) -> Self {
        Self((self.0 & !(0x3FF << 1)) | ((val & 0x3FF) << 1))
    }

    /// Set extended opcode in XO instruction format
    #[cfg(test)]
    pub fn set_xo_xo(self, val: u32) -> Self {
        Self((self.0 & !(0x3FF << 1)) | ((val & 0x3FF) << 1))
    }

    #[cfg(test)]
    pub fn set_rd(self, val: usize) -> Self {
        let val = val as u32;
        Self((self.0 & !(0x1F << 21)) | ((val & 0x1F) << 21))
    }

    #[cfg(test)]
    pub fn set_frd(self, val: usize) -> Self {
        self.set_rd(val)
    }

    #[cfg(test)]
    pub fn set_crbd(self, val: u32) -> Self {
        self.set_rd(val as usize)
    }

    #[cfg(test)]
    pub fn set_rs(self, val: usize) -> Self {
        self.set_rd(val)
    }

    #[cfg(test)]
    pub fn set_frs(self, val: usize) -> Self {
        self.set_rd(val)
    }

    #[cfg(test)]
    pub fn set_ra(self, val: usize) -> Self {
        let val = val as u32;
        Self((self.0 & !(0x1F << 16)) | ((val & 0x1F) << 16))
    }

    #[cfg(test)]
    pub fn set_fra(self, val: usize) -> Self {
        self.set_ra(val)
    }

    #[cfg(test)]
    pub fn set_crba(self, val: u32) -> Self {
        self.set_ra(val as usize)
    }

    #[cfg(test)]
    pub fn set_rb(self, val: usize) -> Self {
        let val = val as u32;
        Self((self.0 & !(0x1F << 11)) | ((val & 0x1F) << 11))
    }

    #[cfg(test)]
    pub fn set_frb(self, val: usize) -> Self {
        self.set_rb(val)
    }

    #[cfg(test)]
    pub fn set_crbb(self, val: u32) -> Self {
        self.set_rb(val as usize)
    }

    #[cfg(test)]
    pub fn set_rc(self, val: u32) -> Self {
        Self((self.0 & !0x1) | (val & 0x1))
    }

    #[cfg(test)]
    pub fn set_frc(self, val: usize) -> Self {
        let val = val as u32;
        Self((self.0 & !(0x1F << 6)) | ((val & 0x1F) << 6))
    }

    #[cfg(test)]
    pub fn set_crbc(self, val: u32) -> Self {
        Self((self.0 & !(0x1F << 6)) | ((val & 0x1F) << 6))
    }

    #[cfg(test)]
    pub fn set_oe(self, val: u32) -> Self {
        Self((self.0 & !(0x1 << 10)) | ((val & 0x1) << 10))
    }

    #[cfg(test)]
    pub fn set_crfd(self, val: u32) -> Self {
        Self((self.0 & !(0x7 << 23)) | ((val & 0x7) << 23))
    }

    #[cfg(test)]
    pub fn set_crfs(self, val: u32) -> Self {
        Self((self.0 & !(0x7 << 18)) | ((val & 0x7) << 18))
    }

    #[cfg(test)]
    pub fn set_l(self, val: u32) -> Self {
        Self((self.0 & !0x20_0000) | (val & 0x20_0000))
    }

    #[cfg(test)]
    pub fn set_simm(self, val: u32) -> Self {
        Self((self.0 & !0xFFFF) | (val & 0xFFFF))
    }

    #[cfg(test)]
    pub fn set_uimm(self, val: u32) -> Self {
        Self((self.0 & !0xFFFF) | (val & 0xFFFF))
    }

    #[cfg(test)]
    pub fn set_uimm_1(self, val: u32) -> Self {
        Self((self.0 & !0xFFF) | (val & 0xFFF))
    }

    #[cfg(test)]
    pub fn set_i(self, val: u32) -> Self {
        Self((self.0 & !(0x7 << 12)) | ((val & 0x7) << 12))
    }

    #[cfg(test)]
    pub fn set_w(self, val: u32) -> Self {
        Self((self.0 & !(0x1 << 15)) | ((val & 0x1) << 15))
    }

    #[cfg(test)]
    pub fn set_li(self, val: u32) -> Self {
        Self((self.0 & !(0xFF_FFFF << 2)) | ((val & 0xFF_FFFF) << 2))
    }

    #[cfg(test)]
    pub fn set_bo(self, val: u32) -> Self {
        Self((self.0 & !(0x1F << 21)) | ((val & 0x1F) << 21))
    }

    #[cfg(test)]
    pub fn set_bi(self, val: u32) -> Self {
        Self((self.0 & !(0x1F << 16)) | ((val & 0x1F) << 16))
    }

    #[cfg(test)]
    pub fn set_bd(self, val: u32) -> Self {
        Self((self.0 & !(0x3FFF << 2)) | ((val & 0x3FFF) << 2))
    }

    #[cfg(test)]
    pub fn set_aa(self, val: u32) -> Self {
        Self((self.0 & !(0x1 << 1)) | ((val & 0x1) << 1))
    }

    #[cfg(test)]
    pub fn set_lk(self, val: u32) -> Self {
        Self((self.0 & !0x1) | (val & 0x1))
    }

    #[cfg(test)]
    pub fn set_sr(self, val: u32) -> Self {
        Self((self.0 & !(0xF << 16)) | ((val & 0xF) << 16))
    }

    #[cfg(test)]
    pub fn set_sh(self, val: u32) -> Self {
        Self((self.0 & !(0x1F << 11)) | ((val & 0x1F) << 11))
    }

    #[cfg(test)]
    pub fn set_mb(self, val: u32) -> Self {
        Self((self.0 & !(0x1F << 6)) | ((val & 0x1F) << 6))
    }

    #[cfg(test)]
    pub fn set_me(self, val: u32) -> Self {
        Self((self.0 & !(0x1F << 1)) | ((val & 0x1F) << 1))
    }

    #[cfg(test)]
    pub fn set_spr(self, val: u32) -> Self {
        let spr = ((val & 0x1F) << 5) + ((val & 0x1F) >> 5);

        Self((self.0 & !(0x3FF << 11)) | ((spr & 0x3FF) << 11))
    }

    #[cfg(test)]
    pub fn set_crm(self, val: u32) -> Self {
        Self((self.0 & !(0xFF << 12)) | ((val & 0xFF) << 12))
    }

    // Investigate !!!
    #[cfg(test)]
    pub fn set_tbr(self, val: u32) -> Self {
        Self((self.0 & !0x3FF) | ((val & 0x3E0) << 6) | ((val & 0x1F) << 16))
    }

    #[cfg(test)]
    pub fn set_fm(self, val: u32) -> Self {
        Self((self.0 & !(0xFF << 17)) | ((val & 0xFF) << 17))
    }

    #[cfg(test)]
    pub fn set_to(self, val: u32) -> Self {
        Self((self.0 & !(0x1F << 21)) | ((val & 0x1F) << 21))
    }

    #[cfg(test)]
    pub fn set_nb(self, val: u32) -> Self {
        Self((self.0 & !(0x1F << 11)) | ((val & 0x1F) << 11))
    }

    #[cfg(test)]
    pub fn set_imm(self, val: u32) -> Self {
        Self((self.0 & !(0xF << 12)) | ((val & 0xF) << 12))
    }

    #[cfg(test)]
    pub fn new_bx(li: u32) -> Self {
        Self::new(OPCODE_BX).set_li(li)
    }

    #[cfg(test)]
    pub fn new_bcx(bo: u32, bi: u32, bd: u32) -> Self {
        Self::new(OPCODE_BCX).set_bo(bo).set_bi(bi).set_bd(bd)
    }

    #[cfg(test)]
    pub fn new_bcctrx(bo: u32, bi: u32) -> Self {
        Self::new(OPCODE_EXTENDED19)
            .set_xo_x(OPCODE_BCCTRX)
            .set_bo(bo)
            .set_bi(bi)
    }

    #[cfg(test)]
    pub fn new_bclrx(bo: u32, bi: u32) -> Self {
        Self::new(OPCODE_EXTENDED19)
            .set_xo_x(OPCODE_BCLRX)
            .set_bo(bo)
            .set_bi(bi)
    }

    #[cfg(test)]
    pub fn new_crand(crbd: u32, crba: u32, crbb: u32) -> Self {
        Self::new(OPCODE_EXTENDED19)
            .set_xo_x(OPCODE_CRAND)
            .set_crbd(crbd)
            .set_crba(crba)
            .set_crbb(crbb)
    }

    #[cfg(test)]
    pub fn new_crandc(crbd: u32, crba: u32, crbb: u32) -> Self {
        Self::new(OPCODE_EXTENDED19)
            .set_xo_x(OPCODE_CRANDC)
            .set_crbd(crbd)
            .set_crba(crba)
            .set_crbb(crbb)
    }

    #[cfg(test)]
    pub fn new_creqv(crbd: u32, crba: u32, crbb: u32) -> Self {
        Self::new(OPCODE_EXTENDED19)
            .set_xo_x(OPCODE_CREQV)
            .set_crbd(crbd)
            .set_crba(crba)
            .set_crbb(crbb)
    }

    #[cfg(test)]
    pub fn new_crnand(crbd: u32, crba: u32, crbb: u32) -> Self {
        Self::new(OPCODE_EXTENDED19)
            .set_xo_x(OPCODE_CRNAND)
            .set_crbd(crbd)
            .set_crba(crba)
            .set_crbb(crbb)
    }

    #[cfg(test)]
    pub fn new_crnor(crbd: u32, crba: u32, crbb: u32) -> Self {
        Self::new(OPCODE_EXTENDED19)
            .set_xo_x(OPCODE_CRNOR)
            .set_crbd(crbd)
            .set_crba(crba)
            .set_crbb(crbb)
    }

    #[cfg(test)]
    pub fn new_cror(crbd: u32, crba: u32, crbb: u32) -> Self {
        Self::new(OPCODE_EXTENDED19)
            .set_xo_x(OPCODE_CROR)
            .set_crbd(crbd)
            .set_crba(crba)
            .set_crbb(crbb)
    }

    #[cfg(test)]
    pub fn new_crorc(crbd: u32, crba: u32, crbb: u32) -> Self {
        Self::new(OPCODE_EXTENDED19)
            .set_xo_x(OPCODE_CRORC)
            .set_crbd(crbd)
            .set_crba(crba)
            .set_crbb(crbb)
    }

    #[cfg(test)]
    pub fn new_crxor(crbd: u32, crba: u32, crbb: u32) -> Self {
        Self::new(OPCODE_EXTENDED19)
            .set_xo_x(OPCODE_CRXOR)
            .set_crbd(crbd)
            .set_crba(crba)
            .set_crbb(crbb)
    }

    #[cfg(test)]
    pub fn new_mcrf(crfd: u32, crfs: u32) -> Self {
        Self::new(OPCODE_EXTENDED19)
            .set_xo_x(OPCODE_MCRF)
            .set_crfd(crfd)
            .set_crfs(crfs)
    }

    #[cfg(test)]
    pub fn new_mcrxr(crfd: u32) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_MCRXR)
            .set_crfd(crfd)
    }

    #[cfg(test)]
    pub fn new_mfcr(rd: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_MFCR)
            .set_rd(rd)
    }

    #[cfg(test)]
    pub fn new_mtcrf(crm: u32, rs: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_MTCRF)
            .set_crm(crm)
            .set_rs(rs)
    }

    #[cfg(test)]
    pub fn new_fabsx(frd: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .set_xo_x(OPCODE_FABSX)
            .set_frd(frd)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_faddsx(frd: usize, fra: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED59)
            .set_xo_a(OPCODE_FADDSX)
            .set_frd(frd)
            .set_fra(fra)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_faddx(frd: usize, fra: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .set_xo_a(OPCODE_FADDX)
            .set_frd(frd)
            .set_fra(fra)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_fcmpo(crfd: u32, fra: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .set_xo_x(OPCODE_FCMPO)
            .set_crfd(crfd)
            .set_fra(fra)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_fcmpu(crfd: u32, fra: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .set_xo_x(OPCODE_FCMPU)
            .set_crfd(crfd)
            .set_fra(fra)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_fctiwzx(frd: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .set_xo_x(OPCODE_FCTIWZX)
            .set_frd(frd)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_fctiwx(frd: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .set_xo_x(OPCODE_FCTIWX)
            .set_frd(frd)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_fdivsx(frd: usize, fra: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED59)
            .set_xo_a(OPCODE_FDIVSX)
            .set_frd(frd)
            .set_fra(fra)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_fdivx(frd: usize, fra: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .set_xo_a(OPCODE_FDIVX)
            .set_frd(frd)
            .set_fra(fra)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_fmaddsx(frd: usize, fra: usize, frc: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED59)
            .set_xo_a(OPCODE_FMADDSX)
            .set_frd(frd)
            .set_fra(fra)
            .set_frc(frc)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_fmaddx(frd: usize, fra: usize, frc: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .set_xo_a(OPCODE_FMADDX)
            .set_frd(frd)
            .set_fra(fra)
            .set_frc(frc)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_fmrx(frd: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .set_xo_x(OPCODE_FMRX)
            .set_frd(frd)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_fmsubsx(frd: usize, fra: usize, frc: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED59)
            .set_xo_a(OPCODE_FMSUBSX)
            .set_frd(frd)
            .set_fra(fra)
            .set_frc(frc)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_fmsubx(frd: usize, fra: usize, frc: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .set_xo_a(OPCODE_FMSUBX)
            .set_frd(frd)
            .set_fra(fra)
            .set_frc(frc)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_fmulsx(frd: usize, fra: usize, frc: usize) -> Self {
        Self::new(OPCODE_EXTENDED59)
            .set_xo_a(OPCODE_FMULSX)
            .set_frd(frd)
            .set_fra(fra)
            .set_frc(frc)
    }

    #[cfg(test)]
    pub fn new_fmulx(frd: usize, fra: usize, frc: usize) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .set_xo_a(OPCODE_FMULX)
            .set_frd(frd)
            .set_fra(fra)
            .set_frc(frc)
    }

    #[cfg(test)]
    pub fn new_fnabsx(frd: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .set_xo_x(OPCODE_FNABSX)
            .set_frd(frd)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_fnegx(frd: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .set_xo_x(OPCODE_FNEGX)
            .set_frd(frd)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_fnmaddsx(frd: usize, fra: usize, frc: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED59)
            .set_xo_a(OPCODE_FNMADDSX)
            .set_frd(frd)
            .set_fra(fra)
            .set_frc(frc)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_fnmaddx(frd: usize, fra: usize, frc: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .set_xo_a(OPCODE_FNMADDX)
            .set_frd(frd)
            .set_fra(fra)
            .set_frc(frc)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_fnmsubsx(frd: usize, fra: usize, frc: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED59)
            .set_xo_a(OPCODE_FNMSUBSX)
            .set_frd(frd)
            .set_fra(fra)
            .set_frc(frc)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_fnmsubx(frd: usize, fra: usize, frc: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .set_xo_a(OPCODE_FNMSUBX)
            .set_frd(frd)
            .set_fra(fra)
            .set_frc(frc)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_fresx(frd: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED59)
            .set_xo_a(OPCODE_FRESX)
            .set_frd(frd)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_frspx(frd: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .set_xo_x(OPCODE_FRSPX)
            .set_frd(frd)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_frsqrtex(frd: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .set_xo_a(OPCODE_FRSQRTEX)
            .set_frd(frd)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_fselx(frd: usize, fra: usize, frc: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .set_xo_a(OPCODE_FSELX)
            .set_frd(frd)
            .set_fra(fra)
            .set_frc(frc)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_fsubsx(frd: usize, fra: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED59)
            .set_xo_a(OPCODE_FSUBSX)
            .set_frd(frd)
            .set_fra(fra)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_ps_absx(frd: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .set_xo_x(OPCODE_PS_ABSX)
            .set_frd(frd)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_ps_addx(frd: usize, fra: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .set_xo_a(OPCODE_PS_ABSX)
            .set_frd(frd)
            .set_fra(fra)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_ps_cmpo0(crfd: u32, fra: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .set_xo_x(OPCODE_PS_CMPO0)
            .set_crfd(crfd)
            .set_fra(fra)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_ps_cmpo1(crfd: u32, fra: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .set_xo_x(OPCODE_PS_CMPO1)
            .set_crfd(crfd)
            .set_fra(fra)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_ps_cmpu0(crfd: u32, fra: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .set_xo_x(OPCODE_PS_CMPU0)
            .set_crfd(crfd)
            .set_fra(fra)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_ps_cmpu1(crfd: u32, fra: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .set_xo_x(OPCODE_PS_CMPU1)
            .set_crfd(crfd)
            .set_fra(fra)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_ps_divx(frd: usize, fra: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .set_xo_a(OPCODE_PS_DIVX)
            .set_frd(frd)
            .set_fra(fra)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_ps_maddx(frd: usize, fra: usize, frc: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .set_xo_a(OPCODE_PS_MADDX)
            .set_frd(frd)
            .set_fra(fra)
            .set_frc(frc)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_ps_madds0x(frd: usize, fra: usize, frc: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .set_xo_a(OPCODE_PS_MADDS0X)
            .set_frd(frd)
            .set_fra(fra)
            .set_frc(frc)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_ps_madds1x(frd: usize, fra: usize, frc: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .set_xo_a(OPCODE_PS_MADDS1X)
            .set_frd(frd)
            .set_fra(fra)
            .set_frc(frc)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_ps_merge00x(frd: usize, fra: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .set_xo_a(OPCODE_PS_MERGE_00X)
            .set_frd(frd)
            .set_fra(fra)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_ps_merge01x(frd: usize, fra: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .set_xo_a(OPCODE_PS_MERGE_01X)
            .set_frd(frd)
            .set_fra(fra)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_ps_merge10x(frd: usize, fra: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .set_xo_a(OPCODE_PS_MERGE_10X)
            .set_frd(frd)
            .set_fra(fra)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_ps_merge11x(frd: usize, fra: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .set_xo_a(OPCODE_PS_MERGE_11X)
            .set_frd(frd)
            .set_fra(fra)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_ps_mrx(frd: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .set_xo_a(OPCODE_PS_MRX)
            .set_frd(frd)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_ps_msubx(frd: usize, fra: usize, frc: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .set_xo_a(OPCODE_PS_MSUBX)
            .set_frd(frd)
            .set_fra(fra)
            .set_frc(frc)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_ps_mulx(frd: usize, fra: usize, frc: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .set_xo_a(OPCODE_PS_MULX)
            .set_frd(frd)
            .set_fra(fra)
            .set_frc(frc)
    }

    #[cfg(test)]
    pub fn new_ps_muls0x(frd: usize, fra: usize, frc: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .set_xo_a(OPCODE_PS_MULS0X)
            .set_frd(frd)
            .set_fra(fra)
            .set_frc(frc)
    }

    #[cfg(test)]
    pub fn new_ps_muls1x(frd: usize, fra: usize, frc: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .set_xo_a(OPCODE_PS_MULS1X)
            .set_frd(frd)
            .set_fra(fra)
            .set_frc(frc)
    }

    #[cfg(test)]
    pub fn new_ps_nabsx(frd: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .set_xo_a(OPCODE_PS_NABSX)
            .set_frd(frd)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_ps_negx(frd: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .set_xo_a(OPCODE_PS_NEGX)
            .set_frd(frd)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_ps_nmaddx(frd: usize, fra: usize, frc: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .set_xo_a(OPCODE_PS_NMADDX)
            .set_frd(frd)
            .set_fra(fra)
            .set_frc(frc)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_ps_nmsubx(frd: usize, fra: usize, frc: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .set_xo_a(OPCODE_PS_NMSUBX)
            .set_frd(frd)
            .set_fra(fra)
            .set_frc(frc)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_ps_resx(frd: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .set_xo_a(OPCODE_PS_RESX)
            .set_frd(frd)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_ps_rsqrtex(frd: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .set_xo_a(OPCODE_PS_RSQRTEX)
            .set_frd(frd)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_ps_selx(frd: usize, fra: usize, frc: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .set_xo_a(OPCODE_PS_SELX)
            .set_frd(frd)
            .set_fra(fra)
            .set_frc(frc)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_ps_subx(frd: usize, fra: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .set_xo_a(OPCODE_PS_SUBX)
            .set_frd(frd)
            .set_fra(fra)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_ps_sum0x(frd: usize, fra: usize, frc: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .set_xo_a(OPCODE_PS_SUM0X)
            .set_frd(frd)
            .set_fra(fra)
            .set_frc(frc)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_ps_sum1x(frd: usize, fra: usize, frc: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .set_xo_a(OPCODE_PS_SUM1X)
            .set_frd(frd)
            .set_fra(fra)
            .set_frc(frc)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_fsubx(frd: usize, fra: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .set_xo_a(OPCODE_FSUBX)
            .set_frd(frd)
            .set_fra(fra)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_mcrfs(crfd: u32, crfs: u32) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .set_xo_x(OPCODE_MCRFS)
            .set_crfd(crfd)
            .set_crfs(crfs)
    }

    #[cfg(test)]
    pub fn new_mffsx(frd: usize) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .set_xo_x(OPCODE_MFFSX)
            .set_frd(frd)
    }

    #[cfg(test)]
    pub fn new_mtfsb0x(crbd: u32) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .set_xo_x(OPCODE_MTFSB0X)
            .set_crbd(crbd)
    }

    #[cfg(test)]
    pub fn op_mtfsb1x(crbd: u32) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .set_xo_x(OPCODE_MTFSB1X)
            .set_crbd(crbd)
    }

    #[cfg(test)]
    pub fn op_mtfsfix(crbd: u32, imm: u32) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .set_xo_x(OPCODE_MTFSFIX)
            .set_crbd(crbd)
            .set_imm(imm)
    }

    #[cfg(test)]
    pub fn op_mtfsfx(fm: u32, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .set_xo_x(OPCODE_MTFSFX)
            .set_fm(fm)
            .set_frb(frb)
    }

    #[cfg(test)]
    pub fn new_addcx(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_xo(OPCODE_ADDCX)
            .set_rd(rd)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_addx(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_xo(OPCODE_ADDX)
            .set_rd(rd)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_addi(rd: usize, ra: usize, simm: u32) -> Self {
        Self::new(OPCODE_ADDI).set_rd(rd).set_ra(ra).set_simm(simm)
    }

    #[cfg(test)]
    pub fn new_addic(rd: usize, ra: usize, simm: u32) -> Self {
        Self::new(OPCODE_ADDIC).set_rd(rd).set_ra(ra).set_simm(simm)
    }

    #[cfg(test)]
    pub fn new_addic_rc(rd: usize, ra: usize, simm: u32) -> Self {
        Self::new(OPCODE_ADDIC_RC)
            .set_rd(rd)
            .set_ra(ra)
            .set_simm(simm)
    }

    #[cfg(test)]
    pub fn new_addis(rd: usize, ra: usize, simm: u32) -> Self {
        Self::new(OPCODE_ADDIS).set_rd(rd).set_ra(ra).set_simm(simm)
    }

    #[cfg(test)]
    pub fn new_addmex(rd: usize, ra: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_xo(OPCODE_ADDMEX)
            .set_rd(rd)
            .set_ra(ra)
    }

    #[cfg(test)]
    pub fn new_addex(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_xo(OPCODE_ADDEX)
            .set_rd(rd)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_addzex(rd: usize, ra: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_xo(OPCODE_ADDZEX)
            .set_rd(rd)
            .set_ra(ra)
    }

    #[cfg(test)]
    pub fn new_andcx(ra: usize, rs: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_ANDCX)
            .set_ra(ra)
            .set_rs(rs)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_andi_rc(ra: usize, rs: usize, uimm: u32) -> Self {
        Self::new(OPCODE_ANDI_RC)
            .set_ra(ra)
            .set_rs(rs)
            .set_uimm(uimm)
    }

    #[cfg(test)]
    pub fn new_andis_rc(ra: usize, rs: usize, uimm: u32) -> Self {
        Self::new(OPCODE_ANDIS_RC)
            .set_ra(ra)
            .set_rs(rs)
            .set_uimm(uimm)
    }

    #[cfg(test)]
    pub fn new_andx(ra: usize, rs: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_ANDX)
            .set_ra(ra)
            .set_rs(rs)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_cmp(crfd: u32, l: u32, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_CMP)
            .set_crfd(crfd)
            .set_l(l)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_cmpi(crfd: u32, l: u32, ra: usize, simm: u32) -> Self {
        Self::new(OPCODE_CMPI)
            .set_crfd(crfd)
            .set_l(l)
            .set_ra(ra)
            .set_simm(simm)
    }

    #[cfg(test)]
    pub fn new_cmpl(crfd: u32, l: u32, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_CMPL)
            .set_crfd(crfd)
            .set_l(l)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_cmpli(crfd: u32, l: u32, ra: usize, uimm: u32) -> Self {
        Self::new(OPCODE_CMPLI)
            .set_crfd(crfd)
            .set_l(l)
            .set_ra(ra)
            .set_uimm(uimm)
    }

    #[cfg(test)]
    pub fn new_cntlzwx(ra: usize, rs: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_CNTLZWX)
            .set_ra(ra)
            .set_rs(rs)
    }

    #[cfg(test)]
    pub fn new_divwux(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_xo(OPCODE_DIVWUX)
            .set_rd(rd)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_divwx(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_xo(OPCODE_DIVWX)
            .set_rd(rd)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_eqvx(ra: usize, rs: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_EQVX)
            .set_ra(ra)
            .set_rs(rs)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_extsbx(ra: usize, rs: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_EXTSBX)
            .set_ra(ra)
            .set_rs(rs)
    }

    #[cfg(test)]
    pub fn new_extshx(ra: usize, rs: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_EXTSHX)
            .set_ra(ra)
            .set_rs(rs)
    }

    #[cfg(test)]
    pub fn new_mulhwux(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_xo(OPCODE_MULHWUX)
            // always 0
            .set_rd(rd)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_mulhwx(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_xo(OPCODE_MULHWX)
            // always 0
            .set_rd(rd)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_mulli(rd: usize, ra: usize, simm: u32) -> Self {
        Self::new(OPCODE_MULLI).set_rd(rd).set_ra(ra).set_simm(simm)
    }

    #[cfg(test)]
    pub fn new_mullwx(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_xo(OPCODE_MULLWX)
            .set_rd(rd)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_nandx(ra: usize, rs: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_NANDX)
            .set_ra(ra)
            .set_rs(rs)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_negx(rd: usize, ra: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_xo(OPCODE_NEGX)
            .set_rd(rd)
            .set_ra(ra)
    }

    #[cfg(test)]
    pub fn new_norx(ra: usize, rs: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_NORX)
            .set_ra(ra)
            .set_rs(rs)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_orx(ra: usize, rs: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_ORX)
            .set_ra(ra)
            .set_rs(rs)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_orcx(ra: usize, rs: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_ORCX)
            .set_ra(ra)
            .set_rs(rs)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_ori(ra: usize, rs: usize, uimm: u32) -> Self {
        Self::new(OPCODE_ORI).set_ra(ra).set_rs(rs).set_uimm(uimm)
    }

    #[cfg(test)]
    pub fn new_oris(ra: usize, rs: usize, uimm: u32) -> Self {
        Self::new(OPCODE_ORIS).set_ra(ra).set_rs(rs).set_uimm(uimm)
    }

    #[cfg(test)]
    pub fn new_rlwimix(ra: usize, rs: usize, sh: u32, mb: u32, me: u32) -> Self {
        Self::new(OPCODE_RLWIMIX)
            .set_ra(ra)
            .set_rs(rs)
            .set_sh(sh)
            .set_mb(mb)
            .set_me(me)
    }

    #[cfg(test)]
    pub fn new_rlwinmx(ra: usize, rs: usize, sh: u32, mb: u32, me: u32) -> Self {
        Self::new(OPCODE_RLWINMX)
            .set_ra(ra)
            .set_rs(rs)
            .set_sh(sh)
            .set_mb(mb)
            .set_me(me)
    }

    #[cfg(test)]
    pub fn new_rlwnmx(ra: usize, rs: usize, rb: usize, mb: u32, me: u32) -> Self {
        Self::new(OPCODE_RLWNMX)
            .set_ra(ra)
            .set_rs(rs)
            .set_rb(rb)
            .set_mb(mb)
            .set_me(me)
    }

    #[cfg(test)]
    pub fn new_slwx(ra: usize, rs: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_SLWX)
            .set_ra(ra)
            .set_rs(rs)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_srawix(ra: usize, rs: usize, sh: u32) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_SRAWIX)
            .set_ra(ra)
            .set_rs(rs)
            .set_sh(sh)
    }

    #[cfg(test)]
    pub fn new_srawx(ra: usize, rs: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_SRAWX)
            .set_ra(ra)
            .set_rs(rs)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_srwx(ra: usize, rs: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_SRWX)
            .set_ra(ra)
            .set_rs(rs)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_subfcx(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_xo(OPCODE_SUBFCX)
            .set_rd(rd)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_subfex(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_xo(OPCODE_SUBFEX)
            .set_rd(rd)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_subfic(rd: usize, ra: usize, simm: u32) -> Self {
        Self::new(OPCODE_SUBFIC)
            .set_rd(rd)
            .set_ra(ra)
            .set_simm(simm)
    }

    #[cfg(test)]
    pub fn new_subfmex(rd: usize, ra: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_xo(OPCODE_SUBFMEX)
            .set_rd(rd)
            .set_ra(ra)
    }

    #[cfg(test)]
    pub fn new_subfzex(rd: usize, ra: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_xo(OPCODE_SUBFZEX)
            .set_rd(rd)
            .set_ra(ra)
    }

    #[cfg(test)]
    pub fn new_subfx(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_xo(OPCODE_SUBFX)
            .set_rd(rd)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_tw(to: u32, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_TW)
            .set_to(to)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_twi(to: u32, ra: usize, simm: u32) -> Self {
        Self::new(OPCODE_TWI).set_to(to).set_ra(ra).set_simm(simm)
    }

    #[cfg(test)]
    pub fn new_xori(ra: usize, rs: usize, uimm: u32) -> Self {
        Self::new(OPCODE_XORI).set_ra(ra).set_rs(rs).set_uimm(uimm)
    }

    #[cfg(test)]
    pub fn new_xoris(ra: usize, rs: usize, uimm: u32) -> Self {
        Self::new(OPCODE_XORIS).set_ra(ra).set_rs(rs).set_uimm(uimm)
    }

    #[cfg(test)]
    pub fn new_xorx(ra: usize, rs: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_XORX)
            .set_ra(ra)
            .set_rs(rs)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_dcbf(ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_DCBF)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_dcbi(ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_DCBI)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_dcbst(ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_DCBST)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_dcbt(ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_DCBT)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_dcbtst(ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_DCBTST)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_dcbz(ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_DCBZ)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_dcbz_l(ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .set_xo_x(OPCODE_DCBZ_L)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_eciwx(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_ECIWX)
            .set_rd(rd)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_ecowx(rs: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_ECOWX)
            .set_rs(rs)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_icbi(ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_ICBI)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_lbz(rd: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_LBZ).set_rd(rd).set_ra(ra).set_uimm(d)
    }

    #[cfg(test)]
    pub fn new_lbzu(rd: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_LBZU).set_rd(rd).set_ra(ra).set_uimm(d)
    }

    #[cfg(test)]
    pub fn new_lbzux(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_LBZUX)
            .set_rd(rd)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_lbzx(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_LBZX)
            .set_rd(rd)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_lfd(frd: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_LFD).set_frd(frd).set_ra(ra).set_uimm(d)
    }

    #[cfg(test)]
    pub fn new_lfdu(frd: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_LFDU).set_frd(frd).set_ra(ra).set_uimm(d)
    }

    #[cfg(test)]
    pub fn new_lfdux(frd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_LFDUX)
            .set_frd(frd)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_lfdx(frd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_LFDX)
            .set_frd(frd)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_lfs(frd: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_LFS).set_frd(frd).set_ra(ra).set_uimm(d)
    }

    #[cfg(test)]
    pub fn new_lfsu(frd: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_LFSU).set_frd(frd).set_ra(ra).set_uimm(d)
    }

    #[cfg(test)]
    pub fn new_lfsux(frd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_LFSUX)
            .set_frd(frd)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_lfsx(frd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_LFSX)
            .set_frd(frd)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_lha(rd: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_LHA).set_rd(rd).set_ra(ra).set_uimm(d)
    }

    #[cfg(test)]
    pub fn new_lhau(rd: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_LHAU).set_rd(rd).set_ra(ra).set_uimm(d)
    }

    #[cfg(test)]
    pub fn new_lhaux(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_LHAUX)
            .set_rd(rd)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_lhax(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_LHAX)
            .set_rd(rd)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_lhbrx(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_LHBRX)
            .set_rd(rd)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_lhz(rd: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_LHZ).set_rd(rd).set_ra(ra).set_uimm(d)
    }

    #[cfg(test)]
    pub fn new_lhzu(rd: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_LHZU).set_rd(rd).set_ra(ra).set_uimm(d)
    }

    #[cfg(test)]
    pub fn new_lhzux(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_LHZUX)
            .set_rd(rd)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_lhzx(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_LHZX)
            .set_rd(rd)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_lmw(rd: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_LMW).set_rd(rd).set_ra(ra).set_uimm(d)
    }

    #[cfg(test)]
    pub fn new_lswi(rd: usize, ra: usize, nb: u32) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_LSWI)
            .set_rd(rd)
            .set_ra(ra)
            .set_nb(nb)
    }

    #[cfg(test)]
    pub fn new_lswx(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_LSWI)
            .set_rd(rd)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_lwarx(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_LWARX)
            .set_rd(rd)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_lwbrx(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_LWBRX)
            .set_rd(rd)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_lwz(rd: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_LWZ).set_rd(rd).set_ra(ra).set_uimm(d)
    }

    #[cfg(test)]
    pub fn new_lwzu(rd: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_LWZU).set_rd(rd).set_ra(ra).set_uimm(d)
    }

    #[cfg(test)]
    pub fn new_lwzux(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_LWZUX)
            .set_rd(rd)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_lwzx(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_LWZX)
            .set_rd(rd)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_psq_l(frd: usize, ra: usize, d: u32, w: u32, i: u32) -> Self {
        Self::new(OPCODE_PSQ_L)
            .set_frd(frd)
            .set_ra(ra)
            .set_uimm_1(d)
            .set_w(w)
            .set_i(i)
    }

    #[cfg(test)]
    pub fn new_psq_lu(frd: usize, ra: usize, d: u32, w: u32, i: u32) -> Self {
        Self::new(OPCODE_PSQ_LU)
            .set_frd(frd)
            .set_ra(ra)
            .set_uimm_1(d)
            .set_w(w)
            .set_i(i)
    }

    #[cfg(test)]
    pub fn new_psq_lux(frd: usize, ra: usize, rb: usize, w: u32, i: u32) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .set_xo_a(OPCODE_PSQ_LUX)
            .set_frd(frd)
            .set_ra(ra)
            .set_rb(rb)
            .set_w(w)
            .set_i(i)
    }

    #[cfg(test)]
    pub fn new_psq_lx(frd: usize, ra: usize, rb: usize, w: u32, i: u32) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .set_xo_a(OPCODE_PSQ_LX)
            .set_frd(frd)
            .set_ra(ra)
            .set_rb(rb)
            .set_w(w)
            .set_i(i)
    }

    #[cfg(test)]
    pub fn new_psq_st(frs: usize, ra: usize, d: u32, w: u32, i: u32) -> Self {
        Self::new(OPCODE_PSQ_ST)
            .set_frs(frs)
            .set_ra(ra)
            .set_uimm_1(d)
            .set_w(w)
            .set_i(i)
    }

    #[cfg(test)]
    pub fn new_psq_stu(frs: usize, ra: usize, d: u32, w: u32, i: u32) -> Self {
        Self::new(OPCODE_PSQ_STU)
            .set_frs(frs)
            .set_ra(ra)
            .set_uimm_1(d)
            .set_w(w)
            .set_i(i)
    }

    #[cfg(test)]
    pub fn new_psq_stux(frs: usize, ra: usize, rb: usize, w: u32, i: u32) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .set_xo_a(OPCODE_PSQ_STUX)
            .set_frs(frs)
            .set_ra(ra)
            .set_rb(rb)
            .set_w(w)
            .set_i(i)
    }

    #[cfg(test)]
    pub fn new_psq_stx(frs: usize, ra: usize, rb: usize, w: u32, i: u32) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .set_xo_a(OPCODE_PSQ_STX)
            .set_frs(frs)
            .set_ra(ra)
            .set_rb(rb)
            .set_w(w)
            .set_i(i)
    }

    #[cfg(test)]
    pub fn new_stb(rs: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_STB).set_rs(rs).set_ra(ra).set_uimm(d)
    }

    #[cfg(test)]
    pub fn new_stbu(rs: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_STBU).set_rs(rs).set_ra(ra).set_uimm(d)
    }

    #[cfg(test)]
    pub fn new_stbux(rs: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_STBUX)
            .set_rs(rs)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_stbx(rs: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_STBX)
            .set_rs(rs)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_stfd(frs: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_STFD).set_frs(frs).set_ra(ra).set_uimm(d)
    }

    #[cfg(test)]
    pub fn new_stfdu(frs: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_STFDU).set_frs(frs).set_ra(ra).set_uimm(d)
    }

    #[cfg(test)]
    pub fn new_stfdux(frs: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_STFDUX)
            .set_frs(frs)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_stfdx(frs: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_STFDX)
            .set_frs(frs)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_stfiwx(frs: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_STFIWX)
            .set_frs(frs)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_stfs(frs: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_STFS).set_frs(frs).set_ra(ra).set_uimm(d)
    }

    #[cfg(test)]
    pub fn new_stfsu(frs: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_STFSU).set_frs(frs).set_ra(ra).set_uimm(d)
    }

    #[cfg(test)]
    pub fn new_stfsux(frs: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_STFSUX)
            .set_frs(frs)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_stfsx(frs: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_STFSX)
            .set_frs(frs)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_sth(rs: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_STH).set_rs(rs).set_ra(ra).set_uimm(d)
    }

    #[cfg(test)]
    pub fn new_sthbrx(rs: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_STHBRX)
            .set_rs(rs)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_sthu(rs: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_STHU).set_rs(rs).set_ra(ra).set_uimm(d)
    }

    #[cfg(test)]
    pub fn new_sthux(rs: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_STHUX)
            .set_rs(rs)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_sthx(rs: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_STHX)
            .set_rs(rs)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_stmw(rs: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_STMW).set_rs(rs).set_ra(ra).set_uimm(d)
    }

    #[cfg(test)]
    pub fn new_stswi(rs: usize, ra: usize, nb: u32) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_STSWI)
            .set_rs(rs)
            .set_ra(ra)
            .set_nb(nb)
    }

    #[cfg(test)]
    pub fn new_stswx(rs: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_STSWX)
            .set_rs(rs)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_stw(rs: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_STW).set_rs(rs).set_ra(ra).set_uimm(d)
    }

    #[cfg(test)]
    pub fn new_stwbrx(rs: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_STWBRX)
            .set_rs(rs)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_stwcx_rc(rs: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_STWCX_RC)
            .set_rs(rs)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_stwu(rs: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_STWU).set_rs(rs).set_ra(ra).set_uimm(d)
    }

    #[cfg(test)]
    pub fn new_stwux(rs: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_STWUX)
            .set_rs(rs)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_stwx(rs: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_STWX)
            .set_rs(rs)
            .set_ra(ra)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_tlbie(rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_TBLIE)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_eieio() -> Self {
        Self::new(OPCODE_EXTENDED31).set_xo_x(OPCODE_EIEIO)
    }

    #[cfg(test)]
    pub fn new_isync() -> Self {
        Self::new(OPCODE_EXTENDED19).set_xo_x(OPCODE_ISYNC)
    }

    #[cfg(test)]
    pub fn new_mfmsr(rd: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_MFMSR)
            .set_rd(rd)
    }

    #[cfg(test)]
    pub fn new_mfspr(rd: usize, spr: u32) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_MFMSR)
            .set_rd(rd)
            .set_spr(spr)
    }

    #[cfg(test)]
    pub fn new_mfsr(rd: usize, sr: u32) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_MFSR)
            .set_rd(rd)
            .set_sr(sr)
    }

    #[cfg(test)]
    pub fn new_mfsrin(rd: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_MFSRIN)
            .set_rd(rd)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_mftb(rd: usize, tbr: u32) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_MFTB)
            .set_rd(rd)
            .set_tbr(tbr)
    }

    #[cfg(test)]
    pub fn new_mtmsr(rs: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_MTMSR)
            .set_rs(rs)
    }

    #[cfg(test)]
    pub fn new_mtspr(spr: u32, rs: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_MTSPR)
            .set_spr(spr)
            .set_rs(rs)
    }

    #[cfg(test)]
    pub fn new_mtsr(sr: u32, rs: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_MTSR)
            .set_sr(sr)
            .set_rs(rs)
    }

    #[cfg(test)]
    pub fn new_mtsrin(rs: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .set_xo_x(OPCODE_MTSRIN)
            .set_rs(rs)
            .set_rb(rb)
    }

    #[cfg(test)]
    pub fn new_rfi() -> Self {
        Self::new(OPCODE_EXTENDED19).set_xo_x(OPCODE_RFI)
    }

    #[cfg(test)]
    pub fn new_sc() -> Self {
        Self::new(OPCODE_SC)
    }

    #[cfg(test)]
    pub fn new_sync() -> Self {
        Self::new(OPCODE_EXTENDED31).set_xo_x(OPCODE_SYNC)
    }

    #[cfg(test)]
    pub fn new_tlbsync() -> Self {
        Self::new(OPCODE_EXTENDED31).set_xo_x(OPCODE_TLBSYNC)
    }
}
