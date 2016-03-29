
#[derive(Debug)]
#[allow(dead_code)]
pub enum Interrupt {
    SystemReset                  = 0x00100,
    MachineCheck                 = 0x00200,
    DataStorage                  = 0x00300,
    InstructionStorage           = 0x00400,
    External                     = 0x00500,
    Alignment                    = 0x00600,
    Program                      = 0x00700,
    FloatingPointUnavailable     = 0x00800,
    Decrementor                  = 0x00900,
    SystemCall                   = 0x00C00,
    Trace                        = 0x00D00,
    FloatingPointAssist          = 0x00E00,
    PerformanceMonitor           = 0x00F00, // Gekko Only
    InstructionAddressBreakpoint = 0x01300, // Gekko Only
    ThermalManagement            = 0x01700  // Gekko Only
}
