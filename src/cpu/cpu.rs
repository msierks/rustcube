use std::fmt;

use super::condition_register::ConditionRegister;
use super::floating_point_sc_register::FloatingPointScRegister;
use super::hid::Hid2;
use super::interrupt::Interrupt;
use super::integer_exception_register::IntegerExceptionRegister;
use super::instruction::Instruction;
use super::machine_status::MachineStatus;
use super::time_base_register::{TimeBaseRegister,Tbr};
use super::super::debugger::Debugger;
use super::super::memory::Interconnect;
use super::spr::Spr;

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
    lr: u32,
    tb: TimeBaseRegister,
    hid0: u32,
    hid2: Hid2,
    xer: IntegerExceptionRegister,
    fpscr: FloatingPointScRegister,
    gqr: [u32; NUM_GQR],
    l2cr: u32
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
            l2cr: 0
        };

        cpu.exception(Interrupt::SystemReset);
        cpu
    }

    pub fn read_instruction(&mut self) -> Instruction {
        self.interconnect.read_instruction(&self.msr, self.cia)
    }

    pub fn run_instruction(&mut self, debugger: &mut Debugger) {
        let instr = self.read_instruction();

        if self.cia >= 0x81300000 && self.cia < 0xFFF00000 {
            println!("{:#x}: OP: {}", self.cia, instr.opcode());
        }

        self.nia = self.cia + 4;

        debugger.set_cia(self);

        match instr.opcode() {
             7 => self.mulli(instr),
             8 => self.subfic(instr),
            10 => self.cmpli(instr),
            11 => self.cmpi(instr),
            13 => self.addic_rc(instr),
            14 => self.addi(instr),
            15 => self.addis(instr),
            16 => self.bcx(instr),
            17 => self.sc(instr),
            18 => self.bx(instr),
            19 => {
                match instr.subopcode() {
                    16 => self.bclrx(instr),
                    150 => self.isync(instr),
                    193 => self.crxor(instr),
                    528 => self.bcctrx(instr),
                    _ => panic!("Unrecognized instruction subopcode {} {}", instr.opcode(), instr.subopcode())
                }
            },
            20 => self.rlwimix(instr),
            21 => self.rlwinmx(instr),
            24 => self.ori(instr),
            25 => self.oris(instr),
            28 => self.andi_rc(instr),
            31 => {
                match instr.subopcode() {
                      0 => self.cmp(instr),
                     23 => self.lwzx(instr),
                     24 => self.slwx(instr),
                     26 => self.cntlzwx(instr),
                     28 => self.andx(instr),
                     32 => self.cmpl(instr),
                     40 => self.subfx(instr),
                     60 => self.andcx(instr),
                     83 => self.mfmsr(instr),
                     86 => self.dcbf(instr),
                    124 => self.norx(instr),
                    146 => self.mtmsr(instr),
                    151 => self.stwx(instr),
                    210 => self.mtsr(instr),
                    266 => self.addx(instr),
                    339 => self.mfspr(instr),
                    371 => self.mftb(instr),
                    444 => self.orx(instr),
                    467 => self.mtspr(instr),
                    470 => self.dcbi(instr),
                    536 => self.srwx(instr),
                    598 => self.sync(instr),
                    922 => self.extshx(instr),
                    982 => self.icbi(instr),
                    _   => panic!("Unrecognized instruction subopcode {} {}", instr.opcode(), instr.subopcode())
                }
            },
            32 => self.lwz(instr),
            33 => self.lwzu(instr),
            34 => self.lbz(instr),
            35 => self.lbzu(instr),
            37 => self.stwu(instr),
            36 => self.stw(instr),
            38 => self.stb(instr),
            39 => self.stbu(instr),
            40 => self.lhz(instr),
            41 => self.lhzu(instr),
            44 => self.sth(instr),
            46 => self.lmw(instr),
            47 => self.stmw(instr),
            48 => self.lfs(instr),
            50 => self.lfd(instr),
            52 => self.stfs(instr),
            53 => self.stfsu(instr),
            63 => {
                match instr.subopcode() {
                    72 => self.fmrx(instr),
                    _   => panic!("Unrecognized instruction subopcode {} {}", instr.opcode(), instr.subopcode())
                }
            },
            _  => panic!("Unrecognized instruction {} {} {}, cia {:#x}", instr.0, instr.opcode(), instr.subopcode(), self.cia)
        }

        self.cia = self.nia;

        // tick timer
        self.tb.advance();
    }

    // FixMe: handle exceptions properly
    pub fn exception(&mut self, interrupt: Interrupt) {
        println!("{:?} exception occurred", interrupt);

        let nia = interrupt as u32;

        if self.msr.exception_prefix {
            self.cia = nia | 0xFFF00000
        } else {
            self.cia = nia
        }
    }

    // multiply low immediate
    fn mulli(&mut self, instr: Instruction) {
        self.gpr[instr.d()] = (self.gpr[instr.a()] as i32).wrapping_mul(instr.simm() as i32) as u32;
    }

    // compare
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

    // complare logic immediate
    fn cmpli(&mut self, instr: Instruction) {
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

    fn cmpi(&mut self, instr: Instruction) {
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

    // add immediate
    fn addi(&mut self, instr: Instruction) {
        if instr.a() == 0 {
            self.gpr[instr.d()] = instr.simm() as u32;
        } else {
            self.gpr[instr.d()] = self.gpr[instr.a()].wrapping_add(instr.simm() as u32);
        }
    }

    fn addic_rc(&mut self, instr: Instruction) {
        self.gpr[instr.d()] = self.gpr[instr.a()].wrapping_add(instr.simm() as u32);

        self.cr.update_cr0(self.gpr[instr.d()], &self.xer);
    }

    // add immediate shifted
    fn addis(&mut self, instr: Instruction) {
        if instr.a() == 0 {
            self.gpr[instr.d()] = instr.uimm() << 16;
        } else {
            self.gpr[instr.d()] = self.gpr[instr.a()].wrapping_add(instr.uimm() << 16);
        }
    }

    // branch conditional
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

        let mut cond_ok = if bon(bo, 0) == 0 {
            bon(bo, 1) == self.cr.get_bit(instr.bi())
        } else {
            true
        };

        // skip dsp condition till it can be implemented properly
        if self.cia == 0x81333a7c {
            cond_ok = false;
        }

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

    // branch
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

    // branch conditional to link register
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

    #[allow(unused_variables)]
    // isync - instruction synchronize
    fn isync(&mut self, instr: Instruction) {
        // don't do anything
    }

    #[allow(unused_variables)]
    // synchronize
    fn sync(&mut self, instr: Instruction) {
        // don't do anything
    }

    // shift right word
    fn srwx(&mut self, instr: Instruction) {
        let r = self.gpr[instr.b()];
        let n  = (r & 0x1F) as u8;
        let m = if r & 20 == 0 {
            mask(n, 31)
        } else {
            0
        };

        self.gpr[instr.a()] = rotl(self.gpr[instr.s()], 32 - n) & m;

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.a()], &self.xer);
        }
    }

    // condition register XOR
    fn crxor(&mut self, instr: Instruction) {
        let d = self.cr.get_bit(instr.a()) ^ self.cr.get_bit(instr.b());

        self.cr.set_bit(instr.d(), d);
    }

    // branch condition to count register
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

    fn rlwimix(&mut self, instr: Instruction) {
        let m = mask(instr.mb(), instr.me());
        let r = rotl(self.gpr[instr.s()], instr.sh());

        self.gpr[instr.a()] = (r & m) | (self.gpr[instr.a()] & !m);

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.a()], &self.xer);
        }
    }

    // rotate word immediate then AND with mask
    fn rlwinmx(&mut self, instr: Instruction) {
        let mask = mask(instr.mb(), instr.me());

        self.gpr[instr.a()] = rotl(self.gpr[instr.s()], instr.sh()) & mask;

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.a()], &self.xer);
        }
    }

    // OR immediate
    fn ori(&mut self, instr: Instruction) {
        self.gpr[instr.a()] = self.gpr[instr.s()] | instr.uimm();
    }

    // OR immediate shifted
    fn oris(&mut self, instr: Instruction) {
        self.gpr[instr.a()] = self.gpr[instr.s()] | (instr.uimm() << 16);
    }

    fn andi_rc(&mut self, instr: Instruction) {
        self.gpr[instr.a()] = self.gpr[instr.s()] & instr.uimm();

        self.cr.update_cr0(self.gpr[instr.a()], &self.xer);
    }

    // load word and zero index
    fn lwzx(&mut self, instr: Instruction) {
        let ea = if instr.a() == 0 {
            self.gpr[instr.b()]
        } else {
            self.gpr[instr.a()].wrapping_add(self.gpr[instr.b()])
        };

        self.gpr[instr.d()] = self.interconnect.read_u32(&self.msr, ea);
    }

    // shift left word
    fn slwx(&mut self, instr: Instruction) {
        let n = (self.gpr[instr.a()] & 0x1F) as u8;
        let r = rotl(self.gpr[instr.s()], n);

        let m = if (self.gpr[instr.a()] & 20) << 5 == 0 {
            mask(0, 31 - n)
        } else {
            0
        };

        self.gpr[instr.a()] = r & m;

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.a()], &self.xer);
        }
    }

    // count leading zeroes word
    fn cntlzwx(&mut self, instr: Instruction) {
        let mut n = 0;
        let s = self.gpr[instr.s()];

        while n < 32 {
            if s & (0x80000000 >> n) != 0 {
                break;
            }

            n += 1;
        }

        self.gpr[instr.a()] = n;

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.a()], &self.xer);
        }
    }

    fn andx(&mut self, instr: Instruction) {
        self.gpr[instr.a()] = self.gpr[instr.s()] & self.gpr[instr.b()];

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.a()], &self.xer);
        }
    }

    // ToDo: if L = 1, instruction form is invalid
    fn cmpl(&mut self, instr: Instruction) {
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

    // subtract from
    fn subfx(&mut self, instr: Instruction) {
        self.gpr[instr.d()] = self.gpr[instr.b()].wrapping_sub(self.gpr[instr.a()]);

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.d()], &self.xer);
        }

        if instr.oe() {
            panic!("OE: subfx");
        }
    }

    // subtract from immediate
    fn subfic(&mut self, instr: Instruction) {
        self.gpr[instr.d()] = (instr.simm() as u32).wrapping_sub(self.gpr[instr.a()]);

        // FixMe: update XER
    }

    // and with complement
    fn andcx(&mut self, instr: Instruction) {
        self.gpr[instr.a()] = self.gpr[instr.s()] & !self.gpr[instr.b()];

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.a()], &self.xer);
        }
    }

    // move from machine state register
    fn mfmsr(&mut self, instr: Instruction) {
        self.gpr[instr.d()] = self.msr.as_u32();

        // TODO: check privelege level
    }

    #[allow(unused_variables)]
    // data cache block flush
    fn dcbf(&mut self, instr: Instruction) {
        println!("FixMe: dcbf");
    }

    // move to machine state register
    fn mtmsr(&mut self, instr: Instruction) {
        self.msr = self.gpr[instr.s()].into();

        // TODO: check privelege level
    }

    // move to segment register
    fn mtsr(&mut self, instr: Instruction) {
        self.sr[instr.sr()] = self.gpr[instr.s()];

        // TODO: check privelege level -> supervisor level instruction
    }

    fn addx(&mut self, instr: Instruction) {
        self.gpr[instr.d()] = self.gpr[instr.a()].wrapping_add(self.gpr[instr.b()]);

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.d()], &self.xer);
        }

        if instr.oe() {
            panic!("OE: subfx");
        }
    }

    // move from special purpose register
    fn mfspr(&mut self, instr: Instruction) {
        match instr.spr() {
            Spr::LR   => self.gpr[instr.s()] = self.lr,
            Spr::CTR  => self.gpr[instr.s()] = self.ctr,
            Spr::HID0 => self.gpr[instr.s()] = self.hid0,
            Spr::HID2 => self.gpr[instr.s()] = self.hid2.as_u32(),
            Spr::GQR0   => self.gpr[instr.s()] = self.gqr[0],
            Spr::L2CR   => self.gpr[instr.s()] = self.l2cr,
            _ => panic!("mfspr not implemented for {:#?}", instr.spr()) // FixMe: properly handle this case
        }

        // TODO: check privelege level
    }

    // move from time base
    fn mftb(&mut self, instr: Instruction) {
        match instr.tbr() {
            Tbr::TBL => self.gpr[instr.d()] = self.tb.l(),
            Tbr::TBU => self.gpr[instr.d()] = self.tb.u(),
            Tbr::UNKNOWN => panic!("mftb unknown tbr {:#?}", instr.tbr()) // FixMe: properly handle this case
        }
    }

    fn orx(&mut self, instr: Instruction) {
        self.gpr[instr.a()] = self.gpr[instr.s()] | self.gpr[instr.b()];

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.a()], &self.xer);
        }
    }

    fn norx(&mut self, instr: Instruction) {
        self.gpr[instr.a()] = !(self.gpr[instr.s()] | self.gpr[instr.b()]);

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.a()], &self.xer);
        }
    }

    // move special purpose register
    fn mtspr(&mut self, instr: Instruction) {
        let spr = instr.spr();

        match spr {
            Spr::LR  => self.lr  = self.gpr[instr.s()],
            Spr::CTR => self.ctr = self.gpr[instr.s()],
            _ => {

                if self.msr.privilege_level { // if user privelege level
                    // FixMe: properly handle this case
                    self.exception(Interrupt::Program);
                    panic!("mtspr: user privelege level prevents setting spr {:#?}", spr);
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
                    _ => panic!("mtspr not implemented for {:#?}", spr)
                }
            }
        }
    }

    // data cache block invalidate
    fn dcbi(&mut self, instr: Instruction) {
        println!("FixMe: dcbi");
    }

    // extend sign half word
    fn extshx(&mut self, instr: Instruction) {
        self.gpr[instr.a()] = ((self.gpr[instr.s()] as i16) as i32) as u32;

        if instr.rc() {
            self.cr.update_cr0(self.gpr[instr.a()], &self.xer);
        }
    }

    #[allow(unused_variables)]
    // instruction cache block invalidate
    fn icbi(&mut self, instr: Instruction) {
        println!("FixMe: icbi");
    }

    // load word and zero
    fn lwz(&mut self, instr: Instruction) {
        let ea = if instr.a() == 0 {
            instr.simm() as u32
        } else {
            self.gpr[instr.a()].wrapping_add(instr.simm() as u32)
        };

        self.gpr[instr.d()] = self.interconnect.read_u32(&self.msr, ea);
    }

    // load word and zero with update
    fn lwzu(&mut self, instr: Instruction) {
        let ea = self.gpr[instr.a()].wrapping_add(instr.simm() as u32);

        self.gpr[instr.d()] = self.interconnect.read_u32(&self.msr, ea);
        self.gpr[instr.a()] = ea;
    }

    // load byte and zero
    fn lbz(&mut self, instr: Instruction) {
        let ea = if instr.a() == 0 {
            instr.simm() as u32
        } else {
            self.gpr[instr.a()].wrapping_add(instr.simm() as u32)
        };

        self.gpr[instr.d()] = self.interconnect.read_u8(&self.msr, ea) as u32;
    }

    // load byte and zero with update
    fn lbzu(&mut self, instr: Instruction) {
        if instr.a() == 0 || instr.a() == instr.d() {
            panic!("lbzu: invalid instruction");
        }

        let ea   = self.gpr[instr.a()].wrapping_add(instr.simm() as u32);

        self.gpr[instr.d()] = self.interconnect.read_u8(&self.msr, ea) as u32;
        self.gpr[instr.a()] = ea;
    }

    // store word with update
    fn stwu(&mut self, instr: Instruction) {
        if instr.a() == 0 {
            panic!("stwu: invalid instruction");
        }

        let ea = self.gpr[instr.a()].wrapping_add(instr.simm() as u32);

        self.interconnect.write_u32(&self.msr, ea, self.gpr[instr.s()]);

        self.gpr[instr.a()] = ea; // is this conditional ???
    }

    // store word
    fn stw(&mut self, instr: Instruction) {
        let ea = if instr.a() == 0 {
            instr.simm() as u32
        } else {
            self.gpr[instr.a()].wrapping_add(instr.simm() as u32)
        };

        self.interconnect.write_u32(&self.msr, ea, self.gpr[instr.s()]);
    }

    // store byte
    fn stb(&mut self, instr: Instruction) {
        let ea = if instr.a() == 0 {
            instr.simm() as u32
        } else {
            self.gpr[instr.a()].wrapping_add(instr.simm() as u32)
        };

        self.interconnect.write_u8(&self.msr, ea, self.gpr[instr.d()] as u8);
    }

    // store byte with update
    fn stbu(&mut self, instr: Instruction) {
        let ea = self.gpr[instr.a()].wrapping_add(instr.simm() as u32);

        self.interconnect.write_u8(&self.msr, ea, self.gpr[instr.d()] as u8);

        self.gpr[instr.a()] = ea;
    }

    // store word indexed
    fn stwx(&mut self, instr: Instruction) {
        let ea = if instr.a() == 0 {
            self.gpr[instr.b()]
        } else {
            self.gpr[instr.a()].wrapping_add(self.gpr[instr.b()])
        };

        self.interconnect.write_u32(&self.msr, ea, self.gpr[instr.s()]);
    }

    // load half word and zero
    fn lhz(&mut self, instr: Instruction) {
        let ea = if instr.a() == 0 {
            instr.simm() as u32
        } else {
            self.gpr[instr.a()].wrapping_add(instr.simm() as u32)
        };

        self.gpr[instr.d()] = self.interconnect.read_u16(&self.msr, ea) as u32;
    }

    // load half word and zero with update
    fn lhzu(&mut self, instr: Instruction) {
        let ea = self.gpr[instr.a()].wrapping_add(instr.simm() as u32);

        self.gpr[instr.d()] = self.interconnect.read_u16(&self.msr, ea) as u32;
        self.gpr[instr.a()] = ea;
    }

    // store half word
    fn sth(&mut self, instr: Instruction) {
        let ea = if instr.a() == 0 {
            instr.simm() as u32
        } else {
            self.gpr[instr.a()].wrapping_add(instr.simm() as u32)
        };

        self.interconnect.write_u16(&self.msr, ea, self.gpr[instr.s()] as u16);
    }

    // load multiple word
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

    // store multiple word
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

    // load floating point single
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

    // load floating point double
    fn lfd(&mut self, instr: Instruction) {
        let ea = if instr.a() == 0 {
            instr.simm() as u32
        } else {
            self.gpr[instr.a()].wrapping_add(instr.simm() as u32)
        };

        self.fpr[instr.d()] = self.interconnect.read_u64(&self.msr, ea);
    }

    // store floating point single
    fn stfs(&mut self, instr: Instruction) {
        let ea = if instr.a() == 0 {
            instr.simm() as u32
        } else {
            self.gpr[instr.a()].wrapping_add(instr.simm() as u32)
        };

        let val = convert_to_single(self.fpr[instr.s()]);

        self.interconnect.write_u32(&self.msr, ea, val);
    }

    // store floating point single with update
    fn stfsu(&mut self, instr: Instruction) {
        if instr.a() == 0 {
            panic!("stfsu: invalid instruction");
        }

        let ea  = self.gpr[instr.a()].wrapping_add(instr.simm() as u32);
        let val = convert_to_single(self.fpr[instr.s()]);

        self.interconnect.write_u32(&self.msr, ea, val);
    }

    // floating move register (double-precision)
    fn fmrx(&mut self, instr: Instruction) {
        self.fpr[instr.d()] = self.fpr[instr.b()];

        if instr.rc() {
            self.cr.update_cr1(self.fpr[instr.d()], &self.fpscr);
        }
    }

    // system call
    fn sc(&mut self, instr: Instruction) {
        println!("FixMe: sc");
    }
}

fn rotl(x: u32, shift: u8) -> u32 {
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

// FIXME
fn convert_to_double(x: u32) -> u64 {
    panic!("FixMe: convert_to_double");
}

fn convert_to_single(x: u64) -> u32 {
    0
}

// Note: A cast from a signed value widens with signed-extension
//       A cast from an unsigned value widens with zero-extension
fn sign_ext_16(x: u16) -> i32 {
    (x as i16) as i32
}

fn sign_ext_26(x: u32) -> i32 {
    if x & 0x2000000 != 0 {
        (x | 0xFC000000) as i32
    } else {
        x as i32
    }
}

fn bon(bo: u8, n: u8) -> u8 {
    (bo >> (4 - n)) & 1
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MSR: {:?} gpr: {:?}, sr: {:?}, cr:{:?}", self.msr, self.gpr, self.sr, self.cr)
    }
}
