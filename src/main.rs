#![warn(clippy::all)]
#![allow(nonstandard_style)]

extern crate sfml;
extern  crate staticvec;

pub mod gba;
pub mod bus;
pub mod cpu;
pub mod mem;
pub mod DMA;
pub mod PPU;
pub mod ARM;
pub mod irqs;
pub mod io;
pub mod timers;
pub mod joypad;
pub mod Thumb;
pub mod barrelShifter;
pub mod helpers;
pub mod scheduler;

use gba::GBA;
use sfml::graphics::*;
use sfml::window::*; // TODO: Not import the entire thing

fn main() {
    let gameName = "suite";
    let mut gba = GBA::new(format!("ROMs/{}.gba", gameName));
    gba.init();

    let mut window = RenderWindow::new(VideoMode::new(240, 160, 32),
                            &format!("Beeg Advanced: {}", gameName),
                            Style::RESIZE | Style::CLOSE,
                  &ContextSettings::default());
    window.set_framerate_limit(0);
    
    loop {
        gba.executeFrame(&mut window);
    }
}