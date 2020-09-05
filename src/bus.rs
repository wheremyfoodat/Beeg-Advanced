use crate::mem::*;

pub struct Bus {
    mem: Memory
}

impl Bus {
    pub fn new(romPath: String) -> Bus {
        Bus {
            mem: Memory::new(romPath)
        }
    }

    pub fn read8 (&self, address: u32) -> u8 {
        todo!("Unimplemented 8-bit read at address {:08X}", address);
    }

    pub fn read16 (&self, address: u32) -> u16 {
        todo!("Unimplemented 16-bit read at address {:08X}", address);
    }

    pub fn read32 (&self, address: u32) -> u32 {
        let mut val: u32;

        match (address >> 24) & 0xF { // these 4 bits show us which memory range the addr belongs to
            8 => {
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

    }

    pub fn write16 (&mut self, address: u32, val: u16) {

    }

    pub fn write32 (&mut self, address: u32, val: u32) {
        
    }
}