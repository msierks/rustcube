#[cfg(test)]
mod tests {
    use super::*;

    // TODO: expand on these test cases
    #[test]
    fn test_convert_to_double() {
        let test_values: [(f32, f64); 3] = [
            (0.0, 0.0),
            (1.0, 1.0),
            (1.1754942e-38, 1.1754942106924411e-38),
        ];

        for t in test_values.iter() {
            let result = f64::from_bits(convert_to_double(f32::to_bits(t.0)));

            assert_eq!(result, t.1);
        }
    }

    // TODO: expand on these test cases
    #[test]
    fn test_convert_to_single() {
        let test_values: [(f64, f32); 4] = [
            (0.0, 0.0),
            (1.0, 1.0),
            (4.484155085839414e-44, 4.3e-44),
            (1.4693679385492415e-39, 1.469368e-39),
        ];

        for t in test_values.iter() {
            let result = f32::from_bits(convert_to_single(f64::to_bits(t.0)));

            assert_eq!(result, t.1);
        }
    }

    #[test]
    fn test_f32_is_snan() {
        let snan = f32::from_bits(0xFF800001);

        assert!(snan.is_nan());
        assert!(snan.is_snan());
        assert!(!snan.is_qnan());

        let snan = f32::from_bits(0xFF800301);

        assert!(snan.is_nan());
        assert!(snan.is_snan());
        assert!(!snan.is_qnan());
    }

    #[test]
    fn test_f64_is_snan() {
        let snan = f64::from_bits(0x7FF0000000000001);

        assert!(snan.is_nan());
        assert!(snan.is_snan());
        assert!(!snan.is_qnan());

        let snan = f64::from_bits(0x7FF0000000020001);

        assert!(snan.is_nan());
        assert!(snan.is_snan());
        assert!(!snan.is_qnan());
    }

    #[test]
    fn test_f64_is_qnan() {
        let qnan = f64::from_bits(0x7FF8000000000001);

        assert!(qnan.is_nan());
        assert!(!qnan.is_snan());
        assert!(qnan.is_qnan());

        let qnan = f64::from_bits(0x7FF8000000020001);

        assert!(qnan.is_nan());
        assert!(!qnan.is_snan());
        assert!(qnan.is_qnan());
    }

    #[test]
    fn test_f632_is_qnan() {
        let qnan = f32::from_bits(0xFFC00001);

        assert!(qnan.is_nan());
        assert!(!qnan.is_snan());
        assert!(qnan.is_qnan());

        let qnan = f32::from_bits(0xFFC00301);

        assert!(qnan.is_nan());
        assert!(!qnan.is_snan());
        assert!(qnan.is_qnan());
    }

    #[test]
    fn test_condition_register() {
        let mut cr = ConditionRegister(0x00F0_F0F0);

        cr.set_bit(2, 1);
        assert_eq!(cr.0, 0x20F0_F0F0);
        assert_eq!(cr.get_bit(2), 1);

        cr.set_bit(2, 0);
        assert_eq!(cr.0, 0x00F0_F0F0);
        assert_eq!(cr.get_bit(2), 0);

        cr.set_field(0, 0xF);
        assert_eq!(cr.0, 0xF0F0_F0F0);

        cr.set_field(0, 0x3);
        assert_eq!(cr.0, 0x30F0_F0F0);

        cr.set_field(0, 0x0);
        assert_eq!(cr.0, 0x00F0_F0F0);
    }

    #[test]
    fn test_op_bcx() {
        let mut ctx = Context::default();

        // addi 8,0,3
        let (d, a) = (8, 0);
        let instr = Instruction(((d as u32) << 21) | ((a as u32) << 16) | 0x3);

        op_addi(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x0000_0003);

        // mtctr 8
        let instr = Instruction((0x8 << 21) | (0x9 << 16));
        op_mtspr(&mut ctx, instr);

        // check counter register is set to 0x3
        assert_eq!(ctx.cpu.spr[SPR_CTR], 0x0000_0003);

        // addic. 9,8,0x1
        let (d, a) = (9, 8);
        let instr = Instruction(((d as u32) << 21) | ((a as u32) << 16) | 0x1);

        op_addic_rc(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x0000_0004);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x0000_0004);

        // bc 0xC,1,0x456
        let instr = Instruction((0xC << 21) | (1 << 16) | (0x456 << 2));

        op_bcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.nia, 0xFFF0_1258);

        // bcl 0x8,1,0x456
        let instr = Instruction((0x8 << 21) | (1 << 16) | (0x456 << 2) | 1);

        op_bcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.spr[SPR_CTR], 0x2);
        assert_eq!(ctx.cpu.spr[SPR_LR], 0xFFF0_0104);
    }

    #[test]
    fn test_op_addi() {
        let mut ctx = Context::default();

        let (d, a) = (4, 5);
        let instr = Instruction(((d as u32) << 21) | ((a as u32) << 16) | 0x8FF0);

        ctx.cpu.gpr[a] = 0x0000_0900;
        op_addi(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0xFFFF_98F0);
    }

    #[test]
    fn test_op_addic() {
        let mut ctx = Context::default();

        let (d, a) = (6, 4);
        let instr = Instruction(((d as u32) << 21) | ((a as u32) << 16) | 0xFFFF);

        ctx.cpu.gpr[a] = 0x0000_2346;

        op_addic(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x0000_2345)
    }

    #[test]
    fn test_op_addic_rc() {
        let a: usize = 3;
        let d: usize = 31;

        let mut ctx = Context::default();
        let instr = Instruction(((d as u32) << 21) | ((a as u32) << 16) | 0x1);

        ctx.cpu.gpr[d] = 0xDEAD_BEEF;
        ctx.cpu.gpr[a] = 0xFFFF_FFFF;

        op_addic_rc(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x0);
        assert_eq!(ctx.cpu.gpr[a], 0xFFFF_FFFF); // confirm gpr source remains unmodified
        assert_eq!(ctx.cpu.xer.carry(), true);

        ctx.cpu.gpr[a] = 0xFFFF_FFFE;

        op_addic_rc(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0xFFFF_FFFF);
        assert_eq!(ctx.cpu.xer.carry(), false);
    }

    #[test]
    fn test_op_addis() {
        let mut ctx = Context::default();

        let (d, a) = (7, 6);
        let instr = Instruction(((d as u32) << 21) | ((a as u32) << 16) | 0x0011);

        ctx.cpu.gpr[a] = 0x0000_4000;
        op_addis(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x0011_4000);
    }

    #[test]
    fn test_op_addex() {
        let mut ctx = Context::default();

        let (d, a, b) = (6, 4, 10);
        let instr = Instruction(((d as u32) << 21) | ((a as u32) << 16) | ((b as u32) << 11));

        ctx.cpu.gpr[a] = 0x1000_0400;
        ctx.cpu.gpr[b] = 0x1000_0400;
        ctx.cpu.xer.set_carry(true);
        op_addex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x2000_0801);

        ctx.cpu.gpr[a] = 0x9000_3000;
        ctx.cpu.gpr[b] = 0x7B41_92C0;
        ctx.cpu.xer.set_carry(false);
        op_addex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x0B41_C2C0);

        ctx.cpu.gpr[a] = 0x1000_0400;
        ctx.cpu.gpr[b] = 0xEFFF_FFFF;
        ctx.cpu.xer.set_carry(true);
        op_addex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x0000_0400);

        ctx.cpu.gpr[a] = 0x9000_3000;
        ctx.cpu.gpr[b] = 0x8000_7000;
        ctx.cpu.xer.set_carry(false);
        op_addex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x1000_A000);
    }

    #[test]
    fn test_op_addzex() {
        let mut ctx = Context::default();

        let (d, a) = (6, 4);
        let instr = Instruction(((d as u32) << 21) | ((a as u32) << 16));

        ctx.cpu.gpr[a] = 0x7B41_92C0;
        ctx.cpu.xer.set_carry(false);
        op_addzex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x7B41_92C0);

        ctx.cpu.gpr[a] = 0xEFFF_FFFF;
        ctx.cpu.xer.set_carry(true);
        op_addzex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0xF000_0000);

        ctx.cpu.gpr[a] = 0x9000_3000;
        ctx.cpu.xer.set_carry(true);
        op_addzex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x9000_3001);

        ctx.cpu.gpr[a] = 0xEFFF_FFFF;
        ctx.cpu.xer.set_carry(false);
        op_addzex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0xEFFF_FFFF);
    }

    #[test]
    fn test_op_addx() {
        let mut ctx = Context::default();

        let (d, a, b) = (4, 6, 3);
        let instr = Instruction(((d as u32) << 21) | ((a as u32) << 16) | ((b as u32) << 11));

        ctx.cpu.gpr[a] = 0x0004_0000;
        ctx.cpu.gpr[b] = 0x0000_4000;
        op_addx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x0004_4000);
        assert_eq!(ctx.cpu.xer.carry(), false);

        let instr = Instruction(instr.0 | 1); // Enable rc

        ctx.cpu.gpr[a] = 0x8000_7000;
        ctx.cpu.gpr[b] = 0x7000_8000;
        op_addx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0xF000_F000);
        assert_eq!(ctx.cpu.xer.carry(), false);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT

        ctx.cpu.gpr[a] = 0xEFFF_FFFF;
        ctx.cpu.gpr[b] = 0x8000_0000;
        op_addx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x6FFF_FFFF);
        // FixMe check Summary Overflow, Overflow and carry bits are set in Fixed point
        // register

        ctx.cpu.gpr[a] = 0xEFFF_FFFF;
        ctx.cpu.gpr[b] = 0xEFFF_FFFF;
        op_addx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0xDFFF_FFFE);
        // FixMe: check check Summary Overflow, Overflow and carry bits are set in Fixed point
        // register, as well as condition register field 0 updated
    }

    #[test]
    fn test_op_addcx() {
        let mut ctx = Context::default();

        let (d, a, b) = (6, 4, 10);
        let instr = Instruction(((d as u32) << 21) | ((a as u32) << 16) | ((b as u32) << 11));

        ctx.cpu.gpr[a] = 0x9000_3000;
        ctx.cpu.gpr[b] = 0x8000_7000;
        op_addcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x1000_A000);

        let instr = Instruction(instr.0 | 1); // Enable rc

        ctx.cpu.gpr[a] = 0x7000_3000;
        ctx.cpu.gpr[b] = 0xFFFF_FFFF;
        op_addcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x7000_2FFF);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT

        let instr = Instruction(instr.0 | (1 << 10)); // Enable oe

        ctx.cpu.gpr[a] = 0x9000_3000;
        ctx.cpu.gpr[b] = 0x7B41_92C0;
        op_addcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x0B41_C2C0);
        assert_eq!(ctx.cpu.xer.carry(), true);
        // FixMe: check Summary Overflow and Overflow bits are set

        ctx.cpu.gpr[a] = 0x8000_0000;
        ctx.cpu.gpr[b] = 0x8000_7000;
        ctx.cpu.xer.set_carry(false);
        op_addcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x0000_7000);
        assert_eq!(ctx.cpu.xer.carry(), true);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x5); // GT, SO

        // FixMe: check Summery Overflow and Overflow bits set
    }

    #[test]
    fn test_op_andx() {
        let mut ctx = Context::default();

        let (a, s, b) = (6, 4, 7);
        let instr = Instruction(((s as u32) << 21) | ((a as u32) << 16) | ((b as u32) << 11));

        ctx.cpu.gpr[s] = 0xFFF2_5730;
        ctx.cpu.gpr[b] = 0x7B41_92C0;
        op_andx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0x7B40_1200);

        let instr = Instruction(instr.0 | 1); // Enable rc

        ctx.cpu.gpr[s] = 0xFFF2_5730;
        ctx.cpu.gpr[b] = 0xFFFF_EFFF;
        op_andx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0xFFF2_4730);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT
    }

    #[test]
    fn test_op_andcx() {
        let mut ctx = Context::default();

        let (a, s, b) = (6, 4, 5);
        let instr = Instruction(((s as u32) << 21) | ((a as u32) << 16) | ((b as u32) << 11));

        ctx.cpu.gpr[s] = 0x9000_3000;
        ctx.cpu.gpr[b] = 0xFFFF_FFFF;
        op_andcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0x0000_0000);

        let instr = Instruction(instr.0 | 1); // Enable rc

        ctx.cpu.gpr[s] = 0xB004_3000;
        ctx.cpu.gpr[b] = 0x7676_7676;
        op_andcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0x8000_0000);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT
    }

    #[test]
    fn test_op_andi_rc() {
        let mut ctx = Context::default();

        let (a, s) = (6, 4);
        let uimm = 0x5730;
        let instr = Instruction(((s as u32) << 21) | ((a as u32) << 16) | uimm);

        ctx.cpu.gpr[s] = 0x7B41_92C0;
        op_andi_rc(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0x0000_1200);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT
    }

    #[test]
    fn test_op_cmp() {
        let mut ctx = Context::default();

        let (a, b) = (4, 6);
        let instr = Instruction(((a as u32) << 16) | ((b as u32) << 11));

        ctx.cpu.gpr[a] = 0xFFFF_FFE7;
        ctx.cpu.gpr[b] = 0x0000_0011;
        op_cmp(&mut ctx, instr);

        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT
    }

    #[test]
    fn test_op_cmpi() {
        let mut ctx = Context::default();

        let a = 4;
        let simm = 0x11;
        let instr = Instruction(((a as u32) << 16) | simm);

        ctx.cpu.gpr[a] = 0xFFFF_FFE7;
        op_cmpi(&mut ctx, instr);

        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT
    }

    #[test]
    fn test_op_cmpl() {
        let mut ctx = Context::default();

        let (a, b) = (4, 5);
        let instr = Instruction(((a as u32) << 16) | ((b as u32) << 11));

        ctx.cpu.gpr[a] = 0xFFFF_0000;
        ctx.cpu.gpr[b] = 0x7FFF_0000;
        op_cmpl(&mut ctx, instr);

        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT
    }

    #[test]
    fn test_op_cmpli() {
        let mut ctx = Context::default();

        let a = 4;
        let uimm = 0xFF;
        let instr = Instruction(((a as u32) << 16) | uimm);

        ctx.cpu.gpr[a] = 0x0000_00FF;
        op_cmpli(&mut ctx, instr);

        assert_eq!(ctx.cpu.cr.get_cr0(), 0x2); // EQ
    }

    #[test]
    fn test_op_cntlzwx() {
        let mut ctx = Context::default();

        let (a, s) = (3, 3);
        let instr = Instruction(((s as u32) << 21) | ((a as u32) << 16));

        ctx.cpu.gpr[a] = 0x0061_9920;
        op_cntlzwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[s], 0x0000_0009);
    }

    #[test]
    fn test_op_divwx() {
        let mut ctx = Context::default();

        let (d, a, b) = (4, 4, 6);
        let instr = Instruction(((d as u32) << 21) | ((a as u32) << 16) | ((b as u32) << 11));

        ctx.cpu.gpr[a] = 0x0000_0000;
        ctx.cpu.gpr[b] = 0x0000_0002;
        op_divwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x0000_0000);

        let instr = Instruction(instr.0 | 1); // Enable rc

        ctx.cpu.gpr[a] = 0x0000_0002;
        ctx.cpu.gpr[b] = 0x0000_0002;
        op_divwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x0000_0001);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT

        ctx.cpu.gpr[a] = 0x0000_0001;
        ctx.cpu.gpr[b] = 0x0000_0002;
        op_divwx(&mut ctx, instr);

        // Undefined
        assert_eq!(ctx.cpu.gpr[d], 0x0000_0000);

        ctx.cpu.gpr[a] = 0x8000_0000;
        ctx.cpu.gpr[b] = 0xFFFF_FFFF;
        op_divwx(&mut ctx, instr);

        // Undefined
        assert_eq!(ctx.cpu.gpr[d], 0x0000_0000);
    }

    #[test]
    fn test_op_divwux() {}

    #[test]
    fn test_op_extsbx() {
        let mut ctx = Context::default();

        let (a, s) = (4, 6);
        let instr = Instruction(((s as u32) << 21) | ((a as u32) << 16));

        ctx.cpu.gpr[s] = 0x5A5A_5A5A;
        op_extsbx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0x0000_005A);

        ctx.cpu.gpr[s] = 0xA5A5_A5A5;
        op_extsbx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0xFFFF_FFA5);
    }

    #[test]
    fn test_op_extshx() {
        let mut ctx = Context::default();

        let (a, s) = (4, 6);
        let instr = Instruction(((s as u32) << 21) | ((a as u32) << 16));

        ctx.cpu.gpr[s] = 0x0000_FFFF;
        op_extshx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0xFFFF_FFFF);

        ctx.cpu.gpr[s] = 0x0000_2FFF;
        op_extshx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0x0000_2FFF);
    }

    #[test]
    fn test_op_mulhwux() {
        let mut ctx = Context::default();

        let (d, a, b) = (6, 4, 10);
        let instr = Instruction(((d as u32) << 21) | ((a as u32) << 16) | ((b as u32) << 11));

        ctx.cpu.gpr[a] = 0x0000_0003;
        ctx.cpu.gpr[b] = 0x0000_0002;
        op_mulhwux(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x0000_0000);

        let instr = Instruction(instr.0 | 1); // Enable rc

        ctx.cpu.gpr[a] = 0x0000_4500;
        ctx.cpu.gpr[b] = 0x8000_7000;
        op_mulhwux(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x0000_2280);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT
    }

    #[test]
    fn test_op_mulli() {
        let mut ctx = Context::default();

        let (d, a, simm) = (6, 4, 10);
        let instr = Instruction(((d as u32) << 21) | ((a as u32) << 16) | simm);

        ctx.cpu.gpr[a] = 0x0000_3000;
        op_mulli(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x0001_E000);
    }

    #[test]
    fn test_op_mullwx() {
        let mut ctx = Context::default();

        let (d, a, b) = (6, 4, 10);
        let instr = Instruction(((d as u32) << 21) | ((a as u32) << 16) | ((b as u32) << 11));

        ctx.cpu.gpr[a] = 0x0000_3000;
        ctx.cpu.gpr[b] = 0x0000_7000;
        op_mullwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x1500_0000);

        let instr = Instruction(instr.0 | 1); // Enable rc

        ctx.cpu.gpr[a] = 0x0000_4500;
        ctx.cpu.gpr[b] = 0x0000_7000;
        op_mullwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x1E30_0000);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT

        let instr = Instruction(instr.0 | (1 << 10)); // Enable oe

        ctx.cpu.gpr[a] = 0x0000_4500;
        ctx.cpu.gpr[b] = 0x0007_0000;
        op_mullwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0xE300_0000);
        // FixMe: check summary overflow and overflow

        ctx.cpu.gpr[a] = 0x0000_4500;
        ctx.cpu.gpr[b] = 0x7FFF_FFFF;
        op_mullwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0xFFFF_BB00);
        // FixMe: check summary overflow and overflow
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x9); // LT SO
    }

    #[test]
    fn test_op_negx() {
        let mut ctx = Context::default();

        let (d, a) = (6, 4);
        let instr = Instruction(((d as u32) << 21) | ((a as u32) << 16));

        ctx.cpu.gpr[a] = 0x9000_3000;
        op_negx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x6FFF_D000);

        let instr = Instruction(instr.0 | 1); // Enable rc

        ctx.cpu.gpr[a] = 0x789A_789B;
        op_negx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x8765_8765);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT

        ctx.cpu.gpr[a] = 0x9000_3000;
        op_negx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x6FFF_D000);
        // FixMe: check summary overflow and overflow bits

        ctx.cpu.gpr[a] = 0x8000_0000;
        op_negx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x8000_0000);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT

        // FixMe: check summary overflow and overflow bits
    }

    #[test]
    fn test_op_norx() {
        let mut ctx = Context::default();

        let (a, s, b) = (6, 4, 7);
        let instr = Instruction(((s as u32) << 21) | ((a as u32) << 16) | ((b as u32) << 11));

        ctx.cpu.gpr[s] = 0x9000_3000;
        ctx.cpu.gpr[b] = 0x789A_789B;
        op_norx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0x0765_8764);

        let instr = Instruction(instr.0 | 1); // Enable rc

        ctx.cpu.gpr[s] = 0xB004_3000;
        ctx.cpu.gpr[b] = 0x789A_789B;
        op_norx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0x0761_8764);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT
    }

    #[test]
    fn test_op_orx() {
        let mut ctx = Context::default();

        let (a, s, b) = (6, 4, 7);
        let instr = Instruction(((s as u32) << 21) | ((a as u32) << 16) | ((b as u32) << 11));

        ctx.cpu.gpr[s] = 0x9000_3000;
        ctx.cpu.gpr[b] = 0x789A_789B;
        op_orx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0xF89A_789B);

        let instr = Instruction(instr.0 | 1); // Enable rc

        ctx.cpu.gpr[s] = 0xB004_3000;
        ctx.cpu.gpr[b] = 0x789A_789B;
        op_orx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0xF89E_789B);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT
    }

    #[test]
    fn test_op_ori() {
        let mut ctx = Context::default();

        let (s, a, uimm) = (6, 4, 0x0079);
        let instr = Instruction(((s as u32) << 21) | ((a as u32) << 16) | uimm);

        ctx.cpu.gpr[s] = 0x9000_3000;
        op_ori(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0x9000_3079);
    }

    #[test]
    fn test_op_oris() {
        let mut ctx = Context::default();

        let (s, a, uimm) = (6, 4, 0x0079);
        let instr = Instruction(((s as u32) << 21) | ((a as u32) << 16) | uimm);

        ctx.cpu.gpr[s] = 0x9000_3000;
        op_oris(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0x9079_3000);
    }

    #[test]
    fn test_op_rlwimix() {
        let mut ctx = Context::default();

        let (a, s, sh, mb, me) = (6, 4, 2, 0, 0x1D);
        let instr =
            Instruction(((s as u32) << 21) | ((a as u32) << 16) | sh << 11 | mb << 6 | me << 1);

        ctx.cpu.gpr[s] = 0x9000_3000;
        ctx.cpu.gpr[a] = 0x0000_0003;
        op_rlwimix(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0x4000_C003);

        let (mb, me) = (0, 0x1A);
        let instr =
            Instruction(((s as u32) << 21) | ((a as u32) << 16) | sh << 11 | mb << 6 | me << 1 | 1); // enable rc

        ctx.cpu.gpr[s] = 0x789A_789B;
        ctx.cpu.gpr[a] = 0x3000_0003;
        op_rlwimix(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0xE269_E263);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT
    }

    #[test]
    fn test_op_rlwinmx() {
        let mut ctx = Context::default();

        let (a, s, sh, mb, me) = (6, 4, 2, 0, 0x1D);
        let instr =
            Instruction(((s as u32) << 21) | ((a as u32) << 16) | sh << 11 | mb << 6 | me << 1);

        ctx.cpu.gpr[s] = 0x9000_3000;
        ctx.cpu.gpr[a] = 0xFFFF_FFFF;
        op_rlwinmx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0x4000_C000);

        let instr = Instruction(instr.0 | 1); // Enable rc

        ctx.cpu.gpr[s] = 0xB004_3000;
        ctx.cpu.gpr[a] = 0xFFFF_FFFF;
        op_rlwinmx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0xC010_C000);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT
    }

    #[test]
    fn test_op_slwx() {
        let mut ctx = Context::default();

        let (a, s, b) = (6, 4, 5);
        let instr = Instruction(((s as u32) << 21) | ((a as u32) << 16) | ((b as u32) << 11));

        ctx.cpu.gpr[b] = 0x0000_002F;
        ctx.cpu.gpr[s] = 0xFFFF_FFFF;
        op_slwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0x0000_0000);

        let instr = Instruction(instr.0 | 1); // Enable rc

        ctx.cpu.gpr[b] = 0x0000_0005;
        ctx.cpu.gpr[s] = 0xB004_3000;
        op_slwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0x0086_0000);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT
    }

    #[test]
    fn test_op_srawx() {
        let mut ctx = Context::default();

        let (a, s, b) = (6, 4, 5);
        let instr = Instruction(((s as u32) << 21) | ((a as u32) << 16) | ((b as u32) << 11));

        ctx.cpu.gpr[s] = 0x9000_3000;
        ctx.cpu.gpr[b] = 0x0000_0024;
        op_srawx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0xFFFF_FFFF);
        assert_eq!(ctx.cpu.xer.carry(), true);

        let instr = Instruction(instr.0 | 1); // Enable rc

        ctx.cpu.gpr[s] = 0xB004_3000;
        ctx.cpu.gpr[b] = 0x0000_0004;
        op_srawx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0xFB00_4300);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT

        //assert_eq!(ctx.cpu.xer.carry(), true);
    }

    #[test]
    fn test_op_srawix() {
        let mut ctx = Context::default();

        let (a, s, sh) = (6, 4, 0x4);
        let instr = Instruction(((s as u32) << 21) | ((a as u32) << 16) | ((sh as u32) << 11));

        ctx.cpu.gpr[s] = 0x9000_3000;
        op_srawix(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0xF900_0300);
        assert_eq!(ctx.cpu.xer.carry(), true);

        ctx.cpu.gpr[s] = 0xB004_3000;
        op_srawix(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0xFB00_4300);
        assert_eq!(ctx.cpu.xer.carry(), true);
    }

    #[test]
    fn test_op_srwx() {
        let mut ctx = Context::default();

        let (a, s, b) = (6, 4, 5);
        let instr = Instruction(((s as u32) << 21) | ((a as u32) << 16) | ((b as u32) << 11));

        ctx.cpu.gpr[s] = 0x9000_3000;
        ctx.cpu.gpr[b] = 0x0000_0024;
        op_srwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0x0000_0000);

        let instr = Instruction(instr.0 | 1); // Enable rc

        ctx.cpu.gpr[s] = 0xB004_3001;
        ctx.cpu.gpr[b] = 0x0000_0004;
        op_srwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0x0B00_4300);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT
    }

    #[test]
    fn test_op_subfx() {
        let mut ctx = Context::default();

        let (d, a, b) = (6, 4, 10);
        let instr = Instruction(((d as u32) << 21) | ((a as u32) << 16) | ((b as u32) << 11));

        ctx.cpu.gpr[a] = 0x8000_7000;
        ctx.cpu.gpr[b] = 0x9000_3000;
        op_subfx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x0FFF_C000);

        let instr = Instruction(instr.0 | 1); // Enable rc

        ctx.cpu.gpr[a] = 0x0000_4500;
        ctx.cpu.gpr[b] = 0x8000_7000;
        op_subfx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x8000_2B00);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT

        let instr = Instruction(instr.0 | (1 << 10)); // Enable oe

        ctx.cpu.gpr[a] = 0x8000_0000;
        ctx.cpu.gpr[b] = 0x0000_4500;
        op_subfx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x8000_4500);
        // FixMe: check SO and O

        ctx.cpu.gpr[a] = 0x8000_0000;
        ctx.cpu.gpr[b] = 0x0000_7000;
        op_subfx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x8000_7000);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x9); // LT

        // FixMe: check SO and O
    }

    #[test]
    fn test_op_subfcx() {
        let mut ctx = Context::default();

        let (d, a, b) = (6, 4, 10);
        let instr = Instruction(((d as u32) << 21) | ((a as u32) << 16) | ((b as u32) << 11));

        ctx.cpu.gpr[a] = 0x8000_7000;
        ctx.cpu.gpr[b] = 0x9000_3000;
        op_subfcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x0FFF_C000);
        assert_eq!(ctx.cpu.xer.carry(), true);

        let instr = Instruction(instr.0 | 1); // Enable rc

        ctx.cpu.gpr[a] = 0x0000_4500;
        ctx.cpu.gpr[b] = 0x8000_7000;
        op_subfcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x8000_2B00);
        assert_eq!(ctx.cpu.xer.carry(), true);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT

        let instr = Instruction(instr.0 | (1 << 10)); // Enable oe

        ctx.cpu.gpr[a] = 0x8000_0000;
        ctx.cpu.gpr[b] = 0x0000_4500;
        op_subfcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x8000_4500);
        assert_eq!(ctx.cpu.xer.carry(), false);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x9); // LT

        ctx.cpu.gpr[a] = 0x8000_0000;
        ctx.cpu.gpr[b] = 0x0000_7000;
        op_subfcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x8000_7000);
        assert_eq!(ctx.cpu.xer.carry(), false);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x9); // LT
    }

    #[test]
    fn test_op_subfex() {
        let mut ctx = Context::default();

        let (d, a, b) = (6, 4, 10);
        let instr = Instruction(((d as u32) << 21) | ((a as u32) << 16) | ((b as u32) << 11));

        ctx.cpu.gpr[a] = 0x9000_3000;
        ctx.cpu.gpr[b] = 0x8000_7000;
        ctx.cpu.xer.set_carry(true);
        op_subfex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0xF000_4000);
        assert_eq!(ctx.cpu.xer.carry(), false);

        let instr = Instruction(instr.0 | 1); // Enable rc

        ctx.cpu.gpr[a] = 0x0000_4500;
        ctx.cpu.gpr[b] = 0x8000_7000;
        ctx.cpu.xer.set_carry(false);
        op_subfex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x8000_2AFF);
        assert_eq!(ctx.cpu.xer.carry(), true);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT

        let instr = Instruction(instr.0 | (1 << 10)); // Enable oe

        ctx.cpu.gpr[a] = 0x8000_0000;
        ctx.cpu.gpr[b] = 0xEFFF_FFFF;
        ctx.cpu.xer.set_carry(true);
        op_subfex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x6FFF_FFFF);
        assert_eq!(ctx.cpu.xer.carry(), true);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT

        ctx.cpu.gpr[a] = 0x8000_0000;
        ctx.cpu.gpr[b] = 0xEFFF_FFFF;
        ctx.cpu.xer.set_carry(false);
        op_subfex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x6FFF_FFFE);
        assert_eq!(ctx.cpu.xer.carry(), true);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT
    }

    #[test]
    fn test_op_subfic() {
        let mut ctx = Context::default();

        let (d, a, simm) = (6, 4, 0x7000);
        let instr = Instruction(((d as u32) << 21) | ((a as u32) << 16) | simm);

        ctx.cpu.gpr[a] = 0x9000_3000;
        op_subfic(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x7000_4000);
    }

    #[test]
    fn test_op_subfzex() {
        let mut ctx = Context::default();

        let (d, a) = (6, 4);
        let instr = Instruction(((d as u32) << 21) | ((a as u32) << 16));

        ctx.cpu.gpr[a] = 0x9000_3000;
        ctx.cpu.xer.set_carry(true);
        op_subfzex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x6FFF_D000);
        assert_eq!(ctx.cpu.xer.carry(), false);

        ctx.cpu.gpr[a] = 0xB004_3000;
        ctx.cpu.xer.set_carry(true);
        op_subfzex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x4FFB_D000);
        assert_eq!(ctx.cpu.xer.carry(), false);

        ctx.cpu.gpr[a] = 0xEFFF_FFFF;
        ctx.cpu.xer.set_carry(false);
        op_subfzex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x1000_0000);
        assert_eq!(ctx.cpu.xer.carry(), false);

        ctx.cpu.gpr[a] = 0x70FB_6500;
        ctx.cpu.xer.set_carry(false);
        op_subfzex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[d], 0x8F04_9AFF);
        assert_eq!(ctx.cpu.xer.carry(), false);
    }

    #[test]
    fn test_op_twi() {
        let mut ctx = Context::default();

        let a = 4;
        let instr = Instruction((0x4 << 21) | ((a as u32) << 16) | 0x10);

        ctx.cpu.gpr[a] = 0x0000_0010;
        op_twi(&mut ctx, instr);

        assert_eq!(ctx.cpu.exceptions, EXCEPTION_PROGRAM);

        // FIXME: check cause is trap
    }

    #[test]
    fn test_op_xorx() {
        let mut ctx = Context::default();

        let (a, s, b) = (6, 4, 3);
        let instr = Instruction(((s as u32) << 21) | ((a as u32) << 16) | ((b as u32) << 11));

        ctx.cpu.gpr[s] = 0x9000_3000;
        ctx.cpu.gpr[b] = 0x789A_789B;
        op_xorx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0xE89A_489B);

        let instr = Instruction(instr.0 | 1); // Enable rc

        ctx.cpu.gpr[s] = 0xB004_3000;
        ctx.cpu.gpr[b] = 0x789A_789B;
        op_xorx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0xC89E_489B);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT
    }

    #[test]
    fn test_op_xoris() {
        let mut ctx = Context::default();

        let (a, s, uimm) = (6, 4, 0x0079);
        let instr = Instruction(((s as u32) << 21) | ((a as u32) << 16) | uimm);

        ctx.cpu.gpr[s] = 0x9000_3000;
        op_xoris(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[a], 0x9079_3000);
    }
}
