
// Machine Status Register (MSR)

#[derive(Debug)]
pub struct MachineStatus {
    // POW
    power_management: bool,

    // ILE
    exception_little_endian: bool,

    // EE
    external_interrupt: bool,

    // PR
    pub privilege_level: bool,

    // FP
    floating_point: bool,

    // ME
    machine_check: bool,

    // FE0
    fp_exception_mode_0: bool,

    // FE1
    fp_exception_mode_1: bool,

    // SE
    single_step_trace: bool,

    // BE
    branch_trace: bool,

    // IP
    pub exception_prefix: bool,

    // IR
    pub instr_address_translation: bool,

    // DR
    pub data_address_translation: bool,

    // PM
    performance_monitor_marked: bool,

    // RI
    reset_recoverable: bool,

    // LE
    little_endian: bool
}

impl Default for MachineStatus {
    fn default() -> MachineStatus {
        MachineStatus {
            power_management:           false,
            exception_little_endian:    false,
            external_interrupt:         false,
            privilege_level:            false,
            floating_point:             false,
            machine_check:              false,
            fp_exception_mode_0:        false,
            fp_exception_mode_1:        false,
            single_step_trace:          false,
            branch_trace:               false,
            exception_prefix:           true,
            instr_address_translation:  false,
            data_address_translation:   false,
            performance_monitor_marked: false,
            reset_recoverable:          false,
            little_endian:              false
        }
    }
}

impl MachineStatus {
    pub fn as_u32(&self) -> u32 {
        let mut value = 0;

        value ^= (self.power_management as u32)           << 18;
        value ^= (self.exception_little_endian as u32)    << 16;
        value ^= (self.external_interrupt as u32)         << 15;
        value ^= (self.privilege_level as u32)            << 14;
        value ^= (self.floating_point as u32)             << 13;
        value ^= (self.machine_check as u32)              << 12;
        value ^= (self.fp_exception_mode_0 as u32)        << 11;
        value ^= (self.fp_exception_mode_1 as u32)        <<  8;
        value ^= (self.single_step_trace as u32)          << 10;
        value ^= (self.branch_trace as u32)               <<  9;
        value ^= (self.exception_prefix as u32)           <<  6;
        value ^= (self.instr_address_translation as u32)  <<  5;
        value ^= (self.data_address_translation as u32)   <<  4;
        value ^= (self.performance_monitor_marked as u32) <<  2;
        value ^= (self.reset_recoverable as u32)          <<  1;
        value ^=  self.little_endian as u32;

        value
    }
}

impl From<u32> for MachineStatus {
    fn from(value: u32) -> Self {
        MachineStatus {
            power_management:           (value & (1 << 18)) != 0,
            exception_little_endian:    (value & (1 << 16)) != 0,
            external_interrupt:         (value & (1 << 15)) != 0,
            privilege_level:            (value & (1 << 14)) != 0,
            floating_point:             (value & (1 << 13)) != 0,
            machine_check:              (value & (1 << 12)) != 0,
            fp_exception_mode_0:        (value & (1 << 11)) != 0,
            fp_exception_mode_1:        (value & (1 <<  8)) != 0,
            single_step_trace:          (value & (1 << 10)) != 0,
            branch_trace:               (value & (1 <<  9)) != 0,
            exception_prefix:           (value & (1 <<  6)) != 0,
            instr_address_translation:  (value & (1 <<  5)) != 0,
            data_address_translation:   (value & (1 <<  4)) != 0,
            performance_monitor_marked: (value & (1 <<  2)) != 0,
            reset_recoverable:          (value & (1 <<  1)) != 0,
            little_endian:              (value &  1)        != 0
        }
    }
}

#[cfg(test)]
mod test {
    use super::MachineStatus;

    #[test]
    fn default() {
        let msr = MachineStatus::default();

        assert!(msr.exception_prefix); // exception prefix enabled by default
    }

    #[test]
    fn u32_conversion() {
        let msr: MachineStatus = 0x55555.into();

        assert!(msr.power_management);
        assert!(msr.exception_little_endian);
        assert!(!msr.external_interrupt);
        assert!(msr.privilege_level);
        assert!(!msr.floating_point);
        assert!(msr.machine_check);
        assert!(!msr.fp_exception_mode_0);
        assert!(msr.single_step_trace);
        assert!(!msr.branch_trace);
        assert!(msr.fp_exception_mode_1);
        assert!(msr.exception_prefix);
        assert!(!msr.instr_address_translation);
        assert!(msr.data_address_translation);
        assert!(msr.performance_monitor_marked);
        assert!(!msr.reset_recoverable);
        assert!(msr.little_endian);

        assert_eq!(0x55555, msr.as_u32());

        let msr: MachineStatus = 0xAA22.into();

        assert!(!msr.power_management);
        assert!(!msr.exception_little_endian);
        assert!(msr.external_interrupt);
        assert!(!msr.privilege_level);
        assert!(msr.floating_point);
        assert!(!msr.machine_check);
        assert!(msr.fp_exception_mode_0);
        assert!(!msr.single_step_trace);
        assert!(msr.branch_trace);
        assert!(!msr.fp_exception_mode_1);
        assert!(!msr.exception_prefix);
        assert!(msr.instr_address_translation);
        assert!(!msr.data_address_translation);
        assert!(!msr.performance_monitor_marked);
        assert!(msr.reset_recoverable);
        assert!(!msr.little_endian);

        assert_eq!(0xAA22, msr.as_u32());
    }
}
