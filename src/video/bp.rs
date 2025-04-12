use crate::mem::Memory;

const IND_IMASK: u32 = 0x0F;
const IND_CMD0: u32 = 0x10;
const SCISSOR_0: u32 = 0x20;
const SCISSOR_1: u32 = 0x21;
const SU_LPSIZE: u32 = 0x22;
const SU_COUNTER: u32 = 0x23;
const RAS_COUNTER: u32 = 0x24;
const RAS1_SS0: u32 = 0x25;
const RAS1_SS1: u32 = 0x26;
const RAS1_TREF0: u32 = 0x28;
const RAS1_TREF1: u32 = 0x29;
const RAS1_TREF2: u32 = 0x2A;
const RAS1_TREF3: u32 = 0x2B;
const RAS1_TREF4: u32 = 0x2C;
const RAS1_TREF5: u32 = 0x2D;
const RAS1_TREF6: u32 = 0x2E;
const RAS1_TREF7: u32 = 0x2F;
const SU_SSIZE0: u32 = 0x30;
const SU_SSIZE1: u32 = 0x32;
const SU_SSIZE2: u32 = 0x34;
const SU_SSIZE3: u32 = 0x36;
const SU_SSIZE4: u32 = 0x38;
const SU_SSIZE5: u32 = 0x3A;
const SU_SSIZE6: u32 = 0x3C;
const SU_SSIZE7: u32 = 0x3E;
const PE_ZMODE: u32 = 0x40;
const PE_CMODE0: u32 = 0x41;
const PE_CMODE1: u32 = 0x42;
const PE_CONTROL: u32 = 0x43; // ZCOMPARE ???
const FIELD_MASK: u32 = 0x44;
const PE_DONE: u32 = 0x45;
const CLOCK_0: u32 = 0x46;
const PE_TOKEN: u32 = 0x47;
const PE_TOKEN_INT: u32 = 0x48;
const EFB_BOXCOORD: u32 = 0x49;
const EFB_BOXSIZE: u32 = 0x4A;
const XFB_ADDR: u32 = 0x4B;
const XFB_STRIDE: u32 = 0x4D;
const DISP_COPY: u32 = 0x4E;
const CLEAR_AR: u32 = 0x4F;
const CLEAR_GB: u32 = 0x50;
const CLEAR_Z: u32 = 0x51;
const COPY_CONTROL: u32 = 0x52;
const COPY_FILTER0: u32 = 0x53;
const COPY_FILTER1: u32 = 0x54;
const BOUNDING_BOX0: u32 = 0x55;
const BOUNDING_BOX1: u32 = 0x56;
const UNKNOWN: u32 = 0x58;
const SCISSOR_BOX: u32 = 0x59;
const UNKNOWN1: u32 = 0x66;
const FIELD_MODE: u32 = 0x68;
const CLOCK_1: u32 = 0x69;
const FOG_RANGE: u32 = 0xE8;
const TEV_FOG_PARAM_0: u32 = 0xEE;
const TEV_FOG_PARAM_1: u32 = 0xEF;
const TEV_FOG_PARAM_2: u32 = 0xF0;
const TEV_FOG_PARAM_3: u32 = 0xF1;
const TEV_FOG_COLOR: u32 = 0xF2;
const TEV_ALPHAFUNC: u32 = 0xF3;
const TEV_Z_ENV_0: u32 = 0xF4;
const TEV_Z_ENV_1: u32 = 0xF5;
const TEV_KSEL_0: u32 = 0xF6;

#[derive(Default, Debug)]
pub struct BlittingProcessor {
    imask: u32,
    clock_0: u32,
    clock_1: u32,
    copy_control: CopyControl,
    xfb_addr: u32, // Note: physical address shifted right by 5
    efb_coord: Coords,
    efb_boxsize: Coords,
    xfb_stride: u32,
    disp_copy_y_scale: u32,
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    pub struct CopyControl(u32);
    impl Debug;
    pub rid, _ : 24;
    pub copy_to_xfb, _ : 14;
    pub frame_2_field_mode, _ : 13, 12;
    pub clear, _ : 11;
    pub scale_invert, _ : 10;
    pub disp_copy_gamma, _ : 8, 7;
    pub xfb_format, _ : 4;
    pub clamp2, _ : 1;
    pub clamp1, _ : 0;
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    pub struct PeControl(u32);
    impl Debug;
    pub rid, _ : 24;
    pub z_comp_loc, _ : 6;
    pub z_format, _ : 5, 3;
    pub pixel_format, _ : 2, 0;
}

#[derive(Default, Debug)]
struct Coords(u32);

impl Coords {
    fn x(&self) -> u32 {
        (self.0 >> 10) & 0x3FF
    }

    fn y(&self) -> u32 {
        self.0 & 0x3FF
    }
}

impl BlittingProcessor {
    pub fn load(&mut self, value: u32, _: &mut Memory) {
        let reg = value >> 24;
        let new_value = value & 0xFF_FFFF;

        match reg {
            0x0 => (),         // GEN_MODE
            0x01..=0x04 => (), // display copy filter
            IND_IMASK => self.imask = new_value,
            IND_CMD0..=0x1F => (), // tex indeirect 0
            SCISSOR_0 => (),       // x0,y0
            SCISSOR_1 => (),       // x1,y1
            SU_LPSIZE => (),       // field mode .. line width - point
            SU_COUNTER => (),
            RAS_COUNTER => (),
            RAS1_SS0 => (),
            RAS1_SS1 => (),
            RAS1_TREF0 => (), // tev order 0
            RAS1_TREF1 => (), // tev order 1
            RAS1_TREF2 => (), // tev order 2
            RAS1_TREF3 => (), // tev order 3
            RAS1_TREF4 => (), // tev order 4
            RAS1_TREF5 => (), // tev order 5
            RAS1_TREF6 => (), // tev order 6
            RAS1_TREF7 => (), // tev order 7
            SU_SSIZE0 => (),  // texture offset 0
            SU_SSIZE1 => (),  // texture offset 1
            SU_SSIZE2 => (),  // texture offset 2
            SU_SSIZE3 => (),  // texture offset 3
            SU_SSIZE4 => (),  // texture offset 4
            SU_SSIZE5 => (),  // texture offset 5
            SU_SSIZE6 => (),  // texture offset 6
            SU_SSIZE7 => (),  // texture offset 7
            PE_ZMODE => (),
            PE_CMODE0 => (),
            PE_CMODE1 => (),
            PE_CONTROL => (),
            FIELD_MASK => (),
            PE_DONE => {
                panic!("PE_DONE");
            }
            CLOCK_0 => self.clock_0 = new_value,
            PE_TOKEN => {
                panic!("PE_TOKEN");

                // FIXME: test PE token
            }
            PE_TOKEN_INT => {
                panic!("PE_TOKEN_INT");
                // FIXME: test PE token
            }
            EFB_BOXCOORD => self.efb_coord = Coords(new_value),
            EFB_BOXSIZE => self.efb_boxsize = Coords(new_value),
            XFB_ADDR => self.xfb_addr = new_value,
            XFB_STRIDE => self.xfb_stride = new_value,
            DISP_COPY => self.disp_copy_y_scale = new_value,
            CLEAR_AR => (), // set clear alpha and red components
            CLEAR_GB => (), // green and blue
            CLEAR_Z => (),  // 24-bit Z
            COPY_CONTROL => {
                self.copy_control = CopyControl(new_value);

                let dest_addr = self.xfb_addr << 5;

                let src_rec_left = self.efb_coord.x() as i32;
                let src_rec_top = self.efb_coord.y() as i32;

                let src_rec_right = src_rec_left + self.efb_boxsize.x() as i32 + 1;
                let src_rec_bottom = src_rec_top + self.efb_boxsize.y() as i32 + 1;

                let mut y_scale = if self.copy_control.scale_invert() {
                    256 - (256.0 / self.disp_copy_y_scale as f32) as u32
                } else {
                    (self.disp_copy_y_scale as f32 / 256.0) as u32
                };

                y_scale &= 0x1ff;

                let num_xfb_lines = 1 + self.efb_boxsize.y() * y_scale;

                let height = num_xfb_lines;

                if self.copy_control.copy_to_xfb() {
                    // EFB to XFB
                    info!("RenderToXFB: destAddr: {:#x} | srcRect {{{} {} {} {}}} | fbWidth: {} | fbHeight: {} | fbStride: {} | yScale: {}", 
                        dest_addr, src_rec_left, src_rec_top, src_rec_right, src_rec_bottom,
                        self.efb_boxsize.x() + 1, height, self.xfb_stride << 5, y_scale);
                } else {
                    // EFB to texture
                    panic!("Don't copy to XFB");
                }

                if self.copy_control.clear() {
                    info!("ToDo: Clear Screen");
                }
            } // PE execute ??? trigger frpom efb to xfb
            COPY_FILTER0 => (),
            COPY_FILTER1 => (),
            BOUNDING_BOX0 => (),
            BOUNDING_BOX1 => (),
            UNKNOWN => (),
            SCISSOR_BOX => (), // scissor-box offset
            UNKNOWN1 => (),
            FIELD_MODE => (),
            CLOCK_1 => self.clock_1 = new_value,
            FOG_RANGE => (),
            0xC0..=0xDF => (), // TEV
            TEV_FOG_PARAM_0 => (),
            TEV_FOG_PARAM_1 => (),
            TEV_FOG_PARAM_2 => (),
            TEV_FOG_PARAM_3 => (),
            TEV_FOG_COLOR => (),
            TEV_ALPHAFUNC => (),
            TEV_Z_ENV_0 => (), // z texture 0
            TEV_Z_ENV_1 => (), // z texture 1
            TEV_KSEL_0..=0xFD => (),
            _ => warn!("Unhandled BP Reg: {:#x} Value: {:#x}", reg, new_value),
        }
    }
}
