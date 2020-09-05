use crate::cpu::CPU;
use crate::bus::Bus;

pub struct GBA {
    cpu: CPU,
    bus: Bus
}

impl GBA {
    pub fn new(romPath: String) -> GBA {
        GBA {
            cpu: CPU::new(),
            bus: Bus::new(romPath)
        }
    }

    pub fn init(&mut self) {
        self.cpu.init(&self.bus);
    }

    pub fn step(&mut self) {
        self.cpu.step(&mut self.bus);
    }
}