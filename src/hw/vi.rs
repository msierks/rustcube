use crate::{
    bus::Bus,
    cpu::CpuState,
    hw::{
        mmio::{Mmio, MmioDevice},
        pi::{ProcessorInterface, PI_INTERRUPT_VI},
    },
    utils::Halveable,
};

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
const VI_FB_BOTTOM_LEFT_HI: u32 = 0x24;
const VI_FB_BOTTOM_LEFT_LO: u32 = 0x26;
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
const _VI_HORIZONTAL_SCALING_REGISTER: u32 = 0x4A;
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
    /// Vertical Timing Register
    vtr: VerticalTimingRegister,
    /// Display Config Register
    config: DisplayConfigRegister,
    // Horizontal Timing 0
    htr0: HorizontalTiming0Register,
    /// Horizontal Timing 1
    htr1: HorizontalTiming1Register,
    /// Odd Field Vertical Timing
    vto: VerticalBlankTimingRegister,
    /// Even Field Vertical Timing
    vte: VerticalBlankTimingRegister,
    /// Burst Blanking Odd Interval
    ofbbi: OddFieldBurstBlankingIntervalRegister,
    /// Top Field Base Register Left
    tfbl: u32,
    /// Bottom Field Base Register
    bfbl: u32,
    /// Burst Blanking Even Interval
    efbbi: EvenFieldBurstBlankingIntervalRegister,
    /// Current Vertical Beam Position
    vbp: u16,
    // Current Horizontal Beam Position
    //hbp: u16,
    /// Display Interrupts
    di: [DisplayInterrupt; 4],
    /// Scaling Width
    hsw: HorizontalScalingWidthRegister,
    /// Filter Coefficient
    fct: [u32; 7],
    /// Clock Select
    clock: u16,
    /// Unknown,
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
            bfbl: 0,
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

impl MmioDevice for VideoInterface {
    const BASE_ADDR: u32 = 0x0C00_2000;

    fn register_mmio(mmio: &mut Mmio) {
        mmio.register_write_u16(Self::BASE_ADDR + VI_VERTICAL_TIMING, |bus, _, _, val| {
            bus.vi.vtr = val.into();
        });
        mmio.register_u16(
            Self::BASE_ADDR + VI_DISPLAY_CONFIG,
            |bus, _, _| bus.vi.config.into(),
            |bus, cpu_state, _, val| {
                bus.vi.config = val.into();
                if bus.vi.config.reset() {
                    bus.vi.config.set_reset(false);
                    bus.vi.di[0] = 0.into();
                    bus.vi.di[1] = 0.into();
                    bus.vi.di[2] = 0.into();
                    bus.vi.di[3] = 0.into();
                    Self::update_interrupts(bus, cpu_state);
                }
            },
        );
        mmio.register_write_u32(
            Self::BASE_ADDR + VI_VERTICAL_TIMING,
            |bus, cpu_state, _, val| {
                bus.vi.vtr = val.hi().into();
                bus.vi.config = val.lo().into();
                if bus.vi.config.reset() {
                    bus.vi.config.set_reset(false);
                    bus.vi.di[0] = 0.into();
                    bus.vi.di[1] = 0.into();
                    bus.vi.di[2] = 0.into();
                    bus.vi.di[3] = 0.into();
                    Self::update_interrupts(bus, cpu_state);
                }
            },
        );
        mmio.register_write_u16(
            Self::BASE_ADDR + VI_HORIZONTAL_TIMING_0_HI,
            |bus, _, _, val| {
                bus.vi.htr0 = bus.vi.htr0.set_hi(val);
            },
        );
        mmio.register_write_u32(
            Self::BASE_ADDR + VI_HORIZONTAL_TIMING_0_HI,
            |bus, _, _, val| {
                bus.vi.htr0 = val.into();
            },
        );
        mmio.register_write_u16(
            Self::BASE_ADDR + VI_HORIZONTAL_TIMING_0_LO,
            |bus, _, _, val| {
                bus.vi.htr0 = bus.vi.htr0.set_lo(val);
            },
        );
        mmio.register_write_u16(
            Self::BASE_ADDR + VI_HORIZONTAL_TIMING_1_HI,
            |bus, _, _, val| {
                bus.vi.htr1 = bus.vi.htr1.set_hi(val);
            },
        );
        mmio.register_write_u32(
            Self::BASE_ADDR + VI_HORIZONTAL_TIMING_1_HI,
            |bus, _, _, val| {
                bus.vi.htr1 = val.into();
            },
        );
        mmio.register_write_u16(
            Self::BASE_ADDR + VI_HORIZONTAL_TIMING_1_LO,
            |bus, _, _, val| {
                bus.vi.htr1 = bus.vi.htr1.set_lo(val);
            },
        );
        mmio.register_write_u16(
            Self::BASE_ADDR + VI_VERTICAL_TIMING_ODD_HI,
            |bus, _, _, val| {
                bus.vi.vto = bus.vi.vto.set_hi(val);
            },
        );
        mmio.register_write_u32(
            Self::BASE_ADDR + VI_VERTICAL_TIMING_ODD_HI,
            |bus, _, _, val| {
                bus.vi.vto = val.into();
            },
        );
        mmio.register_write_u16(
            Self::BASE_ADDR + VI_VERTICAL_TIMING_ODD_LO,
            |bus, _, _, val| {
                bus.vi.vto = bus.vi.vto.set_lo(val);
            },
        );
        mmio.register_write_u16(
            Self::BASE_ADDR + VI_VERTICAL_TIMING_EVEN_HI,
            |bus, _, _, val| {
                bus.vi.vte = bus.vi.vte.set_hi(val);
            },
        );
        mmio.register_write_u32(
            Self::BASE_ADDR + VI_VERTICAL_TIMING_EVEN_HI,
            |bus, _, _, val| {
                bus.vi.vte = val.into();
            },
        );
        mmio.register_write_u16(
            Self::BASE_ADDR + VI_VERTICAL_TIMING_EVEN_LO,
            |bus, _, _, val| {
                bus.vi.vte = bus.vi.vte.set_lo(val);
            },
        );
        mmio.register_write_u16(
            Self::BASE_ADDR + VI_BURST_BLANKING_ODD_HI,
            |bus, _, _, val| {
                bus.vi.ofbbi = bus.vi.ofbbi.set_hi(val);
            },
        );
        mmio.register_write_u32(
            Self::BASE_ADDR + VI_BURST_BLANKING_ODD_HI,
            |bus, _, _, val| {
                bus.vi.ofbbi = val.into();
            },
        );
        mmio.register_write_u16(
            Self::BASE_ADDR + VI_BURST_BLANKING_ODD_LO,
            |bus, _, _, val| {
                bus.vi.ofbbi = bus.vi.ofbbi.set_lo(val);
            },
        );
        mmio.register_write_u16(
            Self::BASE_ADDR + VI_BURST_BLANKING_EVEN_HI,
            |bus, _, _, val| {
                bus.vi.efbbi = bus.vi.efbbi.set_hi(val);
            },
        );
        mmio.register_write_u32(
            Self::BASE_ADDR + VI_BURST_BLANKING_EVEN_HI,
            |bus, _, _, val| {
                bus.vi.efbbi = val.into();
            },
        );
        mmio.register_write_u16(
            Self::BASE_ADDR + VI_BURST_BLANKING_EVEN_LO,
            |bus, _, _, val| {
                bus.vi.efbbi = bus.vi.efbbi.set_lo(val);
            },
        );
        mmio.register_write_u16(Self::BASE_ADDR + VI_FB_TOP_LEFT_HI, |bus, _, _, val| {
            bus.vi.tfbl = bus.vi.tfbl.set_hi(val);
        });
        mmio.register_write_u32(Self::BASE_ADDR + VI_FB_TOP_LEFT_HI, |bus, _, _, val| {
            bus.vi.tfbl = val;
        });
        mmio.register_write_u16(Self::BASE_ADDR + VI_FB_TOP_LEFT_LO, |bus, _, _, val| {
            bus.vi.tfbl = bus.vi.tfbl.set_lo(val);
        });
        mmio.register_write_u16(Self::BASE_ADDR + VI_FB_BOTTOM_LEFT_HI, |bus, _, _, val| {
            bus.vi.bfbl = bus.vi.bfbl.set_hi(val);
        });
        mmio.register_write_u32(Self::BASE_ADDR + VI_FB_BOTTOM_LEFT_HI, |bus, _, _, val| {
            bus.vi.bfbl = val;
        });
        mmio.register_write_u16(Self::BASE_ADDR + VI_FB_BOTTOM_LEFT_LO, |bus, _, _, val| {
            bus.vi.bfbl = bus.vi.bfbl.set_lo(val);
        });

        mmio.register_read_u16(Self::BASE_ADDR + VI_BEAM_POSITION_VERTICAL, |bus, _, _| {
            1 + (bus.vi.half_line_count / 2) as u16
        });

        mmio.register_u16(
            Self::BASE_ADDR + VI_DISPLAY_INTERRUPT_0_HI,
            |bus, _, _| bus.vi.di[0].0.hi(),
            |bus, cpu_state, _, val| {
                bus.vi.di[0] = bus.vi.di[0].0.set_hi(val).into();
                Self::update_interrupts(bus, cpu_state);
            },
        );
        mmio.register_u16(
            Self::BASE_ADDR + VI_DISPLAY_INTERRUPT_0_LO,
            |bus, _, _| bus.vi.di[0].0.lo(),
            |bus, _, _, val| {
                bus.vi.di[0] = bus.vi.di[0].0.set_lo(val).into();
            },
        );
        mmio.register_u32(
            Self::BASE_ADDR + VI_DISPLAY_INTERRUPT_0_HI,
            |bus, _, _| bus.vi.di[0].0,
            |bus, cpu_state, _, val| {
                bus.vi.di[0] = val.into();
                Self::update_interrupts(bus, cpu_state);
            },
        );
        mmio.register_u16(
            Self::BASE_ADDR + VI_DISPLAY_INTERRUPT_1_HI,
            |bus, _, _| bus.vi.di[1].0.hi(),
            |bus, cpu_state, _, val| {
                bus.vi.di[1] = bus.vi.di[1].0.set_hi(val).into();
                Self::update_interrupts(bus, cpu_state);
            },
        );
        mmio.register_u16(
            Self::BASE_ADDR + VI_DISPLAY_INTERRUPT_1_LO,
            |bus, _, _| bus.vi.di[1].0.lo(),
            |bus, _, _, val| {
                bus.vi.di[1] = bus.vi.di[1].0.set_lo(val).into();
            },
        );
        mmio.register_u32(
            Self::BASE_ADDR + VI_DISPLAY_INTERRUPT_1_HI,
            |bus, _, _| bus.vi.di[1].0,
            |bus, cpu_state, _, val| {
                bus.vi.di[1] = val.into();
                Self::update_interrupts(bus, cpu_state);
            },
        );
        mmio.register_u16(
            Self::BASE_ADDR + VI_DISPLAY_INTERRUPT_2_HI,
            |bus, _, _| bus.vi.di[2].0.hi(),
            |bus, cpu_state, _, val| {
                bus.vi.di[2] = bus.vi.di[2].0.set_hi(val).into();
                Self::update_interrupts(bus, cpu_state);
            },
        );
        mmio.register_u16(
            Self::BASE_ADDR + VI_DISPLAY_INTERRUPT_2_LO,
            |bus, _, _| bus.vi.di[2].0.lo(),
            |bus, _, _, val| {
                bus.vi.di[2] = bus.vi.di[2].0.set_lo(val).into();
            },
        );
        mmio.register_u32(
            Self::BASE_ADDR + VI_DISPLAY_INTERRUPT_2_HI,
            |bus, _, _| bus.vi.di[2].0,
            |bus, cpu_state, _, val| {
                bus.vi.di[2] = val.into();
                Self::update_interrupts(bus, cpu_state);
            },
        );
        mmio.register_u16(
            Self::BASE_ADDR + VI_DISPLAY_INTERRUPT_3_HI,
            |bus, _, _| bus.vi.di[3].0.hi(),
            |bus, cpu_state, _, val| {
                bus.vi.di[3] = bus.vi.di[3].0.set_hi(val).into();
                Self::update_interrupts(bus, cpu_state);
            },
        );
        mmio.register_u16(
            Self::BASE_ADDR + VI_DISPLAY_INTERRUPT_3_LO,
            |bus, _, _| bus.vi.di[3].0.lo(),
            |bus, _, _, val| {
                bus.vi.di[3] = bus.vi.di[3].0.set_lo(val).into();
            },
        );
        mmio.register_u32(
            Self::BASE_ADDR + VI_DISPLAY_INTERRUPT_3_HI,
            |bus, _, _| bus.vi.di[3].0,
            |bus, cpu_state, _, val| {
                bus.vi.di[3] = val.into();
                Self::update_interrupts(bus, cpu_state);
            },
        );
        mmio.register_write_u16(
            Self::BASE_ADDR + VI_FILTER_COEFFICIENT_0_HI,
            |bus, _, _, val| {
                bus.vi.fct[0] = bus.vi.fct[0].set_hi(val);
            },
        );
        mmio.register_write_u16(
            Self::BASE_ADDR + VI_FILTER_COEFFICIENT_0_LO,
            |bus, _, _, val| {
                bus.vi.fct[0] = bus.vi.fct[0].set_lo(val);
            },
        );
        mmio.register_write_u16(
            Self::BASE_ADDR + VI_FILTER_COEFFICIENT_1_HI,
            |bus, _, _, val| {
                bus.vi.fct[1] = bus.vi.fct[1].set_hi(val);
            },
        );
        mmio.register_write_u16(
            Self::BASE_ADDR + VI_FILTER_COEFFICIENT_1_LO,
            |bus, _, _, val| {
                bus.vi.fct[1] = bus.vi.fct[1].set_lo(val);
            },
        );
        mmio.register_write_u16(
            Self::BASE_ADDR + VI_FILTER_COEFFICIENT_2_HI,
            |bus, _, _, val| {
                bus.vi.fct[2] = bus.vi.fct[2].set_hi(val);
            },
        );
        mmio.register_write_u16(
            Self::BASE_ADDR + VI_FILTER_COEFFICIENT_2_LO,
            |bus, _, _, val| {
                bus.vi.fct[2] = bus.vi.fct[2].set_lo(val);
            },
        );
        mmio.register_write_u16(
            Self::BASE_ADDR + VI_FILTER_COEFFICIENT_3_HI,
            |bus, _, _, val| {
                bus.vi.fct[3] = bus.vi.fct[3].set_hi(val);
            },
        );
        mmio.register_write_u16(
            Self::BASE_ADDR + VI_FILTER_COEFFICIENT_3_LO,
            |bus, _, _, val| {
                bus.vi.fct[3] = bus.vi.fct[3].set_lo(val);
            },
        );
        mmio.register_write_u16(
            Self::BASE_ADDR + VI_FILTER_COEFFICIENT_4_HI,
            |bus, _, _, val| {
                bus.vi.fct[4] = bus.vi.fct[4].set_hi(val);
            },
        );
        mmio.register_write_u16(
            Self::BASE_ADDR + VI_FILTER_COEFFICIENT_4_LO,
            |bus, _, _, val| {
                bus.vi.fct[4] = bus.vi.fct[4].set_lo(val);
            },
        );
        mmio.register_write_u16(
            Self::BASE_ADDR + VI_FILTER_COEFFICIENT_5_HI,
            |bus, _, _, val| {
                bus.vi.fct[5] = bus.vi.fct[5].set_hi(val);
            },
        );
        mmio.register_write_u16(
            Self::BASE_ADDR + VI_FILTER_COEFFICIENT_5_LO,
            |bus, _, _, val| {
                bus.vi.fct[5] = bus.vi.fct[5].set_lo(val);
            },
        );
        mmio.register_write_u16(
            Self::BASE_ADDR + VI_FILTER_COEFFICIENT_6_HI,
            |bus, _, _, val| {
                bus.vi.fct[6] = bus.vi.fct[6].set_hi(val);
            },
        );
        mmio.register_write_u16(
            Self::BASE_ADDR + VI_FILTER_COEFFICIENT_6_LO,
            |bus, _, _, val| {
                bus.vi.fct[6] = bus.vi.fct[6].set_lo(val);
            },
        );
        mmio.register_write_u16(
            Self::BASE_ADDR + VI_HORIZONTAL_SCALING_WIDTH,
            |bus, _, _, val| {
                bus.vi.hsw = val.into();
            },
        );
        //mmio.register_write_u16(Self::BASE_ADDR + VI_HORIZONTAL_SCALING_REGISTER, |_bus, _, _| {});
        mmio.register_u16(
            Self::BASE_ADDR + VI_CLOCK_SELECT,
            |bus, _, _| bus.vi.clock,
            |bus, _, _, val| {
                bus.vi.clock = val;
            },
        );
        mmio.register_write_u16(Self::BASE_ADDR + VI_UNKNOWN, |bus, _, _, val| {
            bus.vi.unknown = val;
        });
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

    pub fn update_interrupts(bus: &mut Bus, cpu_state: &mut CpuState) {
        if bus.vi.di[0].interrupt() && bus.vi.di[0].interrupt_enable()
            || bus.vi.di[1].interrupt() && bus.vi.di[1].interrupt_enable()
            || bus.vi.di[2].interrupt() && bus.vi.di[2].interrupt_enable()
            || bus.vi.di[3].interrupt() && bus.vi.di[3].interrupt_enable()
        {
            ProcessorInterface::set_interrupt(bus, cpu_state, PI_INTERRUPT_VI);
        } else {
            ProcessorInterface::clear_interrupt(bus, cpu_state, PI_INTERRUPT_VI);
        }
    }

    pub fn update(bus: &mut Bus, cpu_state: &mut CpuState) {
        //TODO: this is arbitrary, figure out how often this should execute
        let ticks = cpu_state.timers.get_ticks();
        if ticks - bus.vi.cpu_ticks > 600 {
            // number of ticks per half line drawn
            bus.vi.cpu_ticks = ticks;
        } else {
            return;
        }

        if bus.vi.config.enable() {
            bus.vi.vbp += 1;

            // NTSC
            if bus.vi.config.format() == 0 && bus.vi.vbp > 525 {
                bus.vi.vbp = 1;

                let mut i = bus.vi.tfbl & 0xFF_FFFF;
                let mut j = 0;

                while i < (bus.vi.tfbl & 0xFF_FFFF) + 320 * 480 * 4 {
                    let y1 = i32::from(bus.memory.read_u8(i));
                    let u = i32::from(bus.memory.read_u8(i + 1));
                    let y2 = i32::from(bus.memory.read_u8(i + 2));
                    let v = i32::from(bus.memory.read_u8(i + 3));

                    bus.vi.buffer[j] = yuv_to_rgb(y1, u, v);
                    bus.vi.buffer[j + 1] = yuv_to_rgb(y2, u, v);

                    i += 4;
                    j += 2;
                }

                #[cfg(not(test))]
                bus.vi
                    .window
                    .update_with_buffer(&bus.vi.buffer, WIDTH, HEIGHT)
                    .unwrap();
            }

            bus.vi.half_line_count += 1;
            if bus.vi.half_line_count
                == bus.vi.even_field_half_lines_total() + bus.vi.odd_field_half_lines_total()
            {
                bus.vi.half_line_count = 0;
            }

            let current_line = bus.vi.half_line_count / 2 + 1;

            for di in bus.vi.di.iter_mut() {
                if current_line == di.vct()
                    && ((bus.vi.half_line_count & 1 != 0) == (di.hct() > bus.vi.htr0.hlw()))
                {
                    di.set_interrupt(true);
                }
            }

            Self::update_interrupts(bus, cpu_state);
        }
    }
}

fn yuv_to_rgb(y: i32, u: i32, v: i32) -> u32 {
    let r = ((76283 * (y - 16) + 104_595 * (v - 128)) >> 16).clamp(0, 255) as u32;
    let g = (((76283 * (y - 16) - 53281 * (v - 128) - 25624 * (u - 128)) >> 16).clamp(0, 255)
        as u32)
        << 8;
    let b = (((76283 * (y - 16) + 132_252 * (u - 128)) >> 16).clamp(0, 255) as u32) << 16;

    b | g | r
}
