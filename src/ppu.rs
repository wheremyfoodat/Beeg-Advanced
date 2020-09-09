use crate::io::DISPSTAT;
use crate::helpers::get8BitColor;

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
    cycles: u32,
    pub pixels: [u8; 240 * 160 * 4],
    pub isFrameReady: bool
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

            pixels: [0; 240 * 160 * 4],
            mode: PPUModes::Rendering,
            cycles: 0,
            isFrameReady: false
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

    pub fn readPalette (&self, palNum: usize) -> u16 {
        (self.paletteRAM[palNum * 2] as u16) | ((self.paletteRAM[palNum * 2 + 1] as u16) << 8)
    }

    pub fn renderScanline(&mut self) {

    }

    pub fn renderBuffer(&mut self) {
        let mut bufferIndex = 0;

        for i in 0..240 * 160 {
            let palIndex = self.VRAM[i] as usize;
            let palEntry = self.readPalette(palIndex); // palettes store colors as BGR555
            
            self.pixels[bufferIndex] = get8BitColor((palEntry & 0x1F) as u8);          // red
            self.pixels[bufferIndex+1] = get8BitColor(((palEntry >> 5) & 0x1F) as u8); // green
            self.pixels[bufferIndex+2] = get8BitColor(((palEntry >> 10) & 0x1F) as u8); // blue
            self.pixels[bufferIndex+3] = 255;

            bufferIndex += 4
        }

        self.isFrameReady = true;
    }
}