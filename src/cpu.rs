use colored::Colorize;
use crate::ram::Ram;
use std::fmt;

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
        let hi =
            ram.read_byte(self.pc) as u16;
        let lo = ram.read_byte(self.pc + 1) as u16;
        let instruction: u16 = (hi << 8) | lo;
        println!("--> Instruction: {:#X} --> hi:{:#X} lo:{:#X}", instruction, hi, lo);

        let nnn = instruction & 0x0FFF;
        let nn = (instruction & 0x0FF) as u8;
        let n = instruction & 0x00F;
        let x = (instruction & 0x0F00) >> 8;
        let y = (instruction & 0x00F0) >> 4;

        println!("--> Values: nnn={:?}, nn={:?}, n={:?}, x={:?}, y={:?}", nnn, nn, n, x, y);
        if self.pc == self.prev { panic!("{}", format!("COUNTER NOT INCREMENTED").black().on_truecolor(0, 255, 136)) }
        self.prev = self.pc;

        match (instruction & 0xF000) >> 12 {
            0x1 => {
                self.pc = nnn;
                println!("{} jump to: {}", " JP ".black().on_truecolor(0, 255, 136), nnn)
            }
            0x6 => {
                self.write_reg(x, nn);
                self.increment_pc();
            }
            _ => panic!("unrecognized instruction: {}", format!("{:#X}", instruction).truecolor(0, 255, 136))
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
