use crate::helpers::readFileIntoVec;

pub struct Memory {
    pub BIOS: Vec <u8>,
    pub ROM:  Vec <u8>,
    
    pub eWRAM: [u8; 256 * 1024],
    pub iWRAM: [u8; 32  * 1024]
}

impl Memory {
    pub fn new(romPath: String) -> Memory {
        return Memory {
            BIOS: Vec::new(),
            ROM:  readFileIntoVec(&romPath),
            eWRAM: [0; 256 * 1024],
            iWRAM: [0; 32 * 1024]
        }
    }
}