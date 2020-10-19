use ppu::PPU;
use crate::PPU::*;
use std::cmp::Ordering;
use crate::isBitSet;

const OAM_MAX: usize = 0x3FF;
const SPRITE_SIZES: [[[u16; 2]; 3]; 4] = [ // Look up table of all sprite x/y sizes
                                          // It goes like table[size][shape][x/y]
                        [[8, 8], [16, 8], [8, 16]],
                        [[16, 16], [32, 8], [8, 32]],
                        [[32, 32], [32, 16], [16, 32]],
                        [[64, 64], [64, 32], [32, 64]]
                    ];

pub struct Sprite {
    pub y_coord: u8,  // y coordinate of the sprite
    pub x_coord: u16, // x coordinate of the sprite

    pub tile_num: u16, // The tile the sprite uses
    pub priority: u16, // The priority of the sprite from 0 to 3
    pub palNum: u8, // The palette number the sprite uses (for 4bpp sprites)

    pub is8bpp: bool, // Whether the palette num for each pixel is contained in 4 or 8 bits
    pub h_flip: bool, // Whether the tile is horizontally flipped or not
    pub v_flip: bool, // Whether the tile is vertically flipped or not

    pub shape: u16, // 0: square. 1: horizontal. 2: vertical
    pub size: u16,
    pub doubleSize: bool
}

impl Sprite {
    pub fn new (attr0: u16, attr1: u16, attr2: u16, x_coord: u16) -> Sprite { // A sprite's characteristics are described in 3 16-bit "attributes"
        Sprite {
            y_coord: attr0 as u8,
            x_coord,

            tile_num: attr2 & 0x3FF,
            priority: (attr2 >> 10) & 3,
            palNum: ((attr2 >> 12) & 0xF) as u8,

            is8bpp: isBitSet!(attr0, 13),
            h_flip: isBitSet!(attr1, 12),
            v_flip: isBitSet!(attr1, 13),

            shape: (attr0 >> 14) & 3,
            size: (attr1 >> 14) & 3,
            doubleSize: isBitSet!(attr0, 9)
        }
    }
}

impl Ord for Sprite {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.priority).cmp(&self.priority)
    }
}

impl PartialOrd for Sprite {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Sprite  {
    fn eq(&self, other: &Self) -> bool {
        other.priority == self.priority
    }
}

impl Eq for Sprite  { }


impl PPU {
    pub fn fetchSprites(&mut self) {
        self.sprites = vec![];

        for i in (0..OAM_MAX).step_by(8) {
            let attr0 = self.readOAM16(i);
            let attr1 = self.readOAM16(i + 2);

            if !isBitSet!(attr0, 8) && isBitSet!(attr0, 9) { // Check "OBJ disable" bit
                continue
            } 

            let mut x_coord = (attr1 & 0x1FF);
            let mut y_coord = (attr0 & 0xFF);

            let size = (attr1 >> 14) & 3;
            let shape = (attr0 >> 14) & 3;

            if x_coord >= 240 {x_coord -= 512}
            if y_coord >= 160 {y_coord -= 256}
            
            //x_coord &= 0x1FF;
            y_coord &= 255;

            let SPRITE_Y = SPRITE_SIZES[size as usize][shape as usize][1];  // Todo: Implement different sprite shapes and sizes
            let sprite_end = (SPRITE_Y + y_coord) & 0xFF;

            if (y_coord <= self.vcount && sprite_end > self.vcount) || (sprite_end < y_coord && self.vcount < sprite_end) {
                self.sprites.push(Sprite::new(attr0, attr1, self.readOAM16(i + 4), x_coord));
            } 
        }

        self.sprites.sort(); 
    }

    pub fn renderSprites(&mut self, prio: u16) { // TODO: Account for OBJ priority
        for sprite in &self.sprites {
            if sprite.priority != prio {continue;}

            let SPRITE_X = SPRITE_SIZES[sprite.size as usize][sprite.shape as usize][0];
            let SPRITE_Y = SPRITE_SIZES[sprite.size as usize][sprite.shape as usize][1];
            let linesSinceOBJStart = (self.vcount as u32 - sprite.y_coord as u32) & (SPRITE_Y as u32-1);
            
            //assert!(!sprite.doubleSize); 
            //if sprite.doubleSize {
            //    SPRITE_X *= 2;
            //    SPRITE_Y *= 2;
            //  }
                
            for i in 0..SPRITE_X {
                let x = sprite.x_coord + i;
                if x >= 240 {continue}
                if self.currentLine[x as usize] != 0 {continue}

                let mut tile_x = i;
                let mut tile_y = linesSinceOBJStart as u16;

                if sprite.h_flip {tile_x ^= SPRITE_X-1}
                if sprite.v_flip {tile_y ^= SPRITE_Y-1}

                let mut tile_addr = 0x10000; // TODO: Add sprite support for modes 3-5. These modes DO NOT use 0x10000 as the base, but 0x14000

                let mut pixel: u8;
                
                if sprite.is8bpp {
                    tile_addr += sprite.tile_num as u32 * 64;
                    //tile_addr += ((x as u32) >> 3) * 64;
                    //tile_addr += ((self.vcount as u32 & 31) >> 3) * 0x800;
                    panic!("We don't have 8bpp sprites ree");
                    tile_addr += tile_y as u32 * 8;
                    tile_addr += tile_x as u32;
                    
                    pixel = 0//self.VRAM[tile_addr as usize]
                }
                
                else {
                    tile_addr += sprite.tile_num as u32 * 32;
                    tile_addr += (tile_y & 7) as u32 * 4;
                    tile_addr += ((tile_x as u32 & 7) >> 1);
                    
                    tile_addr += ((tile_x as u32) >> 3) * 32;
                    
                    if self.dispcnt.OBJ1DMapping() { // TODO: Fix whatever in here breaks Mother 3
                        tile_addr += (tile_y as u32 / 8) * 32 * (SPRITE_X as u32 >> 3);
                    }

                    else {
                        tile_addr += ((tile_y as u32) >> 3) * 32 * 0x20; // TODO: Make sure this is correct
                        //println!("Non horizontal OBJ mapping! (Broken?)")
                    }

                    let twoDots = self.VRAM[tile_addr as usize];
                    pixel = (twoDots >> ((tile_x & 1) << 2)) & 0xF;

                    if pixel != 0 {
                        pixel += sprite.palNum * 16;
                    }
                }

                if pixel != 0 {
                    self.currentLine[x as usize] = pixel as u16 + 256;
                }

                else {
                    self.currentLine[x as usize] = 0;
                }
            }
        }
    }
}