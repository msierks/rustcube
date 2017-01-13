extern crate byteorder;
extern crate getopts;
extern crate memmap;
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
mod dsp_interface;
mod dvd_interface;
mod exi;
pub mod gamecube;
mod gp_fifo;
mod interconnect;
mod memory;
mod memory_interface;
mod pixel_engine;
mod processor_interface;
mod serial_interface;
mod video_interface;
