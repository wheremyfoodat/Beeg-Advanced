use crate::io::TMCNT;

pub struct Timers {
    pub counters: [u32; 4],
    pub controlRegs: [TMCNT; 4],
}