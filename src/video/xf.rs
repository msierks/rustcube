use crate::mem;
use crate::video::cp::{MatrixIndexA, MatrixIndexB};

const NUM_COLOR_CHANNELS: usize = 2;

const MEM_SIZE: usize = 0x1000;

const XF_ERROR: u32 = 0x1000;
const XF_DIAGNOSTICS: u32 = 0x1001;
const XF_STATE0: u32 = 0x1002;
const XF_STATE1: u32 = 0x1003;
const XF_CLOCK: u32 = 0x1004;
const XF_CLIPDISABLE: u32 = 0x1005;
const XF_PERF0: u32 = 0x1006;
//const XF_PERF1: u32 = 0x1007;
//const XF_VTXSPEC: u32 = 0x1008;
const XF_NUMCOLORS: u32 = 0x1009;
const XF_AMBIENT0: u32 = 0x100A;
const XF_AMBIENT1: u32 = 0x100B;
const XF_MATERIAL0: u32 = 0x100C;
const XF_MATERIAL1: u32 = 0x100D;
const XF_COLOR0: u32 = 0x100E;
const XF_COLOR1: u32 = 0x100F;
const XF_ALPHA0: u32 = 0x1010;
const XF_ALPHA1: u32 = 0x1011;
//const XF_DUALTEXTRANS: u32 = 0x1012;
const XF_MATRIXINDA: u32 = 0x1018;
const XF_MATRIXINDB: u32 = 0x1019;
const XF_SCALEX: u32 = 0x101A;
const XF_SCALEY: u32 = 0x101B;
const XF_SCALEZ: u32 = 0x101C;
const XF_OFFSETX: u32 = 0x101D;
const XF_OFFSETY: u32 = 0x101E;
const XF_OFFSETZ: u32 = 0x101F;
//const XF_PROJECTIONA: u32 = 0x1020;
//const XF_PROJECTIONB: u32 = 0x1021;
//const XF_PROJECTIONC: u32 = 0x1022;
//const XF_PROJECTIOND: u32 = 0x1023;
//const XF_PROJECTIONE: u32 = 0x1024;
//const XF_PROJECTIONF: u32 = 0x1025;
//const XF_PROJECTORTHO: u32 = 0x1026;
const XF_NUMTEX: u32 = 0x103F;
const XF_TEXTURES0: u32 = 0x1040;
const XF_TEXTURES7: u32 = 0x1047;
const XF_DUALTEX0: u32 = 0x1050;
const XF_DUALTEX7: u32 = 0x1057;

#[derive(Debug)]
pub struct TransformUnit {
    data: Box<[u8; MEM_SIZE]>,
    _dual_tex_trans_enabled: bool,
    num_color: u32,
    _num_tex: u32,
    ambient_color: [u32; NUM_COLOR_CHANNELS],
    material_color: [u32; NUM_COLOR_CHANNELS],
    color: [ColorControl; NUM_COLOR_CHANNELS],
    alpha: [AlphaControl; NUM_COLOR_CHANNELS],
    viewport: Viewport,
    matrix_index_a: MatrixIndexA,
    matrix_index_b: MatrixIndexB,
}

impl Default for TransformUnit {
    fn default() -> Self {
        TransformUnit {
            data: Box::new([0; MEM_SIZE]),
            _dual_tex_trans_enabled: false,
            num_color: 0,
            _num_tex: 0,
            ambient_color: [0; NUM_COLOR_CHANNELS],
            material_color: [0; NUM_COLOR_CHANNELS],
            color: Default::default(),
            alpha: Default::default(),
            viewport: Default::default(),
            matrix_index_a: Default::default(),
            matrix_index_b: Default::default(),
        }
    }
}

impl TransformUnit {
    pub fn load(&mut self, mut size: u32, mut address: u32, ram: &mut mem::Memory, mut index: u32) {
        if size > 0 {
            if address < 0x1000 {
                for i in 0..size {
                    self.data[(address + i) as usize] = ram.read_u8(index + i);
                }
            } else {
                while size > 0 && address < XF_DUALTEX7 + 1 {
                    match address {
                        XF_ERROR | XF_DIAGNOSTICS | XF_STATE0 | XF_STATE1 | XF_CLOCK | XF_PERF0 => {
                        } // ignore
                        XF_CLIPDISABLE => {} // ignore for now
                        XF_NUMCOLORS => {
                            let data = ram.read_u32(index);

                            if self.num_color != data {
                                self.num_color = data;
                            }
                        }
                        XF_AMBIENT0 | XF_AMBIENT1 => {
                            let channel = address - XF_AMBIENT0;
                            let data = ram.read_u32(index);

                            self.ambient_color[channel as usize] = data;
                        }
                        XF_MATERIAL0 | XF_MATERIAL1 => {
                            let channel = address - XF_MATERIAL0;
                            let data = ram.read_u32(index);

                            self.material_color[channel as usize] = data;
                        }
                        XF_ALPHA0 | XF_ALPHA1 => {
                            let channel = address - XF_ALPHA0;

                            self.alpha[channel as usize] = AlphaControl(ram.read_u32(index));
                        }
                        XF_MATRIXINDA => {
                            self.matrix_index_a = MatrixIndexA(ram.read_u32(index));
                        }
                        XF_MATRIXINDB => {
                            self.matrix_index_b = MatrixIndexB(ram.read_u32(index));
                        }
                        XF_COLOR0 | XF_COLOR1 => {
                            let channel = address - XF_COLOR0;

                            self.color[channel as usize] = ColorControl(ram.read_u32(index));
                        }
                        XF_SCALEX => {
                            self.viewport.scalex = ram.read_f32(index);
                        }
                        XF_SCALEY => {
                            self.viewport.scaley = ram.read_f32(index);
                        }
                        XF_SCALEZ => {
                            self.viewport.scalez = ram.read_f32(index);
                        }
                        XF_OFFSETX => {
                            self.viewport.offsetx = ram.read_f32(index);
                        }
                        XF_OFFSETY => {
                            self.viewport.offsety = ram.read_f32(index);
                        }
                        XF_OFFSETZ => {
                            self.viewport.offsetz = ram.read_f32(index);
                        }
                        XF_NUMTEX => {}
                        XF_TEXTURES0..=XF_TEXTURES7 => {}
                        XF_DUALTEX0..=XF_DUALTEX7 => {}
                        _ => println!("XF unknown register write {:#x} {:#x}", address, size),
                    }

                    index += 4;
                    size -= 1;
                    address += 1;
                }
            }
        } else {
            panic!("XF zero size ???");
        }
    }
}

bitfield! {
    #[derive(Default)]
    struct ColorControl(u32);
    impl Debug;
    get_light_7, _: 14;
    get_light_6, _: 13;
    get_light_5, _: 12;
    get_light_4, _: 11;
    get_atten_select, _: 10;
    u8, get_diffuse_atten, _: 8, 7;
    get_ambient_src, _: 6;
    get_light_3, _: 5;
    get_light_2, _: 4;
    get_light_1, _: 3;
    get_light_0, _: 2;
    get_light_func, _: 1;
    get_material_src, _: 0;
}

bitfield! {
    #[derive(Default)]
    struct AlphaControl(u32);
    impl Debug;
    get_light_7, _: 14;
    get_light_6, _: 13;
    get_light_5, _: 12;
    get_light_4, _: 11;
    get_atten_select, _: 10;
    u8, get_diffuse_atten, _: 8, 7;
    get_ambient_src, _: 6;
    get_light_3, _: 5;
    get_light_2, _: 4;
    get_light_1, _: 3;
    get_light_0, _: 2;
    get_light_func, _: 1;
    get_material_src, _: 0;
}

#[derive(Default, Debug)]
struct Viewport {
    scalex: f32,
    scaley: f32,
    scalez: f32,
    offsetx: f32,
    offsety: f32,
    offsetz: f32,
}
