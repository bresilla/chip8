use colored::Colorize;
use crate::bus::Bus;
use std::fmt;
use rand::prelude::*;
use log::{info, warn};

pub const PROGRAM_START: u16 = 0x200;

pub struct Cpu {
    v: [u8; 16],
    i: u16,
    pc: u16,
    lc: u16,
    stack: Vec<u16>,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu{
            v: [0; 16],
            i: 0,
            pc: PROGRAM_START,
            lc: 0,
            stack: Vec::<u16>::new(),
        }
    }

    pub fn execute(&mut self, bus: &mut Bus) {
        let hi = bus.ram_read_byte(self.pc) as u16;
        let lo = bus.ram_read_byte(self.pc + 1) as u16;
        let instruction: u16 = (hi << 8) | lo;
        info!("--> Instruction: {:#X} --> hi:{:#X} lo:{:#X}", instruction, hi, lo);

        let nnn = instruction & 0x0FFF;
        let nn = (instruction & 0x0FF) as u8;
        let n = (instruction & 0x00F) as u8;
        let x = ((instruction & 0x0F00) >> 8) as u8;
        let y = ((instruction & 0x00F0) >> 4) as u8;

        info!("--> Values: nnn={:#X}, nn={:#X}, n={:?}, x={:#X}, y={:#X}", nnn, nn, n, x, y);
        if self.pc == self.lc { panic!("{}", format!("COUNTER NOT INCREMENTED").black().on_truecolor(0, 255, 136)) }
        self.lc = self.pc;

        match (instruction & 0xF000) >> 12 {
            0x0 => {
                match instruction & 0x00FF {
                    0xE0 => {
                        bus.disp_clean_screen();
                        self.increment_pc(2);
                        info!("{} --> clean display", " DISPLAY ".black().on_truecolor(100, 200, 200));
                    }
                    0xEE => {
                        let pc = self.stack.pop().unwrap();
                        self.pc = pc;
                        info!("{} --> return from subrutine", " FLOW ".black().on_truecolor(0, 255, 136));
                    }
                    _ => {
                        info!("{} --> 0x0: unrecognized instruction: {:#X}\n", " ERROR ".black().on_truecolor(212, 60, 58), instruction);
                        unreachable!()
                    }
                }
            }
            0x1 => {
                //goto NNN
                self.pc = nnn;
                info!("{} --> jump to NNN which is: {:#X}", " FLOW ".black().on_truecolor(0, 255, 136), nnn);
            }
            0x2 => {
                //*(0xNNN)()
                self.stack.push(self.pc + 2);
                self.pc = nnn;
                info!("{} --> call subrutine at NNN which is: {:#X}", " FLOW ".black().on_truecolor(0, 255, 136), nnn);
            }
            0x3 => {
                //if (Vx == NN)
                if self.read_reg(x) == nn { self.increment_pc(2) }
                self.increment_pc(2);
                info!("{} --> if VX: {:#X} equals NN: {:#X}, then skip next instruction", " COND ".black().on_truecolor(104, 115, 233), self.read_reg(x), nn);
            }
            0x4 => {
                //if (Vx != NN)
                if self.read_reg(x) != nn { self.increment_pc(2) }
                self.increment_pc(2);
                info!("{} --> if VX: {:#X} does not equals NN: {:#X}, then skip next instruction", " COND ".black().on_truecolor(104, 115, 233), self.read_reg(x), nn);
            }
            0x5 => {
                //if (Vx == Vy)
                if self.read_reg(x) == self.read_reg(y) { self.increment_pc(2) }
                self.increment_pc(2);
                info!("{} --> if VX: {:#X} equals Vy: {:#X}, then skip next instruction", " COND ".black().on_truecolor(104, 115, 233), self.read_reg(x), self.read_reg(y));
            }
            0x6 => {
                //Vx = N
                info!("{} --> set VX: {} to NN: {:#X}", " CONST ".black().on_truecolor(0, 136, 255), x, nn);
                self.write_reg(x, nn);
                self.increment_pc(2);
            }
            0x7 => {
                //Vx += N
                let r = self.read_reg(x);
                self.write_reg(x, r.wrapping_add(nn));
                self.increment_pc(2);
                info!("{} --> set VX: {} to VX+N: {:#X}", " CONST ".black().on_truecolor(0, 136, 255), x, r);
            }
            0x8 => {
                let vy = self.read_reg(y);
                let vx = self.read_reg(x);
                match instruction & 0x000F {
                    0x0 => {
                        //Vx = Vy
                        self.write_reg(x, vy);
                        info!("{} --> set vx:{:#X} = vy:{:#X}", " ASSIGN ".black().on_truecolor(225, 147, 236), x, vy);
                    }
                    0x1 => {
                        //Vx |= Vy
                        self.write_reg(x, vx | vy);
                        info!("{} --> set vx:{:#X} |= vy:{:#X}", " BITOP ".black().on_truecolor(201, 240, 236), vx, vy);
                    }
                    0x2 => {
                        //Vx &= Vy
                        self.write_reg(x, vx & vy);
                        info!("{} --> set vx:{:#X} &= vy:{:#X}", " BITOP ".black().on_truecolor(201, 240, 236), vx, vy);
                    }
                    0x3 => {
                        //Vx ^= Vy
                        self.write_reg(x, vx ^ vy);
                        info!("{} --> set vx:{:#X} ^= vy:{:#X}", " BITOP ".black().on_truecolor(201, 240, 236), vx, vy);
                    }
                    0x4 => {
                        //Vx += Vy
                        let a: u16 = vx as u16 + vy as u16;
                        self.write_reg(x, a as u8);
                        if a > 0xFF { self.write_reg(0xF, 1) }
                        info!("{} --> set vx:{:#X} += vy:{:#X}", " MATH ".black().on_truecolor(215, 215, 140), vx, vy);
                    }
                    0x5 => {
                        //Vx -= Vy
                        let a: i8 = vx as i8 - vy as i8;
                        self.write_reg(x, a as u8);
                        if a < 0 { self.write_reg(0xF, 1) }
                        info!("{} --> set vx:{:#X} -= vy:{:#X}", " MATH ".black().on_truecolor(215, 215, 140), vx, vy);
                    }
                    0x6 => {
                        //Vx >>= 1
                        self.write_reg(0xF, vy & 0x1);
                        self.write_reg(y, vy >> 1);
                        self.write_reg(x, vy >> 1);
                        info!("{} --> set vx:{:#X} >>= vy:{:#X}", " BITOP ".black().on_truecolor(201, 240, 236), vx, vy);
                    }
                    0x7 => {
                        //Vx = Vy - Vx
                        let a: i8 = vy as i8 - vx as i8;
                        self.write_reg(x, a as u8);
                        if a < 0 { self.write_reg(0xF, 1) }
                        info!("{} --> set vx:{:#X} -= vy:{:#X}", " MATH ".black().on_truecolor(215, 215, 140), vx, vy);
                    }
                    0x8 => {
                        //Vx <<= 1
                        self.write_reg(0xF, vx & 0x1);
                        self.write_reg(x, vy << 1);
                        self.write_reg(y, vy << 1);
                        info!("{} --> set vx:{:#X} <<= vy:{:#X}", " BITOP ".black().on_truecolor(201, 240, 236), vx, vy);
                    }
                    _ => {
                        info!("{} --> 0x8: unrecognized instruction: {:#X}\n", " ERROR ".black().on_truecolor(212, 60, 58), instruction);
                        unreachable!()
                    }
                }
                self.increment_pc(2);
            }
            0x9 => {
                //if (Vx != Vy)
                if self.read_reg(x) != self.read_reg(y) { self.increment_pc(2) }
                self.increment_pc(2);
                info!("{} --> if VX: {:#X} equals Vy: {:#X}, then skip next instruction", " COND ".black().on_truecolor(104, 115, 233), self.read_reg(x), self.read_reg(y));
            }
            0xA => {
                self.i = nnn;
                self.increment_pc(2);
                info!("{} --> set i: {} to NNN: {:#X}", " MEM ".black().on_truecolor(169, 105, 231), x, nn);
            }
            0xB => {
                //PC = V0 + NNN 
                self.pc = self.read_reg(0) as u16 + nnn;
                info!("{} --> jump to V0 + NNN which is: {:#X}", " FLOW ".black().on_truecolor(0, 255, 136), self.pc);
            }
            0xC => {
                //Vx = rand() & NN
                let mut rng = thread_rng();
                let r = rng.gen_range(0..254);
                self.write_reg(x, r & nn);
                self.increment_pc(2);
                info!("{} --> set VX: {} to (RANDOM & NN): {:#X}", " RAND ".black().on_truecolor(198, 150, 69), x, (r&nn));
            }
            0xD => {
                //draw(Vx, Vy, N)
                info!("{} --> draw at (VX × VY): ({} × {})", " DISPLAY ".black().on_truecolor(100, 200, 200), self.read_reg(x), self.read_reg(y));
                self.debug_draw_sprite(bus, x, y, n);
                self.increment_pc(2);
            }
            0xE => {
                match instruction & 0x00FF {
                    0xA1 => {
                        //if (key() != Vx) skip next
                        let key = self.read_reg(x);
                        if !bus.key_is_pressed(key) {
                            info!("{} --> key {} is not pressed", " KEYOP ".black().on_truecolor(169, 105, 231), x);
                            self.increment_pc(2);
                        }
                        self.increment_pc(2);
                    }
                    0x9E => {
                        //if (key() == Vx) skip next
                        let key = self.read_reg(x);
                        if bus.key_is_pressed(key) {
                            info!("{} --> key {} is not pressed", " KEYOP ".black().on_truecolor(169, 105, 231), x);
                            self.increment_pc(2);
                        }
                        self.increment_pc(2);
                    }
                    _ => {
                        info!("{} --> 0xE: unrecognized instruction: {:#X}\n", " ERROR ".black().on_truecolor(212, 60, 58), instruction);
                        unreachable!()
                    }
                }
            }
            0xF => {
                let new_i = self.read_reg(x) as u16;
                self.i = new_i;
                info!("{} --> set i: {} to i+=Vx: {:#X}", " MEM ".black().on_truecolor(169, 105, 231), x, new_i);
                self.increment_pc(2);
            }
            _ => {
                info!("{} --> unrecognized instruction: {:#X}\n", " ERROR ".black().on_truecolor(212, 60, 58), instruction);
                unreachable!()
            }
        }
    }

    pub fn increment_pc(&mut self, jumps: u16) { self.pc += jumps }
    pub fn write_reg(&mut self, index: u8, value: u8) { self.v[index as usize] = value; }
    pub fn read_reg(&mut self, index: u8) -> u8 { self.v[index as usize] }

    pub fn debug_draw_sprite(&mut self, bus: &mut Bus, x: u8, y: u8, n: u8) {
        for y in 0 .. n {
            let b = bus.ram_read_byte(self.i + y as u16);
            bus.disp_draw_byte(b, x, y);
        }
        bus.disp_show_pixels();
        self.write_reg(0xF, bus.flipped());
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut v: String = String::new();
        for item in self.v.iter() {
            v = format!("{}{:#X}, ", v, item);
        }
        write!(f, "CPU: pc={}, v=[{}], i={:#X}", format!("{:#X}", self.pc).truecolor(0, 255, 136), v, self.i)
    }
}
