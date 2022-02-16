pub struct Keyboard {

}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard {  }
    }

    pub fn is_pressed(&self, keycode: u8) -> bool {
        true
    }
    pub fn get_key(&self) -> u8 {
        7
    }
}
