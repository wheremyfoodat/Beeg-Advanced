use crate::mem::*;
use crate::ppu::*;

pub struct Bus {
    mem: Memory,
    ppu: PPU
}

impl Bus {
    pub fn new(romPath: String) -> Bus {
        Bus {
            mem: Memory::new(romPath),
            ppu: PPU::new()
        }
    }

    pub fn stepComponents(&mut self, cycles: u32) {
        self.ppu.step(cycles);
    }

    pub fn read8 (&self, address: u32) -> u8 {
        todo!("Unimplemented 8-bit read at address {:08X}", address);
    }

    pub fn read16 (&self, address: u32) -> u16 {
        debug_assert!((address & 1) == 0);
        let mut val: u16;

        match (address >> 24) & 0xF { // these 4 bits show us which memory range the addr belongs to
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
        todo!("8-bit writes unimplemented")
    }

    pub fn write16 (&mut self, address: u32, val: u16) {
        match (address >> 24) & 0xF { // these 4 bits show us which memory range the addr belongs to
            5 => {
                self.ppu.paletteRAM[(address - 0x5000000) as usize] = (val & 0xFF) as u8;
                self.ppu.paletteRAM[(address - 0x5000000 + 1) as usize] = (val >> 8) as u8;
            }

            _ => {
                todo!("Unimplemented 16-bit write to addr {}", address);
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

            4 => self.writeIO32(address, val),
            _=> panic!("32-bit write to unimplemented mem addr {:08X}\n", address)
        }
    }

    pub fn readIO16 (&self, address: u32) -> u16 {
        match address {
            0x4000004 => self.ppu.dispstat.getRaw(),
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
}