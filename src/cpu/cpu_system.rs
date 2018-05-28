
impl Cpu {
    fn crxor(&mut self, instr: Instruction) {
        let d = self.cr.get_bit(instr.a()) ^ self.cr.get_bit(instr.b());

        self.cr.set_bit(instr.d(), d);
    }

    #[allow(unused_variables)]
    fn isync(&mut self, instr: Instruction) {
        // don't do anything
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
            Spr::GQR1 => self.gpr[instr.s()] = self.gqr[1],
            Spr::GQR2 => self.gpr[instr.s()] = self.gqr[2],
            Spr::GQR3 => self.gpr[instr.s()] = self.gqr[3],
            Spr::GQR4 => self.gpr[instr.s()] = self.gqr[4],
            Spr::GQR5 => self.gpr[instr.s()] = self.gqr[5],
            Spr::GQR6 => self.gpr[instr.s()] = self.gqr[6],
            Spr::GQR7 => self.gpr[instr.s()] = self.gqr[7],
            Spr::L2CR => self.gpr[instr.s()] = self.l2cr,
            Spr::PMC1 => self.gpr[instr.s()] = self.pmc1,
            Spr::XER  => self.gpr[instr.s()] = self.xer.as_u32(),
            Spr::DEC  => self.gpr[instr.s()] = self.dec, // FixMe: if bit 0 changes from 0 to 1, then signal DEC exception
            _ => panic!("mfspr not implemented for {:#?}", instr.spr()) // FixMe: properly handle this case
        }

        // TODO: check privilege level
    }

    fn mftb(&mut self, instr: Instruction) {
        match instr.tbr() {
            TBR::TBL => self.gpr[instr.d()] = self.tb.l(),
            TBR::TBU => self.gpr[instr.d()] = self.tb.u(),
            TBR::UNKNOWN => panic!("mftb unknown tbr {:#?}", instr.tbr()) // FixMe: properly handle this case
        }
    }

    fn mtmsr(&mut self, instr: Instruction) {
        self.msr = self.gpr[instr.s()].into();

        // TODO: check privilege level
    }

    fn mtspr(&mut self, instr: Instruction, interconnect: &mut Interconnect) {
        let spr = instr.spr();

        match spr {
            Spr::LR  => self.lr  = self.gpr[instr.s()],
            Spr::CTR => self.ctr = self.gpr[instr.s()],
            _ => {

                if self.msr.privilege_level { // if user privilege level
                    // FixMe: properly handle this case
                    self.exception(Exception::Program);
                    panic!("mtspr: user privilege level prevents setting spr {:#?}", spr);
                }

                match spr {
                    Spr::IBAT0U => interconnect.mmu.write_ibatu(0, self.gpr[instr.s()]),
                    Spr::IBAT0L => interconnect.mmu.write_ibatl(0, self.gpr[instr.s()]),
                    Spr::IBAT1U => interconnect.mmu.write_ibatu(1, self.gpr[instr.s()]),
                    Spr::IBAT1L => interconnect.mmu.write_ibatl(1, self.gpr[instr.s()]),
                    Spr::IBAT2U => interconnect.mmu.write_ibatu(2, self.gpr[instr.s()]),
                    Spr::IBAT2L => interconnect.mmu.write_ibatl(2, self.gpr[instr.s()]),
                    Spr::IBAT3U => interconnect.mmu.write_ibatu(3, self.gpr[instr.s()]),
                    Spr::IBAT3L => interconnect.mmu.write_ibatl(3, self.gpr[instr.s()]),
                    Spr::DBAT0U => interconnect.mmu.write_dbatu(0, self.gpr[instr.s()]),
                    Spr::DBAT0L => interconnect.mmu.write_dbatl(0, self.gpr[instr.s()]),
                    Spr::DBAT1U => interconnect.mmu.write_dbatu(1, self.gpr[instr.s()]),
                    Spr::DBAT1L => interconnect.mmu.write_dbatl(1, self.gpr[instr.s()]),
                    Spr::DBAT2U => interconnect.mmu.write_dbatu(2, self.gpr[instr.s()]),
                    Spr::DBAT2L => interconnect.mmu.write_dbatl(2, self.gpr[instr.s()]),
                    Spr::DBAT3U => interconnect.mmu.write_dbatu(3, self.gpr[instr.s()]),
                    Spr::DBAT3L => interconnect.mmu.write_dbatl(3, self.gpr[instr.s()]),
                    Spr::HID0   => self.hid0 = self.gpr[instr.s()],
                    Spr::HID2   => self.hid2 = self.gpr[instr.s()].into(),
                    Spr::GQR0   => self.gqr[0] = self.gpr[instr.s()],
                    Spr::GQR1   => self.gqr[1] = self.gpr[instr.s()],
                    Spr::GQR2   => self.gqr[2] = self.gpr[instr.s()],
                    Spr::GQR3   => self.gqr[3] = self.gpr[instr.s()],
                    Spr::GQR4   => self.gqr[4] = self.gpr[instr.s()],
                    Spr::GQR5   => self.gqr[5] = self.gpr[instr.s()],
                    Spr::GQR6   => self.gqr[6] = self.gpr[instr.s()],
                    Spr::GQR7   => self.gqr[7] = self.gpr[instr.s()],
                    Spr::L2CR   => self.l2cr = self.gpr[instr.s()],
                    Spr::PMC1   => self.pmc1 = self.gpr[instr.s()],
                    Spr::PMC2   => self.pmc2 = self.gpr[instr.s()],
                    Spr::PMC3   => self.pmc3 = self.gpr[instr.s()],
                    Spr::PMC4   => self.pmc4 = self.gpr[instr.s()],
                    Spr::MMCR0  => self.mmcr0 = self.gpr[instr.s()],
                    Spr::MMCR1  => self.mmcr1 = self.gpr[instr.s()],
                    Spr::DEC    => self.dec = self.gpr[instr.s()],
                    Spr::WPAR   => {
                        assert_eq!(self.gpr[instr.s()], 0x0C008000, "write gather pipe address {:#010x}", self.gpr[instr.s()]);
                        interconnect.gp.reset();
                    },
                    _ => panic!("mtspr not implemented for {:#?} {:#x}", spr, self.gpr[instr.s()])
                }
            }
        }
    }

    fn mtsr(&mut self, instr: Instruction) {
        self.sr[instr.sr()] = self.gpr[instr.s()];

        // TODO: check privilege level -> supervisor level instruction
    }

    fn rfi(&mut self) {
        let mask = 0x87C0FFFF;

        self.msr = ((self.msr.as_u32() & !mask) | (self.srr1 & mask)).into();

        self.msr.power_management = false;

        self.nia = self.srr0 & 0xFFFFFFFE;
    }

    #[allow(unused_variables)]
    fn sc(&mut self, instr: Instruction) {
        self.exception(Exception::SystemCall);
    }

    #[allow(unused_variables)]
    fn sync(&mut self, instr: Instruction) {
        // don't do anything
    }

    fn twi(&mut self, _: Instruction) {
        println!("FixMe: twi");
    }
}