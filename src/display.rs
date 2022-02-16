use colored::Colorize;
use crate::bus::Bus;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Display {
    screen: [[u8; WIDTH]; HEIGHT],
    flipped: bool
}

impl Display {
    pub fn new() -> Self {
        Display {
            screen: [[0; WIDTH]; HEIGHT],
            flipped: false
        }
    }

    pub fn flipped_bit(&self) -> u8 {
        self.flipped as u8 
    }

    pub fn draw_byte(&mut self, byte: u8, x: u8, y: u8) -> bool {
        let mut flipped = false;
        let mut cx = x as usize;
        let cy = y as usize;
        let mut b = byte;
        for _ in 0 .. 8 {
            match (b & 0b1000_0000) >> 7 {
                0 => {
                    if self.screen[cy][cx] == 1 {
                        flipped = true
                    }
                    self.screen[cy][cx] = 0 
                },
                1 => {
                    self.screen[cy][cx] = 1
                },
                _ => unreachable!(),
            };
            cx += 1;
            b = b << 1;
        }
        flipped
    }

    pub fn show_pixels(&self) {
        for y in 0 .. HEIGHT {
            for x in 0 .. WIDTH {
                if self.screen[y][x] == 0 {
                    print!("░");
                } else {
                    print!("{}", "█".red());
                }
            }
            print!("\n");
        }
    }

    pub fn clear_diplay(&mut self) {
        self.screen = [[0; WIDTH]; HEIGHT];
    }
}
