use crate::ARM::arm;
use crate::bus::Bus;
use crate::cpu::CPU;
use crate::cpu::CPUModes;

#[macro_use]
use crate::isBitSet;

impl CPU {
    pub fn ARM_handleBranch (&mut self, bus: &Bus, instruction: u32) {
        let pc = self.getGPR(15);

        if isBitSet!(instruction, 24) { // BL (handle link bit)
            self.setGPR(14, pc - 4);
        }

        let mut imm = (instruction & 0xFFFFFF) as u32;
        if isBitSet!(imm, 23) {
            imm |= 0x3F000000
        }

        imm <<= 2;
        let addr = pc + imm;
        self.setGPR(15, addr);
        
        self.refillPipeline(bus);
    }
}