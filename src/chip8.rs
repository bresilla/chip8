use std::fmt;

use crate::ram::Ram;
use crate::cpu::{Cpu, PROGRAM_START};

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
        for i in 0..data.len() {
            self.ram.write_byte((PROGRAM_START + i as u16) as u16, data[i]);
        }
    }

    pub fn run_instruction(&mut self) {
        self.cpu.execute(&mut self.ram)
    }

}

impl fmt::Display for Chip8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "RAM")
    }
}
