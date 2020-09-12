extern crate sfml;
use crate::io::KEYINPUT;
use sfml::window::Key;
const KEYS: [Key; 10] = [Key::A, Key::S, Key::BackSpace, Key::Return, Key::Right, Key::Left, Key::Up, Key::Down, Key::R, Key::L];

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
        let mut newKeyinput = 0_u16;
        for i in 0..10 {
            if !Key::is_pressed(KEYS[i]) {
                newKeyinput |= (1 << i);
            }
        }

        self.keyinput.setRaw(newKeyinput);
    }
}