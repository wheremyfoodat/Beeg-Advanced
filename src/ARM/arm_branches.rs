use crate::bus::Bus;
use crate::cpu::CPU;

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
        let addr = pc.wrapping_add(imm as u32);
        self.setGPR(15, addr, bus);
    }

    pub fn ARM_handleBranchExchange (&mut self, bus: &mut Bus, instruction: u32) {
        let rm = self.getGPR(instruction & 0xF);
        self.cpsr.setThumbState(rm & 1);
        self.setGPR(15, rm, bus);
    }

    pub fn ARM_handleSWI (&mut self, bus: &mut Bus, instruction: u32) {
        println!("ARM mode SWI at address: {:08X}", self.gprs[15]-8);
        let lr = self.gprs[15] - 4;
        let cpsr = self.cpsr.getRaw();
        self.changeMode(0x13); // switch to SVC mode
        self.gprs[14] = lr;
        self.cpsr.setIRQDisable(1); // disable interrupts

        self.setGPR(15, 0x8, bus); // jump to BIOS SWI handler
        self.spsr.setRaw(cpsr) // Set SPSR to previous CPSR (needed to return from the SWI)
    }
}