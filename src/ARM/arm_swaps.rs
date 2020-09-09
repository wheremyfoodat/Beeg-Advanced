use crate::bus::Bus;
use crate::cpu::CPU;
use crate::isBitSet;

impl CPU {
    pub fn ARM_handleSwap (&mut self, bus: &mut Bus, instruction: u32) {
        let rnIndex = (instruction >> 16) & 0xF;
        let rdIndex = (instruction >> 12) & 0xF;
        let rmIndex = instruction & 0xF;

        let rn = self.getGPR(rnIndex);
        let rm = self.getGPR(rmIndex);

        if isBitSet!(instruction, 22) {
            self.ARM_SWPB(rdIndex, rn, rm, bus)
        }

        else {
            self.ARM_SWP(rdIndex, rn, rm, bus)
        }
    }

    pub fn ARM_SWP (&mut self, rdIndex: u32, rn: u32, rm: u32, bus: &mut Bus) {
        let mut loadedVal = bus.read32(rn & !3); // SWP's 1st part aligns the address like an LDR
        loadedVal = self.ROR(loadedVal, 8 * (rn & 3), false);

        self.setGPR(rdIndex, loadedVal, bus);
        bus.write32(rn & !3, rm); // SWP's second part force aligns the address
    }

    pub fn ARM_SWPB (&mut self, rdIndex: u32, rn: u32, rm: u32, bus: &mut Bus) {
        self.setGPR(rdIndex, bus.read8(rn) as u32, bus);
        bus.write8(rn, rm as u8);
    }
}