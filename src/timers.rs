// Note: CPU timebase and decrementer update at 1/4th the bus speed

const TIMER_RATIO: u64 = 12; // 1/12th the cpu frequency

#[derive(Default, Debug)]
pub struct Timers {
    tb_start_value: u64,
    tb_ticks: u64,
    tb_start_ticks: u64,
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
        self.tb_start_value = (self.tb_start_value & 0xFFFF_FFFF) | (val as u64) << 32;
    }
}
