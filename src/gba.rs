use crate::cpu::CPU;
use crate::bus::Bus;
extern crate sfml;
use sfml::graphics::*;
use sfml::window::*;
use sfml::system::SfBox;

use std::fs::{self, File};
use std::io::prelude::*;
use std::io::LineWriter;    

pub struct GBA {
    cpu: CPU,
    bus: Bus,

    texture: SfBox<Texture>
}

impl GBA {
    pub fn new(romPath: String) -> GBA {
        GBA {
            cpu: CPU::new(),
            bus: Bus::new(romPath),
            texture: Texture::new(240, 160).unwrap()
        }
    }

    pub fn init(&mut self) {
        self.cpu.init(&self.bus);
    }

    pub fn step(&mut self) {
        self.cpu.step(&mut self.bus);
        self.bus.stepComponents(2);
    }

    pub fn executeFrame (&mut self, window: &mut sfml::graphics::RenderWindow) {
        while !self.bus.isFrameReady() {
            self.step();
        }

        self.bus.joypad.update(); // Update joypad

        // poll window events and render screen
        while let Some(event) = window.poll_event() {
            if event == Event::Closed {
                //println!("Writing CPU log to disk\n");
                //let mut file = File::create("CPULog.txt").unwrap();
                //file.write_all(self.cpu.log.as_bytes());
                std::process::exit(0);
            }
        }

        self.bus.ppu.isFrameReady = false;
        let mut sprite = Sprite::new();

        unsafe {
            self.texture.update_from_pixels(&self.bus.ppu.pixels, 240, 160, 0, 0);
            sprite = Sprite::with_texture(&self.texture);
        }

        window.clear(Color::BLACK);
        window.draw(&sprite);
        window.display();
    }
}