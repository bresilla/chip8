use crate::ram::Ram;
use crate::keyboard::Keyboard;
use crate::display::Display;

struct Bus {
    ram: Ram,
    keyboard: Keyboard,
    display: Display,
}
