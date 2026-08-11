use std::{cell::RefCell, rc::Rc};

use crate::{
    cpu::{l1_cache::L1Cache, CpuState},
    dsp::DspInterface,
    hw::{
        ai::AudioInterface,
        bootrom::{Bootrom, IPL_MEM_SIZE},
        di::DvdInterface,
        exi::ExternalInterface,
        gp_fifo::GpFifo,
        memory::{Memory, MEMORY_SIZE},
        mmio::Mmio,
        pe::PixelEngine,
        pi::ProcessorInterface,
        si::SerialInterface,
        vi::VideoInterface,
    },
    video::cp::CommandProcessor,
};

pub trait ReadWrite<T> {
    fn read(bus: &mut Bus, _: &mut CpuState, addr: u32) -> T;
    fn write(bus: &mut Bus, _: &mut CpuState, addr: u32, val: T);
}

pub struct Bus {
    pub(crate) bootrom: Bootrom,
    pub(crate) memory: Memory,
    pub(crate) l1_cache: L1Cache,
    pub(crate) mmio: Mmio,
    pub(crate) ai: AudioInterface,
    pub(crate) cp: CommandProcessor,
    pub(crate) di: DvdInterface,
    pub(crate) dsp: DspInterface,
    pub(crate) exi: ExternalInterface,
    pub(crate) gp_fifo: GpFifo,
    pub(crate) pi: ProcessorInterface,
    pub(crate) pe: PixelEngine,
    pub(crate) si: SerialInterface,
    pub(crate) vi: VideoInterface,
}

impl Default for Bus {
    fn default() -> Self {
        let bootrom_data = Rc::new(RefCell::new(vec![0; IPL_MEM_SIZE]));
        let bootrom = Bootrom::new(bootrom_data.clone());
        let exi = ExternalInterface::new(bootrom_data.clone());

        let mut mmio = Mmio::default();

        mmio.register_device::<AudioInterface>();
        mmio.register_device::<CommandProcessor>();
        mmio.register_device::<DspInterface>();
        mmio.register_device::<DvdInterface>();
        mmio.register_device::<PixelEngine>();
        mmio.register_device::<ProcessorInterface>();
        mmio.register_device::<ExternalInterface>();
        mmio.register_device::<GpFifo>();
        mmio.register_device::<VideoInterface>();
        mmio.register_device::<SerialInterface>();

        Bus {
            bootrom,
            memory: Default::default(),
            l1_cache: Default::default(),
            mmio,
            ai: Default::default(),
            cp: Default::default(),
            di: Default::default(),
            dsp: Default::default(),
            exi,
            gp_fifo: Default::default(),
            pe: Default::default(),
            pi: Default::default(),
            si: Default::default(),
            vi: Default::default(),
        }
    }
}

impl Bus {
    pub fn read<T>(&mut self, cpu_state: &mut CpuState, addr: u32) -> T
    where
        Mmio: ReadWrite<T>,
        Memory: ReadWrite<T>,
        L1Cache: ReadWrite<T>,
        Bootrom: ReadWrite<T>,
    {
        if addr < MEMORY_SIZE {
            Memory::read(self, cpu_state, addr)
        } else if L1Cache::contains(addr) {
            L1Cache::read(self, cpu_state, addr)
        } else if addr < Bootrom::BASE_ADDR {
            Mmio::read(self, cpu_state, addr)
        } else {
            Bootrom::read(self, cpu_state, addr)
        }
    }

    pub fn write<T>(&mut self, cpu_state: &mut CpuState, addr: u32, val: T)
    where
        Mmio: ReadWrite<T>,
        Memory: ReadWrite<T>,
        L1Cache: ReadWrite<T>,
    {
        if addr < MEMORY_SIZE {
            Memory::write(self, cpu_state, addr, val)
        } else if L1Cache::contains(addr) {
            L1Cache::write(self, cpu_state, addr, val)
        } else if addr < Bootrom::BASE_ADDR {
            Mmio::write(self, cpu_state, addr, val)
        } else {
            panic!("Unhandled Physical Address: {:#010x}", addr);
        }
    }

    pub fn write_bytes(&mut self, _: &mut CpuState, addr: u32, data: &[u8]) {
        if addr < MEMORY_SIZE {
            self.memory.write_bytes(addr, data);
        } else if L1Cache::contains(addr) {
            self.l1_cache.write_bytes(addr, data);
        } else {
            panic!("Unhandled Physical Address: {:#010x}", addr);
        }
    }
}
