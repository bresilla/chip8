pub struct Timer {
    clock: usize,
    delay: u8,
}

impl Timer {
    pub fn new() -> Self {
        Timer { 
            clock: 0,
            delay: 0
        }
    }

    pub fn set_delay(&mut self, value: u8) {
        self.delay = value
    }

    pub fn get_delay(&self) -> u8 {
        self.delay
    }

    pub fn get_clock(&self) -> usize {
        self.clock
    }

    pub fn tick(&mut self) {
        self.clock += 1;
        if self.delay > 0 { self.delay -= 1 }
    }
}
