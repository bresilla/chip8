pub struct Keyboard {

}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard {  }
    }

    pub fn is_pressed(&self, keycode: u8) -> bool {
        if keycode == 4 {
            false
        } else {
            true
        }
    }
}
