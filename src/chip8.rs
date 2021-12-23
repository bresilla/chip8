use crate::ram::Ram;

struct Chip8 {
    ram: Ram
}

impl Chip8 {
    fn new() -> Chip8 {
        Chip8 {
            ram: Ram::new()
        }
    } 
}
