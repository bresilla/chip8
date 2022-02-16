#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(unreachable_code)]
#![allow(unused_imports)]

use std::fs::File;
use std::io::Read;

use log;
use minifb::{Key, Window, WindowOptions};

mod ram;
mod cpu;
mod chip8;
mod keyboard;
mod display;
mod bus;
mod timer;

use chip8::Chip8;
use display::{WIDTH, HEIGHT};

fn main() {
    env_logger::init();
    let mut file = File::open("games/INVADERS").unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data);

    let mut chip8 = Chip8::new();
    chip8.load_rom(&data);
    loop{ chip8.run_instruction() }
}
