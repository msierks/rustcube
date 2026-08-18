bitfield! {
    #[derive(Copy, Clone)]
    pub struct Instruction(u32);
    u32;
    /// Primary opcode
    #[inline]
    pub u8, into usize, opcd, set_opcd : 31, 26;
    /// Extended opcode A-Form
    #[inline]
    pub u8, into usize, xo_a, set_xo_a : 5, 1;
    /// Extended opcode (X, XL, XFX, XFL)-Form
    #[inline]
    pub u16, into usize, xo_x, set_xo_x : 10, 1;
    /// Destination register
    #[inline]
    pub u8, into usize, d, set_d : 25, 21;
    /// Source register a
    #[inline]
    pub u8, into usize, a, set_a : 20, 16;
    /// Source register b
    #[inline]
    pub u8, into usize, b, set_b : 15, 11;
    #[inline]
    pub u8, into usize, c, set_c : 10, 6;
    /// Source register
    #[inline]
    pub u8, into usize, s, set_s : 25, 21;
    /// Overflow exception bit
    #[inline]
    pub oe, set_oe : 10;
    /// Record bit
    #[inline]
    pub rc, set_rc : 0;
    /// Destination condition register bit
    #[inline]
    pub u8, crbd, set_crbd : 25, 21;
    /// Destination FPSCR field
    #[inline]
    pub u8, into usize, crfd, set_crfd : 25, 23;
    /// Source FPSCR field
    #[inline]
    pub u8, into usize, crfs, set_crfs : 20, 18;
    /// 64-bit comparison bit (1 = invalid)
    #[inline]
    pub l, set_l : 21;
    /// Signed 16-bit immediate value
    #[inline]
    pub i16, simm, set_simm : 15, 0;
    /// Unsigned 16-bit immediate value
    #[inline]
    pub uimm, set_uimm : 15, 0;
    /// Unsigned 12-bit immegiate value
    #[inline]
    pub u16, uimm_1, set_uimm1 : 11, 0;
    #[inline]
    pub u8, into usize, i, set_i : 14, 12;
    #[inline]
    pub w, set_w : 15;
    #[inline]
    pub li, set_li : 25, 2;
    #[inline]
    pub u8, bo, set_bo : 25, 21;
    #[inline]
    pub u8, into usize, bi, set_bi : 20, 16;
    #[inline]
    pub u16, bd, set_bd : 15, 2;
    /// Absolute address bit
    #[inline]
    pub aa, set_aa : 1;
    /// Link bit
    #[inline]
    pub lk, set_lk : 0;
    /// Segment register
    #[inline]
    pub u8, into usize, sr, set_sr : 19, 16;
    /// Shift amount
    #[inline]
    pub sh, set_sh : 15, 11;
    #[inline]
    pub u8, mb, set_mb : 10, 6;
    #[inline]
    pub u8, me, set_me : 5, 1;
    #[inline]
    pub u8, into usize, crm, set_crm : 19, 12;
    #[inline]
    pub u8, fm, set_fm : 24, 17;
    #[inline]
    pub u8, to, set_to : 25, 21;
    /// Number of bytes to move in an immediate string load and store
    #[inline]
    pub u8, nb, set_nb : 15, 11;
    #[inline]
    pub u8, imm, set_imm : 15, 12;
}

impl Instruction {
    pub fn spr(self) -> usize {
        let spr = (self.0 >> 11) & 0x3FF;

        (((spr & 0x1F) << 5) + ((spr >> 5) & 0x1F)) as usize
    }

    pub fn tbr(self) -> usize {
        self.spr()
    }
}

#[cfg(test)]
#[allow(dead_code)]
use super::opcodes::*;

#[cfg(test)]
impl Instruction {
    pub fn new(opcd: u32) -> Self {
        Self(opcd << 26)
    }

    pub fn with_xo_a(mut self, val: u32) -> Self {
        self.set_xo_a(val as u8);
        self
    }

    pub fn with_xo_x(mut self, val: u32) -> Self {
        self.set_xo_x(val as u16);
        self
    }

    pub fn with_xo_xo(self, val: u32) -> Self {
        self.with_xo_x(val)
    }

    pub fn with_rd(mut self, val: usize) -> Self {
        self.set_d(val as u8);
        self
    }

    pub fn with_frd(self, val: usize) -> Self {
        self.with_rd(val)
    }

    pub fn with_crbd(mut self, val: u32) -> Self {
        self.set_crbd(val as u8);
        self
    }

    pub fn with_rs(self, val: usize) -> Self {
        self.with_rd(val)
    }

    pub fn with_frs(self, val: usize) -> Self {
        self.with_rd(val)
    }

    pub fn with_ra(mut self, val: usize) -> Self {
        self.set_a(val as u8);
        self
    }

    pub fn with_fra(self, val: usize) -> Self {
        self.with_ra(val)
    }

    pub fn with_crba(self, val: u32) -> Self {
        self.with_ra(val as usize)
    }

    pub fn with_rb(mut self, val: usize) -> Self {
        self.set_b(val as u8);
        self
    }

    pub fn with_frb(self, val: usize) -> Self {
        self.with_rb(val)
    }

    pub fn with_crbb(self, val: u32) -> Self {
        self.with_rb(val as usize)
    }

    pub fn with_rc(mut self, val: bool) -> Self {
        self.set_rc(val);
        self
    }

    pub fn with_frc(mut self, val: usize) -> Self {
        self.set_c(val as u8);
        self
    }

    pub fn with_crbc(self, val: u32) -> Self {
        self.with_frc(val as usize)
    }

    pub fn with_oe(mut self, val: bool) -> Self {
        self.set_oe(val);
        self
    }

    pub fn with_crfd(mut self, val: u32) -> Self {
        self.set_crfd(val as u8);
        self
    }

    pub fn with_crfs(mut self, val: u32) -> Self {
        self.set_crfs(val as u8);
        self
    }

    pub fn with_l(mut self, val: u32) -> Self {
        self.set_l(val != 0);
        self
    }

    pub fn with_simm(mut self, val: u32) -> Self {
        self.set_simm(val as i16);
        self
    }

    pub fn with_uimm(mut self, val: u32) -> Self {
        self.set_uimm(val);
        self
    }

    pub fn with_uimm_1(mut self, val: u32) -> Self {
        self.set_uimm1(val as u16);
        self
    }

    pub fn with_i(mut self, val: u32) -> Self {
        self.set_i(val as u8);
        self
    }

    pub fn with_w(mut self, val: u32) -> Self {
        self.set_w(val & 1 != 0);
        self
    }

    pub fn with_li(mut self, val: u32) -> Self {
        self.set_li(val);
        self
    }

    pub fn with_bo(mut self, val: u32) -> Self {
        self.set_bo(val as u8);
        self
    }

    pub fn with_bi(mut self, val: u32) -> Self {
        self.set_bi(val as u8);
        self
    }

    pub fn with_bd(mut self, val: u32) -> Self {
        self.set_bd(val as u16);
        self
    }

    pub fn with_aa(mut self, val: u32) -> Self {
        self.set_aa(val & 1 != 0);
        self
    }

    pub fn with_lk(mut self, val: bool) -> Self {
        self.set_lk(val);
        self
    }

    pub fn with_sr(mut self, val: u32) -> Self {
        self.set_sr(val as u8);
        self
    }

    pub fn with_sh(mut self, val: u32) -> Self {
        self.set_sh(val);
        self
    }

    pub fn with_mb(mut self, val: u32) -> Self {
        self.set_mb(val as u8);
        self
    }

    pub fn with_me(mut self, val: u32) -> Self {
        self.set_me(val as u8);
        self
    }

    pub fn with_crm(mut self, val: u32) -> Self {
        self.set_crm(val as u8);
        self
    }

    pub fn with_fm(mut self, val: u32) -> Self {
        self.set_fm(val as u8);
        self
    }

    pub fn with_to(mut self, val: u32) -> Self {
        self.set_to(val as u8);
        self
    }

    pub fn with_nb(mut self, val: u32) -> Self {
        self.set_nb(val as u8);
        self
    }

    pub fn with_imm(mut self, val: u32) -> Self {
        self.set_imm(val as u8);
        self
    }

    pub fn with_spr(self, val: u32) -> Self {
        let spr = ((val & 0x1F) << 5) | ((val >> 5) & 0x1F);

        Self((self.0 & !(0x3FF << 11)) | (spr << 11))
    }

    pub fn with_tbr(self, val: u32) -> Self {
        self.with_spr(val)
    }

    pub fn new_bx(li: u32) -> Self {
        Self::new(OPCODE_BX).with_li(li)
    }

    pub fn new_bcx(bo: u32, bi: u32, bd: u32) -> Self {
        Self::new(OPCODE_BCX).with_bo(bo).with_bi(bi).with_bd(bd)
    }

    pub fn new_bcctrx(bo: u32, bi: u32) -> Self {
        Self::new(OPCODE_EXTENDED19)
            .with_xo_x(OPCODE_BCCTRX)
            .with_bo(bo)
            .with_bi(bi)
    }

    pub fn new_bclrx(bo: u32, bi: u32) -> Self {
        Self::new(OPCODE_EXTENDED19)
            .with_xo_x(OPCODE_BCLRX)
            .with_bo(bo)
            .with_bi(bi)
    }

    pub fn new_crand(crbd: u32, crba: u32, crbb: u32) -> Self {
        Self::new(OPCODE_EXTENDED19)
            .with_xo_x(OPCODE_CRAND)
            .with_crbd(crbd)
            .with_crba(crba)
            .with_crbb(crbb)
    }

    pub fn new_crandc(crbd: u32, crba: u32, crbb: u32) -> Self {
        Self::new(OPCODE_EXTENDED19)
            .with_xo_x(OPCODE_CRANDC)
            .with_crbd(crbd)
            .with_crba(crba)
            .with_crbb(crbb)
    }

    pub fn new_creqv(crbd: u32, crba: u32, crbb: u32) -> Self {
        Self::new(OPCODE_EXTENDED19)
            .with_xo_x(OPCODE_CREQV)
            .with_crbd(crbd)
            .with_crba(crba)
            .with_crbb(crbb)
    }

    pub fn new_crnand(crbd: u32, crba: u32, crbb: u32) -> Self {
        Self::new(OPCODE_EXTENDED19)
            .with_xo_x(OPCODE_CRNAND)
            .with_crbd(crbd)
            .with_crba(crba)
            .with_crbb(crbb)
    }

    pub fn new_crnor(crbd: u32, crba: u32, crbb: u32) -> Self {
        Self::new(OPCODE_EXTENDED19)
            .with_xo_x(OPCODE_CRNOR)
            .with_crbd(crbd)
            .with_crba(crba)
            .with_crbb(crbb)
    }

    pub fn new_cror(crbd: u32, crba: u32, crbb: u32) -> Self {
        Self::new(OPCODE_EXTENDED19)
            .with_xo_x(OPCODE_CROR)
            .with_crbd(crbd)
            .with_crba(crba)
            .with_crbb(crbb)
    }

    pub fn new_crorc(crbd: u32, crba: u32, crbb: u32) -> Self {
        Self::new(OPCODE_EXTENDED19)
            .with_xo_x(OPCODE_CRORC)
            .with_crbd(crbd)
            .with_crba(crba)
            .with_crbb(crbb)
    }

    pub fn new_crxor(crbd: u32, crba: u32, crbb: u32) -> Self {
        Self::new(OPCODE_EXTENDED19)
            .with_xo_x(OPCODE_CRXOR)
            .with_crbd(crbd)
            .with_crba(crba)
            .with_crbb(crbb)
    }

    pub fn new_mcrf(crfd: u32, crfs: u32) -> Self {
        Self::new(OPCODE_EXTENDED19)
            .with_xo_x(OPCODE_MCRF)
            .with_crfd(crfd)
            .with_crfs(crfs)
    }

    pub fn new_mcrxr(crfd: u32) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_MCRXR)
            .with_crfd(crfd)
    }

    pub fn new_mfcr(rd: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_MFCR)
            .with_rd(rd)
    }

    pub fn new_mtcrf(crm: u32, rs: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_MTCRF)
            .with_crm(crm)
            .with_rs(rs)
    }

    pub fn new_fabsx(frd: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .with_xo_x(OPCODE_FABSX)
            .with_frd(frd)
            .with_frb(frb)
    }

    pub fn new_faddsx(frd: usize, fra: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED59)
            .with_xo_a(OPCODE_FADDSX)
            .with_frd(frd)
            .with_fra(fra)
            .with_frb(frb)
    }

    pub fn new_faddx(frd: usize, fra: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .with_xo_a(OPCODE_FADDX)
            .with_frd(frd)
            .with_fra(fra)
            .with_frb(frb)
    }

    pub fn new_fcmpo(crfd: u32, fra: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .with_xo_x(OPCODE_FCMPO)
            .with_crfd(crfd)
            .with_fra(fra)
            .with_frb(frb)
    }

    pub fn new_fcmpu(crfd: u32, fra: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .with_xo_x(OPCODE_FCMPU)
            .with_crfd(crfd)
            .with_fra(fra)
            .with_frb(frb)
    }

    pub fn new_fctiwzx(frd: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .with_xo_x(OPCODE_FCTIWZX)
            .with_frd(frd)
            .with_frb(frb)
    }

    pub fn new_fctiwx(frd: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .with_xo_x(OPCODE_FCTIWX)
            .with_frd(frd)
            .with_frb(frb)
    }

    pub fn new_fdivsx(frd: usize, fra: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED59)
            .with_xo_a(OPCODE_FDIVSX)
            .with_frd(frd)
            .with_fra(fra)
            .with_frb(frb)
    }

    pub fn new_fdivx(frd: usize, fra: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .with_xo_a(OPCODE_FDIVX)
            .with_frd(frd)
            .with_fra(fra)
            .with_frb(frb)
    }

    pub fn new_fmaddsx(frd: usize, fra: usize, frc: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED59)
            .with_xo_a(OPCODE_FMADDSX)
            .with_frd(frd)
            .with_fra(fra)
            .with_frc(frc)
            .with_frb(frb)
    }

    pub fn new_fmaddx(frd: usize, fra: usize, frc: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .with_xo_a(OPCODE_FMADDX)
            .with_frd(frd)
            .with_fra(fra)
            .with_frc(frc)
            .with_frb(frb)
    }

    pub fn new_fmrx(frd: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .with_xo_x(OPCODE_FMRX)
            .with_frd(frd)
            .with_frb(frb)
    }

    pub fn new_fmsubsx(frd: usize, fra: usize, frc: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED59)
            .with_xo_a(OPCODE_FMSUBSX)
            .with_frd(frd)
            .with_fra(fra)
            .with_frc(frc)
            .with_frb(frb)
    }

    pub fn new_fmsubx(frd: usize, fra: usize, frc: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .with_xo_a(OPCODE_FMSUBX)
            .with_frd(frd)
            .with_fra(fra)
            .with_frc(frc)
            .with_frb(frb)
    }

    pub fn new_fmulsx(frd: usize, fra: usize, frc: usize) -> Self {
        Self::new(OPCODE_EXTENDED59)
            .with_xo_a(OPCODE_FMULSX)
            .with_frd(frd)
            .with_fra(fra)
            .with_frc(frc)
    }

    pub fn new_fmulx(frd: usize, fra: usize, frc: usize) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .with_xo_a(OPCODE_FMULX)
            .with_frd(frd)
            .with_fra(fra)
            .with_frc(frc)
    }

    pub fn new_fnabsx(frd: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .with_xo_x(OPCODE_FNABSX)
            .with_frd(frd)
            .with_frb(frb)
    }

    pub fn new_fnegx(frd: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .with_xo_x(OPCODE_FNEGX)
            .with_frd(frd)
            .with_frb(frb)
    }

    pub fn new_fnmaddsx(frd: usize, fra: usize, frc: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED59)
            .with_xo_a(OPCODE_FNMADDSX)
            .with_frd(frd)
            .with_fra(fra)
            .with_frc(frc)
            .with_frb(frb)
    }

    pub fn new_fnmaddx(frd: usize, fra: usize, frc: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .with_xo_a(OPCODE_FNMADDX)
            .with_frd(frd)
            .with_fra(fra)
            .with_frc(frc)
            .with_frb(frb)
    }

    pub fn new_fnmsubsx(frd: usize, fra: usize, frc: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED59)
            .with_xo_a(OPCODE_FNMSUBSX)
            .with_frd(frd)
            .with_fra(fra)
            .with_frc(frc)
            .with_frb(frb)
    }

    pub fn new_fnmsubx(frd: usize, fra: usize, frc: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .with_xo_a(OPCODE_FNMSUBX)
            .with_frd(frd)
            .with_fra(fra)
            .with_frc(frc)
            .with_frb(frb)
    }

    pub fn new_fresx(frd: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED59)
            .with_xo_a(OPCODE_FRESX)
            .with_frd(frd)
            .with_frb(frb)
    }

    pub fn new_frspx(frd: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .with_xo_x(OPCODE_FRSPX)
            .with_frd(frd)
            .with_frb(frb)
    }

    pub fn new_frsqrtex(frd: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .with_xo_a(OPCODE_FRSQRTEX)
            .with_frd(frd)
            .with_frb(frb)
    }

    pub fn new_fselx(frd: usize, fra: usize, frc: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .with_xo_a(OPCODE_FSELX)
            .with_frd(frd)
            .with_fra(fra)
            .with_frc(frc)
            .with_frb(frb)
    }

    pub fn new_fsubsx(frd: usize, fra: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED59)
            .with_xo_a(OPCODE_FSUBSX)
            .with_frd(frd)
            .with_fra(fra)
            .with_frb(frb)
    }

    pub fn new_ps_absx(frd: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .with_xo_x(OPCODE_PS_ABSX)
            .with_frd(frd)
            .with_frb(frb)
    }

    pub fn new_ps_addx(frd: usize, fra: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .with_xo_a(OPCODE_PS_ABSX)
            .with_frd(frd)
            .with_fra(fra)
            .with_frb(frb)
    }

    pub fn new_ps_cmpo0(crfd: u32, fra: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .with_xo_x(OPCODE_PS_CMPO0)
            .with_crfd(crfd)
            .with_fra(fra)
            .with_frb(frb)
    }

    pub fn new_ps_cmpo1(crfd: u32, fra: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .with_xo_x(OPCODE_PS_CMPO1)
            .with_crfd(crfd)
            .with_fra(fra)
            .with_frb(frb)
    }

    pub fn new_ps_cmpu0(crfd: u32, fra: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .with_xo_x(OPCODE_PS_CMPU0)
            .with_crfd(crfd)
            .with_fra(fra)
            .with_frb(frb)
    }

    pub fn new_ps_cmpu1(crfd: u32, fra: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .with_xo_x(OPCODE_PS_CMPU1)
            .with_crfd(crfd)
            .with_fra(fra)
            .with_frb(frb)
    }

    pub fn new_ps_divx(frd: usize, fra: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .with_xo_a(OPCODE_PS_DIVX)
            .with_frd(frd)
            .with_fra(fra)
            .with_frb(frb)
    }

    pub fn new_ps_maddx(frd: usize, fra: usize, frc: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .with_xo_a(OPCODE_PS_MADDX)
            .with_frd(frd)
            .with_fra(fra)
            .with_frc(frc)
            .with_frb(frb)
    }

    pub fn new_ps_madds0x(frd: usize, fra: usize, frc: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .with_xo_a(OPCODE_PS_MADDS0X)
            .with_frd(frd)
            .with_fra(fra)
            .with_frc(frc)
            .with_frb(frb)
    }

    pub fn new_ps_madds1x(frd: usize, fra: usize, frc: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .with_xo_a(OPCODE_PS_MADDS1X)
            .with_frd(frd)
            .with_fra(fra)
            .with_frc(frc)
            .with_frb(frb)
    }

    pub fn new_ps_merge00x(frd: usize, fra: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .with_xo_a(OPCODE_PS_MERGE_00X)
            .with_frd(frd)
            .with_fra(fra)
            .with_frb(frb)
    }

    pub fn new_ps_merge01x(frd: usize, fra: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .with_xo_a(OPCODE_PS_MERGE_01X)
            .with_frd(frd)
            .with_fra(fra)
            .with_frb(frb)
    }

    pub fn new_ps_merge10x(frd: usize, fra: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .with_xo_a(OPCODE_PS_MERGE_10X)
            .with_frd(frd)
            .with_fra(fra)
            .with_frb(frb)
    }

    pub fn new_ps_merge11x(frd: usize, fra: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .with_xo_a(OPCODE_PS_MERGE_11X)
            .with_frd(frd)
            .with_fra(fra)
            .with_frb(frb)
    }

    pub fn new_ps_mrx(frd: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .with_xo_a(OPCODE_PS_MRX)
            .with_frd(frd)
            .with_frb(frb)
    }

    pub fn new_ps_msubx(frd: usize, fra: usize, frc: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .with_xo_a(OPCODE_PS_MSUBX)
            .with_frd(frd)
            .with_fra(fra)
            .with_frc(frc)
            .with_frb(frb)
    }

    pub fn new_ps_mulx(frd: usize, fra: usize, frc: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .with_xo_a(OPCODE_PS_MULX)
            .with_frd(frd)
            .with_fra(fra)
            .with_frc(frc)
    }

    pub fn new_ps_muls0x(frd: usize, fra: usize, frc: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .with_xo_a(OPCODE_PS_MULS0X)
            .with_frd(frd)
            .with_fra(fra)
            .with_frc(frc)
    }

    pub fn new_ps_muls1x(frd: usize, fra: usize, frc: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .with_xo_a(OPCODE_PS_MULS1X)
            .with_frd(frd)
            .with_fra(fra)
            .with_frc(frc)
    }

    pub fn new_ps_nabsx(frd: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .with_xo_a(OPCODE_PS_NABSX)
            .with_frd(frd)
            .with_frb(frb)
    }

    pub fn new_ps_negx(frd: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .with_xo_a(OPCODE_PS_NEGX)
            .with_frd(frd)
            .with_frb(frb)
    }

    pub fn new_ps_nmaddx(frd: usize, fra: usize, frc: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .with_xo_a(OPCODE_PS_NMADDX)
            .with_frd(frd)
            .with_fra(fra)
            .with_frc(frc)
            .with_frb(frb)
    }

    pub fn new_ps_nmsubx(frd: usize, fra: usize, frc: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .with_xo_a(OPCODE_PS_NMSUBX)
            .with_frd(frd)
            .with_fra(fra)
            .with_frc(frc)
            .with_frb(frb)
    }

    pub fn new_ps_resx(frd: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .with_xo_a(OPCODE_PS_RESX)
            .with_frd(frd)
            .with_frb(frb)
    }

    pub fn new_ps_rsqrtex(frd: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .with_xo_a(OPCODE_PS_RSQRTEX)
            .with_frd(frd)
            .with_frb(frb)
    }

    pub fn new_ps_selx(frd: usize, fra: usize, frc: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .with_xo_a(OPCODE_PS_SELX)
            .with_frd(frd)
            .with_fra(fra)
            .with_frc(frc)
            .with_frb(frb)
    }

    pub fn new_ps_subx(frd: usize, fra: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .with_xo_a(OPCODE_PS_SUBX)
            .with_frd(frd)
            .with_fra(fra)
            .with_frb(frb)
    }

    pub fn new_ps_sum0x(frd: usize, fra: usize, frc: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .with_xo_a(OPCODE_PS_SUM0X)
            .with_frd(frd)
            .with_fra(fra)
            .with_frc(frc)
            .with_frb(frb)
    }

    pub fn new_ps_sum1x(frd: usize, fra: usize, frc: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .with_xo_a(OPCODE_PS_SUM1X)
            .with_frd(frd)
            .with_fra(fra)
            .with_frc(frc)
            .with_frb(frb)
    }

    pub fn new_fsubx(frd: usize, fra: usize, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .with_xo_a(OPCODE_FSUBX)
            .with_frd(frd)
            .with_fra(fra)
            .with_frb(frb)
    }

    pub fn new_mcrfs(crfd: u32, crfs: u32) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .with_xo_x(OPCODE_MCRFS)
            .with_crfd(crfd)
            .with_crfs(crfs)
    }

    pub fn new_mffsx(frd: usize) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .with_xo_x(OPCODE_MFFSX)
            .with_frd(frd)
    }

    pub fn new_mtfsb0x(crbd: u32) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .with_xo_x(OPCODE_MTFSB0X)
            .with_crbd(crbd)
    }

    pub fn op_mtfsb1x(crbd: u32) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .with_xo_x(OPCODE_MTFSB1X)
            .with_crbd(crbd)
    }

    pub fn op_mtfsfix(crbd: u32, imm: u32) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .with_xo_x(OPCODE_MTFSFIX)
            .with_crbd(crbd)
            .with_imm(imm)
    }

    pub fn op_mtfsfx(fm: u32, frb: usize) -> Self {
        Self::new(OPCODE_EXTENDED63)
            .with_xo_x(OPCODE_MTFSFX)
            .with_fm(fm)
            .with_frb(frb)
    }

    pub fn new_addcx(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_xo(OPCODE_ADDCX)
            .with_rd(rd)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_addx(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_xo(OPCODE_ADDX)
            .with_rd(rd)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_addi(rd: usize, ra: usize, simm: u32) -> Self {
        Self::new(OPCODE_ADDI)
            .with_rd(rd)
            .with_ra(ra)
            .with_simm(simm)
    }

    pub fn new_addic(rd: usize, ra: usize, simm: u32) -> Self {
        Self::new(OPCODE_ADDIC)
            .with_rd(rd)
            .with_ra(ra)
            .with_simm(simm)
    }

    pub fn new_addic_rc(rd: usize, ra: usize, simm: u32) -> Self {
        Self::new(OPCODE_ADDIC_RC)
            .with_rd(rd)
            .with_ra(ra)
            .with_simm(simm)
    }

    pub fn new_addis(rd: usize, ra: usize, simm: u32) -> Self {
        Self::new(OPCODE_ADDIS)
            .with_rd(rd)
            .with_ra(ra)
            .with_simm(simm)
    }

    pub fn new_addmex(rd: usize, ra: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_xo(OPCODE_ADDMEX)
            .with_rd(rd)
            .with_ra(ra)
    }

    pub fn new_addex(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_xo(OPCODE_ADDEX)
            .with_rd(rd)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_addzex(rd: usize, ra: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_xo(OPCODE_ADDZEX)
            .with_rd(rd)
            .with_ra(ra)
    }

    pub fn new_andcx(ra: usize, rs: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_ANDCX)
            .with_ra(ra)
            .with_rs(rs)
            .with_rb(rb)
    }

    pub fn new_andi_rc(ra: usize, rs: usize, uimm: u32) -> Self {
        Self::new(OPCODE_ANDI_RC)
            .with_ra(ra)
            .with_rs(rs)
            .with_uimm(uimm)
    }

    pub fn new_andis_rc(ra: usize, rs: usize, uimm: u32) -> Self {
        Self::new(OPCODE_ANDIS_RC)
            .with_ra(ra)
            .with_rs(rs)
            .with_uimm(uimm)
    }

    pub fn new_andx(ra: usize, rs: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_ANDX)
            .with_ra(ra)
            .with_rs(rs)
            .with_rb(rb)
    }

    pub fn new_cmp(crfd: u32, l: u32, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_CMP)
            .with_crfd(crfd)
            .with_l(l)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_cmpi(crfd: u32, l: u32, ra: usize, simm: u32) -> Self {
        Self::new(OPCODE_CMPI)
            .with_crfd(crfd)
            .with_l(l)
            .with_ra(ra)
            .with_simm(simm)
    }

    pub fn new_cmpl(crfd: u32, l: u32, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_CMPL)
            .with_crfd(crfd)
            .with_l(l)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_cmpli(crfd: u32, l: u32, ra: usize, uimm: u32) -> Self {
        Self::new(OPCODE_CMPLI)
            .with_crfd(crfd)
            .with_l(l)
            .with_ra(ra)
            .with_uimm(uimm)
    }

    pub fn new_cntlzwx(ra: usize, rs: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_CNTLZWX)
            .with_ra(ra)
            .with_rs(rs)
    }

    pub fn new_divwux(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_xo(OPCODE_DIVWUX)
            .with_rd(rd)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_divwx(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_xo(OPCODE_DIVWX)
            .with_rd(rd)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_eqvx(ra: usize, rs: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_EQVX)
            .with_ra(ra)
            .with_rs(rs)
            .with_rb(rb)
    }

    pub fn new_extsbx(ra: usize, rs: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_EXTSBX)
            .with_ra(ra)
            .with_rs(rs)
    }

    pub fn new_extshx(ra: usize, rs: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_EXTSHX)
            .with_ra(ra)
            .with_rs(rs)
    }

    pub fn new_mulhwux(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_xo(OPCODE_MULHWUX)
            // always 0
            .with_rd(rd)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_mulhwx(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_xo(OPCODE_MULHWX)
            // always 0
            .with_rd(rd)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_mulli(rd: usize, ra: usize, simm: u32) -> Self {
        Self::new(OPCODE_MULLI)
            .with_rd(rd)
            .with_ra(ra)
            .with_simm(simm)
    }

    pub fn new_mullwx(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_xo(OPCODE_MULLWX)
            .with_rd(rd)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_nandx(ra: usize, rs: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_NANDX)
            .with_ra(ra)
            .with_rs(rs)
            .with_rb(rb)
    }

    pub fn new_negx(rd: usize, ra: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_xo(OPCODE_NEGX)
            .with_rd(rd)
            .with_ra(ra)
    }

    pub fn new_norx(ra: usize, rs: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_NORX)
            .with_ra(ra)
            .with_rs(rs)
            .with_rb(rb)
    }

    pub fn new_orx(ra: usize, rs: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_ORX)
            .with_ra(ra)
            .with_rs(rs)
            .with_rb(rb)
    }

    pub fn new_orcx(ra: usize, rs: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_ORCX)
            .with_ra(ra)
            .with_rs(rs)
            .with_rb(rb)
    }

    pub fn new_ori(ra: usize, rs: usize, uimm: u32) -> Self {
        Self::new(OPCODE_ORI)
            .with_ra(ra)
            .with_rs(rs)
            .with_uimm(uimm)
    }

    pub fn new_oris(ra: usize, rs: usize, uimm: u32) -> Self {
        Self::new(OPCODE_ORIS)
            .with_ra(ra)
            .with_rs(rs)
            .with_uimm(uimm)
    }

    pub fn new_rlwimix(ra: usize, rs: usize, sh: u32, mb: u32, me: u32) -> Self {
        Self::new(OPCODE_RLWIMIX)
            .with_ra(ra)
            .with_rs(rs)
            .with_sh(sh)
            .with_mb(mb)
            .with_me(me)
    }

    pub fn new_rlwinmx(ra: usize, rs: usize, sh: u32, mb: u32, me: u32) -> Self {
        Self::new(OPCODE_RLWINMX)
            .with_ra(ra)
            .with_rs(rs)
            .with_sh(sh)
            .with_mb(mb)
            .with_me(me)
    }

    pub fn new_rlwnmx(ra: usize, rs: usize, rb: usize, mb: u32, me: u32) -> Self {
        Self::new(OPCODE_RLWNMX)
            .with_ra(ra)
            .with_rs(rs)
            .with_rb(rb)
            .with_mb(mb)
            .with_me(me)
    }

    pub fn new_slwx(ra: usize, rs: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_SLWX)
            .with_ra(ra)
            .with_rs(rs)
            .with_rb(rb)
    }

    pub fn new_srawix(ra: usize, rs: usize, sh: u32) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_SRAWIX)
            .with_ra(ra)
            .with_rs(rs)
            .with_sh(sh)
    }

    pub fn new_srawx(ra: usize, rs: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_SRAWX)
            .with_ra(ra)
            .with_rs(rs)
            .with_rb(rb)
    }

    pub fn new_srwx(ra: usize, rs: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_SRWX)
            .with_ra(ra)
            .with_rs(rs)
            .with_rb(rb)
    }

    pub fn new_subfcx(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_xo(OPCODE_SUBFCX)
            .with_rd(rd)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_subfex(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_xo(OPCODE_SUBFEX)
            .with_rd(rd)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_subfic(rd: usize, ra: usize, simm: u32) -> Self {
        Self::new(OPCODE_SUBFIC)
            .with_rd(rd)
            .with_ra(ra)
            .with_simm(simm)
    }

    pub fn new_subfmex(rd: usize, ra: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_xo(OPCODE_SUBFMEX)
            .with_rd(rd)
            .with_ra(ra)
    }

    pub fn new_subfzex(rd: usize, ra: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_xo(OPCODE_SUBFZEX)
            .with_rd(rd)
            .with_ra(ra)
    }

    pub fn new_subfx(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_xo(OPCODE_SUBFX)
            .with_rd(rd)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_tw(to: u32, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_TW)
            .with_to(to)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_twi(to: u32, ra: usize, simm: u32) -> Self {
        Self::new(OPCODE_TWI)
            .with_to(to)
            .with_ra(ra)
            .with_simm(simm)
    }

    pub fn new_xori(ra: usize, rs: usize, uimm: u32) -> Self {
        Self::new(OPCODE_XORI)
            .with_ra(ra)
            .with_rs(rs)
            .with_uimm(uimm)
    }

    pub fn new_xoris(ra: usize, rs: usize, uimm: u32) -> Self {
        Self::new(OPCODE_XORIS)
            .with_ra(ra)
            .with_rs(rs)
            .with_uimm(uimm)
    }

    pub fn new_xorx(ra: usize, rs: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_XORX)
            .with_ra(ra)
            .with_rs(rs)
            .with_rb(rb)
    }

    pub fn new_dcbf(ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_DCBF)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_dcbi(ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_DCBI)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_dcbst(ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_DCBST)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_dcbt(ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_DCBT)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_dcbtst(ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_DCBTST)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_dcbz(ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_DCBZ)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_dcbz_l(ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .with_xo_x(OPCODE_DCBZ_L)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_eciwx(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_ECIWX)
            .with_rd(rd)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_ecowx(rs: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_ECOWX)
            .with_rs(rs)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_icbi(ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_ICBI)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_lbz(rd: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_LBZ).with_rd(rd).with_ra(ra).with_uimm(d)
    }

    pub fn new_lbzu(rd: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_LBZU).with_rd(rd).with_ra(ra).with_uimm(d)
    }

    pub fn new_lbzux(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_LBZUX)
            .with_rd(rd)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_lbzx(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_LBZX)
            .with_rd(rd)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_lfd(frd: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_LFD).with_frd(frd).with_ra(ra).with_uimm(d)
    }

    pub fn new_lfdu(frd: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_LFDU)
            .with_frd(frd)
            .with_ra(ra)
            .with_uimm(d)
    }

    pub fn new_lfdux(frd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_LFDUX)
            .with_frd(frd)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_lfdx(frd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_LFDX)
            .with_frd(frd)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_lfs(frd: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_LFS).with_frd(frd).with_ra(ra).with_uimm(d)
    }

    pub fn new_lfsu(frd: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_LFSU)
            .with_frd(frd)
            .with_ra(ra)
            .with_uimm(d)
    }

    pub fn new_lfsux(frd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_LFSUX)
            .with_frd(frd)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_lfsx(frd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_LFSX)
            .with_frd(frd)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_lha(rd: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_LHA).with_rd(rd).with_ra(ra).with_uimm(d)
    }

    pub fn new_lhau(rd: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_LHAU).with_rd(rd).with_ra(ra).with_uimm(d)
    }

    pub fn new_lhaux(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_LHAUX)
            .with_rd(rd)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_lhax(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_LHAX)
            .with_rd(rd)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_lhbrx(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_LHBRX)
            .with_rd(rd)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_lhz(rd: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_LHZ).with_rd(rd).with_ra(ra).with_uimm(d)
    }

    pub fn new_lhzu(rd: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_LHZU).with_rd(rd).with_ra(ra).with_uimm(d)
    }

    pub fn new_lhzux(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_LHZUX)
            .with_rd(rd)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_lhzx(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_LHZX)
            .with_rd(rd)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_lmw(rd: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_LMW).with_rd(rd).with_ra(ra).with_uimm(d)
    }

    pub fn new_lswi(rd: usize, ra: usize, nb: u32) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_LSWI)
            .with_rd(rd)
            .with_ra(ra)
            .with_nb(nb)
    }

    pub fn new_lswx(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_LSWI)
            .with_rd(rd)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_lwarx(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_LWARX)
            .with_rd(rd)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_lwbrx(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_LWBRX)
            .with_rd(rd)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_lwz(rd: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_LWZ).with_rd(rd).with_ra(ra).with_uimm(d)
    }

    pub fn new_lwzu(rd: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_LWZU).with_rd(rd).with_ra(ra).with_uimm(d)
    }

    pub fn new_lwzux(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_LWZUX)
            .with_rd(rd)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_lwzx(rd: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_LWZX)
            .with_rd(rd)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_psq_l(frd: usize, ra: usize, d: u32, w: u32, i: u32) -> Self {
        Self::new(OPCODE_PSQ_L)
            .with_frd(frd)
            .with_ra(ra)
            .with_uimm_1(d)
            .with_w(w)
            .with_i(i)
    }

    pub fn new_psq_lu(frd: usize, ra: usize, d: u32, w: u32, i: u32) -> Self {
        Self::new(OPCODE_PSQ_LU)
            .with_frd(frd)
            .with_ra(ra)
            .with_uimm_1(d)
            .with_w(w)
            .with_i(i)
    }

    pub fn new_psq_lux(frd: usize, ra: usize, rb: usize, w: u32, i: u32) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .with_xo_a(OPCODE_PSQ_LUX)
            .with_frd(frd)
            .with_ra(ra)
            .with_rb(rb)
            .with_w(w)
            .with_i(i)
    }

    pub fn new_psq_lx(frd: usize, ra: usize, rb: usize, w: u32, i: u32) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .with_xo_a(OPCODE_PSQ_LX)
            .with_frd(frd)
            .with_ra(ra)
            .with_rb(rb)
            .with_w(w)
            .with_i(i)
    }

    pub fn new_psq_st(frs: usize, ra: usize, d: u32, w: u32, i: u32) -> Self {
        Self::new(OPCODE_PSQ_ST)
            .with_frs(frs)
            .with_ra(ra)
            .with_uimm_1(d)
            .with_w(w)
            .with_i(i)
    }

    pub fn new_psq_stu(frs: usize, ra: usize, d: u32, w: u32, i: u32) -> Self {
        Self::new(OPCODE_PSQ_STU)
            .with_frs(frs)
            .with_ra(ra)
            .with_uimm_1(d)
            .with_w(w)
            .with_i(i)
    }

    pub fn new_psq_stux(frs: usize, ra: usize, rb: usize, w: u32, i: u32) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .with_xo_a(OPCODE_PSQ_STUX)
            .with_frs(frs)
            .with_ra(ra)
            .with_rb(rb)
            .with_w(w)
            .with_i(i)
    }

    pub fn new_psq_stx(frs: usize, ra: usize, rb: usize, w: u32, i: u32) -> Self {
        Self::new(OPCODE_EXTENDED4)
            .with_xo_a(OPCODE_PSQ_STX)
            .with_frs(frs)
            .with_ra(ra)
            .with_rb(rb)
            .with_w(w)
            .with_i(i)
    }

    pub fn new_stb(rs: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_STB).with_rs(rs).with_ra(ra).with_uimm(d)
    }

    pub fn new_stbu(rs: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_STBU).with_rs(rs).with_ra(ra).with_uimm(d)
    }

    pub fn new_stbux(rs: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_STBUX)
            .with_rs(rs)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_stbx(rs: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_STBX)
            .with_rs(rs)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_stfd(frs: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_STFD)
            .with_frs(frs)
            .with_ra(ra)
            .with_uimm(d)
    }

    pub fn new_stfdu(frs: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_STFDU)
            .with_frs(frs)
            .with_ra(ra)
            .with_uimm(d)
    }

    pub fn new_stfdux(frs: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_STFDUX)
            .with_frs(frs)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_stfdx(frs: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_STFDX)
            .with_frs(frs)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_stfiwx(frs: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_STFIWX)
            .with_frs(frs)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_stfs(frs: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_STFS)
            .with_frs(frs)
            .with_ra(ra)
            .with_uimm(d)
    }

    pub fn new_stfsu(frs: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_STFSU)
            .with_frs(frs)
            .with_ra(ra)
            .with_uimm(d)
    }

    pub fn new_stfsux(frs: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_STFSUX)
            .with_frs(frs)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_stfsx(frs: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_STFSX)
            .with_frs(frs)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_sth(rs: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_STH).with_rs(rs).with_ra(ra).with_uimm(d)
    }

    pub fn new_sthbrx(rs: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_STHBRX)
            .with_rs(rs)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_sthu(rs: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_STHU).with_rs(rs).with_ra(ra).with_uimm(d)
    }

    pub fn new_sthux(rs: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_STHUX)
            .with_rs(rs)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_sthx(rs: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_STHX)
            .with_rs(rs)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_stmw(rs: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_STMW).with_rs(rs).with_ra(ra).with_uimm(d)
    }

    pub fn new_stswi(rs: usize, ra: usize, nb: u32) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_STSWI)
            .with_rs(rs)
            .with_ra(ra)
            .with_nb(nb)
    }

    pub fn new_stswx(rs: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_STSWX)
            .with_rs(rs)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_stw(rs: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_STW).with_rs(rs).with_ra(ra).with_uimm(d)
    }

    pub fn new_stwbrx(rs: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_STWBRX)
            .with_rs(rs)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_stwcx_rc(rs: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_STWCX_RC)
            .with_rs(rs)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_stwu(rs: usize, ra: usize, d: u32) -> Self {
        Self::new(OPCODE_STWU).with_rs(rs).with_ra(ra).with_uimm(d)
    }

    pub fn new_stwux(rs: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_STWUX)
            .with_rs(rs)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_stwx(rs: usize, ra: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_STWX)
            .with_rs(rs)
            .with_ra(ra)
            .with_rb(rb)
    }

    pub fn new_tlbie(rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_TBLIE)
            .with_rb(rb)
    }

    pub fn new_eieio() -> Self {
        Self::new(OPCODE_EXTENDED31).with_xo_x(OPCODE_EIEIO)
    }

    pub fn new_isync() -> Self {
        Self::new(OPCODE_EXTENDED19).with_xo_x(OPCODE_ISYNC)
    }

    pub fn new_mfmsr(rd: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_MFMSR)
            .with_rd(rd)
    }

    pub fn new_mfspr(rd: usize, spr: u32) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_MFMSR)
            .with_rd(rd)
            .with_spr(spr)
    }

    pub fn new_mfsr(rd: usize, sr: u32) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_MFSR)
            .with_rd(rd)
            .with_sr(sr)
    }

    pub fn new_mfsrin(rd: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_MFSRIN)
            .with_rd(rd)
            .with_rb(rb)
    }

    pub fn new_mftb(rd: usize, tbr: u32) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_MFTB)
            .with_rd(rd)
            .with_tbr(tbr)
    }

    pub fn new_mtmsr(rs: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_MTMSR)
            .with_rs(rs)
    }

    pub fn new_mtspr(spr: u32, rs: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_MTSPR)
            .with_spr(spr)
            .with_rs(rs)
    }

    pub fn new_mtsr(sr: u32, rs: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_MTSR)
            .with_sr(sr)
            .with_rs(rs)
    }

    pub fn new_mtsrin(rs: usize, rb: usize) -> Self {
        Self::new(OPCODE_EXTENDED31)
            .with_xo_x(OPCODE_MTSRIN)
            .with_rs(rs)
            .with_rb(rb)
    }

    pub fn new_rfi() -> Self {
        Self::new(OPCODE_EXTENDED19).with_xo_x(OPCODE_RFI)
    }

    pub fn new_sc() -> Self {
        Self::new(OPCODE_SC)
    }

    pub fn new_sync() -> Self {
        Self::new(OPCODE_EXTENDED31).with_xo_x(OPCODE_SYNC)
    }

    pub fn new_tlbsync() -> Self {
        Self::new(OPCODE_EXTENDED31).with_xo_x(OPCODE_TLBSYNC)
    }
}
