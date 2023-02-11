use crate::Context;

const SI_POLL: u32 = 0x30;
const SI_COMM_CONTROL: u32 = 0x34;
const SI_STATUS: u32 = 0x38;
const SI_EXI_CLOCK_LOCK: u32 = 0x3C;
const SI_IO_BUFFER: u32 = 0x80;

#[derive(Debug, Default)]
pub struct SerialInterface {
    poll: PollRegister,
    comm_cont_status: CommunicationControlStatusRegister,
    status: StatusRegister,
    clock_count: u32,
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    pub struct PollRegister(u32);
    impl Debug;
    pub vblank_copy, _ : 3, 0;
    pub enable, _ : 7, 4;
    pub y_times, _ : 15, 8;
    pub x_lines, _ : 25, 16;
}

impl From<u32> for PollRegister {
    fn from(v: u32) -> Self {
        PollRegister(v)
    }
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    pub struct CommunicationControlStatusRegister(u32);
    impl Debug;
    pub tstart, _ : 0;
    pub channel, _ : 2, 1;
    pub input_length, _ : 14, 8;
    pub output_length, _ : 22, 16;
    pub channel_enable, _ : 24;
    pub channel_number, _ : 26, 25;
    pub rdstint, set_rdstint : 27;   // read status interrupt mask
    pub reat_interrupt_status, _ : 28; // read status interrupt status
    pub comm_error, _ : 29;
    pub transfer_complete_interrupt_mask, _ : 30;
    pub tcint, set_tcint : 31; // transfer complete interrupt status
}

impl From<u32> for CommunicationControlStatusRegister {
    fn from(v: u32) -> Self {
        CommunicationControlStatusRegister(v)
    }
}

impl From<CommunicationControlStatusRegister> for u32 {
    fn from(s: CommunicationControlStatusRegister) -> u32 {
        s.0
    }
}

bitfield! {
    #[derive(Copy, Clone, Default)]
    pub struct StatusRegister(u32);
    impl Debug;
    pub joy_channel_3, _ : 5, 0;
    pub joy_channel_2, _ : 13, 8;
    pub joy_channel_1, _ : 21, 16;
    pub under_run_error, _ : 24;
    pub over_run_error, _ : 25;
    pub collision_error, _ : 26;
    pub no_response_error, _ : 27;
    pub write_status, _ : 28;
    pub read_status, _ : 29;
    pub write, _ : 31;
}

impl From<u32> for StatusRegister {
    fn from(v: u32) -> Self {
        StatusRegister(v)
    }
}

impl From<StatusRegister> for u32 {
    fn from(s: StatusRegister) -> u32 {
        s.0
    }
}

pub fn read_u32(ctx: &mut Context, register: u32) -> u32 {
    match register {
        SI_COMM_CONTROL => ctx.si.comm_cont_status.into(),
        SI_STATUS => ctx.si.status.into(),
        SI_EXI_CLOCK_LOCK => ctx.si.clock_count,
        _ => {
            warn!("read_u32 unrecognized register {:#x}", register);
            0
        }
    }
}

pub fn write_u32(ctx: &mut Context, register: u32, val: u32) {
    match register {
        SI_POLL => ctx.si.poll = val.into(),
        SI_COMM_CONTROL => {
            let mut cont: CommunicationControlStatusRegister = val.into();

            if cont.rdstint() {
                cont.set_rdstint(false);
            }
            if cont.tcint() {
                cont.set_tcint(false);
            }
            ctx.si.comm_cont_status = cont;

            if cont.tstart() {
                warn!("FIXME tstart");
            }

            if !cont.tstart() {
                warn!("Update interrupt");
            }
        }
        SI_STATUS => {
            ctx.si.status = val.into();
        }
        SI_EXI_CLOCK_LOCK => ctx.si.clock_count = val,
        SI_IO_BUFFER => (), // ignore for now
        _ => warn!("write_u32 unrecognized register {:#x}:{}", register, val),
    }
}
