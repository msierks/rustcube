extern crate byteorder;
extern crate memmap;

use std::env;

mod cpu;
mod gamecube;
mod memory;

fn main() {
    let ipl_file_name = match env::args().nth(1) {
        Some(v) => v,
        None => {
            println!("missing bs2(ipl.bin) file name argument");
            std::process::exit(1)
        }
    };

    let mut gamecube = gamecube::Gamecube::new();

    gamecube.bootstrap(ipl_file_name);

    loop {
        //println!("{:?}", gamecube);
        gamecube.run_instruction();
    }
}
