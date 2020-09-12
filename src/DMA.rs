use crate::bus::Bus;
use crate::io::DMACNT;

struct DMAChannel {
    wordCount: u16,
    sourceAddr: u32,
    destAddr: u32,
    controlReg: DMACNT
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