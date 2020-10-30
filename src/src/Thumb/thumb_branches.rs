use crate::bus::Bus;
use crate::cpu::CPU;
use crate::sign_extend_32;

impl CPU {
    pub fn Thumb_handleConditionalBranch (&mut self, bus: &mut Bus, instruction: u32) {
        let condition = (instruction >> 8) & 0xF;

        if self.isConditionTrue(condition) {
            let mut offset = (instruction & 0xFF) << 1;
            offset = sign_extend_32!(offset, 9); // Sign extend offset to 32 bits from 9 bits
    
            let pc = self.gprs[15];
            self.setGPR(15, pc.wrapping_add(offset), bus);
        }
    }

    pub fn Thumb_handleUnconditionalBranch (&mut self, bus: &mut Bus, instruction: u32) {
        let mut offset = (instruction & 0x7FF) << 1;
        offset = sign_extend_32!(offset, 12); // Sign extend the offset to 32 bits from 12

        let pc = self.gprs[15];
        self.setGPR(15, pc.wrapping_add(offset), bus);
    }

    pub fn Thumb_handleBL1 (&mut self, bus: &mut Bus, instruction: u32) {
        let mut offset = (instruction & 0x7FF) as i16;
        offset <<= 5;

        self.gprs[14] = self.gprs[15].wrapping_add(((offset as i32) << 7) as u32);
    }

    pub fn Thumb_handleBL2 (&mut self, bus: &mut Bus, instruction: u32) {
        let offset = (instruction & 0x7FF) << 1;
        let pc = self.gprs[15];
        self.setGPR(15, self.gprs[14] + offset, bus);

        self.gprs[14] = (pc - 2) | 1;
    }

    pub fn Thumb_handleBX (&mut self, address: u32, bus: &mut Bus) {
        self.cpsr.setThumbState(address & 1);
        self.setGPR(15, address, bus);
    }

    pub fn Thumb_handleSWI (&mut self, bus: &mut Bus, instruction: u32) {
        let lr = self.gprs[15] - 2;
        let cpsr = self.cpsr.getRaw();
        self.changeMode(0x13); // switch to SVC mode
        self.gprs[14] = lr;
        self.cpsr.setThumbState(0); // Switch to ARM mode, disable interrupts
        self.cpsr.setIRQDisable(1);

        self.setGPR(15, 0x8, bus); // jump to BIOS SWI handler
        self.spsr.setRaw(cpsr) // Set SPSR to previous CPSR (needed to return from the SWI)
    }
}