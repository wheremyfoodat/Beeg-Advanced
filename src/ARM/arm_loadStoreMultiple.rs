use crate::bus::Bus;
use crate::cpu::CPU;
use crate::isBitSet;
// TODO: implement remaining edge cases

impl CPU {
    pub fn ARM_handleLDM (&mut self, bus: &mut Bus, instruction: u32) {
        let rnIndex = (instruction >> 16) & 0xF;
        let mut writeback = isBitSet!(instruction, 21);
        let switchToUser = isBitSet!(instruction, 22);
        let increment = isBitSet!(instruction, 23);
        let mut changeSPBeforeTransfer = isBitSet!(instruction, 24);
        let currentMode = self.cpsr.getMode();

        if switchToUser {
            if isBitSet!(instruction, 15) {
                todo!("LDM that loads r15 with S mode bit set, might be wrong!");
                self.setCPSR(self.spsr.getRaw());   
            }

            else {
                self.changeMode(0x10);
            }
        }

        let mut sp = self.getGPR(rnIndex);

        debug_assert!((instruction & 0xFFFF) != 0);
        
        if isBitSet!(instruction, rnIndex) {
            writeback = false;
        }
        
        if increment {
            for i in 0..16 {
                if isBitSet!(instruction, i) {
                    if changeSPBeforeTransfer {sp = sp.wrapping_add(4)};
                    self.setGPR(i, bus.read32(sp), bus);
                    if !changeSPBeforeTransfer {sp = sp.wrapping_add(4)};
                }
            }
        }

        else {
            for i in (0..16).rev() {
                if isBitSet!(instruction, i) {
                    if changeSPBeforeTransfer {sp = sp.wrapping_sub(4)};
                    self.setGPR(i, bus.read32(sp), bus);
                    if !changeSPBeforeTransfer {sp = sp.wrapping_sub(4)};
                }
            }
        }

        if writeback {
            self.setGPR(rnIndex, sp, bus);
        }

        if switchToUser && !isBitSet!(instruction, 15) {
            self.changeMode(currentMode);
        }
    }

    pub fn ARM_handleSTM (&mut self, bus: &mut Bus, instruction: u32) {
        let rnIndex = (instruction >> 16) & 0xF;
        let writeback = isBitSet!(instruction, 21);
        let switchToUser = isBitSet!(instruction, 22);
        let increment = isBitSet!(instruction, 23);
        let mut changeSPBeforeTransfer = isBitSet!(instruction, 24);
        let currentMode = self.cpsr.getMode();
        
        if switchToUser {
            todo!("User mode STM");
            self.changeMode(0x10);
        }
        
        let mut sp = self.getGPR(rnIndex);

        debug_assert!((instruction & 0xFFFF) != 0);
        //debug_assert!(!isBitSet!(instruction, rnIndex));
        
        if increment {
            for i in 0..16 {
                if isBitSet!(instruction, i) {
                    if changeSPBeforeTransfer {sp = sp.wrapping_add(4)};
                    bus.write32(sp, self.getGPR(i));
                    if !changeSPBeforeTransfer {sp = sp.wrapping_add(4)};
                }
            }
        }

        else {
            for i in (0..16).rev() {
                if isBitSet!(instruction, i) {
                    if changeSPBeforeTransfer {sp = sp.wrapping_sub(4)};
                    bus.write32(sp, self.getGPR(i));
                    if !changeSPBeforeTransfer {sp = sp.wrapping_sub(4)};
                }
            }
        }

        if writeback {
            self.setGPR(rnIndex, sp, bus);
        }

        if switchToUser {
            self.changeMode(currentMode);
        }
    }
}