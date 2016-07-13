
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
    // RN
}


