use crate::pi::{clear_interrupt, set_interrupt, PI_INTERRUPT_VI};
use crate::utils::Halveable;
use crate::Context;

#[cfg(not(test))]
use minifb::{Window, WindowOptions};

const VI_VERTICAL_TIMING: u32 = 0x00;
const VI_DISPLAY_CONFIG: u32 = 0x02;
const VI_HORIZONTAL_TIMING_0_HI: u32 = 0x04;
const VI_HORIZONTAL_TIMING_0_LO: u32 = 0x06;
const VI_HORIZONTAL_TIMING_1_HI: u32 = 0x08;
const VI_HORIZONTAL_TIMING_1_LO: u32 = 0x0A;
const VI_VERTICAL_TIMING_ODD_HI: u32 = 0x0C;
const VI_VERTICAL_TIMING_ODD_LO: u32 = 0x0E;
const VI_VERTICAL_TIMING_EVEN_HI: u32 = 0x10;
const VI_VERTICAL_TIMING_EVEN_LO: u32 = 0x12;
const VI_BURST_BLANKING_ODD_HI: u32 = 0x14;
const VI_BURST_BLANKING_ODD_LO: u32 = 0x16;
const VI_BURST_BLANKING_EVEN_HI: u32 = 0x18;
const VI_BURST_BLANKING_EVEN_LO: u32 = 0x1A;
const VI_FB_TOP_LEFT_HI: u32 = 0x1C;
const VI_FB_TOP_LEFT_LO: u32 = 0x1E;
//const VI_FB_TOP_RIGHT_HI: u32 = 0x20;
//const VI_FB_TOP_RIGHT_LO: u32 = 0x22;
//const VI_FB_BOTTOM_LEFT_HI: u32 = 0x24;
//const VI_FB_BOTTOM_LEFT_LO: u32 = 0x26;
//const VI_FB_BOTTOM_RIGHT_HI: u32 = 0x28;
//const VI_FB_BOTTOM_RIGHT_LO: u32 = 0x2A;
const VI_BEAM_POSITION_VERTICAL: u32 = 0x2C;
//const VI_BEAM_POSITION_HORIZONTAL: u32 = 0x2E;
const VI_DISPLAY_INTERRUPT_0_HI: u32 = 0x30;
const VI_DISPLAY_INTERRUPT_0_LO: u32 = 0x32;
const VI_DISPLAY_INTERRUPT_1_HI: u32 = 0x34;
const VI_DISPLAY_INTERRUPT_1_LO: u32 = 0x36;
const VI_DISPLAY_INTERRUPT_2_HI: u32 = 0x38;
const VI_DISPLAY_INTERRUPT_2_LO: u32 = 0x3A;
const VI_DISPLAY_INTERRUPT_3_HI: u32 = 0x3C;
const VI_DISPLAY_INTERRUPT_3_LO: u32 = 0x3E;
//const VI_DISPLAY_LATCH_0_LO: u32 = 0x40;
//const VI_DISPLAY_LATCH_0_HI: u32 = 0x42;
//const VI_DISPLAY_LATCH_1_LO: u32 = 0x44;
//const VI_DISPLAY_LATCH_1_HI: u32 = 0x46;
const VI_HORIZONTAL_SCALING_WIDTH: u32 = 0x48;
//const VI_HORIZONTAL_SCALING_REGISTER: u32 = 0x4A;
const VI_FILTER_COEFFICIENT_0_HI: u32 = 0x4C;
const VI_FILTER_COEFFICIENT_0_LO: u32 = 0x4E;
const VI_FILTER_COEFFICIENT_1_HI: u32 = 0x50;
const VI_FILTER_COEFFICIENT_1_LO: u32 = 0x52;
const VI_FILTER_COEFFICIENT_2_HI: u32 = 0x54;
const VI_FILTER_COEFFICIENT_2_LO: u32 = 0x56;
const VI_FILTER_COEFFICIENT_3_HI: u32 = 0x58;
const VI_FILTER_COEFFICIENT_3_LO: u32 = 0x5A;
const VI_FILTER_COEFFICIENT_4_HI: u32 = 0x5C;
const VI_FILTER_COEFFICIENT_4_LO: u32 = 0x5E;
const VI_FILTER_COEFFICIENT_5_HI: u32 = 0x60;
const VI_FILTER_COEFFICIENT_5_LO: u32 = 0x62;
const VI_FILTER_COEFFICIENT_6_HI: u32 = 0x64;
const VI_FILTER_COEFFICIENT_6_LO: u32 = 0x66;
//const VI_UNKOWN_AA_HI: u32 = 0x68;
//const VI_UNKOWN_AA_LO: u32 = 0x6A;
const VI_CLOCK_SELECT: u32 = 0x6C;
//const VI_DTV_STATUS: u32 = 0x6E;
const VI_UNKNOWN: u32 = 0x70;

// Video Clock
// 0 - 27 MHz
// 1 - 54 MHz (used in progressize scan)
//const CLOCK_FREQS: [u32; 2] = [27_000_000, 54_000_000]; // ratio 18 and 9

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

#[derive(Debug)]
pub struct VideoInterface {
    // Vertical Timing Register
    vtr: VerticalTimingRegister,
    // Display Config Register
    config: DisplayConfigRegister,
    // Horizontal Timing 0
    htr0: HorizontalTiming0Register,
    // Horizontal Timing 1
    htr1: HorizontalTiming1Register,
    // Odd Field Vertical Timing
    vto: VerticalBlankTimingRegister,
    // Even Field Vertical Timing
    vte: VerticalBlankTimingRegister,
    // Burst Blanking Odd Interval
    ofbbi: OddFieldBurstBlankingIntervalRegister,
    // Top Field Base Register Left
    tfbl: u32,
    // Burst Blanking Even Interval
    efbbi: EvenFieldBurstBlankingIntervalRegister,
    // Current Vertical Beam Position
    vbp: u16,
    // Current Horizontal Beam Position
    //hbp: u16,
    // Display Interrupts
    di: [DisplayInterrupt; 4],
    // Scaling Width
    hsw: HorizontalScalingWidthRegister,
    // Filter Coefficient
    fct: [u32; 7],
    // Clock Select
    clock: u16,
    // Unknown,
    unknown: u16,
    buffer: Vec<u32>,
    #[cfg(not(test))]
    window: Window,

    cpu_ticks: u64,
    half_line_count: u32,
}

impl Default for VideoInterface {
    fn default() -> Self {
        #[cfg(not(test))]
        let window = Window::new("Rustcube", WIDTH, HEIGHT, WindowOptions::default())
            .unwrap_or_else(|e| {
                panic!("{}", e);
            });

        VideoInterface {
            vtr: 0.into(),
            config: 0.into(),
            htr0: 0.into(),
            htr1: 0.into(),
            vto: 0.into(),
            vte: 0.into(),
            ofbbi: 0.into(),
            efbbi: 0.into(),
            tfbl: 0,
            vbp: 1,
            //hbp: 1,
            di: Default::default(),
            hsw: Default::default(),
            fct: [0; 7],
            clock: 0,
            unknown: 0,
            buffer: vec![0; WIDTH * HEIGHT],
            #[cfg(not(test))]
            window,
            cpu_ticks: 0,
            half_line_count: 0,
        }
    }
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    pub struct VerticalTimingRegister(u16);
    impl Debug;
    pub equ, _ : 3, 0;  // Equalization pulse in half lines.
    pub acv, _ : 13, 4; // Active video in full lines.
}

impl From<u16> for VerticalTimingRegister {
    fn from(v: u16) -> Self {
        VerticalTimingRegister(v)
    }
}

impl From<VerticalTimingRegister> for u16 {
    fn from(s: VerticalTimingRegister) -> u16 {
        s.0
    }
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    pub struct HorizontalTiming0Register(u32);
    impl Debug;
    pub hlw, _ : 9, 0;   // Halfline width (W*16 = Width (720))
    pub hce, _ : 23, 16; // Horizontal Sync Start to Color Burst End
    pub hcs, _ : 30, 24; // Horizontal Sync Start to Color Burst Start
}

impl HorizontalTiming0Register {
    fn set_hi(self, v: u16) -> Self {
        HorizontalTiming0Register((self.0 & 0xFFFF) | ((v as u32) << 16))
    }

    fn set_lo(self, v: u16) -> Self {
        HorizontalTiming0Register((self.0 & 0xFFFF_0000) | (v as u32))
    }
}

impl From<u32> for HorizontalTiming0Register {
    fn from(v: u32) -> Self {
        HorizontalTiming0Register(v)
    }
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    pub struct HorizontalTiming1Register(u32);
    impl Debug;
    pub hsy, _ : 6, 0;   // Horizontal Sync Width
    pub hbe, _ : 16, 7;  // Horizontal Sync Start to horizontal blank end
    pub hbs, _ : 26, 17; // Halfline to horizontal blanking start
}

impl HorizontalTiming1Register {
    fn set_hi(self, v: u16) -> Self {
        HorizontalTiming1Register((self.0 & 0xFFFF) | ((v as u32) << 16))
    }

    fn set_lo(self, v: u16) -> Self {
        HorizontalTiming1Register((self.0 & 0xFFFF_0000) | (v as u32))
    }
}

impl From<u32> for HorizontalTiming1Register {
    fn from(v: u32) -> Self {
        HorizontalTiming1Register(v)
    }
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    pub struct DisplayConfigRegister(u16);
    impl Debug;
    pub enable, _ : 0;
    pub reset, set_reset : 1;
    pub not_interlaced, _ : 2; // 0 Interlaced; 1 Non-Interlaced
    pub mode_3d, _ : 3;
    pub latch_0, _ : 5, 4;
    pub latch_1, _ : 7, 6;
    pub format, _ : 9, 8; // 0 NTSC; 1 PAL; 2 MPAL; 3 Debug
}

impl From<u16> for DisplayConfigRegister {
    fn from(v: u16) -> Self {
        DisplayConfigRegister(v)
    }
}

impl From<DisplayConfigRegister> for u16 {
    fn from(s: DisplayConfigRegister) -> u16 {
        s.0
    }
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    pub struct VerticalBlankTimingRegister(u32);
    impl Debug;
    pub prb, _ : 9, 0;   // Post-blanking in half lines.
    pub psb, _ : 25, 16; // Pre-blanking in half lines.
}

impl VerticalBlankTimingRegister {
    fn set_hi(self, v: u16) -> Self {
        VerticalBlankTimingRegister((self.0 & 0xFFFF) | ((v as u32) << 16))
    }

    fn set_lo(self, v: u16) -> Self {
        VerticalBlankTimingRegister((self.0 & 0xFFFF_0000) | (v as u32))
    }
}

impl From<u32> for VerticalBlankTimingRegister {
    fn from(v: u32) -> Self {
        VerticalBlankTimingRegister(v)
    }
}

impl From<VerticalBlankTimingRegister> for u32 {
    fn from(s: VerticalBlankTimingRegister) -> u32 {
        s.0
    }
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    pub struct OddFieldBurstBlankingIntervalRegister(u32);
    impl Debug;
    pub bs1, _ : 4, 0;   // Field 1 start to burst blanking start in half lines.
    pub be1, _ : 15, 5;  // Field 1 start to burst blanking end in half lines.
    pub bs3, _ : 20, 16; // Field 3 start to burst blanking start in half lines.
    pub be3, _ : 31, 21; // Field 3 start to burst blanking end in half lines.
}

impl OddFieldBurstBlankingIntervalRegister {
    fn set_hi(self, v: u16) -> Self {
        OddFieldBurstBlankingIntervalRegister((self.0 & 0xFFFF) | ((v as u32) << 16))
    }

    fn set_lo(self, v: u16) -> Self {
        OddFieldBurstBlankingIntervalRegister((self.0 & 0xFFFF_0000) | (v as u32))
    }
}

impl From<u32> for OddFieldBurstBlankingIntervalRegister {
    fn from(v: u32) -> Self {
        OddFieldBurstBlankingIntervalRegister(v)
    }
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    pub struct EvenFieldBurstBlankingIntervalRegister(u32);
    impl Debug;
    pub bs2, _ : 4, 0;   // Field 2 start to burst blanking start in half lines.
    pub be2, _ : 15, 5;  // Field 2 start to burst blanking end in half lines.
    pub bs4, _ : 20, 16; // Field 4 start to burst blanking start in half lines.
    pub be4, _ : 31, 21; // Field 4 start to burst blanking end in half lines.
}

impl EvenFieldBurstBlankingIntervalRegister {
    fn set_hi(self, v: u16) -> Self {
        EvenFieldBurstBlankingIntervalRegister((self.0 & 0xFFFF) | ((v as u32) << 16))
    }

    fn set_lo(self, v: u16) -> Self {
        EvenFieldBurstBlankingIntervalRegister((self.0 & 0xFFFF_0000) | (v as u32))
    }
}

impl From<u32> for EvenFieldBurstBlankingIntervalRegister {
    fn from(v: u32) -> Self {
        EvenFieldBurstBlankingIntervalRegister(v)
    }
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    pub struct DisplayInterrupt(u32);
    impl Debug;
    pub hct, _ : 9, 0;                 // HCT - Horizontal position
    pub vct, _ : 25, 16;               // VCT - Vertical position
    pub interrupt_enable, _ : 28;      // ENB - Interrupt enable bit
    pub interrupt, set_interrupt : 31; // INT - Interrupt Status (1 = Active) (Write to clear)
}

impl From<u32> for DisplayInterrupt {
    fn from(v: u32) -> Self {
        DisplayInterrupt(v)
    }
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    pub struct HorizontalScalingWidthRegister(u16);
    impl Debug;
    pub std, _ : 7, 0;
    pub wpl, _ : 14, 8;
}

impl From<u16> for HorizontalScalingWidthRegister {
    fn from(v: u16) -> Self {
        Self(v)
    }
}

impl VideoInterface {
    //pub fn get_clock_rate(&self) -> u32 {
    //    CLOCK_FREQS[self.clock as usize]
    //}

    pub fn even_field_half_lines_total(&self) -> u32 {
        // Pre-Equalization(equ) + Serration(equ) + Post-Equalization(equ) + Pre-Blanking(prb) +
        // Active Video(acv full lines) + Post-Blanking(psb)
        (self.vtr.equ() as u32 * 3) + self.vte.prb() + (self.vtr.acv() as u32 * 2) + self.vte.prb()
    }

    pub fn odd_field_half_lines_total(&self) -> u32 {
        // Pre-Equalization(equ) + Serration(equ) + Post-Equalization(equ) + Pre-Blanking(prb) +
        // Active Video(acv full lines) + Post-Blanking(psb)
        (self.vtr.equ() as u32 * 3) + self.vto.prb() + (self.vtr.acv() as u32 * 2) + self.vto.prb()
    }
}

pub fn read_u16(ctx: &mut Context, register: u32) -> u16 {
    match register {
        VI_DISPLAY_CONFIG => ctx.vi.config.into(),
        VI_BEAM_POSITION_VERTICAL => {
            //ctx.vi.vbp
            1 + (ctx.vi.half_line_count / 2) as u16
        }
        VI_DISPLAY_INTERRUPT_0_HI => ctx.vi.di[0].0.hi(),
        VI_DISPLAY_INTERRUPT_0_LO => ctx.vi.di[0].0.lo(),
        VI_DISPLAY_INTERRUPT_1_HI => ctx.vi.di[1].0.hi(),
        VI_DISPLAY_INTERRUPT_1_LO => ctx.vi.di[1].0.lo(),
        VI_DISPLAY_INTERRUPT_2_HI => ctx.vi.di[2].0.hi(),
        VI_DISPLAY_INTERRUPT_2_LO => ctx.vi.di[2].0.lo(),
        VI_DISPLAY_INTERRUPT_3_HI => ctx.vi.di[3].0.hi(),
        VI_DISPLAY_INTERRUPT_3_LO => ctx.vi.di[3].0.lo(),
        _ => {
            warn!("read_u16 unrecognized register {:#x}", register);
            0
        }
    }
}

pub fn write_u16(ctx: &mut Context, register: u32, val: u16) {
    match register {
        VI_VERTICAL_TIMING => ctx.vi.vtr = val.into(),
        VI_DISPLAY_CONFIG => {
            ctx.vi.config = val.into();
            if ctx.vi.config.reset() {
                ctx.vi.config.set_reset(false);
                // TODO: clear interrupts
            }
        }
        VI_HORIZONTAL_TIMING_0_HI => ctx.vi.htr0 = ctx.vi.htr0.set_hi(val),
        VI_HORIZONTAL_TIMING_0_LO => ctx.vi.htr0 = ctx.vi.htr0.set_lo(val),
        VI_HORIZONTAL_TIMING_1_HI => ctx.vi.htr1 = ctx.vi.htr1.set_hi(val),
        VI_HORIZONTAL_TIMING_1_LO => ctx.vi.htr1 = ctx.vi.htr1.set_lo(val),
        VI_VERTICAL_TIMING_ODD_HI => ctx.vi.vto = ctx.vi.vto.set_hi(val),
        VI_VERTICAL_TIMING_ODD_LO => ctx.vi.vto = ctx.vi.vto.set_lo(val),
        VI_VERTICAL_TIMING_EVEN_HI => ctx.vi.vte = ctx.vi.vte.set_hi(val),
        VI_VERTICAL_TIMING_EVEN_LO => ctx.vi.vte = ctx.vi.vte.set_lo(val),
        VI_BURST_BLANKING_ODD_HI => ctx.vi.ofbbi = ctx.vi.ofbbi.set_hi(val),
        VI_BURST_BLANKING_ODD_LO => ctx.vi.ofbbi = ctx.vi.ofbbi.set_lo(val),
        VI_BURST_BLANKING_EVEN_HI => ctx.vi.efbbi = ctx.vi.efbbi.set_hi(val),
        VI_BURST_BLANKING_EVEN_LO => ctx.vi.efbbi = ctx.vi.efbbi.set_lo(val),
        VI_FB_TOP_LEFT_HI => ctx.vi.tfbl = ctx.vi.tfbl.set_hi(val),
        VI_FB_TOP_LEFT_LO => ctx.vi.tfbl = ctx.vi.tfbl.set_lo(val),
        VI_DISPLAY_INTERRUPT_0_HI => {
            ctx.vi.di[0] = ctx.vi.di[0].0.set_hi(val).into();
            update_interrupt(ctx);
        }
        VI_DISPLAY_INTERRUPT_0_LO => ctx.vi.di[0] = ctx.vi.di[0].0.set_lo(val).into(),
        VI_DISPLAY_INTERRUPT_1_HI => {
            ctx.vi.di[1] = ctx.vi.di[1].0.set_hi(val).into();
            update_interrupt(ctx);
        }
        VI_DISPLAY_INTERRUPT_1_LO => ctx.vi.di[1] = ctx.vi.di[1].0.set_lo(val).into(),
        VI_DISPLAY_INTERRUPT_2_HI => {
            ctx.vi.di[2] = ctx.vi.di[2].0.set_hi(val).into();
            update_interrupt(ctx);
        }
        VI_DISPLAY_INTERRUPT_2_LO => ctx.vi.di[2] = ctx.vi.di[2].0.set_lo(val).into(),
        VI_DISPLAY_INTERRUPT_3_HI => {
            ctx.vi.di[3] = ctx.vi.di[3].0.set_hi(val).into();
            update_interrupt(ctx);
        }
        VI_DISPLAY_INTERRUPT_3_LO => ctx.vi.di[3] = ctx.vi.di[3].0.set_lo(val).into(),
        VI_FILTER_COEFFICIENT_0_HI => ctx.vi.fct[0] = ctx.vi.fct[0].set_hi(val),
        VI_FILTER_COEFFICIENT_0_LO => ctx.vi.fct[0] = ctx.vi.fct[0].set_lo(val),
        VI_FILTER_COEFFICIENT_1_HI => ctx.vi.fct[1] = ctx.vi.fct[1].set_hi(val),
        VI_FILTER_COEFFICIENT_1_LO => ctx.vi.fct[1] = ctx.vi.fct[1].set_lo(val),
        VI_FILTER_COEFFICIENT_2_HI => ctx.vi.fct[2] = ctx.vi.fct[2].set_hi(val),
        VI_FILTER_COEFFICIENT_2_LO => ctx.vi.fct[2] = ctx.vi.fct[2].set_lo(val),
        VI_FILTER_COEFFICIENT_3_HI => ctx.vi.fct[3] = ctx.vi.fct[3].set_hi(val),
        VI_FILTER_COEFFICIENT_3_LO => ctx.vi.fct[3] = ctx.vi.fct[3].set_lo(val),
        VI_FILTER_COEFFICIENT_4_HI => ctx.vi.fct[4] = ctx.vi.fct[4].set_hi(val),
        VI_FILTER_COEFFICIENT_4_LO => ctx.vi.fct[4] = ctx.vi.fct[4].set_lo(val),
        VI_FILTER_COEFFICIENT_5_HI => ctx.vi.fct[5] = ctx.vi.fct[5].set_hi(val),
        VI_FILTER_COEFFICIENT_5_LO => ctx.vi.fct[5] = ctx.vi.fct[5].set_lo(val),
        VI_FILTER_COEFFICIENT_6_HI => ctx.vi.fct[6] = ctx.vi.fct[6].set_hi(val),
        VI_FILTER_COEFFICIENT_6_LO => ctx.vi.fct[6] = ctx.vi.fct[6].set_lo(val),
        VI_HORIZONTAL_SCALING_WIDTH => ctx.vi.hsw = val.into(),
        VI_CLOCK_SELECT => ctx.vi.clock = val,
        VI_UNKNOWN => ctx.vi.unknown = val,
        _ => warn!("write_u16 unrecognized vi register {:#x}", register),
    }
}

pub fn update(ctx: &mut Context) {
    //TODO: this is arbitrary, figure out how often this should execute
    let ticks = ctx.get_ticks();
    if ticks - ctx.vi.cpu_ticks > 600 {
        // number of ticks per half line drawn
        ctx.vi.cpu_ticks = ticks;
    } else {
        return;
    }

    if ctx.vi.config.enable() {
        ctx.vi.vbp += 1;

        // NTSC
        if ctx.vi.config.format() == 0 && ctx.vi.vbp > 525 {
            ctx.vi.vbp = 1;

            let mut i = ctx.vi.tfbl;
            let mut j = 0;

            while i < ctx.vi.tfbl + 320 * 480 * 4 {
                let y1 = i32::from(ctx.mem.read_u8(i));
                let v = i32::from(ctx.mem.read_u8(i + 1));
                let y2 = i32::from(ctx.mem.read_u8(i + 2));
                let u = i32::from(ctx.mem.read_u8(i + 3));

                ctx.vi.buffer[j] = yuv_to_rgb(y1, u, v);
                ctx.vi.buffer[j + 1] = yuv_to_rgb(y2, u, v);

                i += 4;
                j += 2;
            }

            #[cfg(not(test))]
            ctx.vi.window.update_with_buffer(&ctx.vi.buffer).unwrap();
        }

        ctx.vi.half_line_count += 1;
        if ctx.vi.half_line_count
            == ctx.vi.even_field_half_lines_total() + ctx.vi.odd_field_half_lines_total()
        {
            ctx.vi.half_line_count = 0;
        }

        let current_line = ctx.vi.half_line_count / 2 + 1;

        for di in ctx.vi.di.iter_mut() {
            if current_line == di.vct()
                && ((ctx.vi.half_line_count & 1 != 0) == (di.hct() > ctx.vi.htr0.hlw()))
            {
                di.set_interrupt(true);
            }
        }

        update_interrupt(ctx);
    }
}

pub fn update_interrupt(ctx: &mut Context) {
    if ctx.vi.di[0].interrupt() && ctx.vi.di[0].interrupt_enable()
        || ctx.vi.di[1].interrupt() && ctx.vi.di[1].interrupt_enable()
        || ctx.vi.di[2].interrupt() && ctx.vi.di[2].interrupt_enable()
        || ctx.vi.di[3].interrupt() && ctx.vi.di[3].interrupt_enable()
    {
        set_interrupt(ctx, PI_INTERRUPT_VI);
    } else {
        clear_interrupt(ctx, PI_INTERRUPT_VI);
    }
}

fn yuv_to_rgb(y: i32, u: i32, v: i32) -> u32 {
    let r = ((76283 * (y - 16) + 104_595 * (v - 128)).clamp(0, 255) as u32) >> 16;
    let g = (((76283 * (y - 16) - 53281 * (v - 128) - 25624 * (u - 128)) >> 16).clamp(0, 255)
        as u32)
        << 8;
    let b = (((76283 * (y - 16) + 132_252 * (u - 128)) >> 16).clamp(0, 255) as u32) << 16;

    b | g | r
}
