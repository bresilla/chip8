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

fn main() {
    env_logger::init();
    let mut file = File::open("games/INVADERS").unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data);

    let mut chip8 = Chip8::new();
    chip8.load_rom(&data);

    let WIDTH = 640;
    let HEIGHT = 320;

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    for i in buffer.iter_mut() { *i = 0xffff0000; }

    let mut window = Window::new("CHIP8", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| { panic!("{}", e); });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {

        chip8.run_instruction();
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }

    // loop{ chip8.run_instruction() }
}
