use crate::gp_fifo::BURST_SIZE;
use crate::mem;
use crate::utils::Halveable;
use crate::video::bp::BlittingProcessor;
use crate::video::xf::TransformUnit;
use crate::Context;
const CP_STATUS: u32 = 0x00;
const CP_CONTROL: u32 = 0x02;
const CP_CLEAR: u32 = 0x04;

//const CP_TOKEN: u32 = 0x0E;
//const CP_BOUNDING_BOX: u32 = 0x10;

const CP_FIFO_BASE_LO: u32 = 0x20;
const CP_FIFO_BASE_HI: u32 = 0x22;
const CP_FIFO_END_LO: u32 = 0x24;
const CP_FIFO_END_HI: u32 = 0x26;
const CP_FIFO_HIGH_WATERMARK_LO: u32 = 0x28;
const CP_FIFO_HIGH_WATERMARK_HI: u32 = 0x2A;
const CP_FIFO_LOW_WATERMARK_LO: u32 = 0x2C;
const CP_FIFO_LOW_WATERMARK_HI: u32 = 0x2E;
const CP_FIFO_RW_DISTANCE_LO: u32 = 0x30;
const CP_FIFO_RW_DISTANCE_HI: u32 = 0x32;
const CP_FIFO_WRITE_POINTER_LO: u32 = 0x34;
const CP_FIFO_WRITE_POINTER_HI: u32 = 0x36;
const CP_FIFO_READ_POINTER_LO: u32 = 0x38;
const CP_FIFO_READ_POINTER_HI: u32 = 0x3A;
//const CP_FIFO_BREAKPOINT_LO: u32 = 0x3C;
//const CP_FIFO_BREAKPOINT_HI: u32 = 0x3E;

// GP Packet Opcodes
const NO_OPERATION: u8 = 0x00; // NOP - No Operation
const LOAD_CP_REG: u8 = 0x08; // Load CP REG
const LOAD_XF_REG: u8 = 0x10; // Load XF REG

//const LOAD_INDX_A: u8 = 0x20; // Load INDX A
//const LOAD_INDX_B: u8 = 0x28; // Load INDX B
//const LOAD_INDX_C: u8 = 0x30; // Load INDX C
//const LOAD_INDX_D: u8 = 0x38; // Load INDX D
//const CMD_CALL_DL: u8 = 0x40; // CALL DL - Call Displaylist

const CMD_INV_VC: u8 = 0x48; // Invalidate Vertex Cache
const LOAD_BP_REG: u8 = 0x61; // Load BP REG (SU_ByPassCmd)

//0x80    QUADS - Draw Quads (*)
//0x90    TRIANGLES - Draw Triangles (*)
//0x98    TRIANGLESTRIP - Draw Triangle Strip (*)
//0xA0    TRIANGLEFAN - Draw Triangle Fan (*)
//0xA8    LINES - Draw Lines (*)
//0xB0    LINESTRIP - Draw Line Strip (*)
//0xB8    POINTS - Draw Points (*)

// Internal CP Registers
const MATIDX_REG_A: u8 = 0x30; // Texture Matrix Index 0-3
const MATIDX_REG_B: u8 = 0x40; // Texture Matrix Index 4-7

//const VCD_LO: u8 = 0x50; // Vertex Descriptor (VCD) Low
//const VCD_HI: u8 = 0x60; // Vertex Descriptor (VCD) High
const CP_VAT_REG_A: u8 = 0x70; // Vertex Attribute Table (VAT) group 0
const CP_VAT_REG_B: u8 = 0x80; // Vertex Attribute Table (VAT) group 1
const CP_VAT_REG_C: u8 = 0x90; // Vertex Attribute Table (VAT) group 2

//const ARRAY_BASE: u8 = 0xA0;
//const ARRAY_STRIDE: u8 = 0xb0;

const NUM_VAT_REGS: usize = 8;
const VAT_INDEX_MASK: u8 = 7;

#[derive(Debug, Default)]
pub struct CommandProcessor {
    status: StatusRegister,
    control: ControlRegister,
    clear: ClearRegister,
    //token: u16,
    //bounding_box_left: u16,
    //bounding_box_right: u16,
    //bounding_box_top: u16,
    //bounding_box_bottom: u16,
    fifo_base: u32,
    fifo_end: u32,
    fifo_high_watermark: u32,
    fifo_low_watermark: u32,
    fifo_rw_distance: u32,
    fifo_write_pointer: u32,
    fifo_read_pointer: u32,
    //fifo_breakpoint: u32,
    bp: BlittingProcessor,
    xf: TransformUnit,
    matrix_index_a: MatrixIndexA,
    matrix_index_b: MatrixIndexB,
    vat: [Vat; NUM_VAT_REGS],
}

impl CommandProcessor {
    fn fifo_size(&self) -> u32 {
        self.fifo_write_pointer - self.fifo_read_pointer
    }

    // Internal CP Registers
    fn load(&mut self, reg: u8, value: u32) {
        match reg & 0xF0 {
            MATIDX_REG_A => {
                self.matrix_index_a = value.into();
            }
            MATIDX_REG_B => {
                self.matrix_index_b = value.into();
            }
            CP_VAT_REG_A => {
                self.vat[(reg & VAT_INDEX_MASK) as usize].group0 = value.into();
            }
            CP_VAT_REG_B => {
                self.vat[(reg & VAT_INDEX_MASK) as usize].group1 = value.into();
            }
            CP_VAT_REG_C => {
                self.vat[(reg & VAT_INDEX_MASK) as usize].group2 = value.into();
            }
            _ => warn!(
                "Unrecognized internal CP register, reg: {:#x}, value: {:#x}",
                reg, value
            ),
        }
    }
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    pub struct StatusRegister(u16);
    impl Debug;
    pub gfx_fifo_overflow, _ : 0;
    pub gfx_fifo_underflow, _ : 1;
    pub gp_idle_for_reading, _ : 2;
    pub gp_idle_for_commands, _ : 3;
    pub breakpoint, _ : 4;
}

impl From<u16> for StatusRegister {
    fn from(v: u16) -> Self {
        StatusRegister(v)
    }
}

impl From<StatusRegister> for u16 {
    fn from(s: StatusRegister) -> u16 {
        s.0
    }
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    pub struct ControlRegister(u16);
    impl Debug;
    pub gp_fifo_read_enable, _ : 0;
    pub cp_irq_enable, _ : 1;
    pub fifo_overflow_int_enable, _ : 2;
    pub fifo_underflow_int_enable, _ : 3;
    pub gp_link_enable, _ : 4;
    pub bp_enable, _ : 5;
}

impl From<u16> for ControlRegister {
    fn from(v: u16) -> Self {
        ControlRegister(v)
    }
}

impl From<ControlRegister> for u16 {
    fn from(s: ControlRegister) -> u16 {
        s.0
    }
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    pub struct ClearRegister(u16);
    impl Debug;
    pub clear_fifo_overflow, _ : 0;
    pub clear_fifo_underflow, _ : 1;
}

bitfield! {
    #[derive(Default)]
    pub struct MatrixIndexA(u32);
    impl Debug;
    u8;
    get_pos_index, _: 5, 0;     // POSIDX - Index for Position/Normal matrix
    get_tex_0_index, _: 11, 6;  // TEX0IDX - Index for Texture 0 matrix
    get_tex_1_index, _: 17, 12; // TEX1IDX - Index for Texture 1 matrix
    get_tex_2_index, _: 23, 18; // TEX2IDX - Index for Texture 2 matrix
    get_tex_3_index, _: 29, 24; // TEX3IDX - Index for Texture 3 matrix
}

impl From<u32> for MatrixIndexA {
    fn from(v: u32) -> Self {
        MatrixIndexA(v)
    }
}

bitfield! {
    #[derive(Default)]
    pub struct MatrixIndexB(u32);
    impl Debug;
    u8;
    get_tex_4_index, _: 5, 0;   // TEX4IDX - Index for Texture 4 matrix
    get_tex_5_index, _: 11, 6;  // TEX5IDX - Index for Texture 5 matrix
    get_tex_6_index, _: 17, 12; // TEX6IDX - Index for Texture 6 matrix
    get_tex_7_index, _: 23, 18; // TEX7IDX - Index for Texture 7 matrix
}

impl From<u32> for MatrixIndexB {
    fn from(v: u32) -> Self {
        MatrixIndexB(v)
    }
}

bitfield! {
    #[derive(Default)]
    struct VatGroup0(u32);
    impl Debug;
    u32;
    get_normal_index_3, _: 31;    // NORMALINDEX3 (0 - single index per normal, 1 - triple-index per nine-normal)
    get_byte_dequant, _: 30;      // BYTEDEQUANT (should always be 1)
    get_tex_0_shift, _: 29, 25;   // TEX0SHFT
    get_tex_0_format, _:24, 22;   // TEX0FMT
    get_tex_0_count, _: 21;       // TEX0CNT
    get_col_1_format, _: 20, 18;  // COL1FMT (Specular)
    get_col_1_count, _: 17;       // COL1CNT (Specular)
    get_col_0_format, _: 16, 14;  // COL0FMT (Diffused)
    get_col_0_count, _: 13;       // COL0CNT (Diffused)
    get_nrm_format, _: 12, 10;    // NRMFMT
    get_nrm_count, _: 9;          // NRMCNT
    get_pos_shift, _: 8, 4;       // POSSHFT
    get_pos_format, _: 3, 1;      // POSFMT
    get_pos_count, _: 0;          // POSCNT
}

impl From<u32> for VatGroup0 {
    fn from(v: u32) -> Self {
        VatGroup0(v)
    }
}

bitfield! {
    #[derive(Default)]
    struct VatGroup1(u32);
    impl Debug;
    u32;
    get_vcache_enhance, _: 31;    // VCACHE_ENHANCE (must always be 1)
    get_tex_4_format, _: 30, 28;  // TEX4FMT
    get_tex_4_count, _: 27;       // TEX4CNT
    get_tex_3_shift, _: 26, 22;   // TEX3SHFT
    get_tex_3_format, _: 21, 19;  // TEX3FMT
    get_tex_3_count, _: 18;       // TEX3CNT
    get_tex_2_shift, _: 17, 13;   // TEX2SHFT
    get_tex_2_format, _: 12, 10;  // TEX2FMT
    get_tex_2_count, _: 9;        // TEX2CNT
    get_tex_1_shift, _: 8, 4;     // TEX1SHFT
    get_tex_1_format, _: 3, 1;    // TEX1FMT
    get_tex_1_count, _: 0;        // TEX1CNT
}

impl From<u32> for VatGroup1 {
    fn from(v: u32) -> Self {
        VatGroup1(v)
    }
}

bitfield! {
    #[derive(Default)]
    struct VatGroup2(u32);
    impl Debug;
    u32;
    get_tex_7_shift, _: 31, 27;   // TEX7SHFT
    get_tex_7_format, _: 26, 24;  // TEX7FMT
    get_tex_7_count, _: 23;       // TEX7CNT
    get_tex_6_shift, _: 22, 18;   // TEX6SHFT
    get_tex_6_format, _: 17, 15;  // TEX6FMT
    get_tex_6_count, _: 14;       // TEX6CNT
    get_tex_5_shift, _: 13, 9;    // TEX5SHFT
    get_tex_5_format, _: 8, 6;    // TEX5FMT
    get_tex_5_count, _: 5;        // TEX5CNT
    get_tex_4_shift, _: 4, 0;     // TEX4SHFT
}

impl From<u32> for VatGroup2 {
    fn from(v: u32) -> Self {
        VatGroup2(v)
    }
}

#[derive(Default, Debug)]
struct Vat {
    group0: VatGroup0,
    group1: VatGroup1,
    group2: VatGroup2,
}

pub fn write_u16(ctx: &mut Context, register: u32, val: u16) {
    match register {
        CP_STATUS => ctx.cp.status = val.into(),
        CP_CONTROL => ctx.cp.control = val.into(),
        CP_CLEAR => ctx.cp.clear = ClearRegister(val),
        CP_FIFO_BASE_LO => ctx.cp.fifo_base = ctx.cp.fifo_base.set_lo(val),
        CP_FIFO_BASE_HI => ctx.cp.fifo_base = ctx.cp.fifo_base.set_hi(val),
        CP_FIFO_END_LO => ctx.cp.fifo_end = ctx.cp.fifo_end.set_lo(val),
        CP_FIFO_END_HI => ctx.cp.fifo_end = ctx.cp.fifo_end.set_hi(val),
        CP_FIFO_HIGH_WATERMARK_LO => {
            ctx.cp.fifo_high_watermark = ctx.cp.fifo_high_watermark.set_lo(val)
        }
        CP_FIFO_HIGH_WATERMARK_HI => {
            ctx.cp.fifo_high_watermark = ctx.cp.fifo_high_watermark.set_hi(val)
        }
        CP_FIFO_LOW_WATERMARK_LO => {
            ctx.cp.fifo_low_watermark = ctx.cp.fifo_low_watermark.set_lo(val)
        }
        CP_FIFO_LOW_WATERMARK_HI => {
            ctx.cp.fifo_low_watermark = ctx.cp.fifo_low_watermark.set_hi(val)
        }
        CP_FIFO_RW_DISTANCE_LO => ctx.cp.fifo_rw_distance = ctx.cp.fifo_rw_distance.set_lo(val),
        CP_FIFO_RW_DISTANCE_HI => ctx.cp.fifo_rw_distance = ctx.cp.fifo_rw_distance.set_hi(val),
        CP_FIFO_WRITE_POINTER_LO => {
            ctx.cp.fifo_write_pointer = ctx.cp.fifo_write_pointer.set_lo(val)
        }
        CP_FIFO_WRITE_POINTER_HI => {
            ctx.cp.fifo_write_pointer = ctx.cp.fifo_write_pointer.set_hi(val)
        }
        CP_FIFO_READ_POINTER_LO => ctx.cp.fifo_read_pointer = ctx.cp.fifo_read_pointer.set_lo(val),
        CP_FIFO_READ_POINTER_HI => ctx.cp.fifo_read_pointer = ctx.cp.fifo_read_pointer.set_hi(val),
        _ => warn!("write_u16 unrecognized cp register {:#x}", register),
    }
}

pub fn gather_pipe_burst(ctx: &mut Context) {
    if !ctx.cp.control.gp_link_enable() {
        panic!("cp::gather_pipe_burst disabled");
    }

    ctx.cp.fifo_write_pointer += BURST_SIZE as u32;

    if ctx.cp.fifo_write_pointer == ctx.cp.fifo_end {
        ctx.cp.fifo_write_pointer = ctx.cp.fifo_base;
    }

    if ctx.cp.fifo_write_pointer >= ctx.cp.fifo_read_pointer {
        ctx.cp.fifo_rw_distance = ctx.cp.fifo_write_pointer - ctx.cp.fifo_read_pointer;
    }

    while ctx.cp.control.gp_fifo_read_enable() && ctx.cp.fifo_rw_distance != 0 {
        let opcode = ctx.mem.read_u8(ctx.cp.fifo_read_pointer);

        ctx.cp.fifo_read_pointer += 1;

        match opcode {
            NO_OPERATION => (),
            LOAD_BP_REG => {
                if ctx.cp.fifo_size() < 4 {
                    ctx.cp.fifo_read_pointer -= 1;
                    break;
                }

                let value = mem::read_u32(ctx, ctx.cp.fifo_read_pointer);

                ctx.cp.bp.load(value, &mut ctx.mem);

                ctx.cp.fifo_read_pointer += 4;
            }
            LOAD_CP_REG => {
                if ctx.cp.fifo_size() < 5 {
                    ctx.cp.fifo_read_pointer -= 1;
                    break;
                }

                let cmd = ctx.mem.read_u8(ctx.cp.fifo_read_pointer);
                let value = mem::read_u32(ctx, ctx.cp.fifo_read_pointer + 1);

                ctx.cp.load(cmd, value);

                ctx.cp.fifo_read_pointer += 5;
            }
            LOAD_XF_REG => {
                let cmd = mem::read_u32(ctx, ctx.cp.fifo_read_pointer);
                let xf_size = ((cmd >> 16) & 15) + 1;
                let xf_address = cmd & 0xFFFF;

                if ctx.cp.fifo_size() < (xf_size * 4) + 4 {
                    ctx.cp.fifo_read_pointer -= 1;
                    break;
                }

                ctx.cp.fifo_read_pointer += 4; // cmd

                ctx.cp
                    .xf
                    .load(xf_size, xf_address, &mut ctx.mem, ctx.cp.fifo_read_pointer);

                ctx.cp.fifo_read_pointer += xf_size * 4;
            }
            CMD_INV_VC => warn!("FIXME: Invalidate Vertex Cache"),
            _ => {
                if opcode & 0x80 != 0 {
                    panic!("Vertex Opcode: {:#x}", opcode ^ 0x80);
                } else {
                    println!("gp_fifo unexpected opcode {:#x}", opcode);
                    panic!("Error, maybe dump here");
                }
            }
        }

        if ctx.cp.fifo_read_pointer == ctx.cp.fifo_end {
            ctx.cp.fifo_read_pointer = ctx.cp.fifo_base;
        }

        ctx.cp.fifo_rw_distance = ctx.cp.fifo_write_pointer - ctx.cp.fifo_read_pointer;
    }
}
