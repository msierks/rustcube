
extern crate getopts;
extern crate rustcube;

use rustcube::gamecube::Gamecube;
use rustcube::debugger::Debugger;

use std::env;
use getopts::Options;

fn print_usage(program: &str, opts: Options) {
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
        print_usage(&program, opts);
        return;
    }

    let ipl_file_name = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    let mut gamecube = Gamecube::new();

    gamecube.load_ipl(ipl_file_name);

    if matches.opt_present("d") {
        let mut debugger = Debugger::new(gamecube);

        debugger.run();
    } else {
        gamecube.run();
    }
}
