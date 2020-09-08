use crate::io::DISPSTAT;

const RENDERING_MODE_CYCLES: u32 = 960;
const HBLANK_MODE_CYCLES: u32 = 272;
const CYCLES_PER_LINE: u32 = RENDERING_MODE_CYCLES + HBLANK_MODE_CYCLES;

pub enum PPUModes {
    Rendering,
    HBlank,
    VBlank
}

pub struct PPU {
    pub dispcnt: u32,
    pub dispstat: DISPSTAT,
    pub vcount: u8,

    pub paletteRAM: [u8; 1024],
    pub VRAM: [u8; 96 * 1024],
    pub OAM:  [u8; 1024],

    mode: PPUModes,
    cycles: u32
}

impl PPU {
    pub fn new() -> PPU {
        PPU {
            dispcnt: 0,
            dispstat: DISPSTAT(0),
            vcount: 0,
            
            paletteRAM: [0; 1024],
            VRAM: [0; 96 * 1024],
            OAM:  [0; 1024],

            mode: PPUModes::Rendering,
            cycles: 0
        }
    }

    pub fn step(&mut self, cycles: u32) {
        self.cycles += cycles;
        
        match self.mode {
            PPUModes::Rendering => {
                if self.cycles >= RENDERING_MODE_CYCLES {
                    self.cycles -= RENDERING_MODE_CYCLES;
                    self.switchMode(PPUModes::HBlank);
                }
            }

            PPUModes::HBlank => {
                if self.cycles >= HBLANK_MODE_CYCLES {
                    self.cycles -= HBLANK_MODE_CYCLES;
                    self.vcount += 1;
                    if self.vcount == 160 {
                        self.switchMode(PPUModes::VBlank)
                    }

                    else {
                        self.switchMode(PPUModes::Rendering);
                    }
                }
            }

            PPUModes::VBlank => {
                if self.cycles >= HBLANK_MODE_CYCLES {
                    self.dispstat.setHBlankFlag(1);
                    if self.cycles >= CYCLES_PER_LINE {
                        self.cycles -= CYCLES_PER_LINE;
                        self.vcount += 1;

                        self.dispstat.setHBlankFlag(0);
                        if self.vcount == 228 {
                            self.vcount = 0;
                            self.switchMode(PPUModes::Rendering);
                        }
                    }
                }
            }
        }
    }

    pub fn switchMode(&mut self, newMode: PPUModes) {
        self.mode = newMode;
        match self.mode {
            PPUModes::Rendering => {
                self.dispstat.setHBlankFlag(0);
                self.dispstat.setVBlankFlag(0);
            }

            PPUModes::HBlank => {
                self.renderScanline();
                self.dispstat.setHBlankFlag(1);
            }

            PPUModes::VBlank => {
                self.renderBuffer();
                self.dispstat.setVBlankFlag(1);
            }
        }
    }

    pub fn renderScanline(&mut self) {

    }

    pub fn renderBuffer(&mut self) {

    }
}