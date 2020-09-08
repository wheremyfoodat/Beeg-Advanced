use crate::bus::Bus;
use crate::cpu::CPU;
use crate::barrelShifter;
use crate::isBitSet;

impl CPU {
    pub fn ARM_handleLDM (&mut self, bus: &mut Bus, instruction: u32) {
        let rnIndex = (instruction >> 16) & 0xF;
        let writeback = isBitSet!(instruction, 21);
        let switchToUser = isBitSet!(instruction, 22);
        let increment = isBitSet!(instruction, 23);
        let changeSPBeforeTransfer = isBitSet!(instruction, 24);

        let mut sp = self.getGPR(rnIndex);

        debug_assert!((instruction & 0xFFFF) != 0);
        debug_assert!(!switchToUser);
        debug_assert!(!isBitSet!(instruction, rnIndex));
        
        if increment {
            for i in 0..16 {
                if isBitSet!(instruction, i) {
                    if changeSPBeforeTransfer {sp += 4};
                    self.setGPR(i, bus.read32(sp), bus);
                    if !changeSPBeforeTransfer {sp += 4};
                }
            }
        }

        else {
            for i in (0..16).rev() {
                if isBitSet!(instruction, i) {
                    if changeSPBeforeTransfer {sp -= 4};
                    self.setGPR(i, bus.read32(sp), bus);
                    if !changeSPBeforeTransfer {sp -= 4};
                }
            }
        }

        if writeback {
            self.setGPR(rnIndex, sp, bus);
        }
    }

    pub fn ARM_handleSTM (&mut self, bus: &mut Bus, instruction: u32) {
        let rnIndex = (instruction >> 16) & 0xF;
        let writeback = isBitSet!(instruction, 21);
        let switchToUser = isBitSet!(instruction, 22);
        let increment = isBitSet!(instruction, 23);
        let changeSPBeforeTransfer = isBitSet!(instruction, 24);

        let mut sp = self.getGPR(rnIndex);

        debug_assert!((instruction & 0xFFFF) != 0);
        debug_assert!(!switchToUser);
        debug_assert!(!isBitSet!(instruction, rnIndex));
        
        if increment {
            for i in 0..16 {
                if isBitSet!(instruction, i) {
                    if changeSPBeforeTransfer {sp += 4};
                    bus.write32(sp, self.getGPR(i));
                    if !changeSPBeforeTransfer {sp += 4};
                }
            }
        }

        else {
            for i in (0..16).rev() {
                if isBitSet!(instruction, i) {
                    if changeSPBeforeTransfer {sp -= 4};
                    bus.write32(sp, self.getGPR(i));
                    if !changeSPBeforeTransfer {sp -= 4};
                }
            }
        }

        if writeback {
            self.setGPR(rnIndex, sp, bus);
        }
    }
}