use crate::bus::Bus;
use crate::io::DMACNT;

pub struct DMAChannel {
    pub wordCount: u16,
    pub sourceAddr: u32,
    pub destAddr: u32,
    pub controlReg: DMACNT
}

impl DMAChannel {
    pub fn new() -> DMAChannel {
        DMAChannel {
            wordCount: 0,
            sourceAddr: 0,
            destAddr: 0,
            controlReg: DMACNT(0)
        }
    }
}

impl Bus {
    
}