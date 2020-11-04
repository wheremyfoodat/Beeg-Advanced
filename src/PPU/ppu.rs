use crate::io::{BGCNT, DISPSTAT, DISPCNT, BGOFS, BGRefPoint, RotationAndScalingParam};
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
    
    // affine regs
    pub aff_bg_pa: [RotationAndScalingParam; 2], // BG Rotation/Scaling Parameter A
    pub aff_bg_pb: [RotationAndScalingParam; 2], // BG Rotation/Scaling Parameter B
    pub aff_bg_pc: [RotationAndScalingParam; 2], // BG Rotation/Scaling Parameter C
    pub aff_bg_pd: [RotationAndScalingParam; 2], // BG Rotation/Scaling Parameter D
    pub aff_bg_dx: [BGRefPoint; 2], // BG2 Reference Point X-Coordinate
    pub aff_bg_dy: [BGRefPoint; 2], // BG2 Reference Point Y-Coordinate

    pub bldy: u32,
    pub vcount: u16, // Only lower 8 bits are used on the GBA

    pub paletteRAM: [u8; 1024],
    pub VRAM: Vec<u8>,
    pub OAM:  Vec<u8>,

    pub pixels: Vec<u8>,
    pub paletteCache: [[u8; 3]; 512],
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

            aff_bg_pa: [RotationAndScalingParam(0), RotationAndScalingParam(0)], // BG Rotation/Scaling Parameter A
            aff_bg_pb: [RotationAndScalingParam(0), RotationAndScalingParam(0)], // BG Rotation/Scaling Parameter B
            aff_bg_pc: [RotationAndScalingParam(0), RotationAndScalingParam(0)], // BG Rotation/Scaling Parameter C
            aff_bg_pd: [RotationAndScalingParam(0), RotationAndScalingParam(0)], // BG Rotation/Scaling Parameter D

            aff_bg_dx: [BGRefPoint(0), BGRefPoint(0)], // BG Reference Point X-Coordinate
            aff_bg_dy: [BGRefPoint(0), BGRefPoint(0)], // BG Reference Point Y-Coordinate
            
            bldy: 0,
            vcount: 0,
            
            paletteRAM: [0; 1024],
            VRAM: vec![0; 96 * 1024],
            OAM:  vec![0; 1024],

            pixels: vec![0xFF; WIDTH * HEIGHT * 4],
            paletteCache: [[0xFF; 3]; 512],
            sprites: vec![],
            interruptFlags: 0,

            currentLine: [0; WIDTH]
        }
    }

    #[inline(always)]
    pub fn readPalette16 (&self, palNum: u16) -> u16 {
        (self.paletteRAM[palNum as usize * 2] as u16) | ((self.paletteRAM[palNum as usize * 2 + 1] as u16) << 8)
    }

    #[inline(always)]
    pub fn readVRAM16 (&self, address: u32) -> u16 {
        (self.VRAM[address as usize] as u16) | ((self.VRAM[address as usize +1] as u16) << 8)
    }

    #[inline(always)]
    pub fn readOAM16 (&self, address: usize) -> u16 {
        (self.OAM[address] as u16) | ((self.OAM[address + 1] as u16) << 8)
    }

    // cache the newly written BGR555 palette value as an RGB888 value
    #[inline(always)]
    pub fn updatePalette (&mut self, palNum: u16) {
        let palette = self.readPalette16(palNum);
        let red = get8BitColor((palette & 0x1F) as u8);          // red
        let green = get8BitColor(((palette >> 5) & 0x1F) as u8); // green
        let blue = get8BitColor(((palette >> 10) & 0x1F) as u8); // blue

        self.paletteCache[palNum as usize][0] = red;
        self.paletteCache[palNum as usize][1] = green;
        self.paletteCache[palNum as usize][2] = blue;
    }

    pub fn renderScanline(&mut self) {
        
        for i in 0..WIDTH {
            self.currentLine[i] = 0;
        }

        match self.dispcnt.getMode() {
            0 => self.renderMode0(),
            1 => self.renderMode1(),
            //1 => self.renderMode1(),
            3 => { self.renderMode3(); return; }, // Mode 3 isn't palette based, so we gotta early exit
            4 => self.renderMode4(),
            _ => panic!("Unimplemented BG mode {}", self.dispcnt.getMode())
        }

        let mut bufferIndex = self.vcount as usize * WIDTH * 4; // Get the framebuffer position of the current line

        for i in 0..WIDTH { // Copy the rendered line to the fb
            // store rgb888 color to buffer
            self.pixels[bufferIndex] = self.paletteCache[self.currentLine[i] as usize][0];
            self.pixels[bufferIndex+1] = self.paletteCache[self.currentLine[i] as usize][1];
            self.pixels[bufferIndex+2] = self.paletteCache[self.currentLine[i] as usize][2];
            bufferIndex += 4;
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

    pub fn incrementAffineRegs(&mut self) {
        // todo: Make aff regs latch
        self.aff_bg_dx[0].setRaw(self.aff_bg_dx[0].getRaw() + self.aff_bg_pb[0].getRaw() as u32); // inc BG2X by BG2PB
        self.aff_bg_dx[1].setRaw(self.aff_bg_dx[1].getRaw() + self.aff_bg_pb[1].getRaw() as u32);
        self.aff_bg_dy[0].setRaw(self.aff_bg_dy[0].getRaw() + self.aff_bg_pd[0].getRaw() as u32);
        self.aff_bg_dy[1].setRaw(self.aff_bg_dy[1].getRaw() + self.aff_bg_pd[1].getRaw() as u32);
    }
}