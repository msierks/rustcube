use std::{
    fs,
    io::{prelude::*, Error, ErrorKind, Read, SeekFrom},
    path::Path,
};

use byteorder::{BigEndian, ByteOrder};

use crate::{
    bus::Bus,
    cpu::{registers::SPR_LR, Cpu},
};

const DISC_MAGIC: u32 = 0xC2339F3D;
const APL_INIT_OFFSET: u32 = 0x4; // AplInit
const APL_MAIN_OFFSET: u32 = 0x8; // AplMain
const APL_CLOSE_OFFSET: u32 = 0xC; // AplClose

pub struct Disc {
    file: std::fs::File,
}

#[derive(Debug)]
struct Header {
    game_code: u32,
    maker_code: u16,
    game_name: String,
    bootfile_offset: u32,
    fst_offset: u32,
    fst_size: u32,
}

impl Disc {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Disc, Error> {
        let mut buff = [0; 0x440];
        let mut file = fs::File::open(path)?;

        file.read_exact(&mut buff)?;

        let magic = BigEndian::read_u32(&buff[0x1C..]);

        if magic != DISC_MAGIC {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Not a valid gamecube image",
            ));
        }

        let game_code = BigEndian::read_u32(&buff[0x0..]);

        let maker_code = BigEndian::read_u16(&buff[0x04..]);

        let game_name = String::from_utf8_lossy(&buff[0x20..0x3FF])
            .into_owned()
            .trim_matches(char::from(0))
            .to_string();
        let bootfile_offset = BigEndian::read_u32(&buff[0x420..]);
        let fst_offset = BigEndian::read_u32(&buff[0x424..]);
        let fst_size = BigEndian::read_u32(&buff[0x428..]);

        let header = Header {
            game_code,
            maker_code,
            game_name,
            bootfile_offset,
            fst_offset,
            fst_size,
        };

        info!(
            "Reading Disc: game_code {:#x} | maker_code {:#x} | game_name {:} | bootfile_offset: {:#x} | fst_offset {:#x} | fst_size {:#x}",
            header.game_code, header.maker_code, header.game_name, header.bootfile_offset, header.fst_offset, header.fst_size
        );

        Ok(Disc { file })
    }

    /// Execute apploader
    pub fn load(&mut self, cpu: &mut Cpu, bus: &mut Bus) -> Result<(), Error> {
        // TODO: Write disk header information to 0x8000_00F4

        let mut buff = [0; 0x20];

        self.file.seek(SeekFrom::Start(0x2440))?;

        self.file.read_exact(&mut buff)?;

        let apploader_date = String::from_utf8_lossy(&buff[0x00..0x09])
            .into_owned()
            .trim_matches(char::from(0))
            .to_string();

        let apploader_entrypoint = BigEndian::read_u32(&buff[0x10..]);
        let apploader_size = BigEndian::read_u32(&buff[0x14..]);
        let trailer_size = BigEndian::read_u32(&buff[0x18..]);

        info!(
            "Apploader: date {:} | entrypoint {:#x} | size {:#x} | trailer_size: {:}",
            apploader_date, apploader_entrypoint, apploader_size, trailer_size
        );

        let mut buff = vec![0; apploader_size as usize];

        self.file.read_exact(buff.as_mut_slice())?;

        cpu.write_bytes(bus, 0x8120_0000, buff.as_slice());

        let base_addr = 0x8130_0000;

        cpu.write::<u32>(bus, base_addr, 0x4E80_0020); // Set dummy OSReport -> BLR

        cpu.gpr[3] = base_addr + APL_INIT_OFFSET;
        cpu.gpr[4] = base_addr + APL_MAIN_OFFSET;
        cpu.gpr[5] = base_addr + APL_CLOSE_OFFSET;

        info!("Call Apploader Entrypoint");
        run_function(cpu, bus, apploader_entrypoint);

        let apl_init = cpu.read::<u32>(bus, base_addr + APL_INIT_OFFSET);
        let apl_main = cpu.read::<u32>(bus, base_addr + APL_MAIN_OFFSET);
        let apl_close = cpu.read::<u32>(bus, base_addr + APL_CLOSE_OFFSET);

        info!(
            "Apploader: init {:#x} | main {:#x} | close {:#x}",
            apl_init, apl_main, apl_close
        );

        // Execute AplInit
        cpu.gpr[3] = 0x8130_0000; // OSReport callback

        info!("Call Apploader Init");
        run_function(cpu, bus, apl_init);

        cpu.gpr[3] = base_addr + APL_INIT_OFFSET;
        cpu.gpr[4] = base_addr + APL_MAIN_OFFSET;
        cpu.gpr[5] = base_addr + APL_CLOSE_OFFSET;

        info!("Call Apploader Main");
        run_function(cpu, bus, apl_main);

        // Execute AplMain
        while cpu.gpr[3] != 0 {
            let addr = cpu.read::<u32>(bus, base_addr + 0x4);
            let size = cpu.read::<u32>(bus, base_addr + 0x8) as usize;
            let offset = cpu.read::<u32>(bus, base_addr + 0xC) as u64;

            if size > 0 {
                let mut buff = vec![0; size];

                self.file.seek(SeekFrom::Start(offset))?;

                self.file.read_exact(&mut buff)?;

                cpu.write_bytes(bus, addr, buff.as_slice());

                info!(
                    "Apploader Transfer: destAddr {:#x} | size {:#x} | offset {:#x}",
                    addr, size, offset
                );
            }

            cpu.gpr[3] = base_addr + APL_INIT_OFFSET;
            cpu.gpr[4] = base_addr + APL_MAIN_OFFSET;
            cpu.gpr[5] = base_addr + APL_CLOSE_OFFSET;

            run_function(cpu, bus, apl_main);
        }

        info!("Call Apploader Close");
        run_function(cpu, bus, apl_close);

        cpu.cia = cpu.gpr[3];

        Ok(())
    }

    pub fn read_at(&mut self, offset: u64, buf: &mut [u8]) -> Result<(), Error> {
        self.file.seek(SeekFrom::Start(offset))?;
        self.file.read_exact(buf)?;
        Ok(())
    }
}

fn run_function(cpu: &mut Cpu, bus: &mut Bus, address: u32) {
    cpu.spr[SPR_LR] = 0;
    cpu.cia = address;

    while cpu.cia != 0 {
        cpu.step(bus);
    }
}
