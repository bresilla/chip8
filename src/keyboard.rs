pub struct Keyboard {
    key: Option<u8>
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard {
            key: None
        }
    }

    pub fn is_pressed(&self, keycode: u8) -> bool {
        true
    }
    pub fn get_key(&self) -> Option<u8> {
        self.key
    }
    pub fn set_key(&mut self, keycode: Option<u8>) {
        self.key = keycode
    }
}
