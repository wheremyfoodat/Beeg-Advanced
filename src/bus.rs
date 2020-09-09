use crate::mem::*;
use crate::ppu::*;
use crate::joypad::Joypad;

pub struct Bus {
    mem: Memory,
    pub ppu: PPU,
    pub joypad: Joypad
}

impl Bus {
    pub fn new(romPath: String) -> Bus {
        Bus {
            mem: Memory::new(romPath),
            ppu: PPU::new(),
            joypad: Joypad::new()
        }
    }

    pub fn stepComponents(&mut self, cycles: u32) {
        self.ppu.step(cycles);
    }

    pub fn read8 (&self, address: u32) -> u8 {
        match (address >> 24) & 0xF {
            3 => self.mem.iWRAM[(address - 0x3000000) as usize],
            8 => self.mem.ROM[(address - 0x8000000) as usize],
            _ => todo!("Unimplemented 8-bit read at address {:08X}", address)
        }
    }

    pub fn read16 (&self, address: u32) -> u16 {
        debug_assert!((address & 1) == 0);
        let mut val: u16;

        match (address >> 24) & 0xF { // these 4 bits show us which memory range the addr belongs to
            3 => {
                val = self.mem.iWRAM[(address - 0x3000000) as usize] as u16;
                val |= (self.mem.iWRAM[(address - 0x3000000 + 1) as usize] as u16) << 8;
            }

            4 => val = self.readIO16(address),
            
            8 | 9 => {
                    val = self.mem.ROM[(address - 0x8000000) as usize] as u16;
                    val |= (self.mem.ROM[(address - 0x8000000 + 1) as usize] as u16) << 8;
            },

            _ => panic!("16-bit read from unimplemented mem addr {:08X}\n", address)
        }

        val
    }

    pub fn read32 (&self, address: u32) -> u32 {
        debug_assert!((address & 3) == 0);
        let mut val: u32;

        match (address >> 24) & 0xF { // these 4 bits show us which memory range the addr belongs to
            3 => {
                val = self.mem.iWRAM[(address - 0x3000000) as usize] as u32;
                val |= (self.mem.iWRAM[(address - 0x3000000 + 1) as usize] as u32) << 8;
                val |= (self.mem.iWRAM[(address - 0x3000000 + 2) as usize] as u32) << 16;
                val |= (self.mem.iWRAM[(address - 0x3000000 + 3) as usize] as u32) << 24;
            },
            
            8 | 9 => {
                val = self.mem.ROM[(address - 0x8000000) as usize] as u32;
                val |= (self.mem.ROM[(address - 0x8000000 + 1) as usize] as u32) << 8;
                val |= (self.mem.ROM[(address - 0x8000000 + 2) as usize] as u32) << 16;
                val |= (self.mem.ROM[(address - 0x8000000 + 3) as usize] as u32) << 24;
            },

            _=> panic!("32-bit read from unimplemented mem addr {:08X}\n", address)
        }

        val
    }

    pub fn write8 (&mut self, address: u32, val: u8) {
        match (address >> 24) & 0xF {
            3 => self.mem.iWRAM[(address - 0x3000000) as usize] = val,
            _ => todo!("Unimplemented 8-bit write at address {:08X}", address)
        }
    }

    pub fn write16 (&mut self, address: u32, val: u16) {
        match (address >> 24) & 0xF { // these 4 bits show us which memory range the addr belongs to
            3 => {
                self.mem.iWRAM[(address - 0x3000000) as usize] = (val & 0xFF) as u8;
                self.mem.iWRAM[(address - 0x3000000 + 1) as usize] = (val >> 8) as u8;
            }
            
            5 => {
                self.ppu.paletteRAM[(address - 0x5000000) as usize] = (val & 0xFF) as u8;
                self.ppu.paletteRAM[(address - 0x5000000 + 1) as usize] = (val >> 8) as u8;
            }

            6 => {
               // if val != 0 && address == 0x6000138 {panic!("Wrote {:04X} at {:08X}", val, address)}
                self.ppu.VRAM[(address - 0x6000000) as usize] = (val & 0xFF) as u8;
                self.ppu.VRAM[(address - 0x6000000 + 1) as usize] = (val >> 8) as u8;
            }

            _ => {
                todo!("Unimplemented 16-bit write to addr {:08X}", address);
            }
        }
    }

    pub fn write32 (&mut self, address: u32, val: u32) {
        match (address >> 24) & 0xF { // these 4 bits show us which memory range the addr belongs to
            2 => {
                self.mem.eWRAM[(address - 0x2000000) as usize] = (val & 0xFF) as u8;
                self.mem.eWRAM[(address - 0x2000000 + 1) as usize] = (val >> 8) as u8;
                self.mem.eWRAM[(address - 0x2000000 + 2) as usize] = (val >> 16) as u8;
                self.mem.eWRAM[(address - 0x2000000 + 3) as usize] = (val >> 24) as u8;
            }

            3 => {
                self.mem.iWRAM[(address - 0x3000000) as usize] = (val & 0xFF) as u8;
                self.mem.iWRAM[(address - 0x3000000 + 1) as usize] = (val >> 8) as u8;
                self.mem.iWRAM[(address - 0x3000000 + 2) as usize] = (val >> 16) as u8;
                self.mem.iWRAM[(address - 0x3000000 + 3) as usize] = (val >> 24) as u8;
            }

            6 => {
                self.ppu.VRAM[(address - 0x6000000) as usize] = (val & 0xFF) as u8;
                self.ppu.VRAM[(address - 0x6000000 + 1) as usize] = (val >> 8) as u8;
                self.ppu.VRAM[(address - 0x6000000 + 2) as usize] = (val >> 16) as u8;
                self.ppu.VRAM[(address - 0x6000000 + 3) as usize] = (val >> 24) as u8;
            }

            4 => self.writeIO32(address, val),
            _=> panic!("32-bit write to unimplemented mem addr {:08X}\n", address)
        }
    }

    pub fn readIO16 (&self, address: u32) -> u16 {
        match address {
            0x4000004 => self.ppu.dispstat.getRaw(),
            0x4000130 => self.joypad.keyinput.getRaw(),
            _ => panic!("Unimplemented 16-bit read from MMIO address {:08X}", address)
        }
    }

    pub fn writeIO32 (&mut self, address: u32, val: u32) {
        match address {
            0x4000000 => self.ppu.dispcnt = val,
            0x4000208 => self.mem.ime = (val & 1) == 1,
            _ => todo!("Unimplemented 32-bit write to IO address {:08X}\n", address)
        }
    }

    pub fn isFrameReady (&self) -> bool {
        self.ppu.isFrameReady
    }
}