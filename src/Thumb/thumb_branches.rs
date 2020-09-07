use crate::bus::Bus;
use crate::cpu::CPU;

impl CPU {
    pub fn Thumb_handleConditionalBranch (&mut self, bus: &mut Bus, instruction: u32) {
        let mut offset = (instruction & 0xFF) << 1;
        if (offset >> 8) != 0 {
            offset |= 0xFFFFFF00;
        }

        let condition = (instruction >> 8) & 0xF;

        if self.isConditionTrue(condition) {
            let pc = self.gprs[15];
            self.setGPR(15, pc.wrapping_add(offset), bus);
        }
    }

    pub fn Thumb_handleBL1 (&mut self, bus: &mut Bus, instruction: u32) {
        println!("[THUMB] Executing BL at addr {:08X}", self.gprs[15]-4);
        println!("THUMB BL EXECUTED. NEGATIVE OFFSETS ARE BROKEN.");
        let mut offset = instruction & 0x7FF;
        offset <<= 12;

        self.gprs[14] = self.gprs[15] + offset;
    }

    pub fn Thumb_handleBL2 (&mut self, bus: &mut Bus, instruction: u32) {
        let offset = (instruction & 0x7FF) << 1;
        let pc = self.gprs[15];
        self.setGPR(15, self.gprs[14] + offset, bus);
        self.gprs[14] = (pc - 2) | 1;
    }

    pub fn Thumb_handleBX (&mut self, address: u32, bus: &mut Bus) {
        println!("[THUMB] Executing BX at addr {:08X}", self.gprs[15]-4);
        self.cpsr.setThumbState(address & 1);
        self.setGPR(15, address, bus);
    }
}