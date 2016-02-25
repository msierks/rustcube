use std::fmt;

use byteorder::{ByteOrder, BigEndian};
use super::instruction::Instruction;
use super::mmu;
use super::machine_status::MachineStatus;
use super::super::memory;

const NUM_GPR: usize = 32;
const NUM_SPR: usize = 1023;
const NUM_SR : usize = 16;
const NUM_CR : usize = 8;

const XER : usize = 1;
const HID0: usize = 1008;

pub struct Cpu {
    pub memory: memory::Memory,
    pub mmu: mmu::Mmu,
    pub pc: u32,
    gpr: [u32; NUM_GPR],
    spr: [u32; NUM_SPR],
    pub msr: MachineStatus,
    sr: [u32; NUM_SR],
    cr: [u8; NUM_CR]
}

impl Cpu {
    pub fn new(memory: memory::Memory) -> Cpu {
        let msr = MachineStatus::default();

        // initial vector is (either 0x0000_0100 or 0xFFF0_0100) depending on MSR[IP]
        let pc = if msr.exception_prefix {
            0xFFF00100
        } else {
            0x00000100
        };

        Cpu {
            memory: memory,
            mmu: mmu::Mmu::new(),
            pc: pc,
            gpr: [0; NUM_GPR],
            spr: [0; NUM_SPR],
            msr: msr,
            sr: [0; NUM_SR],
            cr: [0; NUM_CR]
        }
    }

    pub fn run_instruction(&mut self) {
        let instr = self.read_instruction();

        println!("{:#x}:{:?}", instr.opcode(), self.gpr);

        match instr.opcode() {
            10 => self.cmpli(instr),
            14 => self.addi(instr),
            15 => self.addis(instr),
            16 => panic!("bc"),
            18 => self.branch(instr),
            19 => {
                match instr.subopcode() {
                    0b00100101100 => { // isync - instruction synchronize
                        // don't do anything
                    },
                    _ => panic!("Unrecognized instruction subopcode {:#b} {:#b}", instr.opcode(), instr.subopcode())
                }
            },
            21 => self.rlwinm(instr),
            24 => { // ori - OR immediate
                self.gpr[instr.a()] = self.gpr[instr.s()] | instr.uimm();
            },
            31 => {

                match instr.subopcode() {
                    56  => panic!("and"),
                    57  => panic!("and."),
                    80  => self.subf(instr),
                    166 => self.mfmsr(instr),
                    292 => self.mtmsr(instr),
                    420 => self.mtsr(instr),
                    678 => self.mfspr(instr),
                    742 => self.mftb(instr),
                    934 => self.mtspr(instr),
                    _   => panic!("Unrecognized instruction subopcode {:#b} {:#b}", instr.opcode(), instr.subopcode())
                }

            },
            32 => self.lwz(instr),
            36 => self.stw(instr),
            44 => self.sth(instr),
            _  => panic!("Unrecognized instruction {:#x} {:#b}", instr.0, instr.opcode())
        }

        self.pc += 4;
    }

    // FixMe: check if msr.exception_prefix (MSR[IP])
    fn read_instruction(&mut self) -> Instruction {
        let mut data = [0u8; 5];

        let addr = self.mmu.instr_address_translate(&self.msr, self.pc);

        self.memory.read(addr, &mut data);

        Instruction(BigEndian::read_u32(&data[0..]))
    }

    // complare logic immediate
    fn cmpli(&mut self, instr: Instruction) {
        let a = self.gpr[instr.a()];
        let b = instr.uimm();
        let mut c:u8;

        if a < b {
            c = 0b1000;
        } else if a > b {
            c = 0b0100;
        } else {
            c = 0b0010;
        }

        c |= self.spr[XER] as u8 & 0b1;

        self.cr[instr.crfd()] = c;
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

    fn branch(&mut self, instr: Instruction) {
        match instr.aa_lk() {
            0b00 => {
                let li = instr.li() << 2; // (li || 0b00)
                let jump_addr = self.pc + li;

                // FixMe: jump to current_address + li

                println!("{:#x}", jump_addr);

                panic!("todo: implement opcode b:18 (branch)");
            },
            _ => {
                panic!("Unrecognized instruction AA + LK {:#b}", instr.aa_lk());
            }
        }
    }

    // rotate word immediate then AND with mask
    fn rlwinm(&mut self, instr: Instruction) {
        let r = self.gpr[instr.s()] << instr.sh();
        let m = mask(instr.mb(), instr.me());

        self.gpr[instr.a()] = r & m;
    }

    // subtract from
    fn subf(&mut self, instr: Instruction) {
        self.gpr[instr.d()] = self.gpr[instr.a()] + self.gpr[instr.b()] + 1;

        // TODO: other registers altered
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
        let n = (instr.spr_upper() << 5) | (instr.spr_lower() & 0b1_1111);

        self.gpr[instr.s()] = self.spr[n as usize];

        // TODO: check privelege level
    }

    // move from time base
    fn mftb(&mut self, instr: Instruction) {
        let n = (instr.spr_upper() << 5) | (instr.spr_lower() & 0b1_1111);

        match n {
            268 => { // TBL
                println!("FIXME: mftb, get time base tbl");
            },
            269 => { // TBR
                println!("FIXME: mftb, get time base tbu");
            },
            _ => panic!("Unrecognized TBR {}", n) // FixMe: invoke error handler
        }
    }

    // move special purpose register
    fn mtspr(&mut self, instr: Instruction) {
        let n = ((instr.spr_upper() << 5) | (instr.spr_lower() & 0b1_1111)) as usize;

        match n {
            528 ... 543 => { // if IBAT or DBAT, write to MMU register
                self.mmu.write_bat_reg(n, self.gpr[instr.s()]);
                //panic!("FixMe: write to BAT registers");
            },
            _ => self.spr[n] = self.gpr[instr.s()]
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

        let addr = self.mmu.data_address_translate(&self.msr, ea);

        self.gpr[instr.d()] = self.memory.read_u32(addr);
    }

    // store word
    fn stw(&mut self, instr: Instruction) {
        let ea = if instr.a() == 0 {
            instr.simm()
        } else {
            self.gpr[instr.a()] + instr.simm()
        };

        let addr = self.mmu.data_address_translate(&self.msr, ea);

        self.memory.write_u32(addr, self.gpr[instr.s()]);
    }

    // store half word
    fn sth(&mut self, instr: Instruction) {
        let ea = if instr.a() == 0 {
            instr.simm()
        } else {
            self.gpr[instr.a()] + instr.simm()
        };

        let addr = self.mmu.data_address_translate(&self.msr, ea);

        self.memory.write_u16(ea, self.gpr[instr.s()] as u16);
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

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MSR: {:?} gpr: {:?}, sr: {:?}, cr:{:?}, HID0: {}", self.msr, self.gpr, self.sr, self.cr, self.spr[HID0])
    }
}
