
impl Cpu {
    fn faddsx(&mut self, _: Instruction) {
        println!("FixMe: faddsx");
    }

    fn fcmpo(&mut self, _: Instruction) {
        println!("FixMe: fcmpo");
    }

    fn fcmpu(&mut self, _: Instruction) {
        println!("FixMe: fcmpu");
    }

    fn fdivsx(&mut self, _: Instruction) {
        println!("FixMe: fdivsx");
    }

    fn fmrx(&mut self, instr: Instruction) {
        self.fpr[instr.d()] = self.fpr[instr.b()];

        if instr.rc() {
            self.cr.update_cr1(self.fpr[instr.d()], &self.fpscr);
        }
    }

    fn fmulsx(&mut self, _: Instruction) {
        println!("FixMe: fmulsx");
    }

    fn fmulx(&mut self, _instr: Instruction) {
        println!("FixMe: fmulx");
    }

    fn fnabsx(&mut self, instr: Instruction) {
        self.fpr[instr.d()] = self.fpr[instr.b()] | (1 << 63);

        if instr.rc() {
            self.cr.update_cr1(self.fpr[instr.d()], &self.fpscr);
        }
    }

    fn fnegx(&mut self, _: Instruction) {
        println!("FixMe: fnegx");
    }

    fn frspx(&mut self, _instr: Instruction) {
        println!("FixMe: frspx");
    }

    fn fsubsx(&mut self, _: Instruction) {
        println!("FixMe: fsubsx");
    }

    fn fsubx(&mut self, instr: Instruction) {
        println!("FixMe: fsubx");

        if instr.rc() {
            panic!("RC: fsubx");
        }
    }

    fn mtfsb1x(&mut self, instr: Instruction) {
        self.fpscr.set_bit(instr.crbd(), true);

        if instr.rc() {
            panic!("RC: mtfsb1x");
        }
    }

    fn mtfsfx(&mut self, _: Instruction) {
        println!("FixMe: mtfsfx");
    }
}
