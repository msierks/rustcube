use crate::dsp::DspContext;

type OpcodeTableFunction = fn(&mut DspContext, u16);
type OpcodeTableItem = (usize, usize, Opcode, OpcodeTableFunction);

const OPTABLE_SIZE: usize = u16::MAX as usize + 1;

pub struct DspCpu {
    pc: u16,
    r: [u16; 4],
    m: [u16; 4],
    l: [u16; 4],
    psr: ProcessStatusRegister,
    x0: u16,
    x1: u16,
    y0: u16,
    y1: u16,
    a: AccumulatorRegister,
    b: AccumulatorRegister,
    dpp: u16,
    pcs: Vec<u16>,
    optable: Box<[OpcodeTableFunction]>,
}

impl Default for DspCpu {
    fn default() -> Self {
        let mut optable: [fn(&mut DspContext, u16); OPTABLE_SIZE] = [op_illegal; OPTABLE_SIZE];

        let mut i = 0;
        while i < optable.len() {
            for op in OPCODE_TABLE.iter() {
                if (i & op.1) == op.0 {
                    optable[i] = op.3;
                    break;
                }
            }
            i += 1;
        }

        DspCpu {
            pc: 0x8000,
            r: [0; 4],
            m: [0; 4],
            l: [0; 4],
            psr: ProcessStatusRegister(0x24),
            x0: 0,
            x1: 0,
            y0: 0,
            y1: 0,
            a: Default::default(),
            b: Default::default(),
            dpp: 0,
            pcs: Vec::with_capacity(4),
            optable: Box::new(optable),
        }
    }
}

impl DspCpu {
    pub fn reset(&mut self, pc: u16) {
        self.pc = pc;
        self.psr = ProcessStatusRegister(0);
        self.x0 = 0;
        self.x1 = 0;
        self.y0 = 0;
        self.y1 = 0;
        self.a = AccumulatorRegister(0);
        self.b = AccumulatorRegister(0);
        self.dpp = 0;
        self.pcs.clear();
    }
}

pub fn dsp_step(ctx: &mut DspContext) {
    let pc = ctx.cpu.pc;

    let instr = ctx.read_imem(pc);

    ctx.cpu.pc += 1;

    ctx.cpu.optable[instr as usize](ctx, instr);
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    pub struct ProcessStatusRegister(u16);
    impl Debug;
    pub c, set_c: 0;      // carry out
    pub v, set_v: 1;      // overflow
    pub z, set_z: 2;      // zero
    pub n, set_n: 3;      // negative
    pub e, set_e: 4;      // extension
    pub u, set_u: 5;      // unnormalization
    pub tb, set_tb: 6;    // test bit
    pub sv, set_sv: 7;    // sticky overflow
    pub te0, set_te0: 8;
    pub te1, set_te1: 9;  // ACRS/ACWE/DCRE enable
    pub te2, set_te2: 10; // AI Interrupt enable
    pub te3, set_te3: 11; // CPU Interrupt enable
    pub et, set_et: 12;   // global interrupt enable
    pub im, set_im: 13;   // integer/fraction mode
    pub xl, set_xl: 14;   // extension_limit mode
    pub dp, set_dp: 15;   // double precision mode
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    pub struct AccumulatorRegister(u64);
    impl Debug;
    u16;
    pub zero, set_zero: 15, 0;
    pub one, set_one: 31, 16;
    u8;
    pub two, set_two: 39, 32;
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Opcode {
    JmpDi,
    JmpIn,
    CallDi,
    CallIn,
    Rets,
    Reti,
    Trap,
    Wait,
    Exec,
    LoopLc,
    LoopR,
    RepRc,
    RepR,
    Pld,
    Nop,
    MrSingle,
    Adsi,
    Adli,
    Cmpsi,
    Cmpli,
    Lsfi,
    Asfi,
    Xoli,
    Anli,
    Orli,
    Norm,
    Div,
    Addc,
    Subc,
    Negc,
    Max,
    LsfNeg1,
    LsfNeg2,
    AsfNeg1,
    AsfNeg2,
    LdSingle,
    StSingle,
    Ldsa,
    Stsa,
    Ldla,
    Stla,
    MvSingle,
    Mvsi,
    Mvli,
    Stli,
    ClrC,
    SetC,
    Btstl,
    Btsth,
    Ass,
    Addl,
    Sub,
    Amv,
    Cmp1,
    Cmp2,
    Inc,
    Dec,
    Abs,
    Neg1,
    Neg2,
    ClrA,
    ClrP,
    Rnd,
    RndP,
    Tst1,
    Tst2,
    Tst3,
    Lsl16,
    Lsr16,
    Asr16,
    AddP,
    Nop2,
    ClrIm,
    ClrDp,
    ClrXl,
    SetIm,
    SetDp,
    SetXl,
    Mpy1,
    Mpy2,
    Mac1,
    Mac2,
    Mac3,
    MacNeg1,
    MacNeg2,
    MacNeg3,
    Mvmpy,
    Rnmpy,
    Admpy,
    Not,
    Xor1,
    Xor2,
    And1,
    And2,
    Or1,
    Or2,
    Lsf1,
    Lsf2,
    Asf1,
    Asf2,
    Ldd1,
    Ldd2,
    Ls1,
    Ls2,
    Ld,
    St,
    Mv,
    Mr,
    #[cfg(test)]
    Illegal,
}

const OPCODE_TABLE: [OpcodeTableItem; 118] = [
    (0x0290, 0xFFF0, Opcode::JmpDi, op_jmp_di),
    (0x1700, 0xFF90, Opcode::JmpIn, op_jmp_in),
    (0x02B0, 0xFFF0, Opcode::CallDi, op_call_di),
    (0x1710, 0xFF90, Opcode::CallIn, op_call_in),
    (0x02D0, 0xFFF0, Opcode::Rets, op_rets),
    (0x02F0, 0xFFF0, Opcode::Reti, op_reti),
    (0x0020, 0xFFFF, Opcode::Trap, op_trap),
    (0x0021, 0xFFFF, Opcode::Wait, op_wait),
    (0x0270, 0xFFF0, Opcode::Exec, op_exec),
    (0x1100, 0xFF00, Opcode::LoopLc, op_loop_lc),
    (0x0060, 0xFFE0, Opcode::LoopR, op_loop_r),
    (0x1000, 0xFF00, Opcode::RepRc, op_rep_rc),
    (0x0040, 0xFFE0, Opcode::RepR, op_rep_r),
    (0x0210, 0xFEF0, Opcode::Pld, op_pld),
    (0x0000, 0xFFFF, Opcode::Nop, op_nop),
    (0x0000, 0xFFE0, Opcode::MrSingle, op_mr_single),
    (0x0400, 0xFE00, Opcode::Adsi, op_adsi),
    (0x0200, 0xFEFF, Opcode::Adli, op_adli),
    (0x0600, 0xFFE0, Opcode::Cmpsi, op_cmpsi),
    (0x0280, 0xFEFF, Opcode::Cmpli, op_cmpli),
    (0x1400, 0xFE80, Opcode::Lsfi, op_lsfi),
    (0x1480, 0xFE80, Opcode::Asfi, op_asfi),
    (0x0220, 0xFEFF, Opcode::Xoli, op_xoli),
    (0x0240, 0xFEFF, Opcode::Anli, op_anli),
    (0x0260, 0xFEFF, Opcode::Orli, op_orli),
    (0x0204, 0xFEFC, Opcode::Norm, op_norm),
    (0x0208, 0xFE9F, Opcode::Div, op_div),
    (0x028C, 0xFEDF, Opcode::Addc, op_addc),
    (0x028D, 0xFEDF, Opcode::Subc, op_subc),
    (0x020D, 0xFEFF, Opcode::Negc, op_negc),
    (0x0209, 0xFE9F, Opcode::Max, op_max),
    (0x024A, 0xFEDF, Opcode::LsfNeg1, op_lsf_neg_1),
    (0x02CA, 0xFEFF, Opcode::LsfNeg2, op_lsf_neg_2),
    (0x024B, 0xFEDF, Opcode::AsfNeg1, op_asf_neg_1),
    (0x02CB, 0xFEFF, Opcode::AsfNeg2, op_asf_neg_2),
    (0x1800, 0xFE00, Opcode::LdSingle, op_ld_single),
    (0x1A00, 0xFE00, Opcode::StSingle, op_st_single),
    (0x2000, 0xF800, Opcode::Ldsa, op_ldsa),
    (0x2800, 0xF800, Opcode::Stsa, op_stsa),
    (0x00C0, 0xFFE0, Opcode::Ldla, op_ldla),
    (0x00E0, 0xFFE0, Opcode::Stla, op_stla),
    (0x1C00, 0xFC00, Opcode::MvSingle, op_mv_single),
    (0x0800, 0xF800, Opcode::Mvsi, op_mvsi),
    (0x0080, 0xFFE0, Opcode::Mvli, op_mvli),
    (0x1600, 0xFF00, Opcode::Stli, op_stli),
    (0x1200, 0xFFF8, Opcode::ClrC, op_clr_c),
    (0x1300, 0xFFF8, Opcode::SetC, op_set_c),
    (0x02A0, 0xFEFF, Opcode::Btstl, op_btstl),
    (0x02C0, 0xFEFF, Opcode::Btsth, op_btsth),
    (0x4000, 0xF000, Opcode::Ass, op_add),
    (0x7000, 0xFC00, Opcode::Addl, op_addl),
    (0x5000, 0xF000, Opcode::Sub, op_sub),
    (0x6000, 0xF000, Opcode::Amv, op_amv),
    (0xC100, 0xE700, Opcode::Cmp1, op_cmp_1),
    (0x8200, 0xFF00, Opcode::Cmp2, op_cmp_2),
    (0x7400, 0xFC00, Opcode::Inc, op_inc),
    (0x7800, 0xFC00, Opcode::Dec, op_dec),
    (0xA100, 0xF700, Opcode::Abs, op_abs),
    (0x7C00, 0xFE00, Opcode::Neg1, op_neg_1),
    (0x7E00, 0xFE00, Opcode::Neg2, op_neg_2),
    (0x8100, 0xF700, Opcode::ClrA, op_clr_a),
    (0x8400, 0xFF00, Opcode::ClrP, op_clr_p),
    (0xFC00, 0xFE00, Opcode::Rnd, op_rnd),
    (0xFE00, 0xFE00, Opcode::RndP, op_rndp),
    (0xB100, 0xF700, Opcode::Tst1, op_tst_1),
    (0x8600, 0xFE00, Opcode::Tst2, op_tst_2),
    (0x8500, 0xFF00, Opcode::Tst3, op_tst_3),
    (0xF000, 0xFE00, Opcode::Lsl16, op_lsl16),
    (0xF400, 0xFE00, Opcode::Lsr16, op_lsr16),
    (0x9100, 0xF700, Opcode::Asr16, op_asr16),
    (0xF800, 0xFC00, Opcode::AddP, op_addp),
    (0x8000, 0xFF00, Opcode::Nop2, op_nop2),
    (0x8A00, 0xFF00, Opcode::ClrIm, op_clr_im),
    (0x8C00, 0xFF00, Opcode::ClrDp, op_clr_dp),
    (0x8E00, 0xFF00, Opcode::ClrXl, op_clr_xl),
    (0x8B00, 0xFF00, Opcode::SetIm, op_set_im),
    (0x8D00, 0xFF00, Opcode::SetDp, op_set_dp),
    (0x8F00, 0xFF00, Opcode::SetXl, op_set_xl),
    (0x8000, 0x8700, Opcode::Mpy1, op_mpy_1),
    (0x8300, 0xFF00, Opcode::Mpy2, op_mpy_2),
    (0xE000, 0xFC00, Opcode::Mac1, op_mac_1),
    (0xE800, 0xFC00, Opcode::Mac2, op_mac_2),
    (0xF200, 0xFE00, Opcode::Mac3, op_mac_3),
    (0xE400, 0xFC00, Opcode::MacNeg1, op_mac_neg_1),
    (0xEC00, 0xFC00, Opcode::MacNeg2, op_mac_neg_2),
    (0xF600, 0xFE00, Opcode::MacNeg3, op_mac_neg_3),
    (0x8600, 0x8600, Opcode::Mvmpy, op_mvmpy),
    (0x8200, 0x8600, Opcode::Rnmpy, op_rnmpy),
    (0x8400, 0x8600, Opcode::Admpy, op_admpy),
    (0x3200, 0xFE80, Opcode::Not, op_not),
    (0x3000, 0xFC80, Opcode::Xor1, op_xor_1),
    (0x3080, 0xFE80, Opcode::Xor2, op_xor_2),
    (0x3400, 0xFC80, Opcode::And1, op_and_1),
    (0x3C00, 0xFE80, Opcode::And2, op_and_2),
    (0x3800, 0xFC80, Opcode::Or1, op_or_1),
    (0x3E00, 0xFE80, Opcode::Or2, op_or_2),
    (0x3480, 0xFC80, Opcode::Lsf1, op_lsf_1),
    (0x3C80, 0xFE80, Opcode::Lsf2, op_lsf_2),
    (0x3880, 0xFC80, Opcode::Asf1, op_asf_1),
    (0x3E80, 0xFE80, Opcode::Asf2, op_asf_2),
    (0x80C0, 0x80C0, Opcode::Ldd1, op_ldd_1),
    (0x80C3, 0x803C, Opcode::Ldd2, op_ldd_2),
    (0x8080, 0x80C2, Opcode::Ls1, op_ls_1),
    (0x4080, 0xC0C2, Opcode::Ls1, op_ls_1),
    (0x8082, 0x80C2, Opcode::Ls2, op_ls_2),
    (0x4082, 0xC0C2, Opcode::Ls2, op_ls_2),
    (0x8040, 0x80C0, Opcode::Ld, op_ld),
    (0x4040, 0xC0C0, Opcode::Ld, op_ld),
    (0x3040, 0xF040, Opcode::Ld, op_ld),
    (0x8020, 0x80E0, Opcode::St, op_st),
    (0x4020, 0xC0E0, Opcode::St, op_st),
    (0x3020, 0xF060, Opcode::St, op_st),
    (0x8010, 0x80F0, Opcode::Mv, op_mv),
    (0x4010, 0xC0F0, Opcode::Mv, op_mv),
    (0x3010, 0xF070, Opcode::Mv, op_mv),
    (0x8000, 0x80F0, Opcode::Mr, op_mr),
    (0x4000, 0xC0F0, Opcode::Mr, op_mr),
    (0x3000, 0xF070, Opcode::Mr, op_mr),
];

fn op_illegal(_ctx: &mut DspContext, instr: u16) {
    panic!("Illegal DSP Instruction {:#06x}", instr);
}

// Jump directly on condition
fn op_jmp_di(ctx: &mut DspContext, instr: u16) {
    let imm = ctx.read_imem(ctx.cpu.pc);

    ctx.cpu.pc += 1;

    let cc = instr & 0xF;

    match cc {
        0x4 => {
            if !ctx.cpu.psr.z() {
                ctx.cpu.pc = imm;
            }
        } // nz
        0x5 => {
            if ctx.cpu.psr.z() {
                ctx.cpu.pc = imm;
            }
        } // z
        0xC => {
            if !ctx.cpu.psr.tb() {
                ctx.cpu.pc = imm;
            }
        } // nt
        0xF => ctx.cpu.pc = imm, // Always
        _ => unimplemented!("op_jmp_di cc {:#x} {:#x}", cc, ctx.cpu.pc),
    }
}

fn op_jmp_in(ctx: &mut DspContext, instr: u16) {
    let cc = instr & 0xF;
    let rn = ((instr >> 5) & 0x3) as usize;
    match cc {
        0x4 => {
            if !ctx.cpu.psr.z() {
                ctx.cpu.pc = ctx.cpu.r[rn];
            }
        } // nz
        0x5 => {
            if ctx.cpu.psr.z() {
                ctx.cpu.pc = ctx.cpu.r[rn];
            }
        } // z
        0xC => {
            if !ctx.cpu.psr.tb() {
                ctx.cpu.pc = ctx.cpu.r[rn];
            }
        } // nt
        0xF => ctx.cpu.pc = ctx.cpu.r[rn], // Always
        _ => unimplemented!("op_jmp_in cc {:#x}", cc),
    }
}

// Call directly on condition
fn op_call_di(ctx: &mut DspContext, instr: u16) {
    let imm = ctx.read_imem(ctx.cpu.pc);

    ctx.cpu.pc += 1;

    // push next pc onto pc stack
    ctx.cpu.pcs.push(ctx.cpu.pc);

    let cc = instr & 0xF;

    match cc {
        0xF => ctx.cpu.pc = imm,
        _ => unimplemented!("op_call_di cc {:#x} {:#x}", cc, ctx.cpu.pc),
    }
}

fn op_call_in(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_call_in");
}

// Return from subroutine
fn op_rets(ctx: &mut DspContext, instr: u16) {
    let cc = instr & 0xF;
    match cc {
        0xF => ctx.cpu.pc = ctx.cpu.pcs.pop().unwrap(), // always
        _ => unimplemented!("Unrecognized cc {:#x}", cc),
    }
}

fn op_reti(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_reit");
}

fn op_trap(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_trap");
}

fn op_wait(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_wait");
}

fn op_exec(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_exec");
}

fn op_loop_lc(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_loop_lc");
}

fn op_loop_r(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_loop_r");
}

fn op_rep_rc(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_rep_rc");
}

fn op_rep_r(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_rep_r");
}

fn op_pld(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_pld");
}

fn op_nop(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_nop");
}

fn op_mr_single(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_mr_single");
}

fn op_adsi(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_adsi");
}

fn op_adli(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_adli");
}

fn op_cmpsi(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_cmpsi");
}

fn op_cmpli(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_cmpli");
}

fn op_lsfi(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_lsfi");
}

fn op_asfi(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_asfi");
}

fn op_xoli(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_xoli");
}

fn op_anli(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_anli");
}

fn op_orli(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_arli");
}

fn op_norm(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_norm");
}

fn op_div(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_div");
}

fn op_addc(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_addc");
}

fn op_subc(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_subc");
}

fn op_negc(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_negc");
}

fn op_max(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_max");
}

fn op_lsf_neg_1(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_lsf_neg_1");
}

fn op_lsf_neg_2(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_lsf_neg_2");
}

fn op_asf_neg_1(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_asf_neg_1");
}

fn op_asf_neg_2(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_asf_neg_2");
}

fn op_ld_single(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_ld_single");
}

fn op_st_single(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op-st_single");
}

// Load data from short address
fn op_ldsa(ctx: &mut DspContext, instr: u16) {
    let addr = (instr & 0xFF) | (ctx.cpu.dpp << 8);

    let d = (instr >> 8) & 0x7;

    let val = ctx.read_dmem(addr);

    match d {
        0x0 => ctx.cpu.x0 = val,        // x0
        0x1 => ctx.cpu.y0 = val,        // y0
        0x2 => ctx.cpu.x1 = val,        // x1
        0x3 => ctx.cpu.y1 = val,        // y1
        0x4 => ctx.cpu.a.set_zero(val), // a0
        0x5 => ctx.cpu.b.set_zero(val), // b0
        0x6 => {
            ctx.cpu.a.set_one(val);
        } // a: XL, a1: nonXL
        0x7 => ctx.cpu.b.set_one(val),  // b: XL, b1: nonXL
        _ => unimplemented!(),
    }
}

fn op_stsa(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_stsa");
}

fn op_ldla(ctx: &mut DspContext, instr: u16) {
    let imm = ctx.read_imem(ctx.cpu.pc);

    ctx.cpu.pc += 1;

    let d = instr & 0x1F;

    let val = ctx.read_dmem(imm);

    match d {
        0x00 => ctx.cpu.r[0] = val,
        0x01 => ctx.cpu.r[1] = val,
        0x02 => ctx.cpu.r[2] = val,
        0x03 => ctx.cpu.r[3] = val,
        0x04 => ctx.cpu.m[0] = val,
        0x05 => ctx.cpu.m[1] = val,
        0x06 => ctx.cpu.m[2] = val,
        0x07 => ctx.cpu.m[3] = val,
        0x08 => ctx.cpu.l[0] = val,
        0x09 => ctx.cpu.l[1] = val,
        0x0A => ctx.cpu.l[2] = val,
        0x0B => ctx.cpu.l[3] = val,
        //0x0C => *ctx.cpu.pcs.last().unwrap_or_default(&0),
        //0x0D => ctx.cpu.pss,
        //0x0E => ctx.cpu.eas,
        //0x0F => ctx.cpu.lcs,
        0x10 => ctx.cpu.a.0 = val as u64,
        0x11 => ctx.cpu.b.0 = val as u64,
        0x12 => ctx.cpu.dpp = val,
        0x13 => ctx.cpu.psr.0 = val,
        //0x14 => ctx.cpu.ps0, // What is ps ???
        //0x15 => ctx.cpu.ps1,
        //0x16 => ctx.cpu.ps2,
        //0x17 => ctx.cpu.pc1,
        0x18 => ctx.cpu.x0 = val,
        0x19 => ctx.cpu.y0 = val,
        0x1A => ctx.cpu.x1 = val,
        0x1B => ctx.cpu.y1 = val,
        0x1C => ctx.cpu.a.set_zero(val),
        0x1D => ctx.cpu.b.set_zero(val),
        0x1E => ctx.cpu.a.set_one(val), // assume non-XL for time being
        0x1F => ctx.cpu.b.set_one(val),
        _ => unreachable!(),
    }
}

fn op_stla(ctx: &mut DspContext, instr: u16) {
    let imm = ctx.read_imem(ctx.cpu.pc);

    ctx.cpu.pc += 1;

    let s = instr & 0x1F;

    let val = match s {
        0x00 => ctx.cpu.r[0],
        0x01 => ctx.cpu.r[1],
        0x02 => ctx.cpu.r[2],
        0x03 => ctx.cpu.r[3],
        0x04 => ctx.cpu.m[0],
        0x05 => ctx.cpu.m[1],
        0x06 => ctx.cpu.m[2],
        0x07 => ctx.cpu.m[3],
        0x08 => ctx.cpu.l[0],
        0x09 => ctx.cpu.l[1],
        0x0A => ctx.cpu.l[2],
        0x0B => ctx.cpu.l[3],
        //0x0C => *ctx.cpu.pcs.last().unwrap_or_default(&0),
        //0x0D => ctx.cpu.pss,
        //0x0E => ctx.cpu.eas,
        //0x0F => ctx.cpu.lcs,
        0x10 => ctx.cpu.a.0 as u16,
        0x11 => ctx.cpu.b.0 as u16,
        0x12 => ctx.cpu.dpp,
        0x13 => ctx.cpu.psr.0,
        //0x14 => ctx.cpu.ps0, // What is ps ???
        //0x15 => ctx.cpu.ps1,
        //0x16 => ctx.cpu.ps2,
        //0x17 => ctx.cpu.pc1,
        0x18 => ctx.cpu.x0,
        0x19 => ctx.cpu.y0,
        0x1A => ctx.cpu.x1,
        0x1B => ctx.cpu.y1,
        0x1C => ctx.cpu.a.zero(),
        0x1D => ctx.cpu.b.zero(),
        0x1E => ctx.cpu.a.one(), // assume non-XL for time being
        0x1F => ctx.cpu.b.one(),
        _ => unreachable!(),
    };

    ctx.write_dmem(imm, val);
}

// Move data between registers
fn op_mv_single(ctx: &mut DspContext, instr: u16) {
    let s = match instr & 0x1F {
        0x00 => ctx.cpu.r[0],
        0x01 => ctx.cpu.r[1],
        0x02 => ctx.cpu.r[2],
        0x03 => ctx.cpu.r[3],
        0x04 => ctx.cpu.m[0],
        0x05 => ctx.cpu.m[1],
        0x06 => ctx.cpu.m[2],
        0x07 => ctx.cpu.m[3],
        0x08 => ctx.cpu.l[0],
        0x09 => ctx.cpu.l[1],
        0x0A => ctx.cpu.l[2],
        0x0B => ctx.cpu.l[3],
        //0x0C => *ctx.cpu.pcs.last().unwrap_or_default(&0),
        //0x0D => ctx.cpu.pss,
        //0x0E => ctx.cpu.eas,
        //0x0F => ctx.cpu.lcs,
        0x10 => ctx.cpu.a.two() as u16,
        0x11 => ctx.cpu.b.two() as u16,
        0x12 => ctx.cpu.dpp,
        0x13 => ctx.cpu.psr.0,
        //0x14 => ctx.cpu.ps0, // What is ps ???
        //0x15 => ctx.cpu.ps1,
        //0x16 => ctx.cpu.ps2,
        //0x17 => ctx.cpu.pc1,
        0x18 => ctx.cpu.x0,
        0x19 => ctx.cpu.y0,
        0x1A => ctx.cpu.x1,
        0x1B => ctx.cpu.y1,
        0x1C => ctx.cpu.a.zero(),
        0x1D => ctx.cpu.b.zero(),
        0x1E => ctx.cpu.a.one(), // assume non-XL for time being
        0x1F => ctx.cpu.b.one(),
        _ => unreachable!(),
    };

    // Destination
    match (instr >> 5) & 0x1F {
        0x00 => ctx.cpu.r[0] = s,
        0x01 => ctx.cpu.r[1] = s,
        0x02 => ctx.cpu.r[2] = s,
        0x03 => ctx.cpu.r[3] = s,
        0x04 => ctx.cpu.m[0] = s,
        0x05 => ctx.cpu.m[1] = s,
        0x06 => ctx.cpu.m[2] = s,
        0x07 => ctx.cpu.m[3] = s,
        0x08 => ctx.cpu.l[0] = s,
        0x09 => ctx.cpu.l[1] = s,
        0x0A => ctx.cpu.l[2] = s,
        0x0B => ctx.cpu.l[3] = s,
        //0x0C => *ctx.cpu.pcs.last().unwrap_or_default(&0),
        //0x0D => ctx.cpu.pss,
        //0x0E => ctx.cpu.eas,
        //0x0F => ctx.cpu.lcs,
        0x10 => ctx.cpu.a.set_two(s as u8),
        0x11 => ctx.cpu.b.set_two(s as u8),
        0x12 => ctx.cpu.dpp = s,
        0x13 => ctx.cpu.psr = ProcessStatusRegister(s),
        //0x14 => ctx.cpu.ps0, // What is ps ???
        //0x15 => ctx.cpu.ps1,
        //0x16 => ctx.cpu.ps2,
        //0x17 => ctx.cpu.pc1,
        0x18 => ctx.cpu.x0 = s,
        0x19 => ctx.cpu.y0 = s,
        0x1A => ctx.cpu.x1 = s,
        0x1B => ctx.cpu.y1 = s,
        0x1C => ctx.cpu.a.set_zero(s),
        0x1D => ctx.cpu.b.set_zero(s),
        0x1E => ctx.cpu.a.set_one(s), // assume non-XL for time being
        0x1F => ctx.cpu.b.set_one(s),
        _ => unreachable!(),
    }
}

fn op_mvsi(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_mvsi");
}

// Move long immediate
fn op_mvli(ctx: &mut DspContext, instr: u16) {
    let d = instr & 0x1F;

    let imm = ctx.read_imem(ctx.cpu.pc);

    ctx.cpu.pc += 1;

    match d {
        0x00 => ctx.cpu.r[0] = imm,
        0x01 => ctx.cpu.r[1] = imm,
        0x02 => ctx.cpu.r[2] = imm,
        0x03 => ctx.cpu.r[3] = imm,
        0x04 => ctx.cpu.m[0] = imm,
        0x05 => ctx.cpu.m[1] = imm,
        0x06 => ctx.cpu.m[2] = imm,
        0x07 => ctx.cpu.m[3] = imm,
        0x08 => ctx.cpu.l[0] = imm,
        0x09 => ctx.cpu.l[1] = imm,
        0x0A => ctx.cpu.l[2] = imm,
        0x0B => ctx.cpu.l[3] = imm,
        //0x0C => *ctx.cpu.pcs.last().unwrap_or_default(&0),
        //0x0D => ctx.cpu.pss,
        //0x0E => ctx.cpu.eas,
        //0x0F => ctx.cpu.lcs,
        0x10 => ctx.cpu.a.set_two(imm as u8),
        0x11 => ctx.cpu.b.set_two(imm as u8),
        0x12 => ctx.cpu.dpp = imm,
        0x13 => ctx.cpu.psr = ProcessStatusRegister(imm),
        //0x14 => ctx.cpu.ps0, // What is ps ???
        //0x15 => ctx.cpu.ps1,
        //0x16 => ctx.cpu.ps2,
        //0x17 => ctx.cpu.pc1,
        0x18 => ctx.cpu.x0 = imm,
        0x19 => ctx.cpu.y0 = imm,
        0x1A => ctx.cpu.x1 = imm,
        0x1B => ctx.cpu.y1 = imm,
        0x1C => ctx.cpu.a.set_zero(imm),
        0x1D => ctx.cpu.b.set_zero(imm),
        0x1E => ctx.cpu.a.set_one(imm), // assume non-XL for time being
        0x1F => ctx.cpu.b.set_one(imm),
        _ => unimplemented!("d {:#x}", d),
    }

    // FIXME: this is wrong since accumulator register is out there
}

fn op_stli(ctx: &mut DspContext, instr: u16) {
    let addr = instr | 0xFF00;

    let imm = ctx.read_imem(ctx.cpu.pc);

    ctx.cpu.pc += 1;

    ctx.write_dmem(addr, imm);
}

fn op_clr_c(ctx: &mut DspContext, instr: u16) {
    let psr_bit = instr & 0x7;

    match psr_bit {
        0 => ctx.cpu.psr.set_tb(false),
        1 => ctx.cpu.psr.set_sv(false),
        2 => ctx.cpu.psr.set_te0(false),
        3 => ctx.cpu.psr.set_te1(false),
        4 => ctx.cpu.psr.set_te2(false),
        5 => ctx.cpu.psr.set_te3(false),
        6 => ctx.cpu.psr.set_et(false),
        _ => panic!("op_clr_c: invalid psr_bit {:#x}", psr_bit),
    }

    // TODO: handle dual access
    //unimplemented!("op_clr_c {:#x}", instr);
}

fn op_set_c(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_set_c");
}

// Bit test low
fn op_btstl(ctx: &mut DspContext, instr: u16) {
    let mask = ctx.read_imem(ctx.cpu.pc);

    ctx.cpu.pc += 1;

    let d = if instr & 0x100 != 0 {
        // b1
        ctx.cpu.b.one()
    } else {
        // a1
        ctx.cpu.a.one()
    };

    ctx.cpu.psr.set_tb(d & mask == 0);
}

// bit test high
fn op_btsth(ctx: &mut DspContext, instr: u16) {
    let mask = ctx.read_imem(ctx.cpu.pc);

    ctx.cpu.pc += 1;

    let d = if instr & 0x100 != 0 {
        // b1
        ctx.cpu.b.one()
    } else {
        // a1
        ctx.cpu.a.one()
    };

    ctx.cpu.psr.set_tb(d & mask == mask);
}

fn op_add(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_add");
}

fn op_addl(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_addl");
}

fn op_sub(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("opp_sub");
}

fn op_amv(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_amv");
}

fn op_cmp_1(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_cmp_1");
}

// FIXME: this is entirely incomplete
fn op_cmp_2(ctx: &mut DspContext, _instr: u16) {
    let a = ctx.cpu.a.0 as i64;
    let b = ctx.cpu.b.0 as i64;
    let res = convert_i64_to_i40(a.wrapping_sub(b));

    update_psr_sub(ctx, a, b, res);

    // TODO: handle dual bus move
}

fn op_inc(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_inc");
}

fn op_dec(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_dec");
}

fn op_abs(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_abs");
}

fn op_neg_1(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_neg_1");
}

fn op_neg_2(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_neg_2");
}

// Clear Accumulator
fn op_clr_a(ctx: &mut DspContext, instr: u16) {
    // b
    if instr & 0x800 != 0 {
        ctx.cpu.b = AccumulatorRegister(0);
    // a
    } else {
        ctx.cpu.a = AccumulatorRegister(0);
    }

    // TODO: handle dual access
}

fn op_clr_p(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_clr_p");
}

fn op_rnd(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_rnd");
}

fn op_rndp(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_rndp");
}

// Test
fn op_tst_1(ctx: &mut DspContext, instr: u16) {
    // b
    let s = if instr & 0x800 != 0 {
        ctx.cpu.b.0
    // a
    } else {
        ctx.cpu.a.0
    };

    // Unormalized
    let unn = ((s >> 31) & 1) ^ ((s >> 30) & 1);
    ctx.cpu.psr.set_u(unn == 1);

    // extension
    let ext = (s >> 31) & 0x1FF;
    ctx.cpu.psr.set_e(!(ext == 0 || ext == 0x1FF));

    // Negative
    ctx.cpu.psr.set_n(((s >> 39) & 1) != 0);

    // zero
    ctx.cpu.psr.set_z(s == 0);

    // overflow
    ctx.cpu.psr.set_v(false);

    // carry
    ctx.cpu.psr.set_c(false);

    // TODO: handle dual access
}

fn op_tst_2(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_tst_2");
}

fn op_tst_3(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_tst_3");
}

fn op_lsl16(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_lsl16");
}

fn op_lsr16(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_lsr16");
}

fn op_asr16(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_asr16");
}

fn op_addp(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_addp");
}

fn op_nop2(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_nop2");
}

fn op_clr_im(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_clr_im");
}

fn op_clr_dp(ctx: &mut DspContext, _instr: u16) {
    ctx.cpu.psr.set_dp(false);

    // TODO: handle dual access
}

fn op_clr_xl(ctx: &mut DspContext, _instr: u16) {
    ctx.cpu.psr.set_xl(false);

    // TODO: handle dual access
}

fn op_set_im(ctx: &mut DspContext, _instr: u16) {
    ctx.cpu.psr.set_im(true);

    // TODO: handle dual access
}

fn op_set_dp(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_set_dp");
}

fn op_set_xl(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_set_xl");
}

fn op_mpy_1(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_mpy_1");
}

fn op_mpy_2(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_mpy_2");
}

fn op_mac_1(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_mac_1");
}

fn op_mac_2(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_mac_2");
}

fn op_mac_3(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_mac_3");
}

fn op_mac_neg_1(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_mac_neg_1");
}

fn op_mac_neg_2(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_mac_neg_2");
}

fn op_mac_neg_3(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_mac_neg_3");
}

fn op_mvmpy(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_mvmpy");
}

fn op_rnmpy(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_rnmpy");
}

fn op_admpy(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_admpy");
}

fn op_not(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_not");
}

fn op_xor_1(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_xor_1");
}

fn op_xor_2(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_xor_2");
}

fn op_and_1(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_and_1");
}

fn op_and_2(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_and_2");
}

fn op_or_1(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_or_1");
}

fn op_or_2(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_or_2");
}

fn op_lsf_1(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_lsf_1");
}

fn op_lsf_2(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_lsf_2");
}

fn op_asf_1(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_asf_1");
}

fn op_asf_2(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_asf_2");
}

fn op_ldd_1(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_ldd_1");
}

fn op_ldd_2(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_ldd_2");
}

fn op_ls_1(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_ls_1");
}

fn op_ls_2(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_ls_2");
}

fn op_ld(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_ld");
}

fn op_st(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_st");
}

fn op_mv(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_mv");
}

fn op_mr(_ctx: &mut DspContext, _instr: u16) {
    unimplemented!("op_mr");
}

fn convert_i64_to_i40(val: i64) -> i64 {
    (val << 24) >> 24
}

// TODO: Incomplete
fn update_psr_sub(ctx: &mut DspContext, _a: i64, _b: i64, res: i64) {
    ctx.cpu.psr.set_z(res == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn optable_lookup() {
        let mut optable: [Opcode; OPTABLE_SIZE] = [Opcode::Illegal; OPTABLE_SIZE];

        let mut i = 0;
        while i < optable.len() {
            for op in OPCODE_TABLE.iter() {
                if (i & op.1) == op.0 {
                    optable[i] = op.2;
                    break;
                }
            }
            i += 1;
        }

        let data = [
            (0x0000, Opcode::Nop),
            (0x029F, Opcode::JmpDi),
            (0x1204, Opcode::ClrC),
            (0x0080, Opcode::Mvli),
            (0x0084, Opcode::Mvli),
            (0x191E, Opcode::LdSingle),
            (0x0064, Opcode::LoopR),
            (0x02FF, Opcode::Reti),
            (0x02A0, Opcode::Btstl),
            (0x16FC, Opcode::Stli),
            (0x16FD, Opcode::Stli),
            (0x27FF, Opcode::Ldsa),
            (0x0021, Opcode::Wait),
            (0x02BF, Opcode::CallDi),
            (0x02DF, Opcode::Rets),
            (0x8200, Opcode::Cmp2),
            (0xB900, Opcode::Tst1),
            //(0x8100, Opcode::ClrC),
            //(0x8200, Opcode::Cmp),
        ];

        for i in data.iter() {
            assert_eq!(optable[i.0], i.1);
        }
    }
}
