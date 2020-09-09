use crate::bus::Bus;
use crate::cpu::CPU;
use crate::barrelShifter;
use crate::isBitSet;

impl CPU {
    pub fn Thumb_handleSPRelativeLoad (&mut self, bus: &mut Bus, instruction: u32) {
        //let imm = (instruction & 0xFF) << 2;
        //let sp = self.gprs[13];
        //let rdIndex = (instruction >> 8) & 0x7;

        //self.gprs[rdIndex as usize] = bus.read32(sp + imm);
        todo!("[THUMB] SP relative load!\n");
    }

    pub fn Thumb_handlePCRelativeLoad (&mut self, bus: &mut Bus, instruction: u32) {
        let imm = (instruction & 0xFF) << 2;
        let rdIndex = (instruction >> 8) & 0x7;
        let addr = (self.gprs[15] & !2) + imm;

        let mut val = bus.read32(addr & !3);
        val = self.ROR(val, 8 * (addr & 0x3), false);

        self.gprs[rdIndex as usize] = val;
    }

    pub fn Thumb_handleStoreHalfword (&mut self, bus: &mut Bus, instruction: u32) {
        let rdIndex = instruction & 7;
        let rb = self.getGPR((instruction >> 3) & 7);
        let offset = ((instruction >> 6) & 0x1F) << 1;

        bus.write16(rb + offset, self.gprs[rdIndex as usize] as u16);
    }

    pub fn Thumb_handleLoadHalfword (&mut self, bus: &mut Bus, instruction: u32) {
        let rdIndex = instruction & 7;
        let rb = self.getGPR((instruction >> 3) & 7);
        let offset = ((instruction >> 6) & 0x1F) << 1;

        self.gprs[rdIndex as usize] = bus.read16(rb + offset) as u32;
    }

    pub fn Thumb_handleLoadWordWithImm (&mut self, bus: &mut Bus, instruction: u32) {
        let rdIndex = instruction & 7;
        let rb = self.getGPR((instruction >> 3) & 7);
        let offset = ((instruction >> 6) & 0x1F) << 2;

        self.gprs[rdIndex as usize] = bus.read32(rb + offset);
    }

    pub fn Thumb_handleStoreWordWithImm (&mut self, bus: &mut Bus, instruction: u32) {
        let rdIndex = instruction & 7;
        let rb = self.getGPR((instruction >> 3) & 7);
        let offset = ((instruction >> 6) & 0x1F) << 2;

        bus.write32(rb + offset, self.gprs[rdIndex as usize]);
    }

    pub fn Thumb_handleLoadByteWithImm (&mut self, bus: &mut Bus, instruction: u32) {
        let rdIndex = instruction & 7;
        let rb = self.getGPR((instruction >> 3) & 7);
        let offset = (instruction >> 6) & 0x1F;

        self.gprs[rdIndex as usize] = bus.read8(rb + offset) as u32;
    }

    pub fn Thumb_handleStoreByteWithImm (&mut self, bus: &mut Bus, instruction: u32) {
        let rdIndex = instruction & 7;
        let rb = self.getGPR((instruction >> 3) & 7);
        let offset = (instruction >> 6) & 0x1F;

        bus.write8(rb + offset, self.gprs[rdIndex as usize] as u8);
    }

    pub fn Thumb_handleLoadAddress (&mut self, bus: &mut Bus, instruction: u32) { // TODO: split into 2 handlers?
        let rdIndex = (instruction >> 8) & 0x7;
        let offset = (instruction & 0xFF) << 2;
        let mut address = offset;

        if isBitSet!(instruction, 11) {
            address += self.gprs[13]
        }

        else {
            address += self.gprs[15]
        }

        self.gprs[rdIndex as usize] = address;
    }
}