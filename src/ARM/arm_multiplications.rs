use crate::bus::Bus;
use crate::cpu::CPU;
use crate::isBitSet;

impl CPU {
    pub fn ARM_handleMultiply (&mut self, bus: &mut Bus, instruction: u32) {
        let rdIndex = (instruction >> 16) & 0xF;
        let rnIndex = (instruction >> 12) & 0xF;
        let rsIndex = (instruction >> 8) & 0xF;
        let rmIndex = instruction & 0xF;

        let rm = self.getGPR(rmIndex);
        let rn = self.getGPR(rnIndex);
        let rs = self.getGPR(rsIndex);

        let updateFlags = isBitSet!(instruction, 20);
        debug_assert!(!(updateFlags && rdIndex == 15));

        if isBitSet!(instruction, 21) {
            self.ARM_MLA (rdIndex, rm, rs, rn, updateFlags, bus);
        }

        else {
            self.ARM_MUL (rdIndex, rm, rs, updateFlags, bus);
        }
    }

    pub fn ARM_handleMultiplyLong (&mut self, bus: &mut Bus, instruction: u32) {
        self.logState();
        todo!("[ARM] Multiply long\n")
    }

    pub fn ARM_MUL (&mut self, rdIndex: u32, rm: u32, rs: u32, updateFlags: bool, bus: &mut Bus) {
        let res = rm * rs;
        if updateFlags {
            self.setSignAndZero(res);
        }
        self.setGPR(rdIndex, res, bus);
    }

    pub fn ARM_MLA (&mut self, rdIndex: u32, rm: u32, rs: u32, rn: u32, updateFlags: bool, bus: &mut Bus) {
        self.logState();
        todo!("[ARM] Implement MLA");
    }
}
