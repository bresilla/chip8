use crate::bus::Bus;

pub struct Display { }

impl Display {
    pub fn new() -> Self {
        Display {  }
    }

    fn draw_byte(&self, byte: u8) {
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
 
    pub fn debug_draw_byte(&self, bus: &Bus, i: u16, x: u8, y: u8, height: u8) {
        for r in 0 .. height {
            let b = bus.ram_read_byte(i + r as u16);
            self.draw_byte(b)
        }
        print!("\n");
    }
}
