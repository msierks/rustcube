
impl Cpu {
    fn faddsx(&mut self, instr: Instruction) {
        println!("FixMe: faddsx");
    }

    fn fcmpo(&mut self, instr: Instruction) {
        println!("FixMe: fcmpo");
    }

    fn fcmpu(&mut self, instr: Instruction) {
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

    fn fnabsx(&mut self, instr: Instruction) {
        self.fpr[instr.d()] = self.fpr[instr.b()] | (1 << 63);

        if instr.rc() {
            self.cr.update_cr1(self.fpr[instr.d()], &self.fpscr);
        }
    }

    fn fnegx(&mut self, _: Instruction) {
        println!("FixMe: fnegx");
    }

    fn fsubsx(&mut self, _: Instruction) {
        println!("FixMe: fsubsx");
    }
}
