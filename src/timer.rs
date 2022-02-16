use std::{thread, time::Duration};
use std::fmt;

pub struct Timer {
    clock: u64,
    delay: u8,
    speed: i64
}

impl Timer {
    pub fn new() -> Self {
        Timer { 
            clock: 0,
            delay: 0,
            speed: 1,
        }
    }

    pub fn set_delay(&mut self, value: u8) {
        self.delay = value
    }

    pub fn get_delay(&self) -> u8 {
        self.delay
    }

    pub fn get_clock(&self) -> u64 {
        self.clock
    }

    pub fn tick(&mut self) {
        if self.speed > 0 {
            thread::sleep(Duration::from_millis(self.speed as u64));
        }
        self.clock += 1;
        if self.delay > 0 { self.delay -= 1 }
    }
}

impl fmt::Debug for Timer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Clock: {}, Delay: {}, Speed: {}", self.clock, self.delay, self.speed)
    }
}
