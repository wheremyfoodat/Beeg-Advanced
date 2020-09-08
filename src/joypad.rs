use crate::io::KEYINPUT;

pub struct Joypad {
    pub keyinput: KEYINPUT
}

impl Joypad {
    pub fn new () -> Joypad {
        Joypad {
            keyinput: KEYINPUT(0xFFFF)
        }
    }

    pub fn update(&mut self) {

    }
}