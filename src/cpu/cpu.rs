use std::fmt;

use super::condition_register::ConditionRegister;
use super::exception::Exception;
use super::instruction::Instruction;
use super::mmu;
use super::machine_status::MachineStatus;
use super::time_base_register::TimeBaseRegister;
use super::super::interconnect::Interconnect;

const NUM_GPR: usize = 32;
const NUM_SPR: usize = 1023;
const NUM_SR : usize = 16;

const XER : usize = 1;
const HID0: usize = 1008;

pub struct Cpu {
    pub interconnect: Interconnect,
    pub mmu: mmu::Mmu,
    cia: u32,
    nia: u32,
    ctr: u32,
    gpr: [u32; NUM_GPR],
    spr: [u32; NUM_SPR], // ToDo phase out
    pub msr: MachineStatus,
    sr: [u32; NUM_SR],
    cr: ConditionRegister,
    lr: u32,
    tb: TimeBaseRegister
}

impl Cpu {
    pub fn new(interconnect: Interconnect) -> Cpu {
        let mut cpu = Cpu {
            interconnect: interconnect,
            mmu: mmu::Mmu::new(),
            cia: 0,
            nia: 0,
            ctr: 0,
            gpr: [0; NUM_GPR],
            spr: [0; NUM_SPR],
            msr: MachineStatus::default(),
            sr: [0; NUM_SR],
            cr: ConditionRegister::default(),
            lr: 0,
            tb: TimeBaseRegister::default()
        };

        cpu.exception(Exception::SystemReset); // power on reset
        cpu
    }

    pub fn run_instruction(&mut self) {
        let instr = self.read_instruction();

        println!("{:#x} {}", self.cia, instr.opcode());

        self.nia = self.cia + 4;

        match instr.opcode() {
            10 => self.cmpli(instr),
            14 => self.addi(instr),
            15 => self.addis(instr),
            16 => self.bcx(instr),
            18 => self.bx(instr),
            19 => {
                match instr.subopcode() {
                    16 => self.bclrx(instr),
                    150 => self.isync(instr),
                    _ => panic!("Unrecognized instruction subopcode {} {}", instr.opcode(), instr.subopcode())
                }
            },
            21 => self.rlwinm(instr),
            24 => { // ori - OR immediate
                self.gpr[instr.a()] = self.gpr[instr.s()] | instr.uimm();
            },
            31 => {

                match instr.subopcode() {
                     28 => self.andx(instr),
                     32 => self.cmpl(instr),
                     40 => self.subfx(instr),
                     83 => self.mfmsr(instr),
                    146 => self.mtmsr(instr),
                    210 => self.mtsr(instr),
                    339 => self.mfspr(instr),
                    371 => self.mftb(instr),
                    467 => self.mtspr(instr),
                    _   => panic!("Unrecognized instruction subopcode {} {}", instr.opcode(), instr.subopcode())
                }

            },
            32 => self.lwz(instr),
            36 => self.stw(instr),
            44 => self.sth(instr),
            _  => panic!("Unrecognized instruction {:#x} {:#b}", instr.0, instr.opcode())
        }

        self.cia = self.nia;

        // tick timer
        self.tb.advance();
    }

    fn read_instruction(&mut self) -> Instruction {
        let addr = self.mmu.translate_address(mmu::BatType::Instruction, &self.msr, self.cia);

        Instruction(self.interconnect.read_word(addr))
    }

    // FixMe: handle exceptions properly
    pub fn exception(&mut self, e: Exception) {
        let nia = match e {
            Exception::SystemReset => 0x00100
        };

        if self.msr.exception_prefix {
            self.cia = nia ^ 0xFFF00000
        } else {
            self.cia = nia
        }

        println!("{:#x} exception occurred, nia {:#x}", nia, self.cia);
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

        c |= self.spr[XER] as u8 & 0b1;

        self.cr.set_field(instr.crfd(), c);
    }

    // add immediate
    fn addi(&mut self, instr: Instruction) {
        let a = instr.a();
        if a == 0 {
            self.gpr[instr.d()] = instr.uimm();
        } else {
            self.gpr[instr.d()] = self.gpr[a] + instr.uimm();
        }
    }

    // add immediate shifted
    fn addis(&mut self, instr: Instruction) {
        if instr.a() == 0 { // lis
            self.gpr[instr.d()] = instr.uimm() << 16;
        } else { // subis
            self.gpr[instr.d()] = self.gpr[instr.a()] + (instr.uimm() << 16);
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

        let cond_ok = if bon(bo, 0) == 0 {
            (bon(bo, 1) == (self.cr.get_bit(instr.bi())))
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
            (bon(bo, 1) == (self.cr.get_bit(instr.bi())))
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

    // rotate word immediate then AND with mask
    fn rlwinm(&mut self, instr: Instruction) {
        let r = self.gpr[instr.s()] << instr.sh();
        let m = mask(instr.mb(), instr.me());

        self.gpr[instr.a()] = r & m;
    }

    fn andx(&mut self, instr: Instruction) {
        self.gpr[instr.a()] = self.gpr[instr.d()] & self.gpr[instr.b()];

        if instr.rc() {
            self.cr.set_field(0, self.gpr[instr.a()] as u8);
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

        c |= self.spr[XER] as u8 & 0b1;

        self.cr.set_field(instr.crfd(), c);
    }

    // subtract from
    fn subfx(&mut self, instr: Instruction) {
        self.gpr[instr.d()] = self.gpr[instr.a()].wrapping_sub(self.gpr[instr.b()]);

        if instr.rc() {
            self.cr.set_field(0, self.gpr[instr.d()] as u8);
        }

        // TODO: update XER if OE = 1
    }

    // move from machine state register
    fn mfmsr(&mut self, instr: Instruction) {
        self.gpr[instr.d()] = self.msr.as_u32();

        // TODO: check privelege level
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

    // move from special purpose register
    fn mfspr(&mut self, instr: Instruction) {
        let n = ((instr.spr_upper() << 5) | (instr.spr_lower() & 0b1_1111)) as usize;

        match n {
            8 => self.gpr[instr.s()] = self.lr,
            9 => self.gpr[instr.s()] = self.ctr,
            _ => {
                println!("FIXME: spr {} not implemented", n);
                self.gpr[instr.s()] = self.spr[n];
            }
        }

        // TODO: check privelege level
    }

    // move from time base
    fn mftb(&mut self, instr: Instruction) {
        let n = (instr.spr_upper() << 5) | (instr.spr_lower() & 0b1_1111);

        match n {
            268 => self.gpr[instr.d()] = self.tb.l(),
            269 => self.gpr[instr.d()] = self.tb.u(),
            _ => panic!("Unrecognized TBR {}", n) // FixMe: invoke error handler
        }
    }

    // move special purpose register
    fn mtspr(&mut self, instr: Instruction) {
        let n = ((instr.spr_upper() << 5) | (instr.spr_lower() & 0b1_1111)) as usize;

        match n {
            8 => self.lr = self.gpr[instr.s()],
            9 => self.ctr = self.gpr[instr.s()],
            528 ... 543 => { // if IBAT or DBAT, write to MMU register
                self.mmu.write_bat_reg(n, self.gpr[instr.s()]);
                //panic!("FixMe: write to BAT registers");
            },
            _ => {
                println!("FIXME: mtspr {} not implemented", n);
                self.spr[n] = self.gpr[instr.s()];
            }
        }

        // TODO: check privelege level
    }

    // load word and zero
    fn lwz(&mut self, instr: Instruction) {
        let ea = if instr.a() == 0 {
            instr.simm()
        } else {
            self.gpr[instr.a()] + instr.simm()
        };

        let addr = self.mmu.translate_address(mmu::BatType::Data, &self.msr, ea);

        self.gpr[instr.d()] = self.interconnect.read_word(addr);
    }

    // store word
    fn stw(&mut self, instr: Instruction) {
        let ea = if instr.a() == 0 {
            instr.simm()
        } else {
            self.gpr[instr.a()] + instr.simm()
        };

        let addr = self.mmu.translate_address(mmu::BatType::Data, &self.msr, ea);

        self.interconnect.write_word(addr, self.gpr[instr.s()]);
    }

    // store half word
    fn sth(&mut self, instr: Instruction) {
        let ea = if instr.a() == 0 {
            instr.simm()
        } else {
            self.gpr[instr.a()] + instr.simm()
        };

        let addr = self.mmu.translate_address(mmu::BatType::Data, &self.msr, ea);

        self.interconnect.write_halfword(addr, self.gpr[instr.s()] as u16);
    }
}

//((1 << (x - y +1)) - 1) << y
// FixMe: not sure if this is correct
// actually I think this might be backwards
fn mask(x: u8, y: u8) -> u32 {
    let mut mask:u32 = 0xFFFFFFFF >> y;

    if x >= 31 {
        mask ^= 0;
    } else {
        mask ^= 0xFFFFFFFF >> (x + 1);
    }

    if y > x {
        !mask
    } else {
        mask
    }
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
    (bo >> (4-n)) & 1
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MSR: {:?} gpr: {:?}, sr: {:?}, cr:{:?}, HID0: {}", self.msr, self.gpr, self.sr, self.cr, self.spr[HID0])
    }
}
