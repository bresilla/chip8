use colored::Colorize;
use crate::bus::Bus;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

pub struct Display {
    screen: [u8; WIDTH * HEIGHT],
    flipped: bool
}

impl Display {
    pub fn new() -> Self {
        Display {
            screen: [0; WIDTH * HEIGHT],
            flipped: false
        }
    }

    pub fn flipped_bit(&self) -> u8 {
        self.flipped as u8 
    }

    pub fn index_from_coords(x: usize, y: usize) -> usize {
        y * WIDTH + x
    }

    pub fn draw_byte(&mut self, byte: u8, x: u8, y: u8) {
        self.flipped = false;
        let mut cx = x as usize;
        let cy = y as usize;
        let mut b = byte;

        for _ in 0 .. 8 {
            let index = cy * WIDTH + cx;
            let bit = (b & 0b1000_0000) >> 7;
            let prev_value = self.screen[index];
            self.screen[index] ^= bit;
            if prev_value == 1 && self.screen[index] == 0 {
                self.flipped = true;
            }
            cx += 1;
            b = b << 1;
        }
    }

    pub fn show_pixels(&self) {
        for index in 0..self.screen.len() {
            let pixel = self.screen[index];
            if index % WIDTH == 0 { print!("\n"); }
            match pixel {
                0 => print!("░"),
                1 => print!("{}", "█".red()),
                _ => unreachable!()
            };
        }
        print!("\n");
    }

    pub fn clear_diplay(&mut self) {
        self.screen = [0; WIDTH * HEIGHT];
    }

    pub fn get_buffer(&self) -> &[u8] {
        &self.screen
    }
}
