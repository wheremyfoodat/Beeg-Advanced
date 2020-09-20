#![warn(clippy::all)]
#![allow(nonstandard_style)]

pub mod gba;
pub mod bus;
pub mod cpu;
pub mod mem;
pub mod DMA;
pub mod PPU;
pub mod ARM;
pub mod irqs;
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
    let gameName = "Pokemon Pinball";
    let mut gba = GBA::new(format!("ROMs/{}.gba", gameName).to_string());
    gba.init();

    let mut window = RenderWindow::new(VideoMode::new(240, 160, 32),
                            &format!("Beeg Advanced: {}", gameName),
                            Style::RESIZE | Style::CLOSE,
                  &ContextSettings::default());

    loop {
        gba.executeFrame(&mut window);
    }
}