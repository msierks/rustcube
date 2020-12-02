use crate::emu::Context;
use gdbstub::target::TargetResult;

use gdbstub::arch::{Arch, RegId, Registers};
use gdbstub::target;
use gdbstub::target::ext::breakpoints::WatchKind;
use gdbstub::target::Target;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct PowerPc750Regs {
    pub gpr: [u32; 32],
    pub fpr: [f64; 32],
    pub pc: u32,
    pub msr: u32,
    pub cr: u32,
    pub lr: u32,
    pub ctr: u32,
    pub xer: u32,
    pub fpscr: u32,
    pub sr: [u32; 16],
    pub pvr: u32,
    pub bat: [u32; 16],
    pub sdr1: u32,
    pub asr: u64,
    pub dar: u32,
    pub dsisr: u32,
    pub sprg: [u32; 4],
    pub srr0: u32,
    pub srr1: u32,
    pub tbl: u32,
    pub tbu: u32,
    pub dec: u32,
    pub dabr: u32,
    pub ear: u32,
    pub hid0: u32,
    pub hid1: u32,
    pub iabr: u32,
    pub ummcr0: u32,
    pub upmc1: u32,
    pub upmc2: u32,
    pub usia: u32,
    pub ummcr1: u32,
    pub upmc3: u32,
    pub upmc4: u32,
    pub mmcr0: u32,
    pub pmc1: u32,
    pub pmc2: u32,
    pub sia: u32,
    pub mmcr1: u32,
    pub pmc3: u32,
    pub pmc4: u32,
    pub l2cr: u32,
    pub ictc: u32,
    pub thrm1: u32,
    pub thrm2: u32,
    pub thrm3: u32,
}

impl Registers for PowerPc750Regs {
    fn gdb_serialize(&self, mut write_byte: impl FnMut(Option<u8>)) {
        macro_rules! write_bytes {
            ($bytes:expr) => {
                for b in $bytes {
                    write_byte(Some(*b))
                }
            };
        }

        macro_rules! write_regs {
            ($($reg:ident),*) => {
                $(
                    write_bytes!(&self.$reg.to_be_bytes());
                )*
            }
        }

        for reg in &self.gpr {
            write_bytes!(&reg.to_be_bytes());
        }

        for reg in &self.fpr {
            write_bytes!(&reg.to_be_bytes());
        }

        write_regs!(pc, msr, cr, lr, ctr, xer, fpscr);

        for reg in &self.sr {
            write_bytes!(&reg.to_be_bytes());
        }

        write_regs!(pvr);

        for reg in &self.bat {
            write_bytes!(&reg.to_be_bytes());
        }

        write_regs!(sdr1, asr, dar, dsisr);

        for reg in &self.sprg {
            write_bytes!(&reg.to_be_bytes());
        }

        write_regs!(
            srr0, srr1, tbl, tbu, dec, dabr, ear, hid0, hid1, iabr, dabr, ummcr0, upmc1, upmc2,
            usia, ummcr1, upmc3, upmc4, mmcr0, pmc1, pmc2, sia, mmcr1, pmc3, pmc4, l2cr, ictc,
            thrm1, thrm2, thrm3
        );
    }

    fn gdb_deserialize(&mut self, _bytes: &[u8]) -> Result<(), ()> {
        unimplemented!();
    }
}

pub enum PowerPc750<RegIdImpl: RegId> {
    #[doc(hidden)]
    _Marker(core::marker::PhantomData<RegIdImpl>),
}

impl<RegIdImpl: RegId> Arch for PowerPc750<RegIdImpl> {
    type Usize = u32;
    type Registers = PowerPc750Regs;
    type RegId = RegIdImpl;

    fn target_description_xml() -> Option<&'static str> {
        Some(r#"<target version="1.0"><architecture>powerpc:750</architecture></target>"#)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum PowerPcCoreRegId {
    Gpr(usize),
    Fpr(usize),
    Pc,
    Msr,
    Cr,
    Lr,
    Ctr,
    Xer,
    Fpscr,
    Sr(usize),
    Pvr,
    Ibat0u,
    Ibat0l,
    Ibat1u,
    Ibat1l,
    Ibat2u,
    Ibat2l,
    Ibat3u,
    Ibat3l,
    Dbat0u,
    Dbat0l,
    Dbat1u,
    Dbat1l,
    Dbat2u,
    Dbat2l,
    Dbat3u,
    Dbat3l,
    Sdr1,
    Asr,
    Dar,
    Dsisr,
    Sprg(usize),
    Srr0,
    Srr1,
    Tbl,
    Tbu,
    Dec,
    Dabr,
    Ear,
    Hid0,
    Hid1,
    Iabr,
    Ummcr0,
    Upmc1,
    Upmc2,
    Usia,
    Ummcr1,
    Upmc3,
    Upmc4,
    Mmcr0,
    Pmc1,
    Pmc2,
    Sia,
    Mmcr1,
    Pmc3,
    Pmc4,
    L2cr,
    Ictc,
    Thrm1,
    Thrm2,
    Thrm3,
}

// https://github.com/bminor/binutils-gdb/blob/master/gdb/features/rs6000/powerpc-750.xml
impl RegId for PowerPcCoreRegId {
    fn from_raw_id(id: usize) -> Option<(Self, usize)> {
        let reg = match id {
            0..=31 => Self::Gpr(id),
            32..=63 => Self::Fpr(id - 32),
            64 => Self::Pc,
            65 => Self::Msr,
            66 => Self::Cr,
            67 => Self::Lr,
            68 => Self::Ctr,
            69 => Self::Xer,
            70 => Self::Fpscr,
            71..=86 => Self::Sr(id - 71),
            87 => Self::Pvr,
            88 => Self::Ibat0u,
            89 => Self::Ibat0l,
            90 => Self::Ibat1u,
            91 => Self::Ibat1l,
            92 => Self::Ibat2u,
            93 => Self::Ibat2l,
            94 => Self::Ibat3u,
            95 => Self::Ibat3l,
            96 => Self::Dbat0u,
            97 => Self::Dbat0l,
            98 => Self::Dbat1u,
            99 => Self::Dbat1l,
            100 => Self::Dbat2u,
            101 => Self::Dbat2l,
            102 => Self::Dbat3u,
            103 => Self::Dbat3l,
            104 => Self::Sdr1,
            105 => Self::Asr,
            106 => Self::Dar,
            107 => Self::Dsisr,
            108..=111 => Self::Sprg(id - 108),
            112 => Self::Srr0,
            113 => Self::Srr1,
            114 => Self::Tbl,
            115 => Self::Tbu,
            116 => Self::Dec,
            117 => Self::Dabr,
            118 => Self::Ear,
            119 => Self::Hid0,
            120 => Self::Hid1,
            121 => Self::Iabr,
            122 => Self::Dabr,
            124 => Self::Ummcr0,
            125 => Self::Upmc1,
            126 => Self::Upmc2,
            127 => Self::Usia,
            128 => Self::Ummcr1,
            129 => Self::Upmc3,
            130 => Self::Upmc4,
            131 => Self::Mmcr0,
            132 => Self::Pmc1,
            133 => Self::Pmc2,
            134 => Self::Sia,
            135 => Self::Mmcr1,
            136 => Self::Pmc3,
            137 => Self::Pmc4,
            138 => Self::L2cr,
            139 => Self::Ictc,
            140 => Self::Thrm1,
            141 => Self::Thrm2,
            142 => Self::Thrm3,
            _ => return None,
        };

        match reg {
            Self::Fpr(_) | Self::Asr => Some((reg, 8)),
            _ => Some((reg, 4)),
        }
    }
}

impl Target for Context {
    type Arch = PowerPc750<PowerPcCoreRegId>;
    type Error = &'static str;

    fn base_ops(&mut self) -> target::ext::base::BaseOps<Self::Arch, Self::Error> {
        target::ext::base::BaseOps::SingleThread(self)
    }

    fn sw_breakpoint(&mut self) -> Option<target::ext::breakpoints::SwBreakpointOps<Self>> {
        Some(self)
    }

    fn hw_watchpoint(&mut self) -> Option<target::ext::breakpoints::HwWatchpointOps<Self>> {
        Some(self)
    }
}

impl target::ext::breakpoints::SwBreakpoint for Context {
    fn add_sw_breakpoint(&mut self, addr: u32) -> TargetResult<bool, Self> {
        self.breakpoints.push(addr);
        Ok(true)
    }

    fn remove_sw_breakpoint(&mut self, addr: u32) -> TargetResult<bool, Self> {
        match self.breakpoints.iter().position(|x| *x == addr) {
            None => return Ok(false),
            Some(pos) => self.breakpoints.remove(pos),
        };
        Ok(true)
    }
}

impl target::ext::breakpoints::HwWatchpoint for Context {
    fn add_hw_watchpoint(&mut self, addr: u32, kind: WatchKind) -> TargetResult<bool, Self> {
        match kind {
            WatchKind::Write => self.watchpoints.push(addr),
            WatchKind::Read => self.watchpoints.push(addr),
            WatchKind::ReadWrite => self.watchpoints.push(addr),
        };

        Ok(true)
    }

    fn remove_hw_watchpoint(&mut self, addr: u32, _kind: WatchKind) -> TargetResult<bool, Self> {
        match self.watchpoints.iter().position(|x| *x == addr) {
            None => return Ok(false),
            Some(pos) => self.watchpoints.remove(pos),
        };

        Ok(true)
    }
}
