#![allow(dead_code)]

pub(crate) struct Command {
    data: u32,
}

impl Command {
    pub const fn new() -> Self { Self {data: 0} }

    pub const fn set_datalen(&mut self, datalen: u16) { 
        assert!(datalen <= 16383, "datalen bits must be less than 16383"); 
        self.data |= datalen as u32; 
    }

    pub const fn set_poll(&mut self, poll: u8) {
        assert!(poll <= 1, "poll bit must be less than 1");
        self.data |= (poll as u32) << 14;
    }

    pub const fn set_dout(&mut self, dout: u8) {
        assert!(dout <= 1, "dout bit must be less than 1");
        self.data |= (dout as u32) << 15;
    }

    pub const fn set_intlen(&mut self, intlen: u8) {
        assert!(intlen <= 7, "intlen bits must be less than 7");
        self.data |= (intlen as u32) << 16;
    }

    pub const fn set_fieldform(&mut self, fieldform: u8) {
        assert!(fieldform <= 3, "fieldform bits must be less than 3");
        self.data |= (fieldform as u32) << 19 as u32;
    }

    pub const fn set_frameform(&mut self, frameform: u8) {
        assert!(frameform <= 3, "frameform bits must be less than 3");
        self.data |= (frameform as u32) << 21 as u32;
    }

    pub const fn set_opcode(&mut self, opcode: u8) {
        self.data |= (opcode as u32) << 24 as u32;
    }

    pub const fn bits(&self) -> u32 {
        self.data
    }
}