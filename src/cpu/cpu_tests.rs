#[cfg(test)]
mod tests {
    use super::*;

    // TODO: expand on these test cases
    #[test]
    fn convert_to_double() {
        let test_values: [(f32, f64); 3] = [
            (0.0, 0.0),
            (1.0, 1.0),
            (1.1754942e-38, 1.1754942106924411e-38),
        ];

        for t in test_values.iter() {
            let result = f64::from_bits(super::convert_to_double(f32::to_bits(t.0)));

            assert_eq!(result, t.1);
        }
    }

    // TODO: expand on these test cases
    #[test]
    fn convert_to_single() {
        let test_values: [(f64, f32); 4] = [
            (0.0, 0.0),
            (1.0, 1.0),
            (4.484155085839414e-44, 4.3e-44),
            (1.4693679385492415e-39, 1.469368e-39),
        ];

        for t in test_values.iter() {
            let result = f32::from_bits(super::convert_to_single(f64::to_bits(t.0)));

            assert_eq!(result, t.1);
        }
    }

    #[test]
    fn f32_is_snan() {
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
    fn f64_is_snan() {
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
    fn f64_is_qnan() {
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
    fn f632_is_qnan() {
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
    fn condition_register() {
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
    fn optable_lookup() {
        let mut optable: [Opcode; OPTABLE_SIZE] = [Opcode::Illegal; OPTABLE_SIZE];
        let mut optable4: [Opcode; OPTABLE4_SIZE] = [Opcode::Illegal; OPTABLE4_SIZE];
        let mut optable19: [Opcode; OPTABLE19_SIZE] = [Opcode::Illegal; OPTABLE19_SIZE];
        let mut optable31: [Opcode; OPTABLE31_SIZE] = [Opcode::Illegal; OPTABLE31_SIZE];
        let mut optable59: [Opcode; OPTABLE59_SIZE] = [Opcode::Illegal; OPTABLE59_SIZE];
        let mut optable63: [Opcode; OPTABLE63_SIZE] = [Opcode::Illegal; OPTABLE63_SIZE];

        for op in OPCODE_TABLE.iter() {
            optable[op.0 as usize] = op.1;
        }

        for op in OPCODE4X_TABLE.iter() {
            optable4[op.0 as usize] = op.1;
        }

        for n in 0..32 {
            let fill = n << 5;
            for op in OPCODE4A_TABLE.iter() {
                let xo_x = (op.0 as usize) | fill;
                optable4[xo_x] = op.1;
            }
        }

        for n in 0..16 {
            let fill = n << 6;
            for op in OPCODE4AA_TABLE.iter() {
                let xo_x = (op.0 as usize) | fill;
                optable4[xo_x] = op.1;
            }
        }

        for op in OPCODE19_TABLE.iter() {
            optable19[op.0 as usize] = op.1;
        }

        for op in OPCODE31_TABLE.iter() {
            optable31[op.0 as usize] = op.1;
        }

        for op in OPCODE59_TABLE.iter() {
            optable59[op.0 as usize] = op.1;
        }

        for op in OPCODE63X_TABLE.iter() {
            optable63[op.0 as usize] = op.1;
        }

        for n in 0..32 {
            let fill = n << 5;
            for op in OPCODE63A_TABLE.iter() {
                let xo_x = (op.0 as usize) | fill;
                optable63[xo_x] = op.1;
            }
        }

        let data = [
            (0x7C00_0214, Opcode::Addx),
            (0x7C00_0014, Opcode::Addcx),
            (0x7C00_0114, Opcode::Addex),
            (0x3800_0000, Opcode::Addi),
            (0x3000_0000, Opcode::Addic),
            (0x3400_0000, Opcode::Addicrc),
            (0x3C00_0000, Opcode::Addis),
            (0x7C00_01D4, Opcode::Addmex),
            (0x7C00_0194, Opcode::Addzex),
            (0x7C00_0038, Opcode::Andx),
            (0x7C00_0078, Opcode::Andcx),
            (0x7000_0000, Opcode::Andirc),
            (0x7400_0000, Opcode::Andisrc),
            (0x4800_0000, Opcode::Bx),
            (0x4000_0000, Opcode::Bcx),
            (0x4C00_0420, Opcode::Bcctrx),
            (0x4c00_0020, Opcode::Bclrx),
            (0x7C00_0000, Opcode::Cmp),
            (0x2C00_0000, Opcode::Cmpi),
            (0x7C00_0040, Opcode::Cmpl),
            (0x2800_0000, Opcode::Cmpli),
            (0x7C00_0034, Opcode::Cntlzwx),
            (0x4C00_0202, Opcode::Crand),
            (0x4C00_0102, Opcode::Crandc),
            (0x4C00_0242, Opcode::Creqv),
            (0x4C00_01C2, Opcode::Crnand),
            (0x4C00_0042, Opcode::Crnor),
            (0x4C00_0382, Opcode::Cror),
            (0x4C00_0342, Opcode::Crorc),
            (0x4C00_0182, Opcode::Crxor),
            (0x7C00_00AC, Opcode::Dcbf),
            (0x7C00_03AC, Opcode::Dcbi),
            (0x7C00_006C, Opcode::Dcbst),
            (0x7C00_022C, Opcode::Dcbt),
            (0x7C00_01EC, Opcode::Dcbtst),
            (0x7C00_07EC, Opcode::Dcbz),
            (0x1000_07EC, Opcode::DcbzL),
            (0x7C00_03D6, Opcode::Divwx),
            (0x7C00_0396, Opcode::Divwux),
            (0x7C00_026C, Opcode::Eciwx),
            (0x7C00_036C, Opcode::Ecowx),
            (0x7C00_06AC, Opcode::Eieio),
            (0x7C00_0238, Opcode::Eqvx),
            (0x7C00_0774, Opcode::Extsbx),
            (0x7C00_0734, Opcode::Extshx),
            (0xFC00_0210, Opcode::Fabsx),
            (0xFC00_002A, Opcode::Faddx),
            (0xEC00_002A, Opcode::Faddsx),
            (0xFC00_0040, Opcode::Fcmpo),
            (0xFC00_0000, Opcode::Fcmpu),
            (0xFC00_001C, Opcode::Fctiwx),
            (0xFC00_001E, Opcode::Fctiwzx),
            (0xFC00_0024, Opcode::Fdivx),
            (0xEC00_0024, Opcode::Fdivsx),
            (0xFC00_003A, Opcode::Fmaddx),
            (0xEC00_003A, Opcode::Fmaddsx),
            (0xFC00_0090, Opcode::Fmrx),
            (0xFC00_0038, Opcode::Fmsubx),
            (0xEC00_0038, Opcode::Fmsubsx),
            (0xFC00_0032, Opcode::Fmulx),
            (0xEC00_0032, Opcode::Fmulsx),
            (0xFC00_0110, Opcode::Fnabsx),
            (0xFC00_0050, Opcode::Fnegx),
            (0xFC00_003E, Opcode::Fnmaddx),
            (0xEC00_003E, Opcode::Fnmaddsx),
            (0xFC00_003C, Opcode::Fnmsubx),
            (0xEC00_003C, Opcode::Fnmsubsx),
            (0xEC00_0030, Opcode::Fresx),
            (0xFC00_0018, Opcode::Frspx),
            (0xFC00_0034, Opcode::Frsqrtex),
            (0xFC00_002E, Opcode::Fselx),
            (0xFC00_0028, Opcode::Fsubx),
            (0xEC00_0028, Opcode::Fsubsx),
            (0x7C00_07AC, Opcode::Icbi),
            (0x4C00_012C, Opcode::Isync),
            (0x8800_0000, Opcode::Lbz),
            (0x8C00_0000, Opcode::Lbzu),
            (0x7C00_00EE, Opcode::Lbzux),
            (0x7C00_00AE, Opcode::Lbzx),
            (0xC800_0000, Opcode::Lfd),
            (0xCC00_0000, Opcode::Lfdu),
            (0x7C00_04EE, Opcode::Lfdux),
            (0x7C00_04AE, Opcode::Lfdx),
            (0xC000_0000, Opcode::Lfs),
            (0xC400_0000, Opcode::Lfsu),
            (0x7C00_046E, Opcode::Lfsux),
            (0x7C00_042E, Opcode::Lfsx),
            (0xA800_0000, Opcode::Lha),
            (0xAC00_0000, Opcode::Lhau),
            (0x7C00_02EE, Opcode::Lhaux),
            (0x7C00_02AE, Opcode::Lhax),
            (0x7C00_062C, Opcode::Lhbrx),
            (0xA000_0000, Opcode::Lhz),
            (0xA400_0000, Opcode::Lhzu),
            (0x7C00_026E, Opcode::Lhzux),
            (0x7C00_022E, Opcode::Lhzx),
            (0xB800_0000, Opcode::Lmw),
            (0x7C00_04AA, Opcode::Lswi),
            (0x7C00_042A, Opcode::Lswx),
            (0x7C00_0028, Opcode::Lwarx),
            (0x7C00_042C, Opcode::Lwbrx),
            (0x8000_0000, Opcode::Lwz),
            (0x8400_0000, Opcode::Lwzu),
            (0x7C00_006E, Opcode::Lwzux),
            (0x7C00_002E, Opcode::Lwzx),
            (0x4C00_0000, Opcode::Mcrf),
            (0xFC00_0080, Opcode::Mcrfs),
            (0x7c00_0400, Opcode::Mcrxr),
            (0x7C00_0026, Opcode::Mfcr),
            (0xFC00_048E, Opcode::Mffsx),
            (0x7C00_00A6, Opcode::Mfmsr),
            (0x7C00_02A6, Opcode::Mfspr),
            (0x7C00_04A6, Opcode::Mfsr),
            (0x7C00_0526, Opcode::Mfsrin),
            (0x7C00_02E6, Opcode::Mftb),
            (0x7C00_0120, Opcode::Mtcrf),
            (0xFC00_008C, Opcode::Mtfsb0x),
            (0xFC00_004C, Opcode::Mtfsb1x),
            (0xFC00_058E, Opcode::Mtfsfx),
            (0xFC00_010C, Opcode::Mtfsfix),
            (0x7C00_0124, Opcode::Mtmsr),
            (0x7C00_03A6, Opcode::Mtspr),
            (0x7C00_01A4, Opcode::Mtsr),
            (0x7C00_01E4, Opcode::Mtsrin),
            (0x7C00_0096, Opcode::Mulhwx),
            (0x7C00_0016, Opcode::Mulhwux),
            (0x1C00_0000, Opcode::Mulli),
            (0x7C00_01D6, Opcode::Mullwx),
            (0x7C00_03B8, Opcode::Nandx),
            (0x7C00_00D0, Opcode::Negx),
            (0x7C00_00F8, Opcode::Norx),
            (0x7C00_0378, Opcode::Orx),
            (0x7C00_0338, Opcode::Orcx),
            (0x6000_0000, Opcode::Ori),
            (0x6400_0000, Opcode::Oris),
            (0xE000_0000, Opcode::PsqL),
            (0xE400_0000, Opcode::PsqLu),
            (0x1000_004C, Opcode::PsqLux),
            (0x1000_000C, Opcode::PsqLx),
            (0xF000_0000, Opcode::PsqSt),
            (0xF400_0000, Opcode::PsqStu),
            (0x1000_004E, Opcode::PsqStux),
            (0x1000_000E, Opcode::PsqStx),
            (0x1000_0210, Opcode::PsAbsx),
            (0x1000_002A, Opcode::PsAddx),
            (0x1000_0040, Opcode::PsCmpo0),
            (0x1000_00C0, Opcode::PsCmpo1),
            (0x1000_0000, Opcode::PsCmpu0),
            (0x1000_0080, Opcode::PsCmpu1),
            (0x1000_0024, Opcode::PsDivx),
            (0x1000_003A, Opcode::PsMaddx),
            (0x1000_001C, Opcode::PsMadds0x),
            (0x1000_001E, Opcode::PsMadds1x),
            (0x1000_0420, Opcode::PsMerge00x),
            (0x1000_0460, Opcode::PsMerge01x),
            (0x1000_04A0, Opcode::PsMerge10x),
            (0x1000_04E0, Opcode::PsMerge11x),
            (0x1000_0090, Opcode::PsMrx),
            (0x1000_0038, Opcode::PsMsubx),
            (0x1000_0032, Opcode::PsMulx),
            (0x1000_0018, Opcode::PsMuls0x),
            (0x1000_001A, Opcode::PsMuls1x),
            (0x1000_0110, Opcode::PsNabsx),
            (0x1000_0050, Opcode::PsNegx),
            (0x1000_003E, Opcode::PsNmaddx),
            (0x1000_003C, Opcode::PsNmsubx),
            (0x1000_0030, Opcode::PsResx),
            (0x1000_0034, Opcode::PsRsqrtex),
            (0x1000_002E, Opcode::PsSelx),
            (0x1000_0028, Opcode::PsSubx),
            (0x1000_0014, Opcode::PsSum0x),
            (0x1000_0016, Opcode::PsSum1x),
            (0x4C00_0064, Opcode::Rfi),
            (0x5000_0000, Opcode::Rlwimix),
            (0x5400_0000, Opcode::Rlwinmx),
            (0x5C00_0000, Opcode::Rlwnmx),
            (0x4400_0002, Opcode::Sc),
            (0x7C00_0030, Opcode::Slwx),
            (0x7C00_0630, Opcode::Srawx),
            (0x7C00_0670, Opcode::Srawix),
            (0x7C00_0430, Opcode::Srwx),
            (0x9800_0000, Opcode::Stb),
            (0x9C00_0000, Opcode::Stbu),
            (0x7C00_01EE, Opcode::Stbux),
            (0x7C00_01AE, Opcode::Stbx),
            (0xD800_0000, Opcode::Stfd),
            (0xDC00_0000, Opcode::Stfdu),
            (0x7C00_05EE, Opcode::Stfdux),
            (0x7C00_05AE, Opcode::Stfdx),
            (0x7C00_07AE, Opcode::Stfiwx),
            (0xD000_0000, Opcode::Stfs),
            (0xD400_0000, Opcode::Stfsu),
            (0x7C00_056E, Opcode::Stfsux),
            (0x7C00_052E, Opcode::Stfsx),
            (0xB000_0000, Opcode::Sth),
            (0x7C00_072C, Opcode::Sthbrx),
            (0xB400_0000, Opcode::Sthu),
            (0x7C00_036E, Opcode::Sthux),
            (0x7C00_032E, Opcode::Sthx),
            (0xBC00_0000, Opcode::Stmw),
            (0x7C00_05AA, Opcode::Stswi),
            (0x7C00_052A, Opcode::Stswx),
            (0x9000_0000, Opcode::Stw),
            (0x7C00_052C, Opcode::Stwbrx),
            (0x7C00_012D, Opcode::Stwcxrc),
            (0x9400_0000, Opcode::Stwu),
            (0x7C00_016E, Opcode::Stwux),
            (0x7C00_012E, Opcode::Stwx),
            (0x7C00_0050, Opcode::Subfx),
            (0x7C00_0010, Opcode::Subfcx),
            (0x7C00_0110, Opcode::Subfex),
            (0x2000_0000, Opcode::Subfic),
            (0x7C00_01D0, Opcode::Subfmex),
            (0x7C00_0190, Opcode::Subfzex),
            (0x7C00_04AC, Opcode::Sync),
            (0x7C00_0264, Opcode::Tlbie),
            (0x7C00_046C, Opcode::Tlbsync),
            (0x7C00_0008, Opcode::Tw),
            (0x0C00_0000, Opcode::Twi),
            (0x7C00_0278, Opcode::Xorx),
            (0x6800_0000, Opcode::Xori),
            (0x6C00_0000, Opcode::Xoris),
        ];

        for i in data.iter() {
            let instr = Instruction(i.0);
            let opcode = optable[instr.opcd()];
            match opcode {
                Opcode::Table4 => assert_eq!(optable4[instr.xo_x()], i.1),
                Opcode::Table19 => assert_eq!(optable19[instr.xo_x()], i.1),
                Opcode::Table31 => assert_eq!(optable31[instr.xo_x()], i.1),
                Opcode::Table59 => assert_eq!(optable59[instr.xo_a()], i.1),
                Opcode::Table63 => assert_eq!(optable63[instr.xo_x()], i.1),
                _ => assert_eq!(opcode, i.1),
            }
        }
    }

    #[test]
    fn op_bcx() {
        let mut ctx = Context::default();

        // addi 8,0,3
        let (rd, ra, simm) = (8, 0, 0x3);
        let instr = Instruction::new_addi(rd, ra, simm);

        super::op_addi(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x0000_0003);

        // mtctr 8
        let instr = Instruction::new_mtspr(0x9, 0x8);
        super::op_mtspr(&mut ctx, instr);

        // check counter register is set to 0x3
        assert_eq!(ctx.cpu.spr[SPR_CTR], 0x0000_0003);

        // addic. 9,8,0x1
        let (rd, ra, simm) = (9, 8, 0x1);
        let instr = Instruction::new_addic_rc(rd, ra, simm);

        super::op_addic_rc(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x0000_0004);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x0000_0004);

        // bc 0xC,1,0x456
        let (bo, bi, bd) = (0xC, 1, 0x456);
        let instr = Instruction::new_bcx(bo, bi, bd);

        super::op_bcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.nia, 0xFFF0_1258);

        // bcl 0x8,1,0x456
        let (bo, bi, bd, lk) = (0x8, 1, 0x456, 1);
        let instr = Instruction::new_bcx(bo, bi, bd).set_lk(lk);

        super::op_bcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.spr[SPR_CTR], 0x2);
        assert_eq!(ctx.cpu.spr[SPR_LR], 0xFFF0_0104);
    }

    #[test]
    fn op_addi() {
        let mut ctx = Context::default();

        let (rd, ra, simm) = (4, 5, 0x8FF0);
        let instr = Instruction::new_addi(rd, ra, simm);

        ctx.cpu.gpr[ra] = 0x0000_0900;
        super::op_addi(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0xFFFF_98F0);
    }

    #[test]
    fn op_addic() {
        let mut ctx = Context::default();

        let (rd, ra, simm) = (6, 4, 0xFFFF);
        let instr = Instruction::new_addic(rd, ra, simm);

        ctx.cpu.gpr[ra] = 0x0000_2346;

        super::op_addic(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x0000_2345)
    }

    #[test]
    fn op_addic_rc() {
        let mut ctx = Context::default();
        let (rd, ra, simm) = (31, 3, 1);
        let instr = Instruction::new_addic_rc(rd, ra, simm);

        ctx.cpu.gpr[rd] = 0xDEAD_BEEF;
        ctx.cpu.gpr[ra] = 0xFFFF_FFFF;

        super::op_addic_rc(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x0);
        assert_eq!(ctx.cpu.gpr[ra], 0xFFFF_FFFF); // confirm gpr source remains unmodified
        assert!(ctx.cpu.xer.carry());

        ctx.cpu.gpr[ra] = 0xFFFF_FFFE;

        super::op_addic_rc(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0xFFFF_FFFF);
        assert!(!ctx.cpu.xer.carry());
    }

    #[test]
    fn op_addis() {
        let mut ctx = Context::default();

        let (rd, ra, simm) = (7, 6, 0x0011);
        let instr = Instruction::new_addis(rd, ra, simm);

        ctx.cpu.gpr[ra] = 0x0000_4000;
        super::op_addis(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x0011_4000);
    }

    #[test]
    fn op_addex() {
        let mut ctx = Context::default();

        let (rd, ra, rb) = (6, 4, 10);
        let instr = Instruction::new_addex(rd, ra, rb);

        ctx.cpu.gpr[ra] = 0x1000_0400;
        ctx.cpu.gpr[rb] = 0x1000_0400;
        ctx.cpu.xer.set_carry(true);
        super::op_addex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x2000_0801);

        ctx.cpu.gpr[ra] = 0x9000_3000;
        ctx.cpu.gpr[rb] = 0x7B41_92C0;
        ctx.cpu.xer.set_carry(false);
        super::op_addex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x0B41_C2C0);

        ctx.cpu.gpr[ra] = 0x1000_0400;
        ctx.cpu.gpr[rb] = 0xEFFF_FFFF;
        ctx.cpu.xer.set_carry(true);
        super::op_addex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x0000_0400);

        ctx.cpu.gpr[ra] = 0x9000_3000;
        ctx.cpu.gpr[rb] = 0x8000_7000;
        ctx.cpu.xer.set_carry(false);
        super::op_addex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x1000_A000);
    }

    #[test]
    fn op_addzex() {
        let mut ctx = Context::default();

        let (rd, ra) = (6, 4);
        let instr = Instruction::new_addzex(rd, ra);
        ctx.cpu.gpr[ra] = 0x7B41_92C0;
        ctx.cpu.xer.set_carry(false);
        super::op_addzex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x7B41_92C0);

        ctx.cpu.gpr[ra] = 0xEFFF_FFFF;
        ctx.cpu.xer.set_carry(true);
        super::op_addzex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0xF000_0000);

        ctx.cpu.gpr[ra] = 0x9000_3000;
        ctx.cpu.xer.set_carry(true);
        super::op_addzex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x9000_3001);

        ctx.cpu.gpr[ra] = 0xEFFF_FFFF;
        ctx.cpu.xer.set_carry(false);
        super::op_addzex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0xEFFF_FFFF);
    }

    #[test]
    fn op_addx() {
        let mut ctx = Context::default();

        let (rd, ra, rb) = (4, 6, 3);
        let instr = Instruction::new_addx(rd, ra, rb);
        ctx.cpu.gpr[ra] = 0x0004_0000;
        ctx.cpu.gpr[rb] = 0x0000_4000;
        super::op_addx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x0004_4000);
        assert!(!ctx.cpu.xer.carry());

        let instr = instr.set_rc(1);

        ctx.cpu.gpr[ra] = 0x8000_7000;
        ctx.cpu.gpr[rb] = 0x7000_8000;
        super::op_addx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0xF000_F000);
        assert!(!ctx.cpu.xer.carry());
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT

        ctx.cpu.gpr[ra] = 0xEFFF_FFFF;
        ctx.cpu.gpr[rb] = 0x8000_0000;
        super::op_addx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x6FFF_FFFF);
        // FIXME: check Summary Overflow, Overflow and carry bits are set in Fixed point
        // register

        ctx.cpu.gpr[ra] = 0xEFFF_FFFF;
        ctx.cpu.gpr[rb] = 0xEFFF_FFFF;
        super::op_addx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0xDFFF_FFFE);
        // FIXME: check check Summary Overflow, Overflow and carry bits are set in Fixed point
        // register, as well as condition register field 0 updated
    }

    #[test]
    fn op_addcx() {
        let mut ctx = Context::default();

        let (rd, ra, rb) = (6, 4, 10);
        let instr = Instruction::new_addcx(rd, ra, rb);
        ctx.cpu.gpr[ra] = 0x9000_3000;
        ctx.cpu.gpr[rb] = 0x8000_7000;
        super::op_addcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x1000_A000);

        let instr = instr.set_rc(1);

        ctx.cpu.gpr[ra] = 0x7000_3000;
        ctx.cpu.gpr[rb] = 0xFFFF_FFFF;
        super::op_addcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x7000_2FFF);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT

        let instr = instr.set_oe(1);

        ctx.cpu.gpr[ra] = 0x9000_3000;
        ctx.cpu.gpr[rb] = 0x7B41_92C0;
        super::op_addcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x0B41_C2C0);
        assert!(ctx.cpu.xer.carry());
        // FIXME: check Summary Overflow and Overflow bits are set

        ctx.cpu.gpr[ra] = 0x8000_0000;
        ctx.cpu.gpr[rb] = 0x8000_7000;
        ctx.cpu.xer.set_carry(false);
        super::op_addcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x0000_7000);
        assert!(ctx.cpu.xer.carry());
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x5); // GT, SO

        // FIXME: check Summery Overflow and Overflow bits set
    }

    #[test]
    fn op_andx() {
        let mut ctx = Context::default();

        let (ra, rs, rb) = (6, 4, 7);
        let instr = Instruction::new_andx(ra, rs, rb);

        ctx.cpu.gpr[rs] = 0xFFF2_5730;
        ctx.cpu.gpr[rb] = 0x7B41_92C0;
        super::op_andx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0x7B40_1200);

        let instr = instr.set_rc(1);

        ctx.cpu.gpr[rs] = 0xFFF2_5730;
        ctx.cpu.gpr[rb] = 0xFFFF_EFFF;
        super::op_andx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0xFFF2_4730);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT
    }

    #[test]
    fn op_andcx() {
        let mut ctx = Context::default();

        let (ra, rs, rb) = (6, 4, 5);
        let instr = Instruction::new_andcx(ra, rs, rb);

        ctx.cpu.gpr[rs] = 0x9000_3000;
        ctx.cpu.gpr[rb] = 0xFFFF_FFFF;
        super::op_andcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0x0000_0000);

        let instr = instr.set_rc(1);

        ctx.cpu.gpr[rs] = 0xB004_3000;
        ctx.cpu.gpr[rb] = 0x7676_7676;
        super::op_andcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0x8000_0000);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT
    }

    #[test]
    fn op_andi_rc() {
        let mut ctx = Context::default();

        let (ra, rs) = (6, 4);
        let uimm = 0x5730;
        let instr = Instruction::new_andi_rc(ra, rs, uimm);

        ctx.cpu.gpr[rs] = 0x7B41_92C0;
        super::op_andi_rc(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0x0000_1200);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT
    }

    #[test]
    fn op_cmp() {
        let mut ctx = Context::default();

        let (crfd, l, ra, rb) = (0, 0, 4, 6);
        let instr = Instruction::new_cmp(crfd, l, ra, rb);

        ctx.cpu.gpr[ra] = 0xFFFF_FFE7;
        ctx.cpu.gpr[rb] = 0x0000_0011;
        super::op_cmp(&mut ctx, instr);

        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT
    }

    #[test]
    fn op_cmpi() {
        let mut ctx = Context::default();

        let (crfd, l, ra, simm) = (0, 0, 4, 0x11);
        let instr = Instruction::new_cmpi(crfd, l, ra, simm);

        ctx.cpu.gpr[ra] = 0xFFFF_FFE7;
        super::op_cmpi(&mut ctx, instr);

        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT
    }

    #[test]
    fn op_cmpl() {
        let mut ctx = Context::default();

        let (crfd, l, ra, rb) = (0, 0, 4, 5);
        let instr = Instruction::new_cmpl(crfd, l, ra, rb);

        ctx.cpu.gpr[ra] = 0xFFFF_0000;
        ctx.cpu.gpr[rb] = 0x7FFF_0000;
        super::op_cmpl(&mut ctx, instr);

        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT
    }

    #[test]
    fn op_cmpli() {
        let mut ctx = Context::default();

        let (crfd, l, ra, uimm) = (0, 0, 4, 0xFF);
        let instr = Instruction::new_cmpli(crfd, l, ra, uimm);

        ctx.cpu.gpr[ra] = 0x0000_00FF;
        super::op_cmpli(&mut ctx, instr);

        assert_eq!(ctx.cpu.cr.get_cr0(), 0x2); // EQ
    }

    #[test]
    fn op_cntlzwx() {
        let mut ctx = Context::default();

        let (ra, rs) = (3, 3);
        let instr = Instruction::new_cntlzwx(ra, rs);

        ctx.cpu.gpr[ra] = 0x0061_9920;
        super::op_cntlzwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rs], 0x0000_0009);
    }

    #[test]
    fn op_divwx() {
        let mut ctx = Context::default();

        let (rd, ra, rb) = (4, 4, 6);
        let instr = Instruction::new_divwx(rd, ra, rb);

        ctx.cpu.gpr[ra] = 0x0000_0000;
        ctx.cpu.gpr[rb] = 0x0000_0002;
        super::op_divwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x0000_0000);

        let instr = instr.set_rc(1);

        ctx.cpu.gpr[ra] = 0x0000_0002;
        ctx.cpu.gpr[rb] = 0x0000_0002;
        super::op_divwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x0000_0001);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT

        ctx.cpu.gpr[ra] = 0x0000_0001;
        ctx.cpu.gpr[rb] = 0x0000_0002;
        super::op_divwx(&mut ctx, instr);

        // Undefined
        assert_eq!(ctx.cpu.gpr[rd], 0x0000_0000);

        ctx.cpu.gpr[ra] = 0x8000_0000;
        ctx.cpu.gpr[rb] = 0xFFFF_FFFF;
        super::op_divwx(&mut ctx, instr);

        // Undefined
        assert_eq!(ctx.cpu.gpr[rd], 0x0000_0000);
    }

    #[test]
    fn op_divwux() {
        let mut ctx = Context::default();

        let (rd, ra, rb) = (4, 4, 6);
        let instr = Instruction::new_divwux(rd, ra, rb);

        ctx.cpu.gpr[ra] = 0x0000_0000;
        ctx.cpu.gpr[rb] = 0x0000_0002;
        super::op_divwux(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x0000_0000);

        let instr = instr.set_rc(1);

        ctx.cpu.gpr[ra] = 0x0000_0002;
        ctx.cpu.gpr[rb] = 0x0000_0002;
        super::op_divwux(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x0000_0001);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT

        ctx.cpu.gpr[ra] = 0x0000_0001;
        ctx.cpu.gpr[rb] = 0x0000_0002;
        super::op_divwux(&mut ctx, instr);

        // Undefined
        assert_eq!(ctx.cpu.gpr[rd], 0x0000_0000);

        ctx.cpu.gpr[ra] = 0x8000_0000;
        ctx.cpu.gpr[rb] = 0xFFFF_FFFF;
        super::op_divwux(&mut ctx, instr);

        // Undefined
        assert_eq!(ctx.cpu.gpr[rd], 0x0000_0000);
    }

    #[test]
    fn op_extsbx() {
        let mut ctx = Context::default();

        let (ra, rs) = (4, 6);
        let instr = Instruction::new_extsbx(ra, rs);

        ctx.cpu.gpr[rs] = 0x5A5A_5A5A;
        super::op_extsbx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0x0000_005A);

        ctx.cpu.gpr[rs] = 0xA5A5_A5A5;
        super::op_extsbx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0xFFFF_FFA5);
    }

    #[test]
    fn op_extshx() {
        let mut ctx = Context::default();

        let (ra, rs) = (4, 6);
        let instr = Instruction::new_extshx(ra, rs);

        ctx.cpu.gpr[rs] = 0x0000_FFFF;
        super::op_extshx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0xFFFF_FFFF);

        ctx.cpu.gpr[rs] = 0x0000_2FFF;
        super::op_extshx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0x0000_2FFF);
    }

    #[test]
    fn op_mulhwux() {
        let mut ctx = Context::default();

        let (rd, ra, rb) = (6, 4, 10);
        let instr = Instruction::new_mulhwux(rd, ra, rb);

        ctx.cpu.gpr[ra] = 0x0000_0003;
        ctx.cpu.gpr[rb] = 0x0000_0002;
        super::op_mulhwux(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x0000_0000);

        let instr = instr.set_rc(1);

        ctx.cpu.gpr[ra] = 0x0000_4500;
        ctx.cpu.gpr[rb] = 0x8000_7000;
        super::op_mulhwux(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x0000_2280);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT
    }

    #[test]
    fn op_mulli() {
        let mut ctx = Context::default();

        let (rd, ra, simm) = (6, 4, 10);
        let instr = Instruction::new_mulli(rd, ra, simm);

        ctx.cpu.gpr[ra] = 0x0000_3000;
        super::op_mulli(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x0001_E000);
    }

    #[test]
    fn op_mullwx() {
        let mut ctx = Context::default();

        let (rd, ra, rb) = (6, 4, 10);
        let instr = Instruction::new_mullwx(rd, ra, rb);

        ctx.cpu.gpr[ra] = 0x0000_3000;
        ctx.cpu.gpr[rb] = 0x0000_7000;
        super::op_mullwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x1500_0000);

        let instr = instr.set_rc(1);

        ctx.cpu.gpr[ra] = 0x0000_4500;
        ctx.cpu.gpr[rb] = 0x0000_7000;
        super::op_mullwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x1E30_0000);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT

        let instr = instr.set_oe(1);

        ctx.cpu.gpr[ra] = 0x0000_4500;
        ctx.cpu.gpr[rb] = 0x0007_0000;
        super::op_mullwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0xE300_0000);
        // FIXME: check summary overflow and overflow

        ctx.cpu.gpr[ra] = 0x0000_4500;
        ctx.cpu.gpr[rb] = 0x7FFF_FFFF;
        super::op_mullwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0xFFFF_BB00);
        // FIXME: check summary overflow and overflow
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x9); // LT SO
    }

    #[test]
    fn op_negx() {
        let mut ctx = Context::default();

        let (rd, ra) = (6, 4);
        let instr = Instruction::new_negx(rd, ra);

        ctx.cpu.gpr[ra] = 0x9000_3000;
        super::op_negx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x6FFF_D000);

        let instr = instr.set_rc(1);

        ctx.cpu.gpr[ra] = 0x789A_789B;
        super::op_negx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x8765_8765);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT

        ctx.cpu.gpr[ra] = 0x9000_3000;
        super::op_negx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x6FFF_D000);
        // FIXME: check summary overflow and overflow bits

        ctx.cpu.gpr[ra] = 0x8000_0000;
        super::op_negx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x8000_0000);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT

        // FIXME: check summary overflow and overflow bits
    }

    #[test]
    fn op_norx() {
        let mut ctx = Context::default();

        let (ra, rs, rb) = (6, 4, 7);
        let instr = Instruction::new_norx(ra, rs, rb);

        ctx.cpu.gpr[rs] = 0x9000_3000;
        ctx.cpu.gpr[rb] = 0x789A_789B;
        super::op_norx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0x0765_8764);

        let instr = instr.set_rc(1);

        ctx.cpu.gpr[rs] = 0xB004_3000;
        ctx.cpu.gpr[rb] = 0x789A_789B;
        super::op_norx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0x0761_8764);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT
    }

    #[test]
    fn op_orx() {
        let mut ctx = Context::default();

        let (ra, rs, rb) = (6, 4, 7);
        let instr = Instruction::new_orx(ra, rs, rb);

        ctx.cpu.gpr[rs] = 0x9000_3000;
        ctx.cpu.gpr[rb] = 0x789A_789B;
        super::op_orx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0xF89A_789B);

        let instr = instr.set_rc(1);

        ctx.cpu.gpr[rs] = 0xB004_3000;
        ctx.cpu.gpr[rb] = 0x789A_789B;
        super::op_orx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0xF89E_789B);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT
    }

    #[test]
    fn op_ori() {
        let mut ctx = Context::default();

        let (rs, ra, uimm) = (6, 4, 0x0079);
        let instr = Instruction::new_ori(ra, rs, uimm);

        ctx.cpu.gpr[rs] = 0x9000_3000;
        super::op_ori(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0x9000_3079);
    }

    #[test]
    fn op_oris() {
        let mut ctx = Context::default();

        let (rs, ra, uimm) = (6, 4, 0x0079);
        let instr = Instruction::new_oris(ra, rs, uimm);

        ctx.cpu.gpr[rs] = 0x9000_3000;
        super::op_oris(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0x9079_3000);
    }

    #[test]
    fn op_rlwimix() {
        let mut ctx = Context::default();

        let (ra, rs, sh, mb, me) = (6, 4, 2, 0, 0x1D);
        let instr = Instruction::new_rlwimix(ra, rs, sh, mb, me);

        ctx.cpu.gpr[rs] = 0x9000_3000;
        ctx.cpu.gpr[ra] = 0x0000_0003;
        super::op_rlwimix(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0x4000_C003);

        let (mb, me) = (0, 0x1A);
        let instr = Instruction::new_rlwimix(ra, rs, sh, mb, me).set_rc(1);

        ctx.cpu.gpr[rs] = 0x789A_789B;
        ctx.cpu.gpr[ra] = 0x3000_0003;
        super::op_rlwimix(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0xE269_E263);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT
    }

    #[test]
    fn op_rlwinmx() {
        let mut ctx = Context::default();

        let (ra, rs, sh, mb, me) = (6, 4, 2, 0, 0x1D);
        let instr = Instruction::new_rlwinmx(ra, rs, sh, mb, me);

        ctx.cpu.gpr[rs] = 0x9000_3000;
        ctx.cpu.gpr[ra] = 0xFFFF_FFFF;
        super::op_rlwinmx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0x4000_C000);

        let instr = instr.set_rc(1);

        ctx.cpu.gpr[rs] = 0xB004_3000;
        ctx.cpu.gpr[ra] = 0xFFFF_FFFF;
        super::op_rlwinmx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0xC010_C000);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT
    }

    #[test]
    fn op_slwx() {
        let mut ctx = Context::default();

        let (ra, rs, rb) = (6, 4, 5);
        let instr = Instruction::new_slwx(ra, rs, rb);

        ctx.cpu.gpr[rb] = 0x0000_002F;
        ctx.cpu.gpr[rs] = 0xFFFF_FFFF;
        super::op_slwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0x0000_0000);

        let instr = instr.set_rc(1);

        ctx.cpu.gpr[rb] = 0x0000_0005;
        ctx.cpu.gpr[rs] = 0xB004_3000;
        super::op_slwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0x0086_0000);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT
    }

    #[test]
    fn op_srawx() {
        let mut ctx = Context::default();

        let (ra, rs, rb) = (6, 4, 5);
        let instr = Instruction::new_srawx(ra, rs, rb);

        ctx.cpu.gpr[rs] = 0x9000_3000;
        ctx.cpu.gpr[rb] = 0x0000_0024;
        super::op_srawx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0xFFFF_FFFF);
        assert!(ctx.cpu.xer.carry());

        let instr = instr.set_rc(1);

        ctx.cpu.gpr[rs] = 0xB004_3000;
        ctx.cpu.gpr[rb] = 0x0000_0004;
        super::op_srawx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0xFB00_4300);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT

        //assert_eq!(ctx.cpu.xer.carry(), true);
    }

    #[test]
    fn op_srawix() {
        let mut ctx = Context::default();

        let (ra, rs, sh) = (6, 4, 0x4);
        let instr = Instruction::new_srawix(ra, rs, sh);

        ctx.cpu.gpr[rs] = 0x9000_3000;
        super::op_srawix(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0xF900_0300);
        assert!(!ctx.cpu.xer.carry());

        ctx.cpu.gpr[rs] = 0xB004_3008;
        super::op_srawix(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0xFB00_4300);
        assert!(ctx.cpu.xer.carry());
    }

    #[test]
    fn op_srwx() {
        let mut ctx = Context::default();

        let (ra, rs, rb) = (6, 4, 5);
        let instr = Instruction::new_srwx(ra, rs, rb);

        ctx.cpu.gpr[rs] = 0x9000_3000;
        ctx.cpu.gpr[rb] = 0x0000_0024;
        super::op_srwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0x0000_0000);

        let instr = instr.set_rc(1);

        ctx.cpu.gpr[rs] = 0xB004_3001;
        ctx.cpu.gpr[rb] = 0x0000_0004;
        super::op_srwx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0x0B00_4300);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT
    }

    #[test]
    fn op_subfx() {
        let mut ctx = Context::default();

        let (rd, ra, rb) = (6, 4, 10);
        let instr = Instruction::new_subfx(rd, ra, rb);

        ctx.cpu.gpr[ra] = 0x8000_7000;
        ctx.cpu.gpr[rb] = 0x9000_3000;
        super::op_subfx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x0FFF_C000);

        let instr = instr.set_rc(1);

        ctx.cpu.gpr[ra] = 0x0000_4500;
        ctx.cpu.gpr[rb] = 0x8000_7000;
        super::op_subfx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x8000_2B00);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT

        let instr = instr.set_oe(1);

        ctx.cpu.gpr[ra] = 0x8000_0000;
        ctx.cpu.gpr[rb] = 0x0000_4500;
        super::op_subfx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x8000_4500);
        // FIXME: check SO and O

        ctx.cpu.gpr[ra] = 0x8000_0000;
        ctx.cpu.gpr[rb] = 0x0000_7000;
        super::op_subfx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x8000_7000);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x9); // LT

        // FIXME: check SO and O
    }

    #[test]
    fn op_subfcx() {
        let mut ctx = Context::default();

        let (rd, ra, rb) = (6, 4, 10);
        let instr = Instruction::new_subfcx(rd, ra, rb);

        ctx.cpu.gpr[ra] = 0x8000_7000;
        ctx.cpu.gpr[rb] = 0x9000_3000;
        super::op_subfcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x0FFF_C000);
        assert!(ctx.cpu.xer.carry());

        let instr = instr.set_rc(1);

        ctx.cpu.gpr[ra] = 0x0000_4500;
        ctx.cpu.gpr[rb] = 0x8000_7000;
        super::op_subfcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x8000_2B00);
        assert!(ctx.cpu.xer.carry());
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT

        let instr = instr.set_oe(1);

        ctx.cpu.gpr[ra] = 0x8000_0000;
        ctx.cpu.gpr[rb] = 0x0000_4500;
        super::op_subfcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x8000_4500);
        assert!(!ctx.cpu.xer.carry());
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x9); // LT

        ctx.cpu.gpr[ra] = 0x8000_0000;
        ctx.cpu.gpr[rb] = 0x0000_7000;
        super::op_subfcx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x8000_7000);
        assert!(!ctx.cpu.xer.carry());
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x9); // LT
    }

    #[test]
    fn op_subfex() {
        let mut ctx = Context::default();

        let (rd, ra, rb) = (6, 4, 10);
        let instr = Instruction::new_subfex(rd, ra, rb);

        ctx.cpu.gpr[ra] = 0x9000_3000;
        ctx.cpu.gpr[rb] = 0x8000_7000;
        ctx.cpu.xer.set_carry(true);
        super::op_subfex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0xF000_4000);
        assert!(!ctx.cpu.xer.carry());

        let instr = instr.set_rc(1);

        ctx.cpu.gpr[ra] = 0x0000_4500;
        ctx.cpu.gpr[rb] = 0x8000_7000;
        ctx.cpu.xer.set_carry(false);
        super::op_subfex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x8000_2AFF);
        assert!(ctx.cpu.xer.carry());
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT

        let instr = instr.set_oe(1);

        ctx.cpu.gpr[ra] = 0x8000_0000;
        ctx.cpu.gpr[rb] = 0xEFFF_FFFF;
        ctx.cpu.xer.set_carry(true);
        super::op_subfex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x6FFF_FFFF);
        assert!(ctx.cpu.xer.carry());
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT

        ctx.cpu.gpr[ra] = 0x8000_0000;
        ctx.cpu.gpr[rb] = 0xEFFF_FFFF;
        ctx.cpu.xer.set_carry(false);
        super::op_subfex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x6FFF_FFFE);
        assert!(ctx.cpu.xer.carry());
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x4); // GT
    }

    #[test]
    fn op_subfic() {
        let mut ctx = Context::default();

        let (rd, ra, simm) = (6, 4, 0x7000);
        let instr = Instruction::new_subfic(rd, ra, simm);

        ctx.cpu.gpr[ra] = 0x9000_3000;
        super::op_subfic(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x7000_4000);
    }

    #[test]
    fn op_subfzex() {
        let mut ctx = Context::default();

        let (rd, ra) = (6, 4);
        let instr = Instruction::new_subfzex(rd, ra);

        ctx.cpu.gpr[ra] = 0x9000_3000;
        ctx.cpu.xer.set_carry(true);
        super::op_subfzex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x6FFF_D000);
        assert!(!ctx.cpu.xer.carry());

        ctx.cpu.gpr[ra] = 0xB004_3000;
        ctx.cpu.xer.set_carry(true);
        super::op_subfzex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x4FFB_D000);
        assert!(!ctx.cpu.xer.carry());

        ctx.cpu.gpr[ra] = 0xEFFF_FFFF;
        ctx.cpu.xer.set_carry(false);
        super::op_subfzex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x1000_0000);
        assert!(!ctx.cpu.xer.carry());

        ctx.cpu.gpr[ra] = 0x70FB_6500;
        ctx.cpu.xer.set_carry(false);
        super::op_subfzex(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x8F04_9AFF);
        assert!(!ctx.cpu.xer.carry());
    }

    #[test]
    fn op_twi() {
        let mut ctx = Context::default();

        let a = 4;
        let instr = Instruction::new_twi(0x4, 4, 0x10);

        ctx.cpu.gpr[a] = 0x0000_0010;
        super::op_twi(&mut ctx, instr);

        assert_eq!(ctx.cpu.exceptions, EXCEPTION_PROGRAM);

        // FIXME: check cause is trap
    }

    #[test]
    fn op_xorx() {
        let mut ctx = Context::default();

        let (ra, rs, rb) = (6, 4, 3);
        let instr = Instruction::new_xorx(ra, rs, rb);

        ctx.cpu.gpr[rs] = 0x9000_3000;
        ctx.cpu.gpr[rb] = 0x789A_789B;
        super::op_xorx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0xE89A_489B);

        let instr = instr.set_rc(1);

        ctx.cpu.gpr[rs] = 0xB004_3000;
        ctx.cpu.gpr[rb] = 0x789A_789B;
        super::op_xorx(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0xC89E_489B);
        assert_eq!(ctx.cpu.cr.get_cr0(), 0x8); // LT
    }

    #[test]
    fn op_xoris() {
        let mut ctx = Context::default();

        let (ra, rs, uimm) = (6, 4, 0x0079);
        let instr = Instruction::new_xoris(ra, rs, uimm);

        ctx.cpu.gpr[rs] = 0x9000_3000;
        super::op_xoris(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[ra], 0x9079_3000);
    }

    // load and store ops
    #[test]
    fn op_dcbf() {
        let mut ctx = Context::default();

        let (ra, rb) = (4, 3);
        let instr = Instruction::new_dcbf(ra, rb);

        super::op_dcbf(&mut ctx, instr);
    }

    // system ops

    #[test]
    fn op_eieio() {}

    #[test]
    fn op_isync() {}

    #[test]
    fn op_mfmsr() {
        let mut ctx = Context::default();

        let rd = 6;
        let instr = Instruction::new_mfmsr(rd);

        ctx.cpu.msr = 0x0D15_AA5E.into();

        super::op_mfmsr(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0x0D15_AA5E);
    }

    #[test]
    fn op_mfspr() {
        let mut ctx = Context::default();

        let (rd, spr) = (6, SPR_LR as u32); // FIXME: make spr a usize
        let instr = Instruction::new_mfspr(rd, spr);

        ctx.cpu.spr[SPR_LR] = 0xDEAD_BEEF;
        super::op_mfspr(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 0xDEAD_BEEF);
    }

    #[test]
    fn op_mfsr() {}

    #[test]
    fn op_mfsrin() {}

    #[test]
    fn op_mftb() {
        let mut ctx = Context::default();

        let (rd, tbr) = (6, TBR_TBL); // FIXME: make tbr usize
        let instr = Instruction::new_mftb(rd, tbr as u32);

        ctx.timers.tick(0x1784);
        super::op_mftb(&mut ctx, instr);

        assert_eq!(ctx.cpu.gpr[rd], 501); // FIXME: this needs to be better
    }

    #[test]
    fn op_mtmsr() {
        let mut ctx = Context::default();

        let rs = 6;
        let instr = Instruction::new_mtmsr(rs);

        ctx.cpu.gpr[rs] = 0x0D15_AA5E;

        super::op_mtmsr(&mut ctx, instr);

        assert_eq!(ctx.cpu.msr.0, 0x0D15_AA5E);
    }

    #[test]
    fn op_mtspr() {}

    #[test]
    fn op_mtsrin() {}

    #[test]
    fn op_rfi() {}

    #[test]
    fn op_sc() {
        let mut ctx = Context::default();

        let instr = Instruction::new_sc();

        super::op_sc(&mut ctx, instr);

        assert_eq!(ctx.cpu.exceptions, EXCEPTION_SYSTEM_CALL);
    }

    #[test]
    fn op_sync() {
        let mut ctx = Context::default();

        let instr = Instruction::new_sync();

        super::op_sync(&mut ctx, instr);
    }

    #[test]
    #[should_panic]
    fn op_tlbsync() {
        let mut ctx = Context::default();

        let instr = Instruction::new_tlbsync();

        super::op_tlbsync(&mut ctx, instr);
    }
}
