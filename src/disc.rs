use byteorder::{BigEndian, ByteOrder};
use std::fs;
use std::io::prelude::*;
use std::io::{Error, ErrorKind, Read, SeekFrom};
use std::path::Path;

use crate::cpu::{step, SPR_LR};
use crate::Context;

const DISC_MAGIC: u32 = 0xC2339F3D;

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
    pub fn load(&mut self, ctx: &mut Context) -> Result<(), Error> {
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

        ctx.write(0x8120_0000, buff.as_slice());

        let base_addr = 0x8130_0000;

        ctx.write_u32(base_addr, 0x4E80_0020); // Set dummy OSReport -> BLR

        ctx.cpu.set_pc(apploader_entrypoint);

        ctx.cpu.mut_gpr()[3] = base_addr + 0x4; // AplInit
        ctx.cpu.mut_gpr()[4] = base_addr + 0x8; // AplMain
        ctx.cpu.mut_gpr()[5] = base_addr + 0xC; // AplClose

        ctx.cpu.mut_spr()[SPR_LR] = 0;

        while ctx.cpu.pc() != 0 {
            step(ctx);
        }

        let apl_init = ctx.read_u32(base_addr + 0x4);
        let apl_main = ctx.read_u32(base_addr + 0x8);
        let apl_close = ctx.read_u32(base_addr + 0xC);

        info!(
            "Apploader: init {:#x} | main {:#x} | close {:#x}",
            apl_init, apl_main, apl_close
        );

        // Execute AplInit
        ctx.cpu.set_pc(apl_init);
        ctx.cpu.mut_gpr()[3] = 0x8130_0000; // OSReport callback
        ctx.cpu.mut_spr()[SPR_LR] = 0;

        while ctx.cpu.pc() != 0 {
            step(ctx);
        }

        // Execute AplMain
        while ctx.cpu.gpr()[3] != 0 {
            ctx.cpu.mut_gpr()[3] = base_addr + 0x4; // AplInit
            ctx.cpu.mut_gpr()[4] = base_addr + 0x8; // AplMain
            ctx.cpu.mut_gpr()[5] = base_addr + 0xC; // AplClose

            ctx.cpu.mut_spr()[SPR_LR] = 0;
            ctx.cpu.set_pc(apl_main);
            while ctx.cpu.pc() != 0 {
                step(ctx);
            }

            let addr = ctx.read_u32(base_addr + 0x4);
            let size = ctx.read_u32(base_addr + 0x8) as usize;
            let offset = ctx.read_u32(base_addr + 0xC) as u64;

            if size > 0 {
                let mut buff = vec![0; size];

                self.file.seek(SeekFrom::Start(offset))?;

                self.file.read_exact(&mut buff)?;

                ctx.write(addr, buff.as_slice());

                info!(
                    "Apploader Transfer: destAddr {:#x} | size {:#x} | offset {:#x}",
                    addr, size, offset
                );
            }
        }

        // Execute AplClose
        ctx.cpu.mut_spr()[SPR_LR] = 0;
        ctx.cpu.set_pc(apl_close);
        while ctx.cpu.pc() != 0 {
            step(ctx);
        }

        ctx.cpu.set_pc(ctx.cpu.gpr()[3]);

        Ok(())
    }
}
