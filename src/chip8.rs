use crate::bus::Bus;
use crate::cpu::{Cpu, PROGRAM_START};
use log::info;

pub struct Chip8 {
    bus: Bus,
    cpu: Cpu,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 { 
            bus: Bus::new(), 
            cpu: Cpu::new(),
        }
    }
    pub fn load_rom(&mut self, data: &Vec<u8>){
        for i in 0..data.len() {
            self.bus.ram_write_byte((PROGRAM_START + i as u16) as u16, data[i]);
        }
    }

    pub fn run_instruction(&mut self) {
        self.bus.timer_tick();
        info!("{:?}", self.bus);
        info!("{:?}", self.cpu);
        self.cpu.execute(&mut self.bus);
        info!("\n");
    }

}
