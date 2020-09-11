use ppu::PPU;
use crate::PPU::*;
use crate::helpers::get8BitColor;
use crate::isBitSet;

impl PPU {
    // TODO: Add support for layers other than 0
    pub fn renderMode0 (&mut self, mapDataBase: u32, tileDataBase: u32, is8bpp: bool) {
        self.bufferIndex = self.vcount as usize * 240 * 4;
        
        let y = self.vcount + self.bg_vofs[0].getOffset();
        let hofs = self.bg_hofs[0].getOffset() as u32;
        let bg_size = self.bg_controls[0].getSize();

        let mapStart = mapDataBase + ((y as u32 >> 3) & 31) * 64; // & 31 => wrap around the 32x32 tile map (TODO: add big map support)

        for x in 0..240 {
            let x_coord = x + hofs;
            let mut tile_x = x_coord & 7;
            let mut tile_y = y & 7;
            let mut mapAddr = mapStart + (((x_coord >> 3) & 31) << 1);

            match bg_size {
                0 => {}, // 32x32
                1 => { // 64x32
                    if (x_coord & 511) > 255 {
                        mapAddr += 0x800;
                    }
                }
                _ => panic!("Unimplemented BG size for mode 0!\n")
            }

            let mapEntry = self.readVRAM16(mapAddr);
            let tileNum = (mapEntry & 0x3FF) as u32;
            let palNum = (mapEntry >> 12) as u8;

            if isBitSet!(mapEntry, 10) { tile_x ^= 7; } // horizontal tile flip
            if isBitSet!(mapEntry, 11) { tile_y ^= 7; } // vertical tile flip

            let mut tileAddr = tileDataBase;
            
            if is8bpp {
                tileAddr += tileNum * 64;
                tileAddr += tile_y as u32 * 8;
                todo!("8bpp tile!")
            }
            
            else {
                tileAddr += tileNum * 32;
                tileAddr += tile_y as u32 * 4;
                tileAddr += (tile_x >> 1);

                let twoDots = self.VRAM[tileAddr as usize];
                let pixel: u8 = (twoDots >> ((tile_x & 1) << 2)) & 0xF;
                let palette = self.readPalette16(pixel + palNum * 16);

                self.pixels[self.bufferIndex] = get8BitColor((palette & 0x1F) as u8);          // red
                self.pixels[self.bufferIndex+1] = get8BitColor(((palette >> 5) & 0x1F) as u8); // green
                self.pixels[self.bufferIndex+2] = get8BitColor(((palette >> 10) & 0x1F) as u8); // blue
                self.pixels[self.bufferIndex+3] = 255; // alpha (always opaque)
            }

            self.bufferIndex += 4;
        }
    }

    // simple stub for AW
    pub fn renderMode4 (&mut self) {
        self.bufferIndex = self.vcount as usize * 240 * 4;
        let mut vramIndex = (self.vcount * 240) as usize;

        for x in 0..240 {
            let palIndex = self.VRAM[vramIndex];
            let palEntry = self.readPalette16(palIndex); // palettes store colors as BGR555
                
            self.pixels[self.bufferIndex] = get8BitColor((palEntry & 0x1F) as u8);          // red
            self.pixels[self.bufferIndex+1] = get8BitColor(((palEntry >> 5) & 0x1F) as u8); // green
            self.pixels[self.bufferIndex+2] = get8BitColor(((palEntry >> 10) & 0x1F) as u8); // blue
            self.pixels[self.bufferIndex+3] = 255;

            self.bufferIndex += 4;
            vramIndex += 1;
        }
    }
}