use crate::bus::Bus;
use crate::cpu::CPU;
use crate::cpu::CPUModes;
use crate::ARM::*;


impl CPU {
    pub fn executeARMInstruction (&mut self, bus: &mut Bus, instruction: u32) {
        
        if !self.isConditionTrue(instruction >> 28) {
            self.advancePipeline(bus, CPUModes::ARM);
            return
        }

        let lutIndex = (((instruction >> 4) & 0xF) | ((instruction >> 20) & 0xF0)) as usize;
        self.armLUT[lutIndex](self, &bus, instruction);

        println!("Attempted to execute instruction: {:08X}\n", instruction);
    }

    pub fn ARM_handleUndefined (&mut self, bus: &Bus, instruction: u32) {
        panic!("Undefined or unimplemented instruction {:08X} at PC: {:08X}", instruction, self.getGPR(15)-8)
    }

    pub fn populateARMLut (&mut self) {
        for x in 0..4096 {
            if (x >> 5) == 0b101 { // Brunch and Brunch with Link
                self.armLUT[x] = Self::ARM_handleBranch;
            }

            else {
                self.armLUT[x] = Self::ARM_handleUndefined;
            }
        }
    }
}