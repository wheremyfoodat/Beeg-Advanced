use crate::bus::Bus;
use crate::cpu::CPU;
use crate::cpu::CPUModes;
use crate::ARM::*;


impl CPU {
    pub fn executeARMInstruction (&mut self, bus: &mut Bus, instruction: u32) {
        println!("Attempting to execute instruction: {:08X}", instruction);

        if !self.isConditionTrue(instruction >> 28) {
            self.advancePipeline(bus);
            return
        }

        let lutIndex = (((instruction >> 4) & 0xF) | ((instruction >> 20) & 0xF0)) as usize;
        self.armLUT[lutIndex](self, bus, instruction);
    }

    pub fn ARM_handleUndefined (&mut self, bus: &mut Bus, instruction: u32) {
        panic!("Undefined or unimplemented instruction {:08X} at PC: {:08X}\n", instruction, self.getGPR(15)-8)
    }

    pub fn populateARMLut (&mut self) {
        for x in 0..4096 {
            if (x >> 5) == 0b101 { // Brunch and Brunch with Link
                self.armLUT[x] = Self::ARM_handleBranch;
            }

            else if (x >> 5) == 0b001 {
                self.armLUT[x] = Self::ARM_handleDataProcessingImm;
            }

            else if (x >> 5) == 0b010 {
                self.armLUT[x] = Self::ARM_handleLoadStoreImm;
            }

            else if (x >> 4) == 0b0001 {
                self.armLUT[x] = Self::ARM_handlePSRTransfer;
            }

            else {
                self.armLUT[x] = Self::ARM_handleUndefined;
            }
        }
    }
}