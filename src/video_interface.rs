
// NTSC  640x480 60Hz
// PAL   640x574 50Hz
// MPAL  640x490 60Hz
// PAL60 640x490 60Hz

const VERTICAL_TIMING:          u32 = 0x00;
const DISPLAY_CONFIG:           u32 = 0x02;
const HORIZONTAL_TIMING_0_HI:   u32 = 0x04;
const HORIZONTAL_TIMING_0_LO:   u32 = 0x06;
const HORIZONTAL_TIMING_1_HI:   u32 = 0x08; // Setting bit 0 seems to blackout screen.
const HORIZONTAL_TIMING_1_LO:   u32 = 0x0A;
const VERTICAL_TIMING_ODD_HI:   u32 = 0x0C; // Sets up the pre-blanking and post-blanking interval of odd fields.
const VERTICAL_TIMING_ODD_LO:   u32 = 0x0E;
const VERTICAL_TIMING_EVEN_HI:  u32 = 0x10; // Sets up the pre-blanking and post-blanking intervals of even fields.
const VERTICAL_TIMING_EVEN_LO:  u32 = 0x12;
const BURST_BLANKING_ODD_HI:    u32 = 0x14;
const BURST_BLANKING_ODD_LO:    u32 = 0x16;
const BURST_BLANKING_EVEN_HI:   u32 = 0x18;
const BURST_BLANKING_EVEN_LO:   u32 = 0x1A;
const FB_TOP_LEFT_HI:           u32 = 0x1C; // Display origin of the top field of a picture in 2D mode or left picture in 3D.
const FB_TOP_LEFT_LO:           u32 = 0x1E;
const FB_TOP_RIGHT_HI:          u32 = 0x20; // Base address of the top field for the right picture in 3D mode.
const FB_TOP_RIGHT_LO:          u32 = 0x22;
const FB_BOTTOM_LEFT_HI:        u32 = 0x24; // Display origin of the the bottom field of a picture in 2D mode or left picture in 3D.
const FB_BOTTOM_LEFT_LO:        u32 = 0x26;
const FB_BOTTOM_RIGHT_HI:       u32 = 0x28; // Base address of the bottom field for the right picture in 3D mode.
const FB_BOTTOM_RIGHT_LO:       u32 = 0x2A;
const BEAM_POSITION_VERTICAL:   u32 = 0x2C; // Count in lines (on frame basis), runs from 1 to # lines per frame. NTSC vcount ranges from 1-263.
const BEAM_POSITION_HORIZONTAL: u32 = 0x2E; // Count in pixels, runs from 1 to # pixels per line.
const DISPLAY_INTERRUPT_0_HI:   u32 = 0x30; // There are 4 display interrupts(0-3). They generate interrupts to the cpu at different positions within field.
const DISPLAY_INTERRUPT_0_LO:   u32 = 0x32;
const DISPLAY_INTERRUPT_1_HI:   u32 = 0x34;
const DISPLAY_INTERRUPT_1_LO:   u32 = 0x36;
const DISPLAY_INTERRUPT_2_HI:   u32 = 0x38;
const DISPLAY_INTERRUPT_2_LO:   u32 = 0x3A;
const DISPLAY_INTERRUPT_3_HI:   u32 = 0x3C;
const DISPLAY_INTERRUPT_3_LO:   u32 = 0x3E;
const DISPLAY_LATCH_0_LO:       u32 = 0x40; // Latches the value of the display position register at the rising edge of the gt0 signal. The trigger is set if a gun trigger is detected.
const DISPLAY_LATCH_0_HI:       u32 = 0x42;
const DISPLAY_LATCH_1_LO:       u32 = 0x44;
const DISPLAY_LATCH_1_HI:       u32 = 0x46;
const SCALING_WIDTH:            u32 = 0x48; // Number of source pixels to be scaled. Only used when horizontal scaler is enabled.
const HORIZONTAL_SCALING:       u32 = 0x4A; // Step size of horizontal stepper.
const FILTER_COEFFICIENT_0_HI:  u32 = 0x4C;
const FILTER_COEFFICIENT_0_LO:  u32 = 0x4E;
const FILTER_COEFFICIENT_1_HI:  u32 = 0x50;
const FILTER_COEFFICIENT_1_LO:  u32 = 0x52;
const FILTER_COEFFICIENT_2_HI:  u32 = 0x54;
const FILTER_COEFFICIENT_2_LO:  u32 = 0x56;
const FILTER_COEFFICIENT_3_HI:  u32 = 0x58;
const FILTER_COEFFICIENT_3_LO:  u32 = 0x5A;
const FILTER_COEFFICIENT_4_HI:  u32 = 0x5C;
const FILTER_COEFFICIENT_4_LO:  u32 = 0x5E;
const FILTER_COEFFICIENT_5_HI:  u32 = 0x60;
const FILTER_COEFFICIENT_5_LO:  u32 = 0x62;
const FILTER_COEFFICIENT_6_HI:  u32 = 0x64;
const FILTER_COEFFICIENT_6_LO:  u32 = 0x66;
const UNKOWN_AA_HI:             u32 = 0x68;
const UNKOWN_AA_LO:             u32 = 0x6A;
const CLOCK_SELECT:             u32 = 0x6C;
const DTV_STATUS:               u32 = 0x6E; // Read status of 2 io pins.
const UNKNOWN:                  u32 = 0x70; // Horizontal stepping ??? progressive scanning ???
const BORDER_BLANK_END:         u32 = 0x72; // Sets up black border around active pixels in debug mode.
const BORDER_BLANK_START:       u32 = 0x74;

#[derive(Debug)]
pub struct VideoInterface {
    vertical_timing: VerticalTiming,
    horizontal_timing: HorizontalTiming,
    display_config: DisplayConfig,
    vertical_beam_position: u16,
    vertical_timing_odd: VBlankTiming,
    vertical_timing_even: VBlankTiming,

    clock_select: bool, // 0: 27MHz, 1: 54 MHz (used in progressive)
}

impl VideoInterface {

    pub fn new() -> VideoInterface {
        VideoInterface {
            vertical_timing: VerticalTiming::default(),
            horizontal_timing: HorizontalTiming::default(),
            display_config: DisplayConfig::default(),
            vertical_beam_position: 1, // default location
            vertical_timing_odd: VBlankTiming::default(),
            vertical_timing_even: VBlankTiming::default(),

            clock_select: false
        }
    }
 
    pub fn update(&mut self) {
        if self.display_config.enable {

            self.vertical_beam_position += 1;

            if self.display_config.format == 0 && self.vertical_beam_position > 525 { // ntsc
                self.vertical_beam_position = 1;
            }
        }
    }

    pub fn read_u16(&mut self, register: u32) -> u16 {
        //println!("READ VI reg {:#x}", register);

        self.update();

        match register {
            DISPLAY_CONFIG => self.display_config.as_u16(),
            BEAM_POSITION_VERTICAL => self.vertical_beam_position,
            DISPLAY_INTERRUPT_0_HI => 0,
            DISPLAY_INTERRUPT_1_HI => 0,
            _ => panic!("VI: unhandled register ({:#x})", register)
        }
    }

    pub fn write_u16(&mut self, register: u32, val: u16) {
        //println!("WRITE VI reg {:#x} {:#x}", register, val);

        match register {
            VERTICAL_TIMING => self.vertical_timing = val.into(),
            DISPLAY_CONFIG => self.display_config = val.into(),
            HORIZONTAL_TIMING_0_HI => self.horizontal_timing.from_hi(val),
            HORIZONTAL_TIMING_0_LO => self.horizontal_timing.from_lo(val),
            HORIZONTAL_TIMING_1_HI => self.horizontal_timing.from_hi_1(val),
            HORIZONTAL_TIMING_1_LO => self.horizontal_timing.from_lo_1(val),
            VERTICAL_TIMING_ODD_HI => self.vertical_timing_odd.from_hi(val),
            VERTICAL_TIMING_ODD_LO => self.vertical_timing_odd.from_lo(val),
            VERTICAL_TIMING_EVEN_HI => self.vertical_timing_even.from_hi(val),
            VERTICAL_TIMING_EVEN_LO => self.vertical_timing_even.from_lo(val),
            BURST_BLANKING_ODD_HI => {},
            BURST_BLANKING_ODD_LO => {},
            BURST_BLANKING_EVEN_HI => {},
            BURST_BLANKING_EVEN_LO => {},
            DISPLAY_INTERRUPT_0_HI => {},
            DISPLAY_INTERRUPT_0_LO => {},
            DISPLAY_INTERRUPT_1_HI => {},
            DISPLAY_INTERRUPT_1_LO => {},
            FILTER_COEFFICIENT_0_HI => {},
            FILTER_COEFFICIENT_0_LO => {},
            FILTER_COEFFICIENT_1_HI => {},
            FILTER_COEFFICIENT_1_LO => {},
            FILTER_COEFFICIENT_2_HI => {},
            FILTER_COEFFICIENT_2_LO => {},
            FILTER_COEFFICIENT_3_HI => {},
            FILTER_COEFFICIENT_3_LO => {},
            FILTER_COEFFICIENT_4_HI => {},
            FILTER_COEFFICIENT_4_LO => {},
            FILTER_COEFFICIENT_5_HI => {},
            FILTER_COEFFICIENT_5_LO => {},
            FILTER_COEFFICIENT_6_HI => {},
            FILTER_COEFFICIENT_6_LO => {},
            SCALING_WIDTH => {},
            CLOCK_SELECT => self.clock_select = val != 0,
            UNKNOWN => {},
            _ => panic!("VI: unhandled register ({:#x})", register)
        }
    }

}

#[derive(Debug, Default)]
struct DisplayConfig {
    format:          u8,   // 0: NTSC, 1: PAL, 2: MPAL, 3: DEBUG // pal50/pal60/ntsc: 0x0101, 0x0001, 0x0001
    display_latch_0: u8,
    display_latch_1: u8,
    display_mode_3d: bool,
    interlaced:      bool,
    reset:           bool,
    enable:          bool
}

impl From<u16> for DisplayConfig {
    fn from(value: u16) -> Self {
        DisplayConfig {
            format:          ((value >> 8) & 3) as u8,
            display_latch_0: ((value >> 6) & 3) as u8,
            display_latch_1: ((value >> 4) & 3) as u8,
            display_mode_3d: (value & (1 << 3)) != 0,
            interlaced:      (value & (1 << 2)) != 0,
            reset:           (value & (1 << 1)) != 0,
            enable:          (value & 1) != 0
        }
    }
}

impl DisplayConfig {
    pub fn as_u16(&self) -> u16 {
        let mut value = 0;

        value |= (self.format as u16)          << 8;
        value |= (self.display_latch_0 as u16) << 6;
        value |= (self.display_latch_1 as u16) << 4;
        value |= (self.display_mode_3d as u16) << 3;
        value |= (self.interlaced as u16)      << 2;
        value |= (self.reset as u16)           << 1;
        value |=  self.enable as u16;

        value
    }
}

#[derive(Debug, Default)]
struct VerticalTiming {
    active_video: u16,
    equalization: u8 // equalization pulse in half lines
}

impl From<u16> for VerticalTiming {
    fn from(value: u16) -> Self {
        VerticalTiming {
            active_video: ((value >> 4) & 0x3FF) as u16,
            equalization: (value & 0xF) as u8
        }
    }
}

#[derive(Debug, Default)]
struct HorizontalTiming {
    horizontal_sync_start: u8,
    horizontal_sync_end: u8,
    halfline_width: u16,
    halfline_blank_start: u16,
    horizontal_blank_end: u16,
    horizontal_sync_width: u8,
}

impl HorizontalTiming {
    pub fn from_hi(&mut self, val: u16) {
        self.horizontal_sync_start = ((val >> 8) & 0x7F) as u8;
        self.horizontal_sync_end = (val & 0x7F) as u8
    }

    pub fn from_lo(&mut self, val: u16) {
        self.halfline_width = val & 0x1FF;
    }

    pub fn from_hi_1(&mut self, val: u16) {
        self.halfline_blank_start = (val >> 1) & 0x3FF;
        self.horizontal_blank_end |= (val & 1) << 9;
    }

    pub fn from_lo_1(&mut self, val: u16) {
        self.horizontal_blank_end = val >> 7;
        self.horizontal_sync_width = (val & 0x7F) as u8;
    }
}

#[derive(Debug, Default)]
struct VBlankTiming {
    post_blanking: u16,
    pre_blanking: u16
}

impl VBlankTiming {
    pub fn from_hi(&mut self, val: u16) {
        self.post_blanking = val;
    }

    pub fn from_lo(&mut self, val: u16) {
        self.pre_blanking = val;
    }
}
