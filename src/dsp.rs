mod cpu;

use std::fs;

use byteorder::{BigEndian, ReadBytesExt};

use self::cpu::{dsp_step, DspCpu, INTERRUPT_RESET};
use crate::{
    bus::Bus,
    cpu::CpuState,
    hw::{
        mmio::{Mmio, MmioDevice},
        pi::{ProcessorInterface, PI_INTERRUPT_DSP},
    },
    utils::Halveable,
};

const DSP_CTDMBH: u32 = 0x00; // CPU -> DSP Mailbox High Address (0xFFFE)
const DSP_CTDMBL: u32 = 0x02; // CPU -> DSP Mailbox Low Address (0xFFFF)
const DSP_DTCMBH: u32 = 0x04; // DSP -> CPU Mailbox High Address (0xFFFC)
const DSP_DTCMBL: u32 = 0x06; // DSP -> CPU Mailbox Low Address (0xFFFD)
const DSP_CTDCR: u32 = 0x0A; // CPU -> DSP Control Register Address
const DSP_ARAMC: u32 = 0x12; // ARAM Configuration Regiser
const DSP_ARAMSF: u32 = 0x16; // ARAM Normal State Flag (0: not ready, 1 ready)
const DSP_ARAMCT: u32 = 0x1A; // ARAM Control Test Reg
const ARAM_DMA_MMAADDR_HI: u32 = 0x20; // ARAM DMA Main Memory Address High Register
const ARAM_DMA_MMAADDR_LO: u32 = 0x22; // ARAM DMA Main Memory Address Low Register
const ARAM_DMA_ARADDR_HI: u32 = 0x24; // ARAM DMA aram Address Register High Register
const ARAM_DMA_ARADDR_LO: u32 = 0x26; // ARAM DMA aram Address Register Low Register
const ARAM_DMA_SIZE_HI: u32 = 0x28; // ARAM DMA Block Length High Register
const ARAM_DMA_SIZE_LO: u32 = 0x2A; // ARAM DMA Block Length High Register
const DSP_AIDMAMAH: u32 = 0x30; // AI DMA Maim Memory Starting Address High Register
const DSP_AIDMAMAL: u32 = 0x32; // AI DMA Maim Memory Starting Address Low Register
const DSP_AIDMABL: u32 = 0x36; // AI DMA Block Length
const DSP_AIDMABR: u32 = 0x3A; // AI DMA Blocks Remaining

const AI_DMA_INT: u16 = 0x0;
//const ARAM_DMA_INT: u16 = 0x0;
//const DSP_INT: u16 = 0x0;

const TIMER_RATIO: u64 = 6;

pub struct DspInterface {
    control_register: ControlRegister,
    aram_conf: AramConfigRegister,
    aram_state: u16,
    aram_refresh: AramControlTestRegister,
    aram_mma_addr: u32,
    aram_ar_addr: u32,
    aram_dma_size: u32,
    aidma: u32,
    aidmabl: u16,
    aidmabr: u16,
    cpu_ticks: u64,
    ctx: DspContext,
}

impl Default for DspInterface {
    fn default() -> Self {
        let mut control_register = ControlRegister(0);

        control_register.set_halt(true);

        let mut ctx = DspContext::default();

        ctx.load_roms();

        DspInterface {
            control_register,
            aram_conf: Default::default(),
            aram_state: 1,
            aram_refresh: AramControlTestRegister(156),
            aram_mma_addr: 0,
            aram_ar_addr: 0,
            aram_dma_size: 0,
            aidma: 0,
            aidmabl: 0,
            aidmabr: 0,
            cpu_ticks: 0,
            ctx,
        }
    }
}

impl MmioDevice for DspInterface {
    const BASE_ADDR: u32 = 0x0C00_5000;

    fn register_mmio(mmio: &mut Mmio) {
        mmio.register_u16(
            Self::BASE_ADDR + DSP_CTDMBH,
            |bus, _, _| bus.dsp.ctx.cdmb.hi(),
            |bus, _, _, val| {
                bus.dsp.ctx.cdmb = bus.dsp.ctx.cdmb.set_hi(val & 0x7FFF); // Clear valid flag
            },
        );
        mmio.register_u16(
            Self::BASE_ADDR + DSP_CTDMBL,
            |bus, _, _| bus.dsp.ctx.cdmb.lo(),
            |bus, _, _, val| {
                bus.dsp.ctx.cdmb = bus.dsp.ctx.cdmb.set_lo(val) | 0x8000_0000; // Set valid flag
            },
        );
        mmio.register_read_u16(Self::BASE_ADDR + DSP_DTCMBH, |bus, _, _| {
            bus.dsp.ctx.dcmb.hi()
        });
        mmio.register_read_u16(Self::BASE_ADDR + DSP_DTCMBL, |bus, _, _| {
            let val = bus.dsp.ctx.dcmb.lo();
            bus.dsp.ctx.dcmb &= 0x7FFF_FFFF; // Clear valid flag
            val
        });
        mmio.register_u16(
            Self::BASE_ADDR + DSP_CTDCR,
            |bus, _, _| bus.dsp.control_register.into(),
            |bus, cpu_state, _, val| {
                let tmp = ControlRegister(val);

                if tmp.reset() {
                    if tmp.dsp_init() {
                        info!("DSP reset");
                        bus.dsp.ctx.reset(INTERRUPT_RESET);
                    } else {
                        bus.dsp.ctx.reset(0x0000);
                    }
                    bus.dsp.aidmabl = 0;
                    bus.dsp.aidmabr = 0;
                }

                bus.dsp.control_register.set_reset(false);
                bus.dsp.control_register.set_interrupt(tmp.interrupt());
                bus.dsp.control_register.set_halt(tmp.halt());
                bus.dsp.control_register.set_init_code(tmp.init_code());
                bus.dsp.control_register.set_dsp_init(tmp.dsp_init());

                bus.dsp
                    .control_register
                    .set_ai_interrupt_mask(tmp.ai_interrupt_mask());
                bus.dsp
                    .control_register
                    .set_aram_interrupt_mask(tmp.aram_interrupt_mask());
                bus.dsp
                    .control_register
                    .set_dsp_interrupt_mask(tmp.dsp_interrupt_mask());

                if tmp.ai_interrupt() {
                    bus.dsp.control_register.set_ai_interrupt(false);
                }
                if tmp.aram_interrupt() {
                    bus.dsp.control_register.set_aram_interrupt(false);
                }
                if tmp.dsp_interrupt() {
                    bus.dsp.control_register.set_dsp_interrupt(false);
                }

                Self::update_interrupts(bus, cpu_state);
            },
        );
        mmio.register_u16(
            Self::BASE_ADDR + DSP_ARAMC,
            |bus, _, _| bus.dsp.aram_conf.0,
            |bus, _, _, val| {
                bus.dsp.aram_conf = AramConfigRegister(val & 0x7F);
            },
        );
        mmio.register_read_u16(Self::BASE_ADDR + DSP_ARAMSF, |bus, _, _| bus.dsp.aram_state);
        mmio.register_u16(
            Self::BASE_ADDR + DSP_ARAMCT,
            |bus, _, _| bus.dsp.aram_refresh.0,
            |bus, _, _, val| {
                bus.dsp.aram_refresh = AramControlTestRegister(val & 0x7FF);
            },
        );
        mmio.register_u16(
            Self::BASE_ADDR + ARAM_DMA_MMAADDR_HI,
            |bus, _, _| bus.dsp.aram_mma_addr.hi(),
            |bus, _, _, val| {
                bus.dsp.aram_mma_addr = bus.dsp.aram_mma_addr.set_hi(val & 0x3FF);
            },
        );
        mmio.register_u16(
            Self::BASE_ADDR + ARAM_DMA_MMAADDR_LO,
            |bus, _, _| bus.dsp.aram_mma_addr.lo(),
            |bus, _, _, val| {
                bus.dsp.aram_mma_addr = bus.dsp.aram_mma_addr.set_lo(val & 0xFFE0);
            },
        );
        mmio.register_write_u32(Self::BASE_ADDR + ARAM_DMA_MMAADDR_HI, |bus, _, _, val| {
            bus.dsp.aram_mma_addr = val & 0x03FF_FFE0;
        });
        mmio.register_u16(
            Self::BASE_ADDR + ARAM_DMA_ARADDR_HI,
            |bus, _, _| bus.dsp.aram_ar_addr.hi(),
            |bus, _, _, val| {
                bus.dsp.aram_ar_addr = bus.dsp.aram_ar_addr.set_hi(val & 0x3FF);
            },
        );
        mmio.register_u16(
            Self::BASE_ADDR + ARAM_DMA_ARADDR_LO,
            |bus, _, _| bus.dsp.aram_ar_addr.lo(),
            |bus, _, _, val| {
                bus.dsp.aram_ar_addr = bus.dsp.aram_ar_addr.set_lo(val & 0xFFE0);
            },
        );
        mmio.register_write_u32(Self::BASE_ADDR + ARAM_DMA_ARADDR_HI, |bus, _, _, val| {
            bus.dsp.aram_ar_addr = val & 0x03FF_FFE0;
        });
        mmio.register_u16(
            Self::BASE_ADDR + ARAM_DMA_SIZE_HI,
            |bus, _, _| bus.dsp.aram_dma_size.hi(),
            |bus, _, _, val| {
                bus.dsp.aram_dma_size = bus.dsp.aram_dma_size.set_hi(val);
            },
        );
        mmio.register_u16(
            Self::BASE_ADDR + ARAM_DMA_SIZE_LO,
            |bus, _, _| bus.dsp.aram_dma_size.lo(),
            |bus, cpu_state, _, val| {
                bus.dsp.aram_dma_size = bus.dsp.aram_dma_size.set_lo(val);

                Self::aram_dma(bus);

                Self::generate_interrupt(bus, cpu_state, 0x20);
            },
        );
        mmio.register_write_u32(
            Self::BASE_ADDR + ARAM_DMA_SIZE_HI,
            |bus, cpu_state, _, val| {
                bus.dsp.aram_dma_size = val;

                Self::aram_dma(bus);

                Self::generate_interrupt(bus, cpu_state, 0x20);
            },
        );
        mmio.register_u16(
            Self::BASE_ADDR + DSP_AIDMAMAH,
            |bus, _, _| bus.dsp.aidma.hi(),
            |bus, _, _, val| {
                bus.dsp.aidma = bus.dsp.aidma.set_hi(val & 0x3FF);
            },
        );
        mmio.register_u16(
            Self::BASE_ADDR + DSP_AIDMAMAL,
            |bus, _, _| bus.dsp.aidma.lo(),
            |bus, _, _, val| {
                bus.dsp.aidma = bus.dsp.aidma.set_lo(val & 0xFFE0);
            },
        );
        mmio.register_u16(
            Self::BASE_ADDR + DSP_AIDMABL,
            |bus, _, _| bus.dsp.aidmabl,
            |bus, cpu_state, _, val| {
                let already_enabled = bus.dsp.aidmabl & 0x8000 != 0;
                bus.dsp.aidmabl = val;
                let control = DmaControlRegister(val);
                if !already_enabled && control.enable() {
                    bus.dsp.aidmabr = control.size();
                    Self::generate_interrupt(bus, cpu_state, AI_DMA_INT);
                    bus.dsp.aidmabr = 0;
                }
            },
        );
        mmio.register_read_u16(Self::BASE_ADDR + DSP_AIDMABR, |bus, _, _| {
            // Remaining is zero-based on hardware; never stuck non-zero or games hang.
            if bus.dsp.aidmabr > 0 {
                bus.dsp.aidmabr - 1
            } else {
                0
            }
        });
    }
}

impl DspInterface {
    fn generate_interrupt(bus: &mut Bus, cpu_state: &mut CpuState, interrupt: u16) {
        bus.dsp.control_register = ControlRegister(bus.dsp.control_register.0 | (interrupt));

        Self::update_interrupts(bus, cpu_state);
    }

    pub fn update_interrupts(bus: &mut Bus, cpu_state: &mut CpuState) {
        let control = bus.dsp.control_register.0;

        if ((control >> 1) & control) & 0xA8 != 0 {
            ProcessorInterface::set_interrupt(bus, cpu_state, PI_INTERRUPT_DSP);
        } else {
            ProcessorInterface::clear_interrupt(bus, cpu_state, PI_INTERRUPT_DSP);
        }
    }

    fn aram_dma(bus: &mut Bus) {
        let mut cnt = bus.dsp.aram_dma_size & 0x3FF_FFE0;
        let dir = (bus.dsp.aram_dma_size & 0x8000_0000) != 0; // 0: MM → ARAM, 1: ARAM → MM

        // Mirrored every 64MB (Dolphin / hardware)
        bus.dsp.aram_ar_addr &= 0x03FF_FFFF;
        bus.dsp.aram_mma_addr &= 0x03FF_FFFF;

        if !dir {
            info!(
                "DMA from Main Memory {:#010x} to ARAM {:#010x} ({:#x})",
                bus.dsp.aram_mma_addr, bus.dsp.aram_ar_addr, cnt,
            );
        } else {
            info!(
                "DMA from ARAM {:#010x} to Main Memory {:#010x} ({:#x})",
                bus.dsp.aram_ar_addr, bus.dsp.aram_mma_addr, cnt,
            );
        }

        if bus.dsp.aram_ar_addr < ARAM_SIZE as u32 && cnt != 0 {
            let aram_mask = (ARAM_SIZE as u32) - 1;
            while cnt != 0 {
                let ar_idx = (bus.dsp.aram_ar_addr & aram_mask) as usize;
                if !dir {
                    // MM -> ARAM
                    bus.dsp.ctx.aram[ar_idx] = bus.memory.read_u8(bus.dsp.aram_mma_addr);
                } else {
                    // ARAM -> MM
                    bus.memory
                        .write_u8(bus.dsp.aram_mma_addr, bus.dsp.ctx.aram[ar_idx]);
                }
                bus.dsp.aram_mma_addr += 1;
                bus.dsp.aram_ar_addr += 1;
                cnt -= 1;
            }

            bus.dsp.aram_dma_size &= 0x8000_0000; // clear count
        }
    }

    pub fn update(bus: &mut Bus, cpu_state: &mut CpuState) {
        let ticks = cpu_state.timers.get_ticks();
        if ticks - bus.dsp.cpu_ticks > TIMER_RATIO {
            bus.dsp.cpu_ticks = ticks;
        } else {
            return;
        }

        bus.dsp.ctx.step();
    }
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    pub struct ControlRegister(u16);
    impl Debug;
    pub reset, set_reset: 0;                              // dsp software reset
    pub interrupt, set_interrupt : 1;                     // cpu interrupt dsp
    pub halt, set_halt : 2;                               // cpu halt dsp
    pub ai_interrupt, set_ai_interrupt : 3;               // ai dma interrupt cpu flag
    pub ai_interrupt_mask, set_ai_interrupt_mask : 4;     // ai dma interrupt cpu mask
    pub aram_interrupt, set_aram_interrupt : 5;           // aram dma interrupt cpu flag
    pub aram_interrupt_mask, set_aram_interrupt_mask : 6; // aram dma interrupt cpu mask
    pub dsp_interrupt, set_dsp_interrupt : 7;             // dsp interrupt cpu flag
    pub dsp_interrupt_mask, set_dsp_interrupt_mask : 8;   // dsp interrupt cpu mask
    pub dma_state, set_dma_state : 9;                     // aram dma busy
    pub init_code, set_init_code : 10;                    // dsp dma busy
    pub dsp_init, set_dsp_init : 11;                      // dsp reset start bit (vector 0 = 0x0000, 1 = 0x8000)
}

impl From<u16> for ControlRegister {
    fn from(v: u16) -> Self {
        ControlRegister(v)
    }
}

impl From<ControlRegister> for u16 {
    fn from(s: ControlRegister) -> u16 {
        s.0
    }
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    pub struct DmaControlRegister(u16);
    impl Debug;
    pub size, set_size: 14, 0;
    pub enable, set_enable : 15;
}

impl From<u16> for DmaControlRegister {
    fn from(v: u16) -> Self {
        DmaControlRegister(v)
    }
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    pub struct AramConfigRegister(u16);
    impl Debug;
    pub int_size, set_int_size: 2, 0;      // Internal ARAM Size (0: 16M, 1: 32M, 2: 64MB, 3: 128MB, 4: 256MB)
    pub exp_size, set_exp_size: 5, 3;      // Expansion ARAM Size (0: 16M, 1: 32M, 2: 64MB, 3: 128MB, 4: 256MB)
    pub mode_setting, set_mode_setting: 6; // ARAM Mode-reg Setting (0: enable, 1: disable)
}

impl From<u16> for AramConfigRegister {
    fn from(v: u16) -> Self {
        AramConfigRegister(v)
    }
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    pub struct AramControlTestRegister(u16);
    impl Debug;
    pub period, set_period: 7, 0;                      // SDRAM refresh perriod
    pub infinite, set_infinite: 8;                     // 0: ARAM refresh period by programmed value,
                                                       // 1: refresh period is inifinite
    pub controller_disable, set_controller_disable: 9; // 0: ARAM controller is enabled
                                                       // 1: ARAM controller is disabled
    pub initial_wait, set_initial_wait: 10;            // 0: wait 202us 1: skip 202us
}

impl From<u16> for AramControlTestRegister {
    fn from(v: u16) -> Self {
        AramControlTestRegister(v)
    }
}

/// ARAM Size: 16MB
const ARAM_SIZE: usize = 0x100_0000;
const IRAM_SIZE: usize = 0x1000;
const IROM_SIZE: usize = 0x1000;
const DRAM_SIZE: usize = 0x1000;
const DROM_SIZE: usize = 0x0800;

pub struct DspContext {
    cpu: DspCpu,
    aram: Box<[u8]>,
    /// Instruction RAM
    iram: [u16; IRAM_SIZE],
    /// Instruction ROM
    irom: [u16; IROM_SIZE],
    /// Data RAM
    dram: [u16; DRAM_SIZE],
    /// Data ROM
    drom: [u16; DROM_SIZE],

    dsma: u32,
    dspa: u16,
    dsbl: u16,
    dscr: u16,
    cdmb: u32,
    dcmb: u32,
}

impl Default for DspContext {
    fn default() -> Self {
        let aram = vec![0; ARAM_SIZE].into_boxed_slice();
        let iram = [0; IRAM_SIZE];
        let irom = [0; IROM_SIZE];
        let dram = [0; DRAM_SIZE];
        let drom = [0; DROM_SIZE];

        DspContext {
            aram,
            iram,
            irom,
            dram,
            drom,
            // dsp regs
            dsma: 0,
            dspa: 0,
            dsbl: 0,
            dscr: 0,
            cdmb: 0,
            dcmb: 0,
            cpu: Default::default(),
        }
    }
}

impl DspContext {
    fn load_roms(&mut self) {
        let irom_filename = "dsp_rom.bin";
        let drom_filename = "dsp_coef.bin";

        let mut irom_file = match fs::File::open(irom_filename) {
            Ok(v) => v,
            Err(e) => {
                warn!("Unable to open file {}\n{}", irom_filename, e);
                return;
            }
        };

        let mut drom_file = match fs::File::open(drom_filename) {
            Ok(v) => v,
            Err(e) => {
                warn!("Unable to open file {}\n{}", drom_filename, e);
                return;
            }
        };

        match irom_file.read_u16_into::<BigEndian>(&mut self.irom) {
            Ok(_) => {}
            Err(e) => {
                panic!("{}", e);
            }
        };

        match drom_file.read_u16_into::<BigEndian>(&mut self.drom) {
            Ok(_) => {}
            Err(e) => {
                panic!("{}", e);
            }
        };
    }

    // 0xxx IRAM
    // 8xxx IROM
    fn read_imem(&self, addr: u16) -> u16 {
        match addr >> 12 {
            0x0 => self.iram[(addr & 0x0FFF) as usize],
            0x8 => self.irom[(addr & 0x0FFF) as usize],
            _ => panic!(),
        }
    }

    // 0xxx DRAM
    // 1xxx DROM (COEF)
    // Fxxx HW regs
    fn read_dmem(&mut self, addr: u16) -> u16 {
        match addr >> 12 {
            0x0 => self.dram[(addr & 0x0FFF) as usize],
            0x1 => self.drom[(addr & 0x0FFF) as usize],
            0xF => match addr {
                DSMAH => self.dsma.hi(),
                DSMAL => self.dsma.lo(),
                DSPA => self.dspa,
                DSBL => self.dsbl,
                DSCR => self.dscr,
                CTDMBH => self.cdmb.hi(),
                CTDMBL => {
                    self.cdmb &= 0x7FFF_FFFF; // clear valid flag
                    self.cdmb.lo()
                }
                _ => unimplemented!("Unrecognized dsp register {:#x}", addr),
            },
            _ => panic!(),
        }
    }

    // 0xxx DRAM
    // 1xxx DROM (COEF)
    // Fxxx HW regs
    fn write_dmem(&mut self, addr: u16, val: u16) {
        match addr >> 12 {
            0x0 => self.dram[(addr & 0x0FFF) as usize] = val,
            0xF => match addr {
                DSMAH => self.dsma = self.dsma.set_hi(val),
                DSMAL => self.dsma = self.dsma.set_lo(val),
                DSPA => self.dspa = val,
                DSBL => self.dsbl = val,
                DSCR => self.dscr = val,
                DTCMBH => {
                    self.dcmb = self.dcmb.set_hi(val & 0x7FFF); // make sure valid flag is not set on writes
                }
                DTCMBL => {
                    self.dcmb = self.dcmb.set_lo(val);
                    self.dcmb |= 0x8000_0000; // set valid flag
                }
                _ => unimplemented!("Unrecognized HW Reg {:#x}", addr),
            },
            _ => panic!(),
        }
    }

    pub fn reset(&mut self, pc: u16) {
        self.cpu.reset(pc);
    }

    pub fn step(&mut self) {
        dsp_step(self);
    }
}

const DSMAH: u16 = 0xFFCE;
const DSMAL: u16 = 0xFFCF;
const DSPA: u16 = 0xFFCD;
const DSBL: u16 = 0xFFCB;
const DSCR: u16 = 0xFFC9;
//const ADM: u16 = 0xFFD1;
//const ACDL: u16 = 0xFFD3;
//const ACSAH: u16 = 0xFFD4;
//const ACSAL: u16 = 0xFFD5;
//const ACEAH: u16 = 0xFFD6;
//const ACEAL: u16 = 0xFFD7;
//const ACCAH: u16 = 0xFFD8;
//const ACCAL: u16 = 0xFFD9;
//const PS: u16 = 0xFFDA;
//const YN1: u16 = 0xFFDB;
//const YN2: u16 = 0xFFDC;
//const YN: u16 = 0xFFDD;
//const GAIN: u16 = 0xFFDE;
//const XN: u16 = 0xFFDF;
//const AMDM: u16 = 0xFFEF;
//const DTCCR: u16 = 0xFFFB;
const DTCMBH: u16 = 0xFFFC;
const DTCMBL: u16 = 0xFFFD;
const CTDMBH: u16 = 0xFFFE;
const CTDMBL: u16 = 0xFFFF;

// Mailboxes
// 0xFFFE CMBH - CPu Mailbox H
// 0xFFFF CMBL - CPU Mailbox L
// 0xFFFC DMBH - DSP Mailbox H
// 0xFFFD DMBL - DSP Mailbox L
//
// DMA Interface
// 0xFFCE DSMAH - Memory Address H
// 0xFFCF DSMAL - Memory address L
// 0xFFCD DSPA - DSP memory address
// 0xFFC9 DSCR - DMA Control
// 0xFFCB DSBL - Block Size
//
// Accelerator
// 0xFFD4 ACSAH - Accelerator start address H
// 0xFFD5 ACSAH - Accelerator start address L
// 0xFFD6 ACEAH - Accelerator end address H
// 0xFFD7 ACEAH - Accelerator end address L
// 0xFFD8 ACCAH - Accelerator current address H
// 0xFFD9 ACCAH - Accelerator current address L
//
// Interrupts
// 0xFFFB DCCR - DSP -> CPU control register
