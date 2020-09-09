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
        let rm = self.gprs[instruction as usize & 0xF];
        let rs = self.gprs[(instruction as usize >> 8) & 0xF];

        let rdLoIndex = (instruction >> 12) & 0xF;
        let rdHiIndex = (instruction >> 16) & 0xF;

        let updateFlags = isBitSet!(instruction, 20);

        let opcode = (instruction >> 21) & 0x3;
        match opcode {
            0 => self.ARM_UMULL(rdLoIndex, rdHiIndex, rm, rs, updateFlags, bus),
            1 => self.ARM_UMLAL(rdLoIndex, rdHiIndex, rm, rs, updateFlags, bus),
            2 => self.ARM_SMULL(rdLoIndex, rdHiIndex, rm, rs, updateFlags, bus),
            _ => self.ARM_SMLAL(rdLoIndex, rdHiIndex, rm, rs, updateFlags, bus)
        }
        //todo!("[ARM] Multiply long\n")
    }

    pub fn ARM_MUL (&mut self, rdIndex: u32, rm: u32, rs: u32, updateFlags: bool, bus: &mut Bus) {
        let res = self._MUL(rm, rs, updateFlags);
        self.setGPR(rdIndex, res, bus);
    }

    pub fn ARM_MLA (&mut self, rdIndex: u32, rm: u32, rs: u32, rn: u32, updateFlags: bool, bus: &mut Bus) {
        let res = rm.wrapping_mul(rs).wrapping_add(rn);
        if updateFlags {
            self.setSignAndZero(res);
        }
        self.setGPR(rdIndex, res, bus);
    }

    pub fn ARM_UMULL(&mut self, rdLoIndex: u32, rdHiIndex: u32, rm: u32, rs: u32, updateFlags: bool, bus: &mut Bus) {
        let res = (rm as u64).wrapping_mul(rs as u64);
        if updateFlags {
            self.cpsr.setZero((res == 0) as u32);
            self.cpsr.setNegative((res >> 63) as u32);
        }

        self.setGPR(rdLoIndex, res as u32, bus);
        self.setGPR(rdHiIndex, (res >> 32) as u32, bus);
    }

    pub fn ARM_UMLAL(&mut self, rdLoIndex: u32, rdHiIndex: u32, rm: u32, rs: u32, updateFlags: bool, bus: &mut Bus) {
        let rd = ((self.getGPR(rdHiIndex) as u64) << 32) | (self.getGPR(rdLoIndex) as u64);
        let res = (rm as u64).wrapping_mul(rs as u64).wrapping_add(rd);

        if updateFlags {
            self.cpsr.setZero((res == 0) as u32);
            self.cpsr.setNegative((res >> 63) as u32);
        }

        self.setGPR(rdLoIndex, res as u32, bus);
        self.setGPR(rdHiIndex, (res >> 32) as u32, bus);
    }

    pub fn ARM_SMULL(&mut self, rdLoIndex: u32, rdHiIndex: u32, rm: u32, rs: u32, updateFlags: bool, bus: &mut Bus) {
        let res = (rm as i32 as i64).wrapping_mul(rs as i32 as i64);
        if updateFlags {
            self.cpsr.setZero((res == 0) as u32);
            self.cpsr.setNegative((res >> 63) as u32);
        }

        self.setGPR(rdLoIndex, res as u32, bus);
        self.setGPR(rdHiIndex, (res >> 32) as u32, bus);
    }

    pub fn ARM_SMLAL(&mut self, rdLoIndex: u32, rdHiIndex: u32, rm: u32, rs: u32, updateFlags: bool, bus: &mut Bus) {
        let rd = ((self.getGPR(rdHiIndex) as u64) << 32) | (self.getGPR(rdLoIndex) as u64);
        let mut res = (rm as i32 as i64).wrapping_mul(rs as i32 as i64) as u64;
        res = res.wrapping_add(rd);

        if updateFlags {
            self.cpsr.setZero((res == 0) as u32);
            self.cpsr.setNegative((res >> 63) as u32);
        }

        self.setGPR(rdLoIndex, res as u32, bus);
        self.setGPR(rdHiIndex, (res >> 32) as u32, bus);
    }
}
