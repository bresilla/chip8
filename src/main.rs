#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(unreachable_code)]
#![allow(unused_imports)]

use std::fs::File;
use std::io::Read;

use log;
use minifb::{Key, KeyRepeat, Window, WindowOptions};

mod ram;
mod cpu;
mod chip8;
mod keyboard;
mod display;
mod bus;
mod timer;

use chip8::Chip8;

fn chip_keycode_of(key: Option<Key>) -> Option<u8> {
    match key {
        Some(Key::Key1) => Some(0x1),
        Some(Key::Key2) => Some(0x2),
        Some(Key::Key3) => Some(0x3),
        Some(Key::Key4) => Some(0xC),

        Some(Key::Q) => Some(0x4),
        Some(Key::W) => Some(0x5),
        Some(Key::E) => Some(0x6),
        Some(Key::R) => Some(0xD),

        Some(Key::A) => Some(0x7),
        Some(Key::S) => Some(0x8),
        Some(Key::D) => Some(0x9),
        Some(Key::F) => Some(0xE),

        Some(Key::Z) => Some(0xA),
        Some(Key::X) => Some(0x0),
        Some(Key::C) => Some(0xB),
        Some(Key::V) => Some(0xF),
        _ => None,
    }
}

fn main() {
    env_logger::init();
    let mut file = File::open("games/PONG").unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data);

    let mut chip8 = Chip8::new();
    chip8.load_rom(&data);

    let scale = 10;
    let width = display::WIDTH*scale;
    let height = display::HEIGHT*scale;

    let mut buffer: Vec<u32> = vec![0; width * height];
    for i in buffer.iter_mut() { *i = 0xffff0000; }

    let mut window = Window::new("CHIP8", width, height, WindowOptions::default())
        .unwrap_or_else(|e| { panic!("{}", e); });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let keys_pressed = window.get_keys_pressed(KeyRepeat::No).iter().next().cloned();
        let key = chip_keycode_of(keys_pressed);
        chip8.chip_keycode_set(key);

        chip8.run_instruction();

        let chip8_buffer = chip8.chip_display_buffer();
        for y in 0..height {
            for x in 0..width {
                let index = (y/scale) * display::WIDTH + (x/scale);
                let pixel = chip8_buffer[index];
                let color_pixel = match pixel {
                    0 => 0x0,
                    1 => 0xffffff,
                    _ => unreachable!()
                };
                buffer[y * width + x] = color_pixel;
            }
        }

        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}
