pub struct PPU {
    pub dispcnt: u32
}

impl PPU {
    pub fn new() -> PPU {
        PPU {
            dispcnt: 0
        }
    }
}