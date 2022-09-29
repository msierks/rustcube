// Floating-Point Status and Control Register

#[derive(Default, Debug)]
pub struct Fpscr {
    // FX
    fp_exception: bool,

    // FEX
    fp_enabled: bool,

    // VX
    invalid_operation_exception: bool,

    // OX
    overflow_exception: bool,

    // UX
    underflow_exception: bool,

    // ZX
    zero_divid_exception: bool,

    // XX
    inexact_exception: bool,

    // VXSNAN
    // VXISI
    // VXIDI
    // VXZDZ
    // VXIMZ
    // VXVC
    // FR
    // FI
    // FPRF
    // VXSOFT
    // VXSQRT
    // VXCVI
    // VE
    // OE
    // UE
    // ZE
    // XE
    // NI
    non_ieee_mode: bool,
    // RN
}

impl Fpscr {
    pub fn set_bit(&mut self, bit: u8, value: bool) {
        match bit {
            29 => self.non_ieee_mode = value,
            _ => panic!("Unhandled Fpscr.set_bit {:}", bit),
        }
    }
}
