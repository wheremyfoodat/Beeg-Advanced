use crate::bus::Bus;
use crate::cpu::CPU;
use crate::barrelShifter;
use crate::isBitSet;

impl CPU {
    pub fn ARM_handlePSRTransfer(&mut self, bus: &mut Bus, instruction: u32) {
        todo!("PSR transfer instruction!")
    }
}