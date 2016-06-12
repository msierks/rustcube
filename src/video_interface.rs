
const VERTICAL_TIMING:          u32 = 0x00;
const DISPLAY_CONFIG:           u32 = 0x02;
const HORIZONTAL_TIMING_0_HI:   u32 = 0x04;
const HORIZONTAL_TIMING_0_LO:   u32 = 0x06;
const HORIZONTAL_TIMING_1_HI:   u32 = 0x08;
const HORIZONTAL_TIMING_1_LO:   u32 = 0x0A;
const VERTICAL_TIMING_ODD_HI:   u32 = 0x0C;
const VERTICAL_TIMING_ODD_LO:   u32 = 0x0E;
const VERTICAL_TIMING_EVEN_HI:  u32 = 0x10;
const VERTICAL_TIMING_EVEN_LO:  u32 = 0x12;
const BURST_BLANKING_ODD_HI:    u32 = 0x14;
const BURST_BLANKING_ODD_LO:    u32 = 0x16;
const BURST_BLANKING_EVEN_HI:   u32 = 0x18;
const BURST_BLANKING_EVEN_LO:   u32 = 0x1A;
const FB_TOP_LEFT_HI:           u32 = 0x1C;
const FB_TOP_LEFT_LO:           u32 = 0x1E;
const FB_TOP_RIGHT_HI:          u32 = 0x20;
const FB_TOP_RIGHT_LO:          u32 = 0x22;
const FB_BOTTOM_LEFT_HI:        u32 = 0x24;
const FB_BOTTOM_LEFT_LO:        u32 = 0x26;
const FB_BOTTOM_RIGHT_HI:       u32 = 0x28;
const FB_BOTTOM_RIGHT_LO:       u32 = 0x2A;
const BEAM_POSITION_VERTICAL:   u32 = 0x2C;
const BEAM_POSITION_HORIZONTAL: u32 = 0x2E;
const DISPLAY_INTERRUPT_0_HI:   u32 = 0x30;
const DISPLAY_INTERRUPT_0_LO:   u32 = 0x32;
const DISPLAY_INTERRUPT_1_HI:   u32 = 0x34;
const DISPLAY_INTERRUPT_1_LO:   u32 = 0x36;
const DISPLAY_INTERRUPT_2_HI:   u32 = 0x38;
const DISPLAY_INTERRUPT_2_LO:   u32 = 0x3A;
const DISPLAY_INTERRUPT_3_HI:   u32 = 0x3C;
const DISPLAY_INTERRUPT_3_LO:   u32 = 0x3E;
const DISPLAY_LATCH_0_LO:       u32 = 0x40;
const DISPLAY_LATCH_0_HI:       u32 = 0x42;
const DISPLAY_LATCH_1_LO:       u32 = 0x44;
const DISPLAY_LATCH_1_HI:       u32 = 0x46;
const SCALING_WIDTH:            u32 = 0x48;
const HORIZONTAL_SCALING:       u32 = 0x4A;
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
const DTV_STATUS:               u32 = 0x6E;
const UNKNOWN:                  u32 = 0x70;
const BORDER_BLANK_END:         u32 = 0x72;
const BORDER_BLANK_START:       u32 = 0x74;

pub struct VideoInterface {
    display_config: u16,
    vertical_beam_position: u16
}

impl VideoInterface {

    pub fn new() -> VideoInterface {
        VideoInterface {
            display_config: 0,
            vertical_beam_position: 0
        }
    }
 
    pub fn update(&mut self) {
        self.vertical_beam_position += 1;
    }

    pub fn read_u16(&self, register: u32) -> u16 {
        println!("READ VI reg {:#x}", register);

        match register {
            DISPLAY_CONFIG => self.display_config,
            BEAM_POSITION_VERTICAL => self.vertical_beam_position,
            DISPLAY_INTERRUPT_0_HI => 0,
            DISPLAY_INTERRUPT_1_HI => 0,
            _ => panic!("VI: unhandled register ({:#x})", register)
        }
    }

    pub fn write_u16(&mut self, register: u32, val: u16) {
        println!("WRITE VI reg {:#x} {}", register, val);

        match register {
            VERTICAL_TIMING => {},
            DISPLAY_CONFIG => self.display_config = val,
            HORIZONTAL_TIMING_0_HI => {},
            HORIZONTAL_TIMING_0_LO => {},
            HORIZONTAL_TIMING_1_HI => {},
            HORIZONTAL_TIMING_1_LO => {},
            VERTICAL_TIMING_ODD_HI => {},
            VERTICAL_TIMING_ODD_LO => {},
            VERTICAL_TIMING_EVEN_HI => {},
            VERTICAL_TIMING_EVEN_LO => {},
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
            CLOCK_SELECT => {},
            UNKNOWN => {},
            _ => panic!("VI: unhandled register ({:#x})", register)
        }
    }

}
