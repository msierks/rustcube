
impl Cpu {
    fn mfcr(&mut self, instr: Instruction) {
        self.gpr[instr.d()] = self.cr.as_u32();
    }
}
