use crate::ram::Ram;
use crate::keyboard::Keyboard;
use crate::display::Display;

pub struct Bus {
    ram: Ram,
    keyboard: Keyboard,
    display: Display,
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            ram: Ram::new(), 
            keyboard: Keyboard::new(),
            display: Display::new() 
        }
    }

    pub fn flipped(&self) -> u8 { self.display.flipped() }

    pub fn key_is_pressed(&self, keycode: u8) -> bool {
        self.keyboard.is_pressed(keycode)
    }
    pub fn ram_write_byte(&mut self, address: u16, value: u8) {
        self.ram.write_byte(address, value);
    }
    pub fn ram_read_byte(&self, address: u16) -> u8 { 
        return self.ram.read_byte(address)
    }
    pub fn disp_draw_byte(&mut self, byte: u8, x: u8, y: u8) -> bool {
        self.display.draw_byte(byte, x, y)
    }
}
