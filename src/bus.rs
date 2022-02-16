use crate::ram::Ram;
use crate::keyboard::Keyboard;
use crate::display::Display;
use crate::timer::Timer;

use std::fmt;

pub struct Bus {
    ram: Ram,
    keyboard: Keyboard,
    display: Display,
    timer: Timer
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            ram: Ram::new(), 
            keyboard: Keyboard::new(),
            display: Display::new(),
            timer: Timer::new(),
        }
    }

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
    pub fn disp_show_pixels(&mut self) {
        self.display.show_pixels()
    }
    pub fn disp_clean_screen(&mut self) {
        self.display.clear_diplay()
    }

    pub fn disp_flipped_bit(&self) -> u8 { 
        self.display.flipped_bit()
    }

    pub fn timer_set_delay(&mut self, value: u8) {
        self.timer.set_delay(value)
    }

    pub fn timer_get_delay(&self) -> u8 {
        self.timer.get_delay()
    }

    pub fn timer_get_clock(&self) -> u64 {
        self.timer.get_clock()
    }

    pub fn timer_tick(&mut self) {
        self.timer.tick()
    }
}

impl fmt::Debug for Bus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.timer)
    }
}
