mod condition_register;
mod floating_point_sc_register;
mod hid;
pub mod instruction;
mod integer_exception_register;
mod interrupt;
pub mod mmu;
pub mod machine_status;
mod spr;
mod time_base_register;
pub mod util;

use std::fmt;

use self::condition_register::ConditionRegister;
use self::floating_point_sc_register::FloatingPointScRegister;
use self::hid::Hid2;
use self::interrupt::Interrupt;
use self::integer_exception_register::IntegerExceptionRegister;
use self::instruction::Instruction;
use self::machine_status::MachineStatus;
use self::time_base_register::{TimeBaseRegister,Tbr};
use self::spr::Spr;
use self::util::*;
use super::memory::Interconnect;

const NUM_FPR: usize = 32;
const NUM_GPR: usize = 32;
const NUM_GQR: usize =  8;
const NUM_SR : usize = 16;

pub struct Cpu {
    pub interconnect: Interconnect,
    pub cia: u32,
    nia: u32,
    ctr: u32,
    fpr: [u64; NUM_FPR],
    pub gpr: [u32; NUM_GPR],
    pub msr: MachineStatus,
    sr: [u32; NUM_SR],
    cr: ConditionRegister,
    pub lr: u32,
    tb: TimeBaseRegister,
    hid0: u32,
    hid2: Hid2,
    xer: IntegerExceptionRegister,
    fpscr: FloatingPointScRegister,
    gqr: [u32; NUM_GQR],
    l2cr: u32,
    srr0: u32,
    srr1: u32,
    pmc1: u32,
    mmcr0: u32,
    dec: u32,
}

impl Cpu {
    pub fn new(interconnect: Interconnect) -> Cpu {
        let mut cpu = Cpu {
            interconnect: interconnect,
            cia: 0,
            nia: 0,
            ctr: 0,
            fpr: [0; NUM_FPR],
            gpr: [0; NUM_GPR],
            msr: MachineStatus::default(),
            sr: [0; NUM_SR],
            cr: ConditionRegister::default(),
            lr: 0,
            tb: TimeBaseRegister::default(),
            hid0: 0,
            hid2: Hid2::default(),
            xer: IntegerExceptionRegister::default(),
            fpscr: FloatingPointScRegister::default(),
            gqr: [0; NUM_GQR],
            l2cr: 0,
            srr0: 0,
            srr1: 0,
            pmc1: 0,
            mmcr0: 0,
            dec: 0,
        };

        cpu.exception(Interrupt::SystemReset);
        cpu
    }

    pub fn read_instruction(&mut self) -> Instruction {
        self.interconnect.read_instruction(&self.msr, self.cia)
    }

    pub fn run_instruction(&mut self) {
        let instr = self.read_instruction();

        self.nia = self.cia + 4;

        match instr.opcode() {
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
                     23 => self.lwzx(instr),
                     24 => self.slwx(instr),
                     26 => self.cntlzwx(instr),
                     28 => self.andx(instr),
                     32 => self.cmpl(instr),
                     40 => self.subfx(instr),
                     60 => self.andcx(instr),
                     83 => self.mfmsr(instr),
                     86 => self.dcbf(instr),
                     87 => self.lbzx(instr),
                    104 => self.negx(instr),
                    124 => self.norx(instr),
                    136 => self.subfex(instr),
                    138 => self.addex(instr),
                    146 => self.mtmsr(instr),
                    151 => self.stwx(instr),
                    202 => self.addzex(instr),
                    210 => self.mtsr(instr),
                    235 => self.mullwx(instr),
                    266 => self.addx(instr),
                    316 => self.xorx(instr),
                    339 => self.mfspr(instr),
                    371 => self.mftb(instr),
                    444 => self.orx(instr),
                    459 => self.divwux(instr),
                    467 => self.mtspr(instr),
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
            32 => self.lwz(instr),
            33 => self.lwzu(instr),
            34 => self.lbz(instr),
            35 => self.lbzu(instr),
            36 => self.stw(instr),
            37 => self.stwu(instr),
            38 => self.stb(instr),
            39 => self.stbu(instr),
            40 => self.lhz(instr),
            41 => self.lhzu(instr),
            42 => self.lha(instr),
            44 => self.sth(instr),
            45 => self.sthu(instr),
            46 => self.lmw(instr),
            47 => self.stmw(instr),
            48 => self.lfs(instr),
            50 => self.lfd(instr),
            52 => self.stfs(instr),
            53 => self.stfsu(instr),
            54 => self.stfd(instr),
            56 => self.psq_l(instr),
            59 => {
                match instr.ext_opcode_a() {
                    18 => self.fdivsx(instr),
                    20 => self.fsubsx(instr),
                    21 => self.faddsx(instr),
                    25 => self.fmulsx(instr),
                    _  => panic!("Unrecognized instruction subopcode {} {}", instr.opcode(), instr.ext_opcode_a())
                }
            },
            60 => self.psq_st(instr),
            63 => {
                match instr.ext_opcode_x() {
                      0 => self.fcmpu(instr),
                     32 => self.fcmpo(instr),
                     40 => self.fnegx(instr),
                     72 => self.fmrx(instr),
                    136 => self.fnabsx(instr),
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
    pub fn exception(&mut self, interrupt: Interrupt) {
        println!("{:?} exception {:#010x}", interrupt, self.cia);

        match interrupt {
            Interrupt::SystemReset => {
                if self.msr.exception_prefix {
                    self.cia = interrupt as u32 | 0xFFF00000
                } else {
                    self.cia = interrupt as u32
                }
            },
            Interrupt::SystemCall => {
                self.srr0 = self.cia + 4;
                self.srr1 = self.msr.as_u32() & 0x87C0FFFF;

                self.msr = (self.msr.as_u32() & !0x04EF36).into();

                self.msr.little_endian = self.msr.exception_little_endian;

                if self.msr.exception_prefix {
                    self.cia = interrupt as u32 | 0xFFF00000
                } else {
                    self.cia = interrupt as u32
                }

                self.nia = self.cia;
            },
            _ => panic!("unhandled exception")
        }
    }

    fn addx(&mut self, instr: Instruction) {
        self.gpr[instr.d()] = self.gpr[instr.a()].wrapping_add(self.gpr[instr.b()]);

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.d()], &self.xer);
        }

        if instr.oe() {
            panic!("OE: addx");
        }
    }

    fn addcx(&mut self, instr: Instruction) {
        let a = self.gpr[instr.a()];
        let b = self.gpr[instr.b()];

        self.gpr[instr.d()] = a.wrapping_add(b);

        self.xer.carry = a > !b;

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.d()], &self.xer);
        }

        if instr.oe() {
            panic!("OE: addcx");
        }
    }

    fn addex(&mut self, instr: Instruction) {
        let a = self.gpr[instr.a()];
        let b = self.gpr[instr.b()];

        self.gpr[instr.d()] = a.wrapping_add(b).wrapping_add(self.xer.carry as u32);

        // FixMe: update carry

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.d()], &self.xer);
        }

        if instr.oe() {
            panic!("OE: addex");
        }
    }

    fn addi(&mut self, instr: Instruction) {
        if instr.a() == 0 {
            self.gpr[instr.d()] = instr.simm() as u32;
        } else {
            self.gpr[instr.d()] = self.gpr[instr.a()].wrapping_add((instr.simm() as i32) as u32);
        }
    }

    fn addic(&mut self, instr: Instruction) {
        let ra  = self.gpr[instr.a()];
        let imm = (instr.simm() as i32) as u32;

        self.gpr[instr.d()] = ra.wrapping_add(imm);

        self.xer.carry = ra > !imm;
    }

    fn addic_rc(&mut self, instr: Instruction) {
        let ra  = self.gpr[instr.a()];
        let imm = (instr.simm() as i32) as u32;

        self.gpr[instr.d()] = ra.wrapping_add(imm);

        self.xer.carry = ra > !imm;

        self.cr.update_cr0(self.gpr[instr.d()], &self.xer);
    }

    fn addis(&mut self, instr: Instruction) {
        if instr.a() == 0 {
            self.gpr[instr.d()] = instr.uimm() << 16;
        } else {
            self.gpr[instr.d()] = self.gpr[instr.a()].wrapping_add(instr.uimm() << 16);
        }
    }

    fn addzex(&mut self, instr: Instruction) {
        let carry = self.xer.carry as u32;
        let ra    = self.gpr[instr.a()];

        self.gpr[instr.d()] = ra.wrapping_add(carry);

        self.xer.carry = ra > !carry;

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.d()], &self.xer);
        }

        if instr.oe() {
            panic!("OE: addzex");
        }
    }

    fn andx(&mut self, instr: Instruction) {
        self.gpr[instr.a()] = self.gpr[instr.s()] & self.gpr[instr.b()];

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.a()], &self.xer);
        }
    }

    fn andcx(&mut self, instr: Instruction) {
        self.gpr[instr.a()] = self.gpr[instr.s()] & (!self.gpr[instr.b()]);

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.a()], &self.xer);
        }
    }

    fn andi_rc(&mut self, instr: Instruction) {
        self.gpr[instr.a()] = self.gpr[instr.s()] & instr.uimm();

        self.cr.update_cr0(self.gpr[instr.a()], &self.xer);
    }

    fn bx(&mut self, instr: Instruction) {
        if instr.aa() == 1 {
            self.nia = sign_ext_26(instr.li() << 2) as u32;
        } else {
            self.nia = self.cia.wrapping_add(sign_ext_26(instr.li() << 2) as u32);
        }

        if instr.lk() == 1 {
            self.lr = self.cia + 4;
        }
    }

    fn bcx(&mut self, instr: Instruction) {
        let bo = instr.bo();

        let ctr_ok = if bon(bo, 2) == 0 {
            self.ctr = self.ctr.wrapping_sub(1);

            if bon(bo, 3) != 0 {
                self.ctr == 0
            } else {
                self.ctr != 0
            }
        } else {
            true
        };

        let cond_ok = if bon(bo, 0) == 0 {
            bon(bo, 1) == self.cr.get_bit(instr.bi())
        } else {
            true
        };

        if ctr_ok && cond_ok {
            if instr.aa() == 1 {
                self.nia = sign_ext_16(instr.bd() << 2) as u32;
            } else {
                self.nia = self.cia.wrapping_add(sign_ext_16(instr.bd() << 2) as u32);
            }

            if instr.lk() == 1 {
                self.lr = self.cia + 4;
            }
        }
    }

    fn bcctrx(&mut self, instr: Instruction) {
        let bo = instr.bo();

        let cond_ok = if bon(bo, 0) == 0 {
            self.cr.get_bit(instr.bi()) == bon(bo, 1)
        } else {
            true
        };

        if cond_ok {
            self.nia = self.ctr & 0xFFFFFFFC;

            if instr.lk() == 1 {
                self.lr = self.cia + 4;
            }
        }
    }

    fn bclrx(&mut self, instr: Instruction) {
        let bo = instr.bo();

        let ctr_ok = if bon(bo, 2) == 0 {
            self.ctr = self.ctr.wrapping_sub(1);

            if bon(bo, 3) != 0 {
                self.ctr == 0
            } else {
                self.ctr != 0
            }
        } else {
            true
        };

        let cond_ok = if bon(bo, 0) == 0 {
            bon(bo, 1) == self.cr.get_bit(instr.bi())
        } else {
            true
        };

        if ctr_ok && cond_ok {
            self.nia = self.lr & 0xFFFFFFFC;

            if instr.lk() == 1 {
                self.lr = self.cia + 4;
            }
        }
    }

    fn cmp(&mut self, instr: Instruction) {
        let a = self.gpr[instr.a()] as i32;
        let b = self.gpr[instr.b()] as i32;

        let mut c:u8 = if a < b {
            0b1000
        } else if a > b {
            0b0100
        } else {
            0b0010
        };

        c |= self.xer.summary_overflow as u8;

        self.cr.set_field(instr.crfd(), c);
    }

    fn cmpi(&mut self, instr: Instruction) {
        if instr.l() {
            panic!("cmpi: invalid instruction");
        }

        let a = self.gpr[instr.a()] as i32;
        let b = instr.simm() as i32;

        let mut c:u8 = if a < b {
            0b1000
        } else if a > b {
            0b0100
        } else {
            0b0010
        };

        c |= self.xer.summary_overflow as u8;

        self.cr.set_field(instr.crfd(), c);
    }

    fn cmpl(&mut self, instr: Instruction) {
        if instr.l() {
            panic!("cmpl: invalid instruction");
        }

        let a = self.gpr[instr.a()];
        let b = self.gpr[instr.b()];

        let mut c:u8 = if a < b {
            0b1000
        } else if a > b {
            0b0100
        } else {
            0b0010
        };

        c |= self.xer.summary_overflow as u8;

        self.cr.set_field(instr.crfd(), c);
    }

    fn cmpli(&mut self, instr: Instruction) {
        if instr.l() {
            panic!("cmpli: invalid instruction");
        }

        let a = self.gpr[instr.a()];
        let b = instr.uimm();

        let mut c:u8 = if a < b {
            0b1000
        } else if a > b {
            0b0100
        } else {
            0b0010
        };

        c |= self.xer.summary_overflow as u8;

        self.cr.set_field(instr.crfd(), c);
    }

    fn cntlzwx(&mut self, instr: Instruction) {
        let mut n = 0;
        let mut mask = 0x80000000;
        let s = self.gpr[instr.s()];

        while n < 32 {
            n += 1;
            mask >>= 1;

            if (s & mask) != 0 {
                break;
            }
        }

        self.gpr[instr.a()] = n;

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.a()], &self.xer);
        }
    }

    fn crxor(&mut self, instr: Instruction) {
        let d = self.cr.get_bit(instr.a()) ^ self.cr.get_bit(instr.b());

        self.cr.set_bit(instr.d(), d);
    }

    #[allow(unused_variables)]
    fn dcbf(&mut self, instr: Instruction) {
        //println!("FixMe: dcbf");
    }

    #[allow(unused_variables)]
    fn dcbi(&mut self, instr: Instruction) {
        //println!("FixMe: dcbi");
    }

    fn divwx(&mut self, instr: Instruction) {
        let a = self.gpr[instr.a()] as i32;
        let b = self.gpr[instr.b()] as i32;

        if b == 0 || (a as u32 == 0x8000_0000 && b == -1) {
            if instr.oe() {
                panic!("OE: divwx");
            }

            if a as u32 == 0x8000_0000 && b == 0 {
                self.gpr[instr.d()] = 0xFFFFFFFF;
            } else {
                self.gpr[instr.d()] = 0;
            }
        } else {
            self.gpr[instr.d()] = (a / b) as u32;
        }

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.d()], &self.xer);
        }
    }

    fn divwux(&mut self, instr: Instruction) {
        let a = self.gpr[instr.a()];
        let b = self.gpr[instr.b()];

        if b == 0 {
            if instr.oe() {
                panic!("OE: divwux");
            }

            self.gpr[instr.d()] = 0;
        } else {
            self.gpr[instr.d()] = a / b;
        }

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.d()], &self.xer);
        }
    }

    fn extsbx(&mut self, instr: Instruction) {
        self.gpr[instr.a()] = ((self.gpr[instr.s()] as i8) as i32) as u32;

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.a()], &self.xer);
        }
    }

    fn extshx(&mut self, instr: Instruction) {
        self.gpr[instr.a()] = ((self.gpr[instr.s()] as i16) as i32) as u32;

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.a()], &self.xer);
        }
    }

    fn faddsx(&mut self, instr: Instruction) {
        println!("FixMe: faddsx");
    }

    fn fcmpo(&mut self, instr: Instruction) {
        println!("FixMe: fcmpo");
    }

    fn fcmpu(&mut self, instr: Instruction) {
        println!("FixMe: fcmpu");
    }

    fn fdivsx(&mut self, instr: Instruction) {
        println!("FixMe: fdivsx");
    }

    fn fmrx(&mut self, instr: Instruction) {
        self.fpr[instr.d()] = self.fpr[instr.b()];

        if instr.rc() {
            self.cr.update_cr1(self.fpr[instr.d()], &self.fpscr);
        }
    }

    fn fmulsx(&mut self, instr: Instruction) {
        println!("FixMe: fmulsx");
    }

    fn fnabsx(&mut self, instr: Instruction) {
        self.fpr[instr.d()] = self.fpr[instr.b()] | (1 << 63);

        if instr.rc() {
            self.cr.update_cr1(self.fpr[instr.d()], &self.fpscr);
        }
    }

    fn fnegx(&mut self, instr: Instruction) {
        println!("FixMe: fnegx");
    }

    fn fsubsx(&mut self, instr: Instruction) {
        println!("FixMe: fsubsx");
    }

    #[allow(unused_variables)]
    fn icbi(&mut self, instr: Instruction) {
        //println!("FixMe: icbi");
    }

    #[allow(unused_variables)]
    fn isync(&mut self, instr: Instruction) {
        // don't do anything
    }

    fn lbz(&mut self, instr: Instruction) {
        let ea = if instr.a() == 0 {
            instr.simm() as u32
        } else {
            self.gpr[instr.a()].wrapping_add(instr.simm() as u32)
        };

        self.gpr[instr.d()] = self.interconnect.read_u8(&self.msr, ea) as u32;
    }

    fn lbzu(&mut self, instr: Instruction) {
        if instr.a() == 0 || instr.a() == instr.d() {
            panic!("lbzu: invalid instruction");
        }

        let ea   = self.gpr[instr.a()].wrapping_add(instr.simm() as u32);

        self.gpr[instr.d()] = self.interconnect.read_u8(&self.msr, ea) as u32;
        self.gpr[instr.a()] = ea;
    }

    fn lbzx(&mut self, instr: Instruction) {
        let ea = if instr.a() == 0 {
            self.gpr[instr.b()]
        } else {
            self.gpr[instr.a()].wrapping_add(self.gpr[instr.b()])
        };

        self.gpr[instr.d()] = self.interconnect.read_u8(&self.msr, ea) as u32;
    }

    fn lfd(&mut self, instr: Instruction) {
        let ea = if instr.a() == 0 {
            instr.simm() as u32
        } else {
            self.gpr[instr.a()].wrapping_add(instr.simm() as u32)
        };

        self.fpr[instr.d()] = self.interconnect.read_u64(&self.msr, ea);
    }

    fn lfs(&mut self, instr: Instruction) {
        let ea = if instr.a() == 0 {
            instr.simm() as u32
        } else {
            self.gpr[instr.a()].wrapping_add(instr.simm() as u32)
        };

        let val = self.interconnect.read_u32(&self.msr, ea);

        if !self.hid2.paired_single {
            self.fpr[instr.d()] = convert_to_double(val);
        } else {
            self.fpr[instr.d()] = ((val as u64) << 32) & val as u64;
        }
    }

    fn lha(&mut self, instr: Instruction) {
        let ea = if instr.a() == 0 {
            instr.simm() as u32
        } else {
            self.gpr[instr.a()].wrapping_add(instr.simm() as u32)
        };

        self.gpr[instr.d()] = ((self.interconnect.read_u16(&self.msr, ea) as i16) as i32) as u32;
    }

    fn lhz(&mut self, instr: Instruction) {
        let ea = if instr.a() == 0 {
            instr.simm() as u32
        } else {
            self.gpr[instr.a()].wrapping_add(instr.simm() as u32)
        };

        self.gpr[instr.d()] = self.interconnect.read_u16(&self.msr, ea) as u32;
    }

    fn lhzu(&mut self, instr: Instruction) {
        let ea = self.gpr[instr.a()].wrapping_add(instr.simm() as u32);

        self.gpr[instr.d()] = self.interconnect.read_u16(&self.msr, ea) as u32;
        self.gpr[instr.a()] = ea;
    }

    fn lmw(&mut self, instr: Instruction) {
        let mut ea = if instr.a() == 0 {
            instr.simm() as u32
        } else {
            self.gpr[instr.a()].wrapping_add(instr.simm() as u32)
        };

        let mut r = instr.d();

        while r <= 31 {
            self.gpr[r] = self.interconnect.read_u32(&self.msr, ea);

            r  += 1;
            ea += 4;
        }
    }

    fn lwz(&mut self, instr: Instruction) {
        let ea = if instr.a() == 0 {
            instr.simm() as u32
        } else {
            self.gpr[instr.a()].wrapping_add(instr.simm() as u32)
        };

        self.gpr[instr.d()] = self.interconnect.read_u32(&self.msr, ea);
    }

    fn lwzx(&mut self, instr: Instruction) {
        let ea = if instr.a() == 0 {
            self.gpr[instr.b()]
        } else {
            self.gpr[instr.a()].wrapping_add(self.gpr[instr.b()])
        };

        self.gpr[instr.d()] = self.interconnect.read_u32(&self.msr, ea);
    }

    fn lwzu(&mut self, instr: Instruction) {
        let ea = self.gpr[instr.a()].wrapping_add(instr.simm() as u32);

        self.gpr[instr.d()] = self.interconnect.read_u32(&self.msr, ea);
        self.gpr[instr.a()] = ea;
    }

    fn mfmsr(&mut self, instr: Instruction) {
        self.gpr[instr.d()] = self.msr.as_u32();

        // TODO: check privilege level
    }

    fn mfspr(&mut self, instr: Instruction) {
        match instr.spr() {
            Spr::LR   => self.gpr[instr.s()] = self.lr,
            Spr::CTR  => self.gpr[instr.s()] = self.ctr,
            Spr::HID0 => self.gpr[instr.s()] = self.hid0,
            Spr::HID2 => self.gpr[instr.s()] = self.hid2.as_u32(),
            Spr::GQR0 => self.gpr[instr.s()] = self.gqr[0],
            Spr::L2CR => self.gpr[instr.s()] = self.l2cr,
            Spr::PMC1 => self.gpr[instr.s()] = self.pmc1,
            _ => panic!("mfspr not implemented for {:#?}", instr.spr()) // FixMe: properly handle this case
        }

        // TODO: check privilege level
    }

    fn mftb(&mut self, instr: Instruction) {
        match instr.tbr() {
            Tbr::TBL => self.gpr[instr.d()] = self.tb.l(),
            Tbr::TBU => self.gpr[instr.d()] = self.tb.u(),
            Tbr::UNKNOWN => panic!("mftb unknown tbr {:#?}", instr.tbr()) // FixMe: properly handle this case
        }
    }

    fn mtmsr(&mut self, instr: Instruction) {
        self.msr = self.gpr[instr.s()].into();

        // TODO: check privilege level
    }

    fn mtspr(&mut self, instr: Instruction) {
        let spr = instr.spr();

        match spr {
            Spr::LR  => self.lr  = self.gpr[instr.s()],
            Spr::CTR => self.ctr = self.gpr[instr.s()],
            _ => {

                if self.msr.privilege_level { // if user privilege level
                    // FixMe: properly handle this case
                    self.exception(Interrupt::Program);
                    panic!("mtspr: user privilege level prevents setting spr {:#?}", spr);
                }

                match spr {
                    Spr::IBAT0U => self.interconnect.mmu.write_ibatu(0, self.gpr[instr.s()]),
                    Spr::IBAT0L => self.interconnect.mmu.write_ibatl(0, self.gpr[instr.s()]),
                    Spr::IBAT1U => self.interconnect.mmu.write_ibatu(1, self.gpr[instr.s()]),
                    Spr::IBAT1L => self.interconnect.mmu.write_ibatl(1, self.gpr[instr.s()]),
                    Spr::IBAT2U => self.interconnect.mmu.write_ibatu(2, self.gpr[instr.s()]),
                    Spr::IBAT2L => self.interconnect.mmu.write_ibatl(2, self.gpr[instr.s()]),
                    Spr::IBAT3U => self.interconnect.mmu.write_ibatu(3, self.gpr[instr.s()]),
                    Spr::IBAT3L => self.interconnect.mmu.write_ibatl(3, self.gpr[instr.s()]),
                    Spr::DBAT0U => self.interconnect.mmu.write_dbatu(0, self.gpr[instr.s()]),
                    Spr::DBAT0L => self.interconnect.mmu.write_dbatl(0, self.gpr[instr.s()]),
                    Spr::DBAT1U => self.interconnect.mmu.write_dbatu(1, self.gpr[instr.s()]),
                    Spr::DBAT1L => self.interconnect.mmu.write_dbatl(1, self.gpr[instr.s()]),
                    Spr::DBAT2U => self.interconnect.mmu.write_dbatu(2, self.gpr[instr.s()]),
                    Spr::DBAT2L => self.interconnect.mmu.write_dbatl(2, self.gpr[instr.s()]),
                    Spr::DBAT3U => self.interconnect.mmu.write_dbatu(3, self.gpr[instr.s()]),
                    Spr::DBAT3L => self.interconnect.mmu.write_dbatl(3, self.gpr[instr.s()]),
                    Spr::HID0   => self.hid0 = self.gpr[instr.s()],
                    Spr::HID2   => self.hid2 = self.gpr[instr.s()].into(),
                    Spr::GQR0   => self.gqr[0] = self.gpr[instr.s()],
                    Spr::L2CR   => self.l2cr = self.gpr[instr.s()],
                    Spr::PMC1   => self.pmc1 = self.gpr[instr.s()],
                    Spr::MMCR0  => self.mmcr0 = self.gpr[instr.s()],
                    Spr::DEC    => self.dec = self.gpr[instr.s()],
                    Spr::WPAR   => println!("Write Gather Pipe: {:#x}", self.gpr[instr.s()]),
                    _ => panic!("mtspr not implemented for {:#?} {:#x}", spr, self.gpr[instr.s()])
                }
            }
        }
    }

    fn mtsr(&mut self, instr: Instruction) {
        self.sr[instr.sr()] = self.gpr[instr.s()];

        // TODO: check privilege level -> supervisor level instruction
    }

    fn mulhwux(&mut self, instr: Instruction) {
        let a = self.gpr[instr.a()] as u64;
        let b = self.gpr[instr.b()] as u64;

        self.gpr[instr.d()] = ((a * b) >> 32) as u32;

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.d()], &self.xer);
        }
    }

    fn mulli(&mut self, instr: Instruction) {
        self.gpr[instr.d()] = (self.gpr[instr.a()] as i32).wrapping_mul(instr.simm() as i32) as u32;
    }

    fn mullwx(&mut self, instr: Instruction) {
        let a = self.gpr[instr.a()] as i32;
        let b = self.gpr[instr.b()] as i32;

        self.gpr[instr.d()] = a.wrapping_mul(b) as u32;

        if instr.oe() {
            panic!("OE: mullwx");
        }

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.d()], &self.xer);
        }
    }

    fn negx(&mut self, instr: Instruction) {
        self.gpr[instr.d()] = !(self.gpr[instr.a()]) + 1;

        // FixMe: ???

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.d()], &self.xer);
        }
    }

    fn norx(&mut self, instr: Instruction) {
        self.gpr[instr.a()] = !(self.gpr[instr.s()] | self.gpr[instr.b()]);

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.a()], &self.xer);
        }
    }

    fn orx(&mut self, instr: Instruction) {
        self.gpr[instr.a()] = self.gpr[instr.s()] | self.gpr[instr.b()];

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.a()], &self.xer);
        }
    }

    fn ori(&mut self, instr: Instruction) {
        self.gpr[instr.a()] = self.gpr[instr.s()] | instr.uimm();
    }

    fn oris(&mut self, instr: Instruction) {
        self.gpr[instr.a()] = self.gpr[instr.s()] | (instr.uimm() << 16);
    }

    fn psq_l(&mut self, instr: Instruction) {
        println!("FixMe: psq_l");
    }

    fn psq_st(&mut self, instr: Instruction) {
        println!("FixMe: psq_st");
    }

    fn rfi(&mut self) {
        let mask = 0x87C0FFFF;

        self.msr = ((self.msr.as_u32() & !mask) | (self.srr1 & mask)).into();

        self.msr.power_management = false;

        self.nia = self.srr0 & 0xFFFFFFFE;
    }

    fn rlwimix(&mut self, instr: Instruction) {
        let m = mask(instr.mb(), instr.me());
        let r = rotl(self.gpr[instr.s()], instr.sh());

        self.gpr[instr.a()] = (r & m) | (self.gpr[instr.a()] & !m);

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.a()], &self.xer);
        }
    }

    fn rlwinmx(&mut self, instr: Instruction) {
        let mask = mask(instr.mb(), instr.me());

        self.gpr[instr.a()] = rotl(self.gpr[instr.s()], instr.sh()) & mask;

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.a()], &self.xer);
        }
    }

    #[allow(unused_variables)]
    fn sc(&mut self, instr: Instruction) {
        self.exception(Interrupt::SystemCall);
    }

    fn slwx(&mut self, instr: Instruction) {
        let r = self.gpr[instr.b()];

        self.gpr[instr.a()] = if r & 0x20 != 0 {
            0
        } else {
            self.gpr[instr.s()] << (r & 0x1F)
        };

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.a()], &self.xer);
        }
    }

    fn srawx(&mut self, instr: Instruction) {
        let rb = self.gpr[instr.b()];

        if rb & 0x20 != 0 {
            if self.gpr[instr.s()] & 0x80000000 != 0 {
                self.gpr[instr.a()] = 0xFFFFFFFF;
                self.xer.carry = true;
            } else {
                self.gpr[instr.a()] = 0;
                self.xer.carry = false;
            }
        } else {
            let n = rb & 0x1F;

            if n != 0 {
                let rs = self.gpr[instr.s()] as i32;

                self.gpr[instr.a()] = (rs >> n) as u32;

                if rs < 0 && (rs << (32 - n) != 0) {
                    self.xer.carry = true;
                } else {
                    self.xer.carry = false;
                }
            } else {
                self.gpr[instr.a()] = self.gpr[instr.s()];
                self.xer.carry = false;
            }
        }

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.a()], &self.xer);
        }
    }

    fn srawix(&mut self, instr: Instruction) {
        let n = instr.sh();

        if n != 0 {
            let rs = self.gpr[instr.s()] as i32;

            self.gpr[instr.a()] = (rs >> n) as u32;

            if rs < 0 && (rs << (32 - n) != 0) {
                self.xer.carry = true;
            } else {
                self.xer.carry = false;
            }
        } else {
            self.gpr[instr.a()] = self.gpr[instr.s()];
            self.xer.carry = false;
        }
    }

    fn srwx(&mut self, instr: Instruction) {
        let r = self.gpr[instr.b()];

        self.gpr[instr.a()] = if r & 0x20 != 0 {
            0
        } else {
            self.gpr[instr.s()] >> (r & 0x1F)
        };

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.a()], &self.xer);
        }
    }

    fn stb(&mut self, instr: Instruction) {
        let ea = if instr.a() == 0 {
            instr.simm() as u32
        } else {
            self.gpr[instr.a()].wrapping_add(instr.simm() as u32)
        };

        self.interconnect.write_u8(&self.msr, ea, self.gpr[instr.d()] as u8);
    }

    fn stbu(&mut self, instr: Instruction) {
        let ea = self.gpr[instr.a()].wrapping_add(instr.simm() as u32);

        self.interconnect.write_u8(&self.msr, ea, self.gpr[instr.d()] as u8);

        self.gpr[instr.a()] = ea;
    }

    fn stfd(&mut self, instr: Instruction) {
        let ea = if instr.a() == 0 {
            instr.simm() as u32
        } else {
            self.gpr[instr.a()].wrapping_add(instr.simm() as u32)
        };

        self.interconnect.write_u64(&self.msr, ea, self.fpr[instr.s()]);
    }

    fn stfs(&mut self, instr: Instruction) {
        let ea = if instr.a() == 0 {
            instr.simm() as u32
        } else {
            self.gpr[instr.a()].wrapping_add(instr.simm() as u32)
        };

        let val = convert_to_single(self.fpr[instr.s()]);

        self.interconnect.write_u32(&self.msr, ea, val);
    }

    fn stfsu(&mut self, instr: Instruction) {
        if instr.a() == 0 {
            panic!("stfsu: invalid instruction");
        }

        let ea  = self.gpr[instr.a()].wrapping_add(instr.simm() as u32);
        let val = convert_to_single(self.fpr[instr.s()]);

        self.interconnect.write_u32(&self.msr, ea, val);
    }

    fn sth(&mut self, instr: Instruction) {
        let ea = if instr.a() == 0 {
            instr.simm() as u32
        } else {
            self.gpr[instr.a()].wrapping_add(instr.simm() as u32)
        };

        self.interconnect.write_u16(&self.msr, ea, self.gpr[instr.s()] as u16);
    }

    fn sthu(&mut self, instr: Instruction) {
        let ea = if instr.a() == 0 {
            instr.simm() as u32
        } else {
            self.gpr[instr.a()].wrapping_add(instr.simm() as u32)
        };

        self.interconnect.write_u16(&self.msr, ea, self.gpr[instr.s()] as u16);

        self.gpr[instr.a()] = ea;
    }

    fn stmw(&mut self, instr: Instruction) {
        let mut ea = if instr.a() == 0 {
            instr.simm() as u32
        } else {
            self.gpr[instr.a()].wrapping_add(instr.simm() as u32)
        };

        let mut r = instr.s();

        while r <= 31 {
            self.interconnect.write_u32(&self.msr, ea, self.gpr[r]);

            r  += 1;
            ea += 4;
        }
    }

    fn stw(&mut self, instr: Instruction) {
        let ea = if instr.a() == 0 {
            instr.simm() as u32
        } else {
            self.gpr[instr.a()].wrapping_add(instr.simm() as u32)
        };

        self.interconnect.write_u32(&self.msr, ea, self.gpr[instr.s()]);
    }

    fn stwx(&mut self, instr: Instruction) {
        let ea = if instr.a() == 0 {
            self.gpr[instr.b()]
        } else {
            self.gpr[instr.a()].wrapping_add(self.gpr[instr.b()])
        };

        self.interconnect.write_u32(&self.msr, ea, self.gpr[instr.s()]);
    }

    fn stwu(&mut self, instr: Instruction) {
        if instr.a() == 0 {
            panic!("stwu: invalid instruction");
        }

        let ea = self.gpr[instr.a()].wrapping_add(instr.simm() as u32);

        self.interconnect.write_u32(&self.msr, ea, self.gpr[instr.s()]);

        self.gpr[instr.a()] = ea; // is this conditional ???
    }

    fn subfx(&mut self, instr: Instruction) {
        self.gpr[instr.d()] = self.gpr[instr.b()].wrapping_sub(self.gpr[instr.a()]);

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.d()], &self.xer);
        }

        if instr.oe() {
            panic!("OE: subfx");
        }
    }

    fn subfcx(&mut self, instr: Instruction) {
        let ra = !self.gpr[instr.a()];
        let rb = self.gpr[instr.b()] + 1;

        self.gpr[instr.d()] = ra.wrapping_add(rb);

        self.xer.carry = (self.gpr[instr.a()]) < ra; // FixMe: ???

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.d()], &self.xer);
        }

        if instr.oe() {
            panic!("OE: subfcx");
        }
    }

    fn subfex(&mut self, instr: Instruction) {
        let ra = !self.gpr[instr.a()];
        self.gpr[instr.b()] + self.xer.carry as u32;

        self.xer.carry = (self.gpr[instr.a()]) < ra; // FixMe: ???

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.d()], &self.xer);
        }

        if instr.oe() {
            panic!("OE: subfex");
        }
    }

    fn subfic(&mut self, instr: Instruction) {
        let ra = !self.gpr[instr.a()] as i32;
        let imm = (instr.simm() as i32) + 1;

        self.gpr[instr.d()] = ra.wrapping_add(imm) as u32;

        self.xer.carry = (self.gpr[instr.a()] as i32) < ra; // FixMe: ???
    }

    #[allow(unused_variables)]
    fn sync(&mut self, instr: Instruction) {
        // don't do anything
    }

    fn xorx(&mut self, instr: Instruction) {
        self.gpr[instr.a()] = self.gpr[instr.s()] ^ self.gpr[instr.b()];

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.a()], &self.xer);
        }
    }

    fn xoris(&mut self, instr: Instruction) {
        self.gpr[instr.a()] = self.gpr[instr.s()] ^ (instr.uimm() << 16)
    }
}

fn rotl(x: u32, shift: u8) -> u32 {
    let shift = shift & 31;

    if shift == 0 {
        x
    } else {
        (x << shift) | (x >> (32 - shift))
    }
}

fn mask(x: u8, y: u8) -> u32 {
    let mut mask:u32 = 0xFFFFFFFF >> x;

    if y >= 31 {
        mask ^= 0;
    } else {
        mask ^= 0xFFFFFFFF >> (y + 1)
    };

    if y < x {
        !mask
    } else {
        mask
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MSR: {:?} gpr: {:?}, sr: {:?}, cr:{:?}", self.msr, self.gpr, self.sr, self.cr)
    }
}

