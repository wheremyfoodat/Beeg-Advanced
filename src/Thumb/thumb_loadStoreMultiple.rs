use crate::bus::*;
use crate::cpu::*;
use crate::isBitSet;

impl CPU {
    pub fn Thumb_handlePUSH (&mut self, bus: &mut Bus, instruction: u32) {
        let storeLR = isBitSet!(instruction, 8);
        debug_assert!((instruction & 0xFF) != 0 || storeLR);

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
        debug_assert!((instruction & 0xFF) != 0 || loadPC);

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
}