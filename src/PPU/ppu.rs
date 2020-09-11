use crate::PPU::*;
use crate::io::DISPSTAT;
use crate::io::DISPCNT;
use crate::io::BGCNT;
use crate::helpers::get8BitColor;

const RENDERING_MODE_CYCLES: u32 = 960;
const HBLANK_MODE_CYCLES: u32 = 272;
const CYCLES_PER_LINE: u32 = RENDERING_MODE_CYCLES + HBLANK_MODE_CYCLES;
const WIDTH: u32 = 240;
const HEIGHT: u32 = 160;

pub enum PPUModes {
    Rendering,
    HBlank,
    VBlank
}

pub struct PPU {
    pub dispcnt: DISPCNT,
    pub dispstat: DISPSTAT,
    pub bg_controls: [BGCNT; 4],
    pub vcount: u16, // Only lower 8 bits are used on the GBA

    pub paletteRAM: [u8; 1024],
    pub VRAM: [u8; 96 * 1024],
    pub OAM:  [u8; 1024],

    mode: PPUModes,
    cycles: u32,
    pub bufferIndex: usize,
    pub pixels: [u8; 240 * 160 * 4],
    pub isFrameReady: bool
}

impl PPU {
    pub fn new() -> PPU {
        PPU {
            dispcnt: DISPCNT(0),
            dispstat: DISPSTAT(0),
            bg_controls: [BGCNT(0), BGCNT(0), BGCNT(0), BGCNT(0)],
            vcount: 0,
            
            paletteRAM: [0; 1024],
            VRAM: [0; 96 * 1024],
            OAM:  [0; 1024],

            pixels: [0; 240 * 160 * 4],
            mode: PPUModes::Rendering,
            cycles: 0,
            bufferIndex: 0,
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

    pub fn readPalette16 (&self, palNum: u8) -> u16 {
        (self.paletteRAM[palNum as usize * 2] as u16) | ((self.paletteRAM[palNum as usize * 2 + 1] as u16) << 8)
    }

    pub fn readVRAM16 (&self, address: u32) -> u16 {
        (self.VRAM[address as usize] as u16) | ((self.VRAM[address as usize +1] as u16) << 8)
    }

    pub fn renderScanline(&mut self) {
        // TODO: Handle multiple BGs
        let bgcnt = &self.bg_controls[0];
        let tileDataBase = (bgcnt.getTileDataBase() as u32) << 14;
        let mapDataBase = (bgcnt.getMapDataBase() as u32) << 11;
        let is8bpp = bgcnt.getBitDepth() == 1;

        match self.dispcnt.getMode() {
            0 => self.renderMode0(mapDataBase, tileDataBase, is8bpp),
            4 => self.renderMode4(),
            _ => println!("Unimplemented BG mode {}", self.dispcnt.getMode())
        }
    }

    pub fn renderBuffer(&mut self) {
        self.isFrameReady = true;
    }
}