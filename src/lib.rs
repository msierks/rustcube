
#![cfg_attr(feature="cargo-clippy", allow(inline_always))]
#![cfg_attr(feature="cargo-clippy", allow(many_single_char_names))]
#![cfg_attr(feature="cargo-clippy", allow(new_without_default))]
#![cfg_attr(feature="cargo-clippy", allow(new_without_default_derive))]
#![cfg_attr(feature="cargo-clippy", allow(unreadable_literal))]

extern crate byteorder;
extern crate getopts;
extern crate minifb;
#[cfg(unix)]
extern crate nix;
extern crate num;
extern crate rustyline;

#[macro_use]
extern crate enum_primitive;

mod audio_interface;
mod command_processor;
mod cpu;
pub mod debugger;
mod dol;
mod dsp_interface;
mod dvd_interface;
mod exi;
pub mod gamecube;
mod gp_fifo;
mod interconnect;
mod memory;
//mod memory_interface;
mod pixel_engine;
mod processor_interface;
mod serial_interface;
mod video_interface;
