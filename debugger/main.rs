#[macro_use]
extern crate log;

mod app;
mod background;
mod gobject;

use self::app::App;
use self::background::BgEvent;
use env_logger::Env;
use getopts::Options;
use gtk::glib;
use gtk::prelude::*;
use rustcube::cpu::disassembler::DecodedInstruction;
use rustcube::cpu::{NUM_FPR, NUM_GPR};
use std::env;
use std::path::Path;

const APP_ID: &str = "com.rustcube-debugger";

pub type DynResult<T> = Result<T, Box<dyn std::error::Error>>;

pub struct Registers {
    gpr: [u32; NUM_GPR],
    fpr: [rustcube::cpu::Fpr; NUM_FPR],
    spr_32: [(&'static str, u32); 48],
    spr_64: [(&'static str, u64); 10],
}

impl Default for Registers {
    fn default() -> Self {
        Registers {
            gpr: Default::default(),
            fpr: Default::default(),
            spr_32: [("", 0); 48],
            spr_64: [("", 0); 10],
        }
    }
}

pub struct Disassembly {
    pc: u32,
    instructions: Vec<DecodedInstruction>,
    breakpoints: Vec<u32>,
}

pub struct Callstack {
    addresses: Vec<u32>,
}

pub struct Memory {
    data: Vec<(u32, [u8; 16])>,
}

pub enum BreakpointType {
    Break,
    Watch,
}

enum BreakpointAccessType {
    Read,
    Write,
    ReadWrite,
}

pub struct Breakpoint {
    type_: BreakpointType,
    num: u32,
    start_address: u32,
    end_address: u32,
    access_type: BreakpointAccessType,
}

pub struct Breakpoints {
    breakpoints: Vec<Breakpoint>,
}

pub enum Event {
    Breakpoints(Box<Breakpoints>),
    Callstack(Box<Callstack>),
    Closed,
    Disassembly(Box<Disassembly>),
    Registers(Box<Registers>),
    Memory(Box<Memory>),
    Paused,
}

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {program} [options] IPL_FILE");
    print!("{}", opts.usage(&brief));
}

fn load_css() {
    let provider = gtk::CssProvider::new();

    provider.load_from_data(include_bytes!("style.css"));

    gtk::StyleContext::add_provider_for_display(
        &gtk::gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
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
        matches.free[0].clone()
    } else {
        print_usage(&program, &opts);
        return Ok(());
    };

    let app = gtk::Application::new(Some(APP_ID), Default::default());

    app.connect_startup(|_| load_css());

    app.connect_activate(move |app| {
        let (tx, rx) = async_channel::unbounded();
        let tx2 = tx.clone();
        let (btx, brx) = async_channel::unbounded();
        let file_name = file_name.clone();

        std::thread::spawn(move || {
            let mut emu_ctx = rustcube::Context::default();

            let file_name = Path::new(&file_name);

            match file_name.extension() {
                Some(ext) => {
                    if ext == "dol" {
                        emu_ctx.load_dol(file_name);
                    } else {
                        // assume ipl
                        emu_ctx.load_ipl(file_name);
                    }
                }
                None => emu_ctx.load_ipl(file_name),
            }

            let ctx = glib::MainContext::new();
            ctx.with_thread_default(|| {
                ctx.block_on(background::run(emu_ctx, tx, brx));
            })
            .unwrap();
        });

        let mut app = App::new(app, tx2, btx);

        let event_handler = async move {
            while let Ok(event) = rx.recv().await {
                match event {
                    Event::Breakpoints(bps) => app.update_breakpoints(*bps),
                    Event::Callstack(cs) => app.update_callstack(*cs),
                    Event::Closed => unimplemented!(),
                    Event::Disassembly(disassembly) => app.update_disassembly(*disassembly),
                    Event::Registers(regs) => app.update_registers(*regs),
                    Event::Memory(mem) => app.update_memory(*mem),
                    Event::Paused => app.paused(),
                }
            }
        };

        glib::MainContext::default().spawn_local(event_handler);
    });

    app.run_with_args(&[""; 0]);

    Ok(())
}
