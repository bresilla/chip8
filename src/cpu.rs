use colored::Colorize;
use crate::ram::Ram;
use std::fmt;
use rand::prelude::*;

pub const PROGRAM_START: u16 = 0x200;

pub struct Cpu {
    vx: [u8; 16],
    pc: u16,
    i: u16,
    prev: u16
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu{
            vx: [0; 16],
            pc: PROGRAM_START,
            i: 0,
            prev: 0
        }
    }

    pub fn execute(&mut self, ram: &mut Ram) {
        let hi = ram.read_byte(self.pc) as u16;
        let lo = ram.read_byte(self.pc + 1) as u16;
        let instruction: u16 = (hi << 8) | lo;
        println!("--> Instruction: {:#X} --> hi:{:#X} lo:{:#X}", instruction, hi, lo);

        let nnn = instruction & 0x0FFF;
        let nn = (instruction & 0x0FF) as u8;
        let n = instruction & 0x00F;
        let x = (instruction & 0x0F00) >> 8;
        let y = (instruction & 0x00F0) >> 4;

        println!("--> Values: nnn={:#X}, nn={:#X}, n={:?}, x={:#X}, y={:#X}", nnn, nn, n, x, y);
        if self.pc == self.prev { panic!("{}", format!("COUNTER NOT INCREMENTED").black().on_truecolor(0, 255, 136)) }
        self.prev = self.pc;

        match (instruction & 0xF000) >> 12 {
            0x1 => {
                self.pc = nnn;
                println!("{} --> jump to NNN which is: {:#X}", " FLOW ".black().on_truecolor(0, 255, 136), nnn)
            }
            0x6 => {
                //Vx = N
                self.write_reg(x, nn);
                self.increment_pc();
                println!("{} --> set VX: {} to NN: {:#X}", " CONST ".black().on_truecolor(0, 136, 255), x, nn)
            }
            0x7 => {
                //Vx += N
                let r = self.vx[x as usize] + n as u8;
                self.write_reg(x, r);
                self.increment_pc();
                println!("{} --> set VX: {} to VX+N: {:#X}", " CONST ".black().on_truecolor(0, 136, 255), x, r)
            }
            0xA => {
                self.i = nnn;
                self.increment_pc();
                println!("{} --> set i: {} to NNN: {:#X}", " MEM ".black().on_truecolor(169, 105, 231), x, nn)
            }
            0xB => {
                //PC = V0 + NNN 
                self.pc = self.vx[0] as u16 + nnn;
                println!("{} --> jump to V0 + NNN which is: {:#X}", " FLOW ".black().on_truecolor(0, 255, 136), self.pc)
            }
            0xC => {
                let mut rng = thread_rng();
                let r = rng.gen_range(0..254);
                self.write_reg(x, r & nn);
                self.increment_pc();
                println!("{} --> set VX: {} to (RANDOM & NN): {:#X}", " RAND ".black().on_truecolor(198, 150, 69), x, (r&nn))
            }
            _ => {
                println!("{} --> unrecognized instruction: {:#X}\n", " ERROR ".black().on_truecolor(212, 60, 58), instruction);
                unreachable!()
            }

        }
    }
    pub fn increment_pc(&mut self) { self.pc += 2 }
    pub fn write_reg(&mut self, index: u16, value: u8) { self.vx[index as usize] = value; }
    pub fn read_reg(&mut self, index: u16) -> u8 { self.vx[index as usize] }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut vx: String = String::new();
        for item in self.vx.iter() {
            vx = format!("{}{:#X}, ", vx, item);
        }
        write!(f, "CPU: pc={}, vx=[{}], i={:#X}", format!("{:#X}", self.pc).truecolor(0, 255, 136), vx, self.i)
    }
}
