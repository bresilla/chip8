#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

use std::fs::File;
use std::io::Read;

mod ram;
mod cpu;
mod chip8;

use chip8::Chip8;

fn main() {
    let mut file = File::open("games/INVADERS").unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data);

    let mut chip8 = Chip8::new();
    chip8.load_rom(&data)
}
