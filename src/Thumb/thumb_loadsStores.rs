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

    pub fn Thumb_handleSTMIA (&mut self, bus: &mut Bus, instruction: u32) {
        let rbIndex = (instruction >> 8) & 7;
        let mut sp = self.gprs[rbIndex as usize];

        debug_assert!((instruction & 0xFF) != 0);
        debug_assert!(!isBitSet!(instruction, rbIndex));

        for i in 0..8 {
            if isBitSet!(instruction, i) {
                bus.write32(sp, self.gprs[i as usize]);
                sp += 4;
            }
        }

        self.gprs[rbIndex as usize] = sp;
    }

    pub fn Thumb_handleLDMIA (&mut self, bus: &mut Bus, instruction: u32) {
        let rbIndex = (instruction >> 8) & 7;
        let mut sp = self.gprs[rbIndex as usize];

        debug_assert!((instruction & 0xFF) != 0);
        debug_assert!(!isBitSet!(instruction, rbIndex));

        for i in 0..8 {
            if isBitSet!(instruction, i) {
                self.gprs[i as usize] = bus.read32(sp);
                sp += 4;
            }
        }

        self.gprs[rbIndex as usize] = sp;
    }

    pub fn Thumb_handleStoreImmOffset (&mut self, bus: &mut Bus, instruction: u32) {
        let rdIndex = instruction & 0x7;
        let rbIndex = (instruction >> 3) & 7;
        let offset = ((instruction >> 6) & 0x1F) << 2;

        let rb = self.gprs[rbIndex as usize];
        let rd = self.gprs[rdIndex as usize];

        bus.write32(rb + offset, rd);
    }

    pub fn Thumb_handlePUSH (&mut self, bus: &mut Bus, instruction: u32) {
        let storeLR = isBitSet!(instruction, 8);
        debug_assert!((instruction & 0xFF) != 0);

        if storeLR {
            self.gprs[13] -= 4;
            bus.write32(self.gprs[13], self.gprs[14]);
        }

        for i in (0..8).rev() {
            if isBitSet!(instruction, i) {
                self.gprs[13] -= 4;
                bus.write32(self.gprs[13], self.gprs[i]);
            }
        }
    }

    pub fn Thumb_handlePOP (&mut self, bus: &mut Bus, instruction: u32) {
        let loadPC = isBitSet!(instruction, 8);
        debug_assert!((instruction & 0xFF) != 0);

        for i in 0..8 {
            if isBitSet!(instruction, i) {
                self.gprs[i] = bus.read32(self.gprs[13]);
                self.gprs[13] += 4;
            }
        }

        if loadPC {
            self.setGPR(15, bus.read32(self.gprs[13]), bus);
            self.gprs[13] += 4;
        }
    }
}