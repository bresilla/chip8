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

    pub fn key_is_pressed(&self) -> bool {
        self.keyboard.is_pressed()
    }
    pub fn ram_write_byte(&mut self, address: u16, value: u8) {
        self.ram.write_byte(address, value);
    }
    pub fn ram_read_byte(&self, address: u16) -> u8 { 
        return self.ram.read_byte(address)
    }
    pub fn disp_draw_byte(&self, byte: u8) {
        self.display.draw_byte(byte)
    }
    pub fn disp_debug_draw_byte(&self, i: u16, x: u8, y: u8, height: u8) {
        self.display.debug_draw_byte(&self, i, x, y, height);
    }
}
