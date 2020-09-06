use crate::bus::Bus;
use crate::cpu::CPU;
use crate::cpu::CPUModes;
use crate::ARM::*;
use crate::isBitSet;


impl CPU {
    pub fn executeARMInstruction (&mut self, bus: &mut Bus, instruction: u32) {
        println!("Attempting to execute instruction: {:08X}", instruction);

        if !self.isConditionTrue(instruction >> 28) {
            self.advancePipeline(bus);
            return
        }

        let lutIndex = (((instruction >> 4) & 0xF) | ((instruction >> 16) & 0xFF0)) as usize;
        self.armLUT[lutIndex](self, bus, instruction);
    }

    pub fn ARM_handleUndefined (&mut self, bus: &mut Bus, instruction: u32) {
        self.logState();
        let lutIndex = (((instruction >> 4) & 0xF) | ((instruction >> 16) & 0xFF0)) as usize;
        println!("LUT index: {:b}", lutIndex);
        panic!("Undefined or unimplemented instruction {:08X} at PC: {:08X}\n", instruction, self.getGPR(15)-8)
    }

    pub fn populateARMLut (&mut self) {
        for x in 0..4096 {
            
            if x == 0b000100100001 {
                self.armLUT[x] = Self::ARM_handleBranchExchange
            }

            else if ((x >> 7) & 0x1F) == 0b00110 && ((x >> 4) & 3) == 00 {
                self.armLUT[x] = Self::ARM_handleUndefined
            }

            else if (x >> 9) == 0b101 { // Brunch and Brunch with Link
                self.armLUT[x] = Self::ARM_handleBranch;
            }

            else if (x >> 9) == 0b010 {
                self.armLUT[x] = Self::ARM_handleLoadStoreImm;
            }

            else if ((x >> 7) == 0b00010 && (x & 0xF) == 0 && !isBitSet!(x, 4)) || ((x >> 7) == 0b00110 && !isBitSet!(x, 4)) {
                self.armLUT[x] = Self::ARM_handlePSRTransfer;
            }

            else if (x >> 9) == 0b001 {
                self.armLUT[x] = Self::ARM_handleDataProcessingImm;
            }

            else {
                self.armLUT[x] = Self::ARM_handleUndefined;
            }
        }
    }
}