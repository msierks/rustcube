use std::net::TcpListener;

use super::cpu::Cpu;

#[derive(Debug)]
pub struct Debugger {
    enabled: bool,
    listener: Option<TcpListener>
}

impl Debugger {

    pub fn new() -> Debugger {
        Debugger {
            enabled: false,
            listener: None
        }
    }

    pub fn enable(&mut self, bind_addr: String) {
        self.enabled  = true;
        self.listener = match TcpListener::bind(bind_addr.as_str()) {
            Ok(v)  => Some(v),
            Err(e) => panic!("Couldn't bind GDB server TCP socket: {}", e),
        };
    }

    pub fn set_cia(&mut self, cpu: &mut Cpu) {
        if self.enabled {
            //println!("debugger enabled");
        }
    }

}
