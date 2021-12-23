use std::fs::File;
use std::io::Read;

mod ram;
mod chip8;

fn main() {
    let mut file = File::open("other/games/INVADERS").unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data);
    println!("{:?}", data);
}
