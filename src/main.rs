extern crate byteorder;
extern crate memmap;
extern crate num;

#[macro_use]
extern crate enum_primitive;

mod cpu;
mod dsp_interface;
mod exi;
mod gamecube;
mod memory;
mod memory_interface;
mod processor_interface;
mod serial_interface;

use std::env;

fn main() {
    let ipl_file_name = match env::args().nth(1) {
        Some(v) => v,
        None => {
            println!("missing ipl.bin file name argument");
            std::process::exit(1)
        }
    };

    let mut gamecube = gamecube::Gamecube::new();

    gamecube.load_ipl(ipl_file_name);

    loop {
        //println!("{:?}", gamecube);
        gamecube.run_instruction();
    }
}
