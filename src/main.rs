extern crate byteorder;
extern crate getopts;
extern crate memmap;
extern crate num;

#[macro_use]
extern crate enum_primitive;

mod audio_interface;
mod cpu;
mod debugger;
mod dsp_interface;
mod exi;
mod gamecube;
mod memory;
mod memory_interface;
mod processor_interface;
mod serial_interface;

use getopts::Options;
use std::env;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] IPL_FILE", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("d", "", "enable gdb server", "127.0.0.1:9001");
    opts.optflag("h", "help", "print this help menu");


    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let ipl_file_name = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    let mut gamecube = gamecube::Gamecube::new();

    match matches.opt_str("d") {
        Some(v) => gamecube.enable_debugger(v),
        None => {}
    }

    gamecube.load_ipl(ipl_file_name);

    loop {
        //println!("{:?}", gamecube);
        gamecube.run_instruction();
    }
}
