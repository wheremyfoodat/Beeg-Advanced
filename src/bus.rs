use ppu::PPU;

use crate::mem::*;
use crate::PPU::ppu;
use crate::joypad::Joypad;

pub struct Bus {
    mem: Memory,
    pub ppu: PPU,
    pub joypad: Joypad,

    // some MMIO registers that don't really fit in the peripheral structs
    // interrupt registers
    pub ime: bool,  // interrupt master enable register
    pub ie: u16,    // interrupt enable register
    pub interrupt_requests : u16, // IF (interrupt request) register

    // stubbed MMIO registers that I need for the BIOS but haven't properly implemented yet
    soundbiasStub: u32,
    waitcnt: u16
} 

impl Bus {
    pub fn new(romPath: String) -> Bus {
        Bus {
            mem: Memory::new(romPath),
            ppu: PPU::new(),
            joypad: Joypad::new(),

            ime: false,
            ie: 0,
            interrupt_requests: 0,
            soundbiasStub: 0,
            waitcnt: 0
        }
    }

    pub fn stepComponents(&mut self, cycles: u32) {
        self.ppu.step(cycles);
    }

    pub fn read8 (&self, address: u32) -> u8 {
        match (address >> 24) & 0xF {
            0 => self.mem.BIOS[address as usize],
            2 => self.mem.eWRAM[(address & 0x3FFFF) as usize],
            3 => self.mem.iWRAM[(address & 0x7FFF) as usize],
            4 => self.readIO8(address),
            6 => self.ppu.VRAM[(address - 0x6000000) as usize],
            8 => self.mem.ROM[(address - 0x8000000) as usize],
            0xE => self.mem.SRAM[(address & 0xFFFF) as usize],
            _ => todo!("Unimplemented 8-bit read at address {:08X}", address)
        }
    }

    pub fn read16 (&self, address: u32) -> u16 {
        debug_assert!((address & 1) == 0);
        let mut val: u16;

        match (address >> 24) & 0xF { // these 4 bits show us which memory range the addr belongs to
            0 => {
                val = self.mem.BIOS[address as usize] as u16;
                val |= (self.mem.BIOS[(address + 1) as usize] as u16) << 8;
            },

            2 => {
                val = self.mem.eWRAM[(address & 0x3FFFF) as usize] as u16;
                val |= (self.mem.eWRAM[((address + 1) & 0x3FFFF) as usize] as u16) << 8;
            },
            
            3 => {
                val = self.mem.iWRAM[(address & 0x7FFF) as usize] as u16;
                val |= (self.mem.iWRAM[((address + 1) & 0x7FFF) as usize] as u16) << 8;
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

        if address > 0xFFFFFFF { 
            println!("Read from invalid memory");
            return 0xFFFFFFFF
        }

        match (address >> 24) & 0xF { // these 4 bits show us which memory range the addr belongs to
            0 => {
                val = self.mem.BIOS[address as usize] as u32;
                val |= (self.mem.BIOS[(address + 1) as usize] as u32) << 8;
                val |= (self.mem.BIOS[(address + 2) as usize] as u32) << 16;
                val |= (self.mem.BIOS[(address + 3) as usize] as u32) << 24;
            },

            2 => {
                val = self.mem.eWRAM[(address & 0x3FFFF) as usize] as u32;
                val |= (self.mem.eWRAM[((address + 1) & 0x3FFFF) as usize] as u32) << 8;
                val |= (self.mem.eWRAM[((address + 2) & 0x3FFFF) as usize] as u32) << 16;
                val |= (self.mem.eWRAM[((address + 3) & 0x3FFFF) as usize] as u32) << 24;
            },
            
            3 => {
                val = self.mem.iWRAM[(address & 0x7FFF) as usize] as u32;
                val |= (self.mem.iWRAM[((address + 1) & 0x7FFF) as usize] as u32) << 8;
                val |= (self.mem.iWRAM[((address + 2) & 0x7FFF) as usize] as u32) << 16;
                val |= (self.mem.iWRAM[((address + 3) & 0x7FFF) as usize] as u32) << 24;
            },

            4 => val = self.readIO32(address),

            7 => {
                val = self.ppu.OAM[(address & 0x3FF) as usize] as u32;
                val |= (self.ppu.OAM[((address + 1) & 0x3FF) as usize] as u32) << 8;
                val |= (self.ppu.OAM[((address + 2) & 0x3FF) as usize] as u32) << 16;
                val |= (self.ppu.OAM[((address + 3) & 0x3FF) as usize] as u32) << 24;
            },
            
            8 | 9 => {
                if address & 0xFFFFFF >= self.mem.ROM.len() as u32 {println!("OoB ROM access"); return 0}

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
            2 => self.mem.eWRAM[(address & 0x3FFFF) as usize] = val,
            3 => self.mem.iWRAM[(address & 0x7FFF) as usize] = val,
            4 => self.writeIO8(address, val),
            0xE => self.mem.SRAM[(address & 0xFFFF) as usize] = val,
            _ => todo!("Unimplemented 8-bit write at address {:08X}", address)
        }
    }

    pub fn write16 (&mut self, address: u32, val: u16) {
        debug_assert!((address & 1) == 0); 

        match (address >> 24) & 0xF { // these 4 bits show us which memory range the addr belongs to
            2 => {
                self.mem.eWRAM[(address & 0x3FFFF) as usize] = (val & 0xFF) as u8;
                self.mem.eWRAM[((address + 1) & 0x3FFFF) as usize] = (val >> 8) as u8;
            },

            3 => {
                self.mem.iWRAM[(address & 0x7FFF) as usize] = (val & 0xFF) as u8;
                self.mem.iWRAM[((address + 1) & 0x7FFF) as usize] = (val >> 8) as u8;
            }

            4 => self.writeIO16(address, val),
            
            5 => {
                self.ppu.paletteRAM[(address - 0x5000000) as usize] = (val & 0xFF) as u8;
                self.ppu.paletteRAM[(address - 0x5000000 + 1) as usize] = (val >> 8) as u8;
            }

            6 => {
                self.ppu.VRAM[(address - 0x6000000) as usize] = (val & 0xFF) as u8;
                self.ppu.VRAM[(address - 0x6000000 + 1) as usize] = (val >> 8) as u8;
            }

            _ => {
                todo!("Unimplemented 16-bit write to addr {:08X}", address);
            }
        }
    }

    pub fn write32 (&mut self, address: u32, val: u32) {
        debug_assert!((address & 3) == 0); 
        match (address >> 24) & 0xF { // these 4 bits show us which memory range the addr belongs to
            2 => {
                self.mem.eWRAM[(address & 0x3FFFF) as usize] = (val & 0xFF) as u8;
                self.mem.eWRAM[((address + 1) & 0x3FFFF) as usize] = (val >> 8) as u8;
                self.mem.eWRAM[((address + 2) & 0x3FFFF) as usize] = (val >> 16) as u8;
                self.mem.eWRAM[((address + 3) & 0x3FFFF) as usize] = (val >> 24) as u8;
            }

            3 => {
                self.mem.iWRAM[(address & 0x7FFF) as usize] = (val & 0xFF) as u8;
                self.mem.iWRAM[((address + 1) & 0x7FFF) as usize] = (val >> 8) as u8;
                self.mem.iWRAM[((address + 2) & 0x7FFF) as usize] = (val >> 16) as u8;
                self.mem.iWRAM[((address + 3) & 0x7FFF) as usize] = (val >> 24) as u8;
            }

            5 => {
                self.ppu.paletteRAM[(address - 0x5000000) as usize] = (val & 0xFF) as u8;
                self.ppu.paletteRAM[(address - 0x5000000 + 1) as usize] = (val >> 8) as u8;
                self.ppu.paletteRAM[(address - 0x5000000 + 2) as usize] = (val >> 16) as u8;
                self.ppu.paletteRAM[(address - 0x5000000 + 3) as usize] = (val >> 24) as u8;
            }

            6 => {
                self.ppu.VRAM[(address - 0x6000000) as usize] = (val & 0xFF) as u8;
                self.ppu.VRAM[(address - 0x6000000 + 1) as usize] = (val >> 8) as u8;
                self.ppu.VRAM[(address - 0x6000000 + 2) as usize] = (val >> 16) as u8;
                self.ppu.VRAM[(address - 0x6000000 + 3) as usize] = (val >> 24) as u8;
            }

            7 => {
                self.ppu.OAM[(address - 0x7000000) as usize] = (val & 0xFF) as u8;
                self.ppu.OAM[(address - 0x7000000 + 1) as usize] = (val >> 8) as u8;
                self.ppu.OAM[(address - 0x7000000 + 2) as usize] = (val >> 16) as u8;
                self.ppu.OAM[(address - 0x7000000 + 3) as usize] = (val >> 24) as u8;
            }

            4 => self.writeIO32(address, val),
            8 => println!("32-bit write to ROM at {:08X}", address),
            _=> println!("32-bit write to unimplemented mem addr {:08X}\n", address)
        }
    }

    pub fn readIO8 (& self, address: u32) -> u8 {
        match address {
            0x4000006 => self.ppu.vcount as u8,
            _ => {println!("Unimplemented 8-bit read from MMIO address {:08X}", address); 0}
        }
    }

    pub fn readIO16 (&self, address: u32) -> u16 {
        match address {
            0x4000000 => self.ppu.dispcnt.getRaw(),
            0x4000004 => self.ppu.dispstat.getRaw(),
            0x4000006 => self.ppu.vcount,
            0x4000088 => { println!("Read from SOUNDBIAS (Unimpl)"); self.soundbiasStub as u16},
            0x4000102 | 0x4000106 | 0x400010A | 0x400010E => { println!("Read from Timer control regs! (Unimpl)"); 0}
            0x4000130 => self.joypad.keyinput.getRaw(),
            0x4000202 => { println!("Read from IF which is semi-stubbed!"); self.ppu.interruptFlags}
            0x4000204 => self.waitcnt,
            0x4000208 => self.ime as u16,
            _ => {println!("Unimplemented 16-bit read from MMIO address {:08X}", address); 0}
        }
    }

    pub fn readIO32 (&self, address: u32) -> u32 {
        match address {
            0x4000000 => self.ppu.dispcnt.getRaw() as u32,
            0x4000200 => (self.ppu.interruptFlags << 8) as u32 | self.ie as u32,
            _ => {println!("Unimplemented 32-bit read from MMIO address {:08X}", address); 0}
        }
    }

    pub fn writeIO8 (&mut self, address: u32, val: u8) {
        match address {
            0x4000070 => println!("Wrote to SOUND3CNT!"),
            0x4000084 => println!("Wrote to SOUNDCNT_X!"),
            0x4000208 => {
                self.ime = (val & 1) != 0;
                if self.ime {println!("Interrupts enabled!\n")}
            }
            0x4000301 => {}, //println!("Wrote to HALTCNT"),
            _ => println!("Unimplemented 8-bit write to IO address {:08X}\n", address)
        }
    }

    pub fn writeIO16 (&mut self, address: u32, val: u16) {
        match address {
            0x4000000 => self.ppu.dispcnt.setRaw(val),
            0x4000004 => self.ppu.dispstat.setRaw(val & 0xFF38),
            0x4000008 => self.ppu.bg_controls[0].setRaw(val),
            0x4000010 => self.ppu.bg_hofs[0].setRaw(val),
            0x4000014 => self.ppu.bg_hofs[1].setRaw(val),
            0x4000018 => self.ppu.bg_hofs[2].setRaw(val),
            0x400001C => self.ppu.bg_hofs[3].setRaw(val),
            0x4000012 => self.ppu.bg_vofs[0].setRaw(val),
            0x4000016 => self.ppu.bg_vofs[1].setRaw(val),
            0x400001A => self.ppu.bg_vofs[2].setRaw(val),
            0x400001E => self.ppu.bg_vofs[3].setRaw(val),
            0x4000088 => { self.soundbiasStub = (val as u32 | self.soundbiasStub & 0xFFFF0000); println!("Wrote to SOUNDBIAS!") },
            0x4000200 => { self.ie = val; } //println!("Wrote {:02X} to IE!", val) }
            0x4000202 => { self.ppu.interruptFlags &= (!val); }//println!("Wrote to IF!")}
            0x4000204 => self.waitcnt = val,
            0x4000208 => { self.ime = (val & 1) != 0}
            _ => println!("16-bit write to unimplemented IO address {:08X}\n", address)
        }
    }

    pub fn writeIO32 (&mut self, address: u32, val: u32) {
        match address {
            0x4000000 => self.ppu.dispcnt.setRaw(val as u16),
            0x4000080 => println!("Wrote to SOUNDCNT!"),
            0x4000088 => { self.soundbiasStub = val; println!("Wrote to SOUNDBIAS!") },
            0x4000208 => self.ime = (val & 1) == 1,
            _ => println!("Unimplemented 32-bit write to IO address {:08X}\n", address)
        }
    }

    pub fn isFrameReady (&self) -> bool {
        self.ppu.isFrameReady
    }
}