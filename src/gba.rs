use crate::cpu::CPU;
use crate::bus::Bus;
use crate::DMA::DMAChannelStatus;

use sfml::graphics::*;
use sfml::system::*;
use crate::scheduler::*;

pub struct GBA {
    cpu: CPU,
    bus: Bus,
    isFrameReady: bool,

    texture: SfBox<Texture>
}

impl GBA {
    pub fn new(romPath: String) -> GBA {
        GBA {
            cpu: CPU::new(),
            bus: Bus::new(romPath),
            isFrameReady: false,
            texture: Texture::new(240, 160).unwrap()
        }
    }

    pub fn init(&mut self) {
        self.cpu.init(&self.bus);
        self.bus.scheduler.pushEvent(EventTypes::HBlank, 960); // Add first HBlank event to the scheduler    
    }

    pub fn step(&mut self) {
        self.cpu.step(&mut self.bus);
        self.advanceScheduler(1);
    }

    pub fn executeFrame (&mut self, window: &mut sfml::graphics::RenderWindow) {
        self.isFrameReady = false;
        
        while !self.isFrameReady {
            self.step();
        }

        self.bus.joypad.update(); // Update joypad

        // poll window events and render screen
        while let Some(event) = window.poll_event() {
            if event == sfml::window::Event::Closed {
                //println!("Writing CPU log to disk\n");
                //let mut file = File::create("CPULog.txt").unwrap();
                //file.write_all(self.cpu.log.as_bytes());
                std::process::exit(0);
            }
        }
                
        let sprite: Sprite;

        unsafe {
            self.texture.update_from_pixels(&self.bus.ppu.pixels, 240, 160, 0, 0);
            sprite = Sprite::with_texture(&self.texture);
        }
        
        // It's not necessary to clear the window since we're redrawing the whole thing anyways
        window.draw(&sprite);
        window.display();
    }

    fn advanceScheduler(&mut self, cycles: u64) {
        self.bus.scheduler.currentTimestamp += cycles;

        loop { // Check which events should be fired
            let event = self.bus.scheduler.getNearestEvent();   
            if event.endTimestamp <= self.bus.scheduler.currentTimestamp { // If the event should be fire it, fire, else exit the loop early  
                self.bus.scheduler.removeEvent();
                self.eventCallback(event.eventType, event.endTimestamp);
            }
       
            else {
              return;
            }
        }
    }

    fn eventCallback (&mut self, eventType: EventTypes, firedEventTimestamp: u64) {
        match eventType {
            EventTypes::PollInterrupts => self.cpu.pollInterrupts(&mut self.bus),
            EventTypes::HBlank => { // TODO: Add HBlank DMA here
                if self.bus.ppu.dispstat.getHBlankIRQEnable() == 1 {
                   self.bus.ppu.interruptFlags |= 0b10; // Request HBlank IRQ
                   self.cpu.pollInterrupts(&mut self.bus);
                }

                if self.bus.ppu.vcount < 160 {
                    self.bus.ppu.renderScanline();
                    self.bus.pollDMAs(DMAChannelStatus::HBlank); // See if there's any HBlank-triggered DMAs to fire. HBlank DMAs DO NOT fire during VBlank
                }

                self.bus.ppu.dispstat.setHBlankFlag(1);
                self.bus.scheduler.pushEvent(EventTypes::EndOfLine, firedEventTimestamp + 272) // HBlank takes 272 cycles. TODO: Use constants
            }

            EventTypes::EndOfLine => {
                self.bus.ppu.vcount += 1;
                self.bus.ppu.dispstat.setHBlankFlag(0);

                if self.bus.ppu.vcount == 160 { // If PPU is entering VBlank, run VBlank events
                    self.isFrameReady = true;
                    self.bus.ppu.dispstat.setVBlankFlag(1);

                    if self.bus.ppu.dispstat.getVBlankIRQEnable() == 1 {
                        self.bus.ppu.interruptFlags |= 1;
                        self.cpu.pollInterrupts(&mut self.bus);
                    }

                    self.bus.pollDMAs(DMAChannelStatus::VBlank); // See if there's any VBlank-triggered DMAs to fire
                }

                else if self.bus.ppu.vcount == 228 {
                    self.bus.ppu.vcount = 0;
                    self.bus.ppu.dispstat.setVBlankFlag(0);
                }

                if self.bus.ppu.compareLYC() { // If LY == LYC, poll for interrupts
                    self.cpu.pollInterrupts(&mut self.bus);
                }

                self.bus.scheduler.pushEvent(EventTypes::HBlank, firedEventTimestamp + 960);
            }
            _ => panic!("unknown event!")
        }
    }
}