use crate::bus::Bus;
use crate::cpu::CPU;

impl CPU {
    pub fn ARM_handleMultiply (&mut self, bus: &mut Bus, instruction: u32) {
        self.logState();
        todo!("[ARM] Multiply\n")
    }

    pub fn ARM_handleMultiplyLong (&mut self, bus: &mut Bus, instruction: u32) {
        self.logState();
        todo!("[ARM] Multiply long\n")
    }
}
