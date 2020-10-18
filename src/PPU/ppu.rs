use crate::io::{BGCNT, DISPSTAT, DISPCNT, BGOFS};
use crate::helpers::get8BitColor;
use crate::PPU::sprites::Sprite;

const HBLANK_MODE_CYCLES: u32 = 272;
const CYCLES_PER_LINE: u32 = 1232;
pub const WIDTH: usize = 240;
pub const HEIGHT: usize = 160;

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

    pub pixels: [u8; HEIGHT * WIDTH * 4],
    pub sprites: Vec<Sprite>,
    pub interruptFlags: u16,
    pub currentLine: [u16; WIDTH] // Palette indices for each pixel of the line. Used for multiple BG rendering
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

            pixels: [0xFF; 240 * 160 * 4],
            sprites: vec![],
            interruptFlags: 0,

            currentLine: [0; WIDTH],
        }
    }

    pub fn readPalette16 (&self, palNum: u16) -> u16 {
        (self.paletteRAM[palNum as usize * 2] as u16) | ((self.paletteRAM[palNum as usize * 2 + 1] as u16) << 8)
    }

    pub fn readVRAM16 (&self, address: u32) -> u16 {
        (self.VRAM[address as usize] as u16) | ((self.VRAM[address as usize +1] as u16) << 8)
    }

    pub fn readOAM16 (&self, address: usize) -> u16 {
        (self.OAM[address] as u16) | ((self.OAM[address + 1] as u16) << 8)
    }

    pub fn renderScanline(&mut self) {
        
        for i in 0..WIDTH {
            self.currentLine[i] = 0;
        }

        match self.dispcnt.getMode() {
            0 => self.renderMode0(),
            1 => self.renderMode0(),
            //3 => { self.renderMode3(); return; }, // Mode 3 isn't palette based, so we gotta early exit
            4 => self.renderMode4(),
            _ => panic!("Unimplemented BG mode {}", self.dispcnt.getMode())
        }

        let mut bufferIndex = self.vcount as usize * WIDTH * 4; // Get the framebuffer position of the current line

        for i in 0..WIDTH { // Copy the rendered line to the fb
            let palette = self.readPalette16(self.currentLine[i]);

            self.pixels[bufferIndex] = get8BitColor((palette & 0x1F) as u8);          // red
            self.pixels[bufferIndex+1] = get8BitColor(((palette >> 5) & 0x1F) as u8); // green
            self.pixels[bufferIndex+2] = get8BitColor(((palette >> 10) & 0x1F) as u8); // blue
            bufferIndex += 4; // +4 instead of +3 cause the framebuffer contains Alpha too (which is always FF)
        }
    }

    // Compare LY with LYC/VCounter, return true if an interrupt is to be scheduled.
    pub fn compareLYC (&mut self) -> bool { 
        let lyc = self.dispstat.getLYC();
        if self.vcount == lyc {
            self.dispstat.setCoincidenceFlag(1);
            if self.dispstat.getLYCIRQEnable() == 1 {
                self.interruptFlags |= 0b100;
                return true;
            }

            false
        }

        else {
            self.dispstat.setCoincidenceFlag(0);
            false
        }
    }
}