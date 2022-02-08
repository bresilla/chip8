pub struct Display {

}

impl Display {
    pub fn new() -> Self {
        Display {  }
    }

    pub fn draw_byte(&self, byte: u8) {
        let mut b = byte;
        for _ in 0 .. 8 {
            match (b & 0b1000_0000) >> 7 {
                0 => print!("░"),
                1 => print!("█"),
                _ => unreachable!(),
            }
            b = b << 1;
        }
        print!("\n");
    }
}
