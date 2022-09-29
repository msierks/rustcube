use byteorder::{BigEndian, ByteOrder};
use std::fs;
use std::io::prelude::*;
use std::io::{Error, Read, SeekFrom};
use std::path::Path;

use super::Context;

const NUM_TEXT: usize = 7;
const NUM_DATA: usize = 11;

#[derive(Default, Debug)]
struct Header {
    text_offset: [u32; NUM_TEXT],
    data_offset: [u32; NUM_DATA],
    text_address: [u32; NUM_TEXT],
    data_address: [u32; NUM_DATA],
    text_size: [u32; NUM_TEXT],
    data_size: [u32; NUM_DATA],
    bss_address: u32,
    bss_size: u32,
    entry_point: u32,
}

#[derive(Debug)]
pub struct Dol {
    header: Header,
    text_sections: Vec<Vec<u8>>,
    data_sections: Vec<Vec<u8>>,
}

impl Dol {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Dol, Error> {
        let mut buff = [0; 0xE4];
        let mut file = fs::File::open(path)?;

        file.read_exact(&mut buff)?;

        let mut offset;
        let mut text_offset = [0; NUM_TEXT];
        let mut text_address = [0; NUM_TEXT];
        let mut text_size = [0; NUM_TEXT];
        let mut data_offset = [0; NUM_DATA];
        let mut data_address = [0; NUM_DATA];
        let mut data_size = [0; NUM_DATA];
        let mut text_sections = Vec::with_capacity(NUM_TEXT);
        let mut data_sections = Vec::with_capacity(NUM_DATA);

        for x in 0..NUM_TEXT {
            offset = x * 4;
            text_offset[x] = BigEndian::read_u32(&buff[offset..]);
            text_address[x] = BigEndian::read_u32(&buff[0x48 + offset..]);
            text_size[x] = BigEndian::read_u32(&buff[0x90 + offset..]);

            if text_offset[x] > 0 {
                let mut section = vec![0; text_size[x] as usize];
                file.seek(SeekFrom::Start(u64::from(text_offset[x])))?;
                file.read_exact(section.as_mut_slice())?;
                text_sections.push(section);
            } else {
                break;
            }
        }

        for x in 0..NUM_DATA {
            offset = x * 4;
            data_offset[x] = BigEndian::read_u32(&buff[0x1C + offset..]);
            data_address[x] = BigEndian::read_u32(&buff[0x64 + offset..]);
            data_size[x] = BigEndian::read_u32(&buff[0xAC + offset..]);

            if data_offset[x] > 0 {
                let mut section = vec![0; data_size[x] as usize];
                file.seek(SeekFrom::Start(u64::from(data_offset[x])))?;
                file.read_exact(section.as_mut_slice())?;
                data_sections.push(section);
            } else {
                break;
            }
        }

        let header = Header {
            text_offset,
            data_offset,
            text_address,
            data_address,
            text_size,
            data_size,
            bss_address: BigEndian::read_u32(&buff[0xD8..]),
            bss_size: BigEndian::read_u32(&buff[0xDC..]),
            entry_point: BigEndian::read_u32(&buff[0xE0..]),
        };

        Ok(Dol {
            header,
            text_sections,
            data_sections,
        })
    }

    pub fn get_entry_point(&self) -> u32 {
        self.header.entry_point
    }

    pub fn load(&self, ctx: &mut Context) {
        for (x, section) in self.text_sections.iter().enumerate() {
            ctx.write(self.header.text_address[x], section.as_slice());
        }

        for (x, section) in self.data_sections.iter().enumerate() {
            ctx.write(self.header.data_address[x], section.as_slice());
        }
    }
}
