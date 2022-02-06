use crate::ram::Ram;
use crate::cpu::{Cpu, PROGRAM_START};
use log::info;

pub struct Chip8 {
    ram: Ram,
    cpu: Cpu,
    itr: usize,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 { 
            ram: Ram::new(), 
            cpu: Cpu::new(),
            itr: 0,
        }
    }
    pub fn load_rom(&mut self, data: &Vec<u8>){
        for i in 0..data.len() {
            self.ram.write_byte((PROGRAM_START + i as u16) as u16, data[i]);
        }
    }

    pub fn run_instruction(&mut self) {
        self.itr += 1;
        info!("\n----------- iteration {} -----------", self.itr);
        info!("--> {:?}", self.cpu);
        self.cpu.execute(&mut self.ram);
    }

}
