use std::fmt;

use crate::ram::Ram;
use crate::cpu::Cpu;

pub struct Chip8 {
    ram: Ram,
    cpu: Cpu
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 { 
            ram: Ram::new(), 
            cpu: Cpu::new()
        }
    }
    pub fn load_rom(&mut self, data: &Vec<u8>){
       let offset = 0x200;
        for i in 0..data.len() {
            self.ram.write_byte((offset + i) as u16, data[i]);
        }
    }
}

impl fmt::Display for Chip8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "RAM")
    }
}
