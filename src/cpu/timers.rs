// Note: CPU timebase and decrementer update at 1/4th the bus speed

pub const CPU_CLOCK: u64 = 486_000_000;
pub const BUS_CLOCK: u64 = 162_000_000; // One third cpu clock
const _TIMER_CLOCK: u64 = BUS_CLOCK / 4;

const TIMER_RATIO: u64 = 12; // 1/12th the cpu frequency

#[derive(Debug)]
pub struct Timers {
    tb_start_value: u64,
    tb_ticks: u64,
    tb_start_ticks: u64,
    dec_start_value: u32,
    dec_start_ticks: u64,
}

impl Default for Timers {
    fn default() -> Self {
        Timers {
            tb_start_value: 0,
            tb_ticks: 0,
            tb_start_ticks: 0,
            dec_start_value: 0xFFFF_FFFF,
            dec_start_ticks: 0,
        }
    }
}

impl Timers {
    // Used to advance cycle count of instruction
    pub fn tick(&mut self, ticks: u32) {
        self.tb_ticks = self.tb_ticks.wrapping_add(ticks as u64);
    }

    pub fn get_ticks(&self) -> u64 {
        self.tb_ticks
    }

    pub fn get_timebase(&mut self) -> u64 {
        self.tb_start_value + ((self.tb_ticks - self.tb_start_ticks) / TIMER_RATIO)
    }

    pub fn set_timebase_lower(&mut self, val: u32) {
        self.tb_start_ticks = self.tb_ticks;
        info!("Set Timebase Lower {val}");
        self.tb_start_value = (self.tb_start_value & !0xFFFF_FFFF) | val as u64;
    }

    pub fn set_timebase_upper(&mut self, val: u32) {
        self.tb_start_ticks = self.tb_ticks;
        info!("Set Timebase Upper {val}");
        self.tb_start_value = (self.tb_start_value & 0xFFFF_FFFF) | ((val as u64) << 32);
    }

    pub fn set_decrementer(&mut self, val: u32) {
        self.dec_start_ticks = self.tb_ticks;
        self.dec_start_value = val;
    }

    pub fn get_decrementer(&self) -> u32 {
        let elapsed = ((self.tb_ticks - self.dec_start_ticks) / TIMER_RATIO) as u32;
        self.dec_start_value.wrapping_sub(elapsed)
    }

    pub fn tick_decrementer(&mut self, ticks: u32) -> bool {
        let old = self.get_decrementer();
        self.tick(ticks);
        let new = self.get_decrementer();
        (old & 0x8000_0000) == 0 && (new & 0x8000_0000) != 0
    }
}
