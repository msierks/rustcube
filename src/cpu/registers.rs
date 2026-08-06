#![allow(dead_code)]

pub const SPR_XER: usize = 1;
pub const SPR_LR: usize = 8;
pub const SPR_CTR: usize = 9;
pub const SPR_DSISR: usize = 18;
pub const SPR_DAR: usize = 19;
pub const SPR_DEC: usize = 22;
pub const SPR_SDR1: usize = 25;
pub const SPR_SRR0: usize = 26;
pub const SPR_SRR1: usize = 27;
pub const SPR_SPRG0: usize = 272;
pub const SPR_EAR: usize = 282;
pub const SPR_TBL: usize = 284;
pub const SPR_TBU: usize = 285;
pub const SPR_PVR: usize = 287;
pub const SPR_IBAT0U: usize = 528;
pub const SPR_IBAT0L: usize = 529;
pub const SPR_IBAT1U: usize = 530;
pub const SPR_IBAT1L: usize = 531;
pub const SPR_IBAT2U: usize = 532;
pub const SPR_IBAT2L: usize = 533;
pub const SPR_IBAT3U: usize = 534;
pub const SPR_IBAT3L: usize = 535;
pub const SPR_DBAT0U: usize = 536;
pub const SPR_DBAT0L: usize = 537;
pub const SPR_DBAT1U: usize = 538;
pub const SPR_DBAT1L: usize = 539;
pub const SPR_DBAT2U: usize = 540;
pub const SPR_DBAT2L: usize = 541;
pub const SPR_DBAT3U: usize = 542;
pub const SPR_DBAT3L: usize = 543;
pub const SPR_GQR0: usize = 912;
pub const SPR_HID2: usize = 920;
pub const SPR_WPAR: usize = 921;
pub const SPR_DMAU: usize = 922;
pub const SPR_UMMCR0: usize = 936;
pub const SPR_UPMC1: usize = 937;
pub const SPR_UPMC2: usize = 938;
pub const SPR_USIA: usize = 939;
pub const SPR_UMMCR1: usize = 940;
pub const SPR_UPMC3: usize = 941;
pub const SPR_UPMC4: usize = 942;
pub const SPR_MMCR0: usize = 952;
pub const SPR_PMC1: usize = 953;
pub const SPR_PMC2: usize = 954;
pub const SPR_SIA: usize = 955;
pub const SPR_MMCR1: usize = 956;
pub const SPR_PMC3: usize = 957;
pub const SPR_PMC4: usize = 958;
pub const SPR_IABR: usize = 1010;
pub const SPR_HID0: usize = 1008;
pub const SPR_HID1: usize = 1009;
pub const SPR_DABR: usize = 1013;
pub const SPR_L2CR: usize = 1017;
pub const SPR_ICTC: usize = 1019;
pub const SPR_THRM1: usize = 1020;

pub const TBR_TBL: usize = 268;
pub const TBR_TBU: usize = 269;

#[derive(Default, Debug)]
pub struct ConditionRegister(u32);

impl ConditionRegister {
    pub fn as_u32(&self) -> u32 {
        self.0
    }

    pub fn set(&mut self, value: u32) {
        self.0 = value;
    }

    pub fn set_field(&mut self, field: usize, value: u32) {
        self.0 = (self.0 & (!(0xF0000000 >> (field * 4)))) | (value << ((7 - field) * 4));
    }

    pub fn get_bit(&self, bit: usize) -> u8 {
        ((self.0 >> (31 - bit)) & 1) as u8
    }

    pub fn set_bit(&mut self, bit: usize, value: u8) {
        self.0 = ((value as u32) << (31 - bit)) | (self.0 & !(0x8000_0000 >> bit));
    }

    pub fn get_cr0(&mut self) -> u8 {
        (self.0 >> 28) as u8
    }
}

impl From<u32> for ConditionRegister {
    fn from(v: u32) -> Self {
        ConditionRegister(v)
    }
}

#[derive(Default, Clone)]
pub struct Fpr {
    ps0: u64,
    ps1: u64,
}

impl Fpr {
    pub fn ps0(&self) -> u64 {
        self.ps0
    }

    pub fn ps1(&self) -> u64 {
        self.ps1
    }

    pub fn set_ps0(&mut self, v: u64) {
        self.ps0 = v;
    }

    pub fn set_ps1(&mut self, v: u64) {
        self.ps1 = v;
    }

    pub fn set_ps0_f64(&mut self, v: f64) {
        self.ps0 = f64::to_bits(v);
    }

    pub fn set_ps1_f64(&mut self, v: f64) {
        self.ps1 = f64::to_bits(v);
    }

    pub fn ps0_as_f64(&self) -> f64 {
        f64::from_bits(self.ps0)
    }

    pub fn ps1_as_f64(&self) -> f64 {
        f64::from_bits(self.ps1)
    }
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    pub struct FloatingPointStatusControlRegister(u32);
    impl Debug;
    u32;
    pub rn, _ : 1, 0;            // Floating-point rounding control
    pub ni, _ : 2;               // Floating-point non-IEEE mode
    pub xe, _ : 3;               // Floating-point inexact exception enable
    pub ze, _ : 4;               // IEEE floating-point zero divide exception enable
    pub ue, _ : 5;               // IEEE floating-point underflow exception enable
    pub oe, _ : 6;               // IEEE floating-point overflow exception enable
    pub ve, _ : 7;               // Floating-point invalid operation exception enable
    pub vxcvi, _ : 8;            // Floating-point invalid operation exception for invalid integer convert
    pub vxsqrt, set_vxsqrt : 9;  // Floating-point invalid operation exception for invalid square root
    pub vxsoft, _ : 10;          // Floating-point invalid operation exceptions for woftware request
    pub fprf, set_fprf : 16, 12; // Floating-point result flags
    pub fpcc, set_fpcc : 15, 12; // Floating-point condition code
    pub fi, _ : 17;              // Floating-point fraction inexact
    pub fr, _ : 18;              // Floating-point fraction round
    pub vxvc, set_vxvc : 19;     // Floating-point invalid operation exception for invalid compare
    pub vximz, set_vximz : 20;   // Floating-point invalid operation exception for (inf) * 0
    pub vxzdz, set_vxzdz : 21;   // Floating-point invalid operation exception for 0 / 0
    pub vxidi, set_vxidi : 22;   // Floating-point invalid operation exception for (inf) / (inf)
    pub vxisi, _ : 23;           // Floating-point invalid operation exception for (inf) - (inf)
    pub vxsnan, set_vxsnan : 24; // Floating-point invalid operation exception for SNaN
    pub xx, _ : 25;              // Floating-point inexact exception
    pub zx, set_zx : 26;         // Floating-point zero divide exception
    pub ux, _ : 27;              // Floating-point underflow exception
    pub ox, _ : 28;              // Floating-point overflow exception
    pub vx, _ : 29;              // Floating-point invalid operation exception summary
    pub fex, _ : 30;             // Floating-point enabled exception summary
    pub fx, _ : 31;              // Floating-point exception summary
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    pub struct Gqr(u32);
    impl Debug;
    u32;
    pub st, _ : 2, 0;
    pub ss, _ : 13, 8;
    pub lt, _ : 18, 16;
    pub ls, _ : 29, 24;
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    pub struct HardwareImplementationDependentRegister2(u32);
    impl Debug;
    pub dqoee, _ : 16;
    pub dcmee, _ : 17;
    pub dncee, _ : 18;
    pub dchee, _ : 19;
    pub dqoerr, _ : 20;
    pub dcmerr, _ : 21;
    pub dncerr, _ : 22;
    pub dcherr, _ : 23;
    pub dmaql, _ : 27, 24;
    pub lce, _ : 28;
    pub pse, _ : 29;
    pub wpe, _ : 30;
    pub lsqe, _ : 31;
}

impl From<u32> for HardwareImplementationDependentRegister2 {
    fn from(v: u32) -> Self {
        HardwareImplementationDependentRegister2(v)
    }
}

bitfield! {
    #[derive(Copy, Clone)]
    pub struct MachineStateRegister(u32);
    impl Debug;
    pub le, set_le : 0;    // Little-endian mode enable
    pub ri, _ : 1;         // System reset of machine check exception is recoverable
    pub pm, _ : 2;         // Performance monitor marked mode
    pub dr, _ : 4;         // Data address trranslation
    pub ir, _ : 5;         // Instruction address translation
    pub ip, _ : 6;         // Exception prefix
    pub fe1, _ : 8;        // IEEE floating-point exception mode 1
    pub be, _ : 9;         // Branch trace enable
    pub se, _ : 10;        // Single-step strace enable
    pub fe0, _ : 11;       // IEEE floating-point exception mode 0
    pub me, _ : 12;        // Machine check enable
    pub fp, _ : 13;        // Floating-point available
    pub pr, _ : 14;        // Privilege level
    pub ee, _ : 15;        // External interrupt enable
    pub ile, _ : 16;       // Exception little-endian mode
    pub pow, set_pow : 18; // Power management enable
}

impl From<u32> for MachineStateRegister {
    fn from(v: u32) -> Self {
        MachineStateRegister(v)
    }
}

impl From<MachineStateRegister> for u32 {
    fn from(v: MachineStateRegister) -> Self {
        v.0
    }
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    pub struct Xer(u32);
    impl Debug;
    pub byte_count, _ : 6, 0;
    pub carry, set_carry : 29;
    pub overflow, set_overflow : 30;
    pub summary_overflow, set_summary_overflow : 31;
}

impl From<u32> for Xer {
    fn from(v: u32) -> Self {
        Xer(v)
    }
}

impl From<Xer> for u32 {
    fn from(s: Xer) -> u32 {
        s.0
    }
}

/// Program-exception reasons
#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ProgramException {
    FloatingPointEnabled,
    IllegalInstruction,
    PrivilegedInstruction,
    Trap,
}

impl ProgramException {
    /// SRR1 reason bits.
    pub fn srr1_bits(self) -> u32 {
        match self {
            Self::FloatingPointEnabled => 1 << (31 - 11),
            Self::IllegalInstruction => 1 << (31 - 12),
            Self::PrivilegedInstruction => 1 << (31 - 13),
            Self::Trap => 1 << (31 - 14),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn condition_register() {
        let mut cr = ConditionRegister(0x00F0_F0F0);

        cr.set_bit(2, 1);
        assert_eq!(cr.0, 0x20F0_F0F0);
        assert_eq!(cr.get_bit(2), 1);

        cr.set_bit(2, 0);
        assert_eq!(cr.0, 0x00F0_F0F0);
        assert_eq!(cr.get_bit(2), 0);

        cr.set_field(0, 0xF);
        assert_eq!(cr.0, 0xF0F0_F0F0);

        cr.set_field(0, 0x3);
        assert_eq!(cr.0, 0x30F0_F0F0);

        cr.set_field(0, 0x0);
        assert_eq!(cr.0, 0x00F0_F0F0);
    }
}
