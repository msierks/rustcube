
extern crate getopts;
extern crate rustcube;

use rustcube::gamecube::Gamecube;
use rustcube::debugger::{ConsoleDebugger,DummyDebugger};

use std::env;
use std::path::Path;
use getopts::Options;

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [options] IPL_FILE", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("d", "", "enable debug console");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, &opts);
        return;
    }

    let file_name = if !matches.free.is_empty() {
        Path::new(matches.free[0].as_str())
    } else {
        print_usage(&program, &opts);
        return;
    };

    let mut gamecube = Gamecube::new();

    match file_name.extension() {
        Some(ext) => {
            if ext == "dol" {
                gamecube.load_dol(file_name);
            } else { // assume ipl
                gamecube.load_ipl(file_name);
            }
        },
        None => gamecube.load_ipl(file_name)
    }

    if matches.opt_present("d") {
        gamecube.run(&mut ConsoleDebugger::new());
    } else {
        gamecube.run(&mut DummyDebugger::new());
    }
}
