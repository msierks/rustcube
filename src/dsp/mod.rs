mod cpu;

use crate::pi::{clear_interrupt, set_interrupt, PI_INTERRUPT_DSP};
use crate::utils::Halveable;
use crate::Context;

use byteorder::{BigEndian, ReadBytesExt};
use std::fs;

use crate::dsp::cpu::{dsp_step, DspCpu};

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
                               //const AI_DMA_BYTES_LEFT: u32 = 0x3A;

//const AI_DMA_INT: u32 = 0x0;
//const ARAM_DMA_INT: u32 = 0x0;
//const DSP_INT: u32 = 0x0;

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
            cpu_ticks: 0,
            ctx,
        }
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

pub fn read_u16(ctx: &mut Context, register: u32) -> u16 {
    match register {
        DSP_CTDMBH => ctx.dsp.ctx.cdmb.hi(),
        DSP_CTDMBL => ctx.dsp.ctx.cdmb.lo(),
        DSP_DTCMBH => ctx.dsp.ctx.dcmb.hi(),
        DSP_DTCMBL => {
            let val = ctx.dsp.ctx.dcmb.lo();
            ctx.dsp.ctx.dcmb &= 0x7FFF_FFFF; // Clear valid flag
            val
        }
        DSP_CTDCR => ctx.dsp.control_register.into(),
        DSP_ARAMC => ctx.dsp.aram_conf.0,
        DSP_ARAMSF => ctx.dsp.aram_state,
        DSP_ARAMCT => ctx.dsp.aram_refresh.0,
        ARAM_DMA_MMAADDR_HI => ctx.dsp.aram_mma_addr.hi(),
        ARAM_DMA_MMAADDR_LO => ctx.dsp.aram_mma_addr.lo(),
        ARAM_DMA_ARADDR_HI => ctx.dsp.aram_ar_addr.hi(),
        ARAM_DMA_ARADDR_LO => ctx.dsp.aram_ar_addr.lo(),
        ARAM_DMA_SIZE_HI => ctx.dsp.aram_dma_size.hi(),
        ARAM_DMA_SIZE_LO => ctx.dsp.aram_dma_size.lo(),
        DSP_AIDMAMAH => ctx.dsp.aidma.hi(),
        DSP_AIDMAMAL => ctx.dsp.aidma.lo(),
        DSP_AIDMABL => ctx.dsp.aidmabl,
        _ => {
            panic!("read_u16 unrecognized dsp register {:#x}", register);
        }
    }
}

pub fn write_u16(ctx: &mut Context, register: u32, val: u16) {
    match register {
        DSP_CTDMBH => ctx.dsp.ctx.cdmb = ctx.dsp.ctx.cdmb.set_hi(val & 0x7FFF), // Clear valid flag
        DSP_CTDMBL => ctx.dsp.ctx.cdmb = ctx.dsp.ctx.cdmb.set_lo(val) | 0x8000_0000, // Set valid flag
        DSP_CTDCR => {
            let tmp = ControlRegister(val);

            if tmp.reset() {
                info!("DSP reset");
                if tmp.dsp_init() {
                    ctx.dsp.ctx.reset(0x8000);
                } else {
                    ctx.dsp.ctx.reset(0x0000);
                }
            }

            ctx.dsp.control_register.set_interrupt(tmp.interrupt());
            ctx.dsp.control_register.set_halt(tmp.halt());
            ctx.dsp.control_register.set_init_code(tmp.init_code());
            ctx.dsp.control_register.set_dsp_init(tmp.dsp_init());

            ctx.dsp
                .control_register
                .set_ai_interrupt_mask(tmp.ai_interrupt_mask());
            ctx.dsp
                .control_register
                .set_aram_interrupt_mask(tmp.aram_interrupt_mask());
            ctx.dsp
                .control_register
                .set_dsp_interrupt_mask(tmp.dsp_interrupt_mask());

            if tmp.ai_interrupt() {
                ctx.dsp.control_register.set_ai_interrupt(false);
            }
            if tmp.aram_interrupt() {
                ctx.dsp.control_register.set_aram_interrupt(false);
            }
            if tmp.dsp_interrupt() {
                ctx.dsp.control_register.set_dsp_interrupt(false);
            }

            update_interrupt(ctx);
        }
        DSP_ARAMC => ctx.dsp.aram_conf = AramConfigRegister(val & 0x7F),
        DSP_ARAMCT => ctx.dsp.aram_refresh = AramControlTestRegister(val & 0x7FF),
        ARAM_DMA_MMAADDR_HI => ctx.dsp.aram_mma_addr = ctx.dsp.aram_mma_addr.set_hi(val & 0x3FF),
        ARAM_DMA_MMAADDR_LO => ctx.dsp.aram_mma_addr = ctx.dsp.aram_mma_addr.set_lo(val & 0xFFF0),
        ARAM_DMA_ARADDR_HI => ctx.dsp.aram_ar_addr = ctx.dsp.aram_ar_addr.set_hi(val & 0x3FF),
        ARAM_DMA_ARADDR_LO => ctx.dsp.aram_ar_addr = ctx.dsp.aram_ar_addr.set_lo(val & 0xFFE0),
        ARAM_DMA_SIZE_HI => ctx.dsp.aram_dma_size = ctx.dsp.aram_dma_size.set_hi(val & 0x83FF),
        ARAM_DMA_SIZE_LO => {
            ctx.dsp.aram_dma_size = ctx.dsp.aram_dma_size.set_lo(val & 0xFFE0);

            aram_dma(ctx);

            generate_interrupt(ctx, 0x20);
        }
        DSP_AIDMAMAH => ctx.dsp.aidma = ctx.dsp.aidma.set_hi(val & 0x3FF),
        DSP_AIDMAMAL => ctx.dsp.aidma = ctx.dsp.aidma.set_lo(val & 0xFFE0),
        DSP_AIDMABL => ctx.dsp.aidmabl = val & 0x7FFF,
        _ => panic!(
            "write_u16 unrecognized dsp register {:#x} {:#x}",
            register, val
        ),
    }
}

fn generate_interrupt(ctx: &mut Context, interrupt: u16) {
    ctx.dsp.control_register = ControlRegister(ctx.dsp.control_register.0 | (interrupt));

    update_interrupt(ctx);
}

pub fn update_interrupt(ctx: &mut Context) {
    let control = ctx.dsp.control_register.0;

    if ((control >> 1) & control) & 0xA8 != 0 {
        set_interrupt(ctx, PI_INTERRUPT_DSP);
    } else {
        clear_interrupt(ctx, PI_INTERRUPT_DSP);
    }
}

fn aram_dma(ctx: &mut Context) {
    let mut cnt = ctx.dsp.aram_dma_size & 0x3FF_FFE0;
    let dir = (ctx.dsp.aram_dma_size & 0x80000) != 0; // 0: main memory -> ARAM, 1: ARAM -> main memory

    if !dir {
        info!(
            "DMA from Main Memory {:#010x} to ARAM {:#010x} ({:#x}) PC: {:#x}",
            ctx.dsp.aram_mma_addr,
            ctx.dsp.aram_ar_addr,
            cnt,
            ctx.cpu.pc()
        );
    } else {
        info!(
            "DMA from ARAM {:#010x} to Main Memory{:#010x} ({:#x}) PC: {:#x}",
            ctx.dsp.aram_ar_addr,
            ctx.dsp.aram_mma_addr,
            cnt,
            ctx.cpu.pc()
        );
    }

    if ctx.dsp.aram_ar_addr >= ARAM_SIZE as u32 {
        return;
    }

    if cnt != 0 {
        while cnt != 0 {
            // MM -> ARAM
            if !dir {
                // go to iram
                // NOTE: DSP IRAM is mapped to first 8kB of ARAM, therefore cpu can directly DMA DSP code to DSP IRAM
                if ctx.dsp.aram_ar_addr < 0x2000 {
                    let data = ctx.mem.read_u16(ctx.dsp.aram_mma_addr);

                    ctx.dsp.ctx.iram[ctx.dsp.aram_ar_addr as usize] = data;
                // go to aram
                } else {
                    ctx.dsp.ctx.aram[ctx.dsp.aram_ar_addr as usize] =
                        ctx.mem.read_u8(ctx.dsp.aram_mma_addr);
                }
            // ARAM -> MM
            } else {
                unimplemented!("DSP ARAM -> MM");
            }

            ctx.dsp.aram_mma_addr += 8;
            ctx.dsp.aram_ar_addr += 8;
            cnt -= 8;
        }

        ctx.dsp.aram_dma_size &= 0x8000_0000; // clear count
    }
}

pub fn update(ctx: &mut Context) {
    let ticks = ctx.get_ticks();
    if ticks - ctx.dsp.cpu_ticks > TIMER_RATIO {
        ctx.dsp.cpu_ticks = ticks;
    } else {
        return;
    }

    ctx.dsp.ctx.step();
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
                panic!("Unable to open file {}\n{}", irom_filename, e);
            }
        };

        let mut drom_file = match fs::File::open(drom_filename) {
            Ok(v) => v,
            Err(e) => {
                panic!("Unable to open file {}\n{}", drom_filename, e);
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
