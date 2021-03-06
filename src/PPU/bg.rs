use ppu::PPU;
use crate::PPU::*;
use crate::sign_extend_32;
use crate::helpers::get8BitColor;

const AFFINE_BG_SIZES: [u32; 4] = [128, 256, 512, 1024]; // Affine BGs are always square bois

impl PPU {
    
    pub fn renderMode0(&mut self) {
        self.fetchSprites();
        //self.renderNonAffineBG(0);
        //self.renderNonAffineBG(1);
        //self.renderNonAffineBG(2);
        //self.renderNonAffineBG(3);

        for prio in 0..4 {
            self.renderSprites(prio);
            if self.bg_controls[0].getPriority() == prio {self.renderNonAffineBG(0)}
            if self.bg_controls[1].getPriority() == prio {self.renderNonAffineBG(1)}
            if self.bg_controls[2].getPriority() == prio {self.renderNonAffineBG(2)}
            if self.bg_controls[3].getPriority() == prio {self.renderNonAffineBG(3)}
        }
    }

    pub fn renderMode1(&mut self) {
        self.fetchSprites();

        for prio in 0..4 {
            self.renderSprites(prio);
            if self.bg_controls[0].getPriority() == prio {self.renderNonAffineBG(0)}
            if self.bg_controls[1].getPriority() == prio {self.renderNonAffineBG(1)}
            if self.bg_controls[2].getPriority() == prio {self.renderAffineBG(2)}
        }
    }

    pub fn renderMode3(&mut self) { // Mode 3 stub
        let mut mapDataBase = self.vcount as u32 * 240 * 2;
        let mut bufferIndex = self.vcount as usize * 240 * 4;

        for i in 0..240 {
            let pixel = self.readVRAM16(mapDataBase); 
            let red = get8BitColor((pixel & 0x1F) as u8); //R
            let green = get8BitColor(((pixel >> 5) & 0x1F) as u8); //G
            let blue = get8BitColor(((pixel >> 10) & 0x1F) as u8); //B

            self.pixels[bufferIndex] = red;
            self.pixels[bufferIndex+1] = green;
            self.pixels[bufferIndex+2] = blue;

            mapDataBase += 2;
            bufferIndex += 4;
        }
    }

    pub fn renderNonAffineBG (&mut self, bgNum: usize) {
        if self.dispcnt.getRaw() & (1 << (8 + bgNum)) == 0 { // If the background is disabled, exit
            return;
        }

        let bgcnt = &self.bg_controls[bgNum];
        let tileDataBase = (bgcnt.getTileDataBase() as u32) << 14;
        let mut mapDataBase = (bgcnt.getMapDataBase() as u32) << 11;
        let is8bpp = bgcnt.getBitDepth() == 1;

        let y = self.vcount + self.bg_vofs[bgNum].getOffset();
        let hofs = self.bg_hofs[bgNum].getOffset() as u32;
        let bg_size = self.bg_controls[bgNum].getSize();

        match bg_size {
            0 | 1 => {} // 32x32, 64x32
            2 => { // 32x64
                if y & 511 > 255 {
                    mapDataBase += 0x800;
                }
            }
            _ => { // 64x64
                 if y & 511 > 255 {
                    mapDataBase += 0x800 * 2;
                }
            }
        }

        let mapStart = mapDataBase + ((y as u32 >> 3) & 31) * 64; // & 31 => wrap around the 32x32 tile map (TODO: add big map support)

        for x in 0..240 {
            
            if self.currentLine[x] != 0 { // If the pixel has already been drawn over by a higher prio BG, skip it
                continue
            }

            let x_coord = x as u32 + hofs;
            let mut tile_x = x_coord & 7;
            let mut tile_y = y & 7;
            let mut mapAddr = mapStart + (((x_coord >> 3) & 31) << 1);

            match bg_size { // TODO: Make branchless
                0 | 2 => {}, // 32x32, 32x64
                _ => { // 64x32, 64x64
                    if (x_coord & 511) > 255 {
                        mapAddr += 0x800;
                   }
                }
            }

            let mapEntry = self.readVRAM16(mapAddr);
            let tileNum = (mapEntry & 0x3FF) as u32;
            let palNum = (mapEntry >> 12) as u8;

            tile_x ^= (sign_extend_32!(mapEntry, 11) >> 12) & 7; // fast-ish flipping impl. TODO: Check if it's actually faster
            tile_y ^= (sign_extend_32!(mapEntry, 12) >> 13) as u16 & 7;
            //if isBitSet!(mapEntry, 10) { tile_x ^= 7; } // horizontal tile flip
            //if isBitSet!(mapEntry, 11) { tile_y ^= 7; } // vertical tile flip

            let mut tileAddr = tileDataBase;
            let mut pixel: u8;
            
            if is8bpp {
                tileAddr += tileNum * 64;
                tileAddr += tile_y as u32 * 8;
                tileAddr += tile_x;
                
                pixel = self.VRAM[tileAddr as usize]
            }
            
            else {
                tileAddr += tileNum * 32;
                tileAddr += tile_y as u32 * 4;
                tileAddr += (tile_x >> 1);

                let twoDots = self.VRAM[tileAddr as usize];
                pixel = (twoDots >> ((tile_x & 1) << 2)) & 0xF;

                if pixel != 0 {
                    pixel += palNum * 16
                }
            }

            self.currentLine[x as usize] = pixel as u16;
        }
    }

    pub fn renderAffineBG (&mut self, bg_num: usize) {
        if self.dispcnt.getRaw() & (1 << (8 + bg_num)) == 0 { // If the background is disabled, exit
            return;
        }

        let bgcnt = &self.bg_controls[bg_num];
        let tileDataBase = (bgcnt.getTileDataBase() as u32) << 14;
        let mut mapDataBase = (bgcnt.getMapDataBase() as u32) << 11;

        assert!(bgcnt.getDisplayAreaOverflow() == 0);
        let pa = self.aff_bg_pa[bg_num - 2].getRaw() as i32;
        let pc = self.aff_bg_pc[bg_num - 2].getRaw() as i32;

        let dx = self.aff_bg_dx[bg_num - 2].getRaw() as i16 as i32;
        let dy = self.aff_bg_dy[bg_num - 2].getRaw() as i16 as i32;

        let size = AFFINE_BG_SIZES[bgcnt.getSize() as usize]; // affine BGs are always square

        // non affine code

        let y = self.vcount;

        for x in 0..240 {
            
            if self.currentLine[x] != 0 { // If the pixel has already been drawn over by a higher prio BG, skip it
                continue
            }

            let x_coord = ((dx + pa * (x as i32)) >> 8) as u32;
            let y_coord = ((dy + pc * (x as i32)) >> 8) as u32;
            let mapStart = mapDataBase + ((y_coord as u32 >> 3) & 31) * 64; // & 31 => wrap around the 32x32 tile map (TODO: add big map support)

            if x_coord > size || y_coord > size {
                continue;
            }

            let mut tile_x = x_coord & 7;
            let mut tile_y = y_coord as u16 & 7;
            let mapAddr = mapStart + (((x_coord >> 3) & 31) << 1);


            let mapEntry = self.readVRAM16(mapAddr);
            let tileNum = (mapEntry & 0x3FF) as u32;

            tile_x ^= (sign_extend_32!(mapEntry, 11) >> 12) & 7; // fast-ish flipping impl. TODO: Check if it's actually faster
            tile_y ^= (sign_extend_32!(mapEntry, 12) >> 13) as u16 & 7;

            let mut tileAddr = tileDataBase;

            tileAddr += tileNum * 64;
            tileAddr += tile_y as u32 * 8;
            tileAddr += tile_x;
                
            let pixel = self.VRAM[tileAddr as usize];
            
            self.currentLine[x as usize] = pixel as u16;
        }
        //self.renderNonAffineBG(bg_num);
    }

    // simple stub for AW
    pub fn renderMode4 (&mut self) {
        let mut vramIndex = (self.vcount * 240) as usize;

        for x in 0..240 {
            self.currentLine[x] = self.VRAM[vramIndex] as u16;
            vramIndex += 1;
        }
    }
}