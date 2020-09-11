#![warn(clippy::all)]
#![allow(nonstandard_style)]

pub mod gba;
pub mod bus;
pub mod cpu;
pub mod mem;
pub mod PPU;
pub mod ARM;
pub mod io;
pub mod joypad;
pub mod Thumb;
pub mod barrelShifter;
pub mod helpers;
//mod frontend;

use gba::GBA;
extern crate sfml;
use sfml::graphics::*;
use sfml::window::*; // TODO: Not import the entire thing

fn main() {
    let mut gba = GBA::new("ROMs/hello.gba".to_string());
    gba.init();

    let mut window = RenderWindow::new(VideoMode::new(240, 160, 32),
                            "Beeg Advanced",
                            Style::RESIZE | Style::CLOSE,
                  &ContextSettings::default());

    loop {
        gba.executeFrame(&mut window);
    }
}