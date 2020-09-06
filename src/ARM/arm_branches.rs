use crate::ARM::arm;
use crate::bus::Bus;
use crate::cpu::CPU;
use crate::cpu::CPUModes;

#[macro_use]
use crate::isBitSet;

impl CPU {
    pub fn ARM_handleBranch (&mut self, bus: &mut Bus, instruction: u32) {
        let pc = self.getGPR(15);

        if isBitSet!(instruction, 24) { // BL (handle link bit)
            self.setGPR(14, pc - 4, bus);
        }

        let mut imm = (instruction & 0xFFFFFF) as i32;
        if isBitSet!(imm, 23) {
            imm |= 0x3F000000
        }

        imm <<= 2;
        let addr = pc + imm as u32;
        self.setGPR(15, addr, bus);
    }

    pub fn ARM_handleBranchExchange (&mut self, bus: &mut Bus, instruction: u32) {
        let rm = self.getGPR(instruction & 0xF);
        self.cpsr.setThumbState(rm & 1);
        self.setGPR(15, rm, bus);
    }
}