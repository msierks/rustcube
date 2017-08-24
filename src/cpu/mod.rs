
mod cr;
mod fpscr;
mod gqr;
mod hid;
mod spr;
mod tbr;
mod xer;

pub mod instruction;
pub mod mmu;
pub mod msr;
pub mod util;

use std::fmt;

use self::cr::Cr;
use self::fpscr::Fpscr;
use self::gqr::Gqr;
use self::hid::Hid2;
use self::xer::Xer;
use self::instruction::Instruction;
use self::msr::Msr;
use self::tbr::{Tbr, TBR};
use self::spr::Spr;
use self::util::*;
use super::debugger::Debugger;
use interconnect::Interconnect;

const NUM_FPR: usize = 32;
const NUM_GPR: usize = 32;
const NUM_GQR: usize =  8;
const NUM_SR : usize = 16;

#[derive(Debug)]
pub enum Exception {
    SystemReset                  = 0x00100,
    MachineCheck                 = 0x00200,
    DataStorage                  = 0x00300,
    InstructionStorage           = 0x00400,
    External                     = 0x00500,
    Alignment                    = 0x00600,
    Program                      = 0x00700,
    FloatingPointUnavailable     = 0x00800,
    Decrementer                  = 0x00900,
    SystemCall                   = 0x00C00,
    Trace                        = 0x00D00,
    FloatingPointAssist          = 0x00E00,
    PerformanceMonitor           = 0x00F00, // Gekko Only
    InstructionAddressBreakpoint = 0x01300, // Gekko Only
    ThermalManagement            = 0x01700  // Gekko Only
}

pub struct Cpu {
    /// Current Instruction Address
    pub cia: u32,
    /// Next Instruction Address
    nia: u32,
    /// General-Purpose Registers
    pub gpr: [u32; NUM_GPR],
    /// Floating-Point Registers    
    fpr: [u64; NUM_FPR],
    /// Condition Register
    cr: Cr,
    /// Floating-Point Status and Control Register
    fpscr: Fpscr,
    /// Integer Exception Register
    xer: Xer,
    /// Link Register
    pub lr: u32,
    /// Count Register
    ctr: u32,
    /// Time Base Registers
    tb: Tbr,
    /// Machine State Register
    pub msr: Msr,
    /// Segment Registers
    sr: [u32; NUM_SR],
    /// Machine Status Save/Restore Register 0
    srr0: u32,
    /// Machine Status Save/Restore Register 1
    srr1: u32,
    /// Decrementer Register
    dec: u32,
    /// Hardware Implementation-Dependent Register 0
    hid0: u32,
    /// Hardware Implementation-Dependent Register 1
    hid2: Hid2,
    /// Graphics Quantization Registers
    gqr: [u32; NUM_GQR],
    /// L2 Cache Control Register
    l2cr: u32,
    /// Monitor Mode Control Register 0
    mmcr0: u32,
    /// Monitor Mode Control Register 1
    mmcr1: u32,
    /// Performance Monitor Counter Register 1
    pmc1: u32,
    /// Performance Monitor Counter Register 2
    pmc2: u32,
    /// Performance Monitor Counter Register 3
    pmc3: u32,
    /// Performance Monitor Counter Register 4
    pmc4: u32,
}

include!("cpu_branch.rs");
include!("cpu_condition.rs");
include!("cpu_float.rs");
include!("cpu_integer.rs");
include!("cpu_load_store.rs");
include!("cpu_system.rs");

impl Cpu {

    pub fn new() -> Cpu {
        let mut cpu = Cpu {
            cia: 0,
            nia: 0,
            gpr: [0; NUM_GPR],
            fpr: [0; NUM_FPR],
            cr: Cr::default(),
            fpscr: Fpscr::default(),
            xer: Xer::default(),
            lr: 0,
            ctr: 0,
            tb: Tbr::default(),
            msr: Msr::default(),
            sr: [0; NUM_SR],
            srr0: 0,
            srr1: 0,
            dec: 0,
            hid0: 0,
            hid2: Hid2::default(),
            gqr: [0; NUM_GQR],
            l2cr: 0,
            mmcr0: 0,
            mmcr1: 0,
            pmc1: 0,
            pmc2: 0,
            pmc3: 0,
            pmc4: 0,
        };

        cpu.exception(Exception::SystemReset);
        cpu
    }

    pub fn read_instruction(&mut self, interconnect: &mut Interconnect) -> Instruction {
        interconnect.read_instruction(&self.msr, self.cia)
    }

    pub fn run_instruction(&mut self, interconnect: &mut Interconnect, debugger: &mut Debugger) {
        let instr = self.read_instruction(interconnect);

        self.nia = self.cia + 4;

        debugger.nia_change(self, interconnect);

        match instr.opcode() {
             3 => self.twi(instr),
             7 => self.mulli(instr),
             8 => self.subfic(instr),
            10 => self.cmpli(instr),
            11 => self.cmpi(instr),
            12 => self.addic(instr),
            13 => self.addic_rc(instr),
            14 => self.addi(instr),
            15 => self.addis(instr),
            16 => self.bcx(instr),
            17 => self.sc(instr),
            18 => self.bx(instr),
            19 => {
                match instr.ext_opcode_x() {
                     16 => self.bclrx(instr),
                     50 => self.rfi(),
                    150 => self.isync(instr),
                    193 => self.crxor(instr),
                    528 => self.bcctrx(instr),
                    _ => panic!("Unrecognized instruction subopcode {} {}", instr.opcode(), instr.ext_opcode_x())
                }
            },
            20 => self.rlwimix(instr),
            21 => self.rlwinmx(instr),
            24 => self.ori(instr),
            25 => self.oris(instr),
            27 => self.xoris(instr),
            28 => self.andi_rc(instr),
            31 => {
                match instr.ext_opcode_x() {
                      0 => self.cmp(instr),
                      8 => self.subfcx(instr),
                     10 => self.addcx(instr),
                     11 => self.mulhwux(instr),
                     19 => self.mfcr(instr),
                     23 => self.lwzx(instr, interconnect),
                     24 => self.slwx(instr),
                     26 => self.cntlzwx(instr),
                     28 => self.andx(instr),
                     32 => self.cmpl(instr),
                     40 => self.subfx(instr),
                     60 => self.andcx(instr),
                     83 => self.mfmsr(instr),
                     86 => self.dcbf(instr),
                     87 => self.lbzx(instr, interconnect),
                    104 => self.negx(instr),
                    124 => self.norx(instr),
                    136 => self.subfex(instr),
                    138 => self.addex(instr),
                    146 => self.mtmsr(instr),
                    151 => self.stwx(instr, interconnect, debugger),
                    200 => self.subfzex(instr),
                    202 => self.addzex(instr),
                    210 => self.mtsr(instr),
                    215 => self.stbx(instr, interconnect, debugger),
                    235 => self.mullwx(instr),
                    266 => self.addx(instr),
                    316 => self.xorx(instr),
                    339 => self.mfspr(instr),
                    371 => self.mftb(instr),
                    444 => self.orx(instr),
                    459 => self.divwux(instr),
                    467 => self.mtspr(instr, interconnect),
                    470 => self.dcbi(instr),
                    491 => self.divwx(instr),
                    536 => self.srwx(instr),
                    598 => self.sync(instr),
                    792 => self.srawx(instr),
                    824 => self.srawix(instr),
                    922 => self.extshx(instr),
                    954 => self.extsbx(instr),
                    982 => self.icbi(instr),
                    _   => panic!("Unrecognized instruction subopcode {} {}", instr.opcode(), instr.ext_opcode_x())
                }
            },
            32 => self.lwz(instr, interconnect),
            33 => self.lwzu(instr, interconnect),
            34 => self.lbz(instr, interconnect),
            35 => self.lbzu(instr, interconnect),
            36 => self.stw(instr, interconnect, debugger),
            37 => self.stwu(instr, interconnect, debugger),
            38 => self.stb(instr, interconnect, debugger),
            39 => self.stbu(instr, interconnect, debugger),
            40 => self.lhz(instr, interconnect),
            41 => self.lhzu(instr, interconnect ),
            42 => self.lha(instr, interconnect),
            44 => self.sth(instr, interconnect, debugger),
            45 => self.sthu(instr, interconnect, debugger),
            46 => self.lmw(instr, interconnect),
            47 => self.stmw(instr, interconnect, debugger),
            48 => self.lfs(instr, interconnect),
            50 => self.lfd(instr, interconnect),
            52 => self.stfs(instr, interconnect, debugger),
            53 => self.stfsu(instr, interconnect, debugger),
            54 => self.stfd(instr, interconnect, debugger),
            56 => self.psq_l(instr, interconnect),
            59 => {
                match instr.ext_opcode_a() {
                    18 => self.fdivsx(instr),
                    20 => self.fsubsx(instr),
                    21 => self.faddsx(instr),
                    25 => self.fmulsx(instr),
                    _  => panic!("Unrecognized instruction subopcode {} {}", instr.opcode(), instr.ext_opcode_a())
                }
            },
            60 => self.psq_st(instr, interconnect),
            63 => {
                match instr.ext_opcode_x() {
                      0 => self.fcmpu(instr),
                     12 => self.frspx(instr),
                     20 => self.fsubx(instr),
                     25 => self.fmulx(instr),
                     32 => self.fcmpo(instr),
                     38 => self.mtfsb1x(instr),
                     40 => self.fnegx(instr),
                     72 => self.fmrx(instr),
                    136 => self.fnabsx(instr),
                    711 => self.mtfsfx(instr),
                    _   => panic!("Unrecognized instruction subopcode {} {}", instr.opcode(), instr.ext_opcode_x())
                }
            },
            _  => panic!("Unrecognized instruction {} {}, cia {:#x}", instr.0, instr.opcode(), self.cia)
        }

        self.cia = self.nia;

        // tick timer
        self.tb.advance();
    }

    // FixMe: handle exceptions properly
    pub fn exception(&mut self, exception: Exception) {
        println!("{:?} exception {:#010x}", exception, self.cia);

        match exception {
            Exception::SystemReset => {
                if self.msr.exception_prefix {
                    self.cia = exception as u32 | 0xFFF00000
                } else {
                    self.cia = exception as u32
                }
            },
            Exception::SystemCall => {
                self.srr0 = self.cia + 4;
                self.srr1 = self.msr.as_u32() & 0x87C0FFFF;

                self.msr = (self.msr.as_u32() & !0x04EF36).into();

                self.msr.little_endian = self.msr.exception_little_endian;

                if self.msr.exception_prefix {
                    self.cia = exception as u32 | 0xFFF00000
                } else {
                    self.cia = exception as u32
                }

                self.nia = self.cia;
            },
            _ => panic!("unhandled exception")
        }
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MSR: {:?} gpr: {:?}, sr: {:?}, cr:{:?}", self.msr, self.gpr, self.sr, self.cr)
    }
}

