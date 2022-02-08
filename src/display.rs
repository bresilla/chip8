use crate::bus::Bus;

const WIDTH: usize = 32;
const HEIGHT: usize = 64;

pub struct Display {
    screen: [[u8; WIDTH]; HEIGHT]
}

impl Display {
    pub fn new() -> Self {
        Display { 
            screen: [[0; WIDTH]; HEIGHT]
        }
    }

    pub fn draw_byte(&mut self, byte: u8, x: u8, y: u8) {
        let mut b = byte;
        let cx = x as usize;
        let cy = y as usize;
        for _ in 0 .. 8 {
            match (b & 0b1000_0000) >> 7 {
                0 => self.screen[cx][cy] = 0,
                1 => self.screen[cx][cy] = 1,
                _ => unreachable!(),
            }
            b = b << 1;
        }
        print!("\n");
    }
}
