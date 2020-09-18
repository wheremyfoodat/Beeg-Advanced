use crate::helpers::readFileIntoVec;

pub struct Memory {
// main. non-IO memory
    pub BIOS:  Vec <u8>,
    pub ROM:   Vec <u8>,
    pub eWRAM: Vec <u8>,
    pub iWRAM: Vec <u8>,
    pub SRAM:  Vec <u8>
}

impl Memory {
    pub fn new(romPath: String) -> Memory {
        Memory {
            BIOS: readFileIntoVec(&"ROMs/NormattBIOS.gba".to_string()),
            ROM:  readFileIntoVec(&romPath),
            eWRAM: vec![0; 256 * 1024],
            iWRAM: vec![0; 32 * 1024],
            SRAM:  vec![0; 64 * 1024]
        }
    }
}