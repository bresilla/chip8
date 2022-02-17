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

    let scale = 25;
    let width = display::WIDTH*scale;
    let height = display::HEIGHT*scale;

    let mut buffer: Vec<u32> = vec![0; width * height];
    for i in buffer.iter_mut() { *i = 0xffff0000; }

    let mut window = Window::new("CHIP8", width, height, WindowOptions::default())
        .unwrap_or_else(|e| { panic!("{}", e); });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
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

    // loop{ chip8.run_instruction() }
}
