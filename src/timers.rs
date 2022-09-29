// Note: CPU timebase and decrementer update at 1/4th the bus speed
const TIMER_RATIO: u64 = 4 * 3;

#[derive(Debug, Default)]
pub struct Timers {
    cycles: u64,
    timebase_cycle_last: u64,
    timebase: u64,
}

impl Timers {
    pub fn tick(&mut self, cycles: u32) {
        self.cycles = self.cycles.wrapping_add(cycles as u64);
    }

    pub fn get_ticks(&self) -> u64 {
        self.cycles
    }

    pub fn get_timebase(&mut self) -> u64 {
        let timebase_jump = (self.cycles - self.timebase_cycle_last) / TIMER_RATIO;

        if timebase_jump > 0 {
            self.timebase += timebase_jump;
            self.timebase_cycle_last += timebase_jump * TIMER_RATIO;
        }

        self.timebase
    }

    pub fn set_timebase_lower(&mut self, val: u32) {
        println!("Set Timebase Lower {}", val);
        self.timebase = (self.timebase & !0xFFFF_FFFF) | val as u64;
    }

    pub fn set_timebase_upper(&mut self, val: u32) {
        println!("Set Timebase Upper {}", val);
        self.timebase = (self.timebase & 0xFFFF_FFFF) | (val as u64) << 32;
    }
}
