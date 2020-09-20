use crate::io::{BGCNT, DISPSTAT, DISPCNT, BGOFS};
use crate::bus::Bus;

const RENDERING_MODE_CYCLES: u32 = 960;
const HBLANK_MODE_CYCLES: u32 = 272;
const CYCLES_PER_LINE: u32 = RENDERING_MODE_CYCLES + HBLANK_MODE_CYCLES;
pub const WIDTH: usize = 240;
pub const HEIGHT: usize = 160;

pub enum PPUModes {
    Rendering,
    HBlank,
    VBlank
}

pub struct PPU {
    pub dispcnt: DISPCNT,
    pub dispstat: DISPSTAT,
    pub bg_controls: [BGCNT; 4],
    pub bg_hofs: [BGOFS; 4],
    pub bg_vofs: [BGOFS; 4],
    pub vcount: u16, // Only lower 8 bits are used on the GBA

    pub paletteRAM: [u8; 1024],
    pub VRAM: Vec<u8>,
    pub OAM:  Vec<u8>,

    mode: PPUModes,
    cycles: u32,
    pub bufferIndex: usize,
    pub pixels: [u8; HEIGHT * WIDTH * 4],
    pub isFrameReady: bool,
    pub isInHBlank: bool, // For HBlank-in-vblank mode
    pub interruptFlags: u16
}

impl PPU {
    pub fn new() -> PPU {
        PPU {
            dispcnt: DISPCNT(0),
            dispstat: DISPSTAT(0),
            bg_controls: [BGCNT(0), BGCNT(0), BGCNT(0), BGCNT(0)],
            bg_hofs: [BGOFS(0), BGOFS(0), BGOFS(0), BGOFS(0)],
            bg_vofs: [BGOFS(0), BGOFS(0), BGOFS(0), BGOFS(0)],
            vcount: 0,
            
            paletteRAM: [0; 1024],
            VRAM: vec![0; 96 * 1024],
            OAM:  vec![0; 1024],

            pixels: [0; 240 * 160 * 4],
            mode: PPUModes::Rendering,
            cycles: 0,
            bufferIndex: 0,
            isFrameReady: false,
            isInHBlank: false,
            interruptFlags: 0
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
                    self.compareLYC();

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
                    if !self.isInHBlank && self.dispstat.getHBlankIRQEnable() == 1{ // Handle "HBlank in VBlank mode"
                        self.interruptFlags |= 0b10; // Request HBlank IRQ
                    }

                    self.isInHBlank = true;
                    
                    self.dispstat.setHBlankFlag(1);
                    if self.cycles >= CYCLES_PER_LINE {
                        self.cycles -= CYCLES_PER_LINE;
                        self.vcount += 1;
                        self.isInHBlank = false;

                        self.dispstat.setHBlankFlag(0);
                        if self.vcount == 228 {
                            self.vcount = 0;
                            self.switchMode(PPUModes::Rendering);
                        }

                        self.compareLYC();
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
                self.isInHBlank = false;
            }

            PPUModes::HBlank => {
                self.isInHBlank = true;
                if self.dispstat.getHBlankIRQEnable() == 1 {
                   self.interruptFlags |= 0b10; // Request HBlank IRQ
                }

                self.renderScanline();
                self.dispstat.setHBlankFlag(1);
            }

            PPUModes::VBlank => {
                if self.dispstat.getVBlankIRQEnable() == 1 {
                    self.isInHBlank = false;
                    self.interruptFlags |= 0b1; // Request VBlank IRQ
                   // println!("Fired VBlank IRQ!")
                }

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
            _ => panic!("Unimplemented BG mode {}", self.dispcnt.getMode())
        }
    }

    pub fn renderBuffer(&mut self) {
        self.isFrameReady = true;
    }

    pub fn compareLYC (&mut self) {
        let lyc = self.dispstat.getLYC();
        if self.vcount == lyc {
            self.dispstat.setCoincidenceFlag(1);
            if self.dispstat.getLYCIRQEnable() == 1 {
                self.interruptFlags |= 0b100;
            }
        }

        else {
            self.dispstat.setCoincidenceFlag(0);
        }
    }
}