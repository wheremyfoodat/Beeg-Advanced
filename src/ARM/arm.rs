use crate::bus::Bus;
use crate::cpu::CPU;
use crate::isBitSet;


impl CPU {
    pub fn executeARMInstruction (&mut self, bus: &mut Bus, instruction: u32) {
        if !self.isConditionTrue(instruction >> 28) {
            return
        }

        let lutIndex = (((instruction >> 4) & 0xF) | ((instruction >> 16) & 0xFF0)) as usize;
        self.armLUT[lutIndex](self, bus, instruction);
    }

    pub fn ARM_handleUndefined (&mut self, bus: &mut Bus, instruction: u32) {
        self.logState();
        let lutIndex = ((instruction >> 4) & 0xF) | ((instruction >> 16) & 0xFF0);
        println!("LUT index: {:b}", lutIndex);
        panic!("[ARM] Undefined or unimplemented instruction {:08X} at PC: {:08X}\n", instruction, self.getGPR(15)-8)
    }

    pub fn populateARMLut (&mut self) {
        for x in 0..4096 {

            if x & 0xF00 == 0xF00 { // SWI
                self.armLUT[x] = Self::ARM_handleSWI;
            }

            else if (x & 0xF8F) == 0x89 {
                self.armLUT[x] = Self::ARM_handleMultiplyLong
            }
            
            else if (x & 0xFCF) == 0x9  {
                self.armLUT[x] = Self::ARM_handleMultiply;
            }

            else if (x >> 9) == 0b101 { // Brunch and Brunch with Link
                self.armLUT[x] = Self::ARM_handleBranch;
            }

            else if x == 0b000100100001 {
                self.armLUT[x] = Self::ARM_handleBranchExchange
            }

            else if ((x >> 7) == 0b00010 && (x & 0xF) == 0 && !isBitSet!(x, 4)) || ((x >> 7) == 0b00110 && !isBitSet!(x, 4)) {
                self.armLUT[x] = Self::ARM_handlePSRTransfer;
            }

            else if x & 0xFBF == 0x109 { // Swaps
                self.armLUT[x] = Self::ARM_handleSwap;
            }

            else if (x >> 9) == 0b010 {
                self.armLUT[x] = Self::ARM_handleLoadStoreImm;
            }

            else if (x >> 9) == 0b011 {
                self.armLUT[x] = Self::ARM_handleLoadStoreWithShift;
            }

            else if (x & 0b1001) == 0b1001 && (x >> 9) == 0 { // todo: separate handler for each type? (speed?)
                self.armLUT[x] = Self::ARM_handleMiscLoadStores;
            } 

            else if (x >> 9) == 0b100 && ((x >> 4) & 1) == 1 { // LDM
                self.armLUT[x] = Self::ARM_handleLDM;
            }

            else if (x >> 9) == 0b100 {
                self.armLUT[x] = Self::ARM_handleSTM;
            }

            else if ((x >> 7) & 0x1F) == 0b00110 && ((x >> 4) & 3) == 00 {
                self.armLUT[x] = Self::ARM_handleUndefined
            }

            else if (x >> 9) == 0b001 {
                self.armLUT[x] = Self::ARM_handleDataProcessingImm;
            }

            else if (x >> 9) == 0 && (x & 1) == 0 {
                self.armLUT[x] = Self::ARM_handleDataProcessingImmShift;
            }

            else if (x >> 9) == 0 {
                self.armLUT[x] = Self::ARM_handleDataProcessingRegister;
            }

            else {
                self.armLUT[x] = Self::ARM_handleUndefined;
            }
        }
    }
}