use crate::ram::Ram;

pub const PROGRAM_START: u16 = 0x200;

#[derive(Debug)]
pub struct Cpu {
    vx: [u8; 16],
    pc: u16,
    i: u16
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu{
            vx: [0; 16],
            pc: PROGRAM_START,
            i: 0
        }
    }

    pub fn execute(&mut self, ram: &mut Ram){}
}
