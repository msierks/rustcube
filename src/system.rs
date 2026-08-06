use std::path::Path;

use crate::{
    bus::Bus,
    cpu::Cpu,
    disc::Disc,
    dol::Dol,
    dsp::DspInterface,
    hw::{ai::AudioInterface, vi::VideoInterface},
};

#[derive(Default)]
pub struct System {
    cpu: Cpu,
    bus: Bus,
}

impl System {
    pub fn load_dol<P: AsRef<Path>>(&mut self, path: P) {
        let dol = Dol::open(path).unwrap();

        self.cpu.emulate_bs2(&mut self.bus);

        dol.load(&mut self.cpu, &mut self.bus);

        self.cpu.cia = dol.get_entry_point();
    }

    pub fn load_iso<P: AsRef<Path>>(&mut self, path: P) {
        let mut disc = Disc::open(path).unwrap();

        self.cpu.emulate_bs2(&mut self.bus);

        disc.load(&mut self.cpu, &mut self.bus).unwrap(); // fix this and don't be lazy

        self.bus.di.set_disc(Some(disc));
    }

    pub fn load_ipl<P: AsRef<Path>>(&mut self, path: P) {
        self.bus.bootrom.load_ipl(path);
    }

    pub fn step(&mut self) {
        VideoInterface::update(&mut self.bus, &mut self.cpu.state);
        DspInterface::update(&mut self.bus, &mut self.cpu.state);
        AudioInterface::update(&mut self.bus, &mut self.cpu.state);

        self.cpu.step(&mut self.bus);
    }
}
