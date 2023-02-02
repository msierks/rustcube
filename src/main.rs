extern crate getopts;
extern crate rustcube;

use rustcube::Context;

use env_logger::Env;
use getopts::Options;
use std::env;
use std::path::Path;

pub type DynResult<T> = Result<T, Box<dyn std::error::Error>>;

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {program} [options] IPL_FILE");
    print!("{}", opts.usage(&brief));
}

fn main() -> DynResult<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!("{}", f.to_string()),
    };

    if matches.opt_present("h") {
        print_usage(&program, &opts);
        return Ok(());
    }

    let file_name = if !matches.free.is_empty() {
        Path::new(matches.free[0].as_str())
    } else {
        print_usage(&program, &opts);
        return Ok(());
    };

    let mut ctx = Context::default();

    match file_name.extension() {
        Some(ext) => {
            if ext == "dol" {
                ctx.load_dol(file_name);
            } else {
                // assume ipl
                ctx.load_ipl(file_name);
            }
        }
        None => ctx.load_ipl(file_name),
    }

    loop {
        ctx.step();
    }
}
