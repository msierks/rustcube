extern crate getopts;
extern crate rustcube;

use rustcube::emu::Context;

use env_logger::Env;
use getopts::Options;
use std::env;
use std::path::Path;

#[cfg(feature = "gdb")]
use std::net::{TcpListener, TcpStream};

#[cfg(feature = "gdb")]
use gdbstub::{Connection, DisconnectReason, GdbStub};

pub type DynResult<T> = Result<T, Box<dyn std::error::Error>>;

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [options] IPL_FILE", program);
    print!("{}", opts.usage(&brief));
}

fn main() -> DynResult<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("g", "", "enable gdb server");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
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

    if matches.opt_present("g") {
        #[cfg(feature = "gdb")]
        let connection: Box<dyn Connection<Error = std::io::Error>> = Box::new(wait_for_tcp(9001)?);

        // hook-up debugger
        #[cfg(feature = "gdb")]
        let mut debugger = GdbStub::new(connection);

        #[cfg(feature = "gdb")]
        match debugger.run(&mut ctx)? {
            DisconnectReason::Disconnect => {
                // run to completion
                loop {
                    ctx.step();
                }
            }
            DisconnectReason::TargetHalted => println!("Target halted!"),
            DisconnectReason::Kill => {
                println!("GDB sent a kill command!");
                return Ok(());
            }
        }
    } else {
        loop {
            ctx.step();
        }
    }

    Ok(())
}

#[cfg(feature = "gdb")]
fn wait_for_tcp(port: u16) -> DynResult<TcpStream> {
    let sockaddr = format!("127.1:{}", port);
    eprintln!("Waiting for a GDB connection on {:?}...", sockaddr);

    let sock = TcpListener::bind(sockaddr)?;
    let (stream, addr) = sock.accept()?;
    eprintln!("Debugger connected from {}", addr);

    Ok(stream)
}
