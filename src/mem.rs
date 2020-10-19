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
        let mut ROM = readFileIntoVec(&romPath);
        let mut len = ROM.len()-1;

        while len < (32 * 1024 * 1024) {// While the ROM is < 32 MB, fill it with valid OoB data 
            let OoBFillerData = (len >> 1) as u16;
            ROM.push(OoBFillerData as u8);
            ROM.push((OoBFillerData >> 8) as u8);
            len += 2;
        }

        Memory {
            BIOS: readFileIntoVec(&"ROMs/NormattBIOS.gba".to_string()),
            ROM,
            eWRAM: vec![0; 256 * 1024],
            iWRAM: vec![0; 32 * 1024],
            SRAM:  vec![0; 64 * 1024]
        }
    }
}