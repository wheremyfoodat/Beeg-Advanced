use crate::bus::Bus;
use crate::cpu::CPU;
use crate::barrelShifter;
use crate::isBitSet;

enum LoadStoreAddrModes {
    ImmediateOffset,
    RegisterOffset,
    ScaledRegister
}

impl CPU {
    pub fn ARM_handleLoadStoreImm(&mut self, bus: &mut Bus, instruction: u32) {
        let isByte = isBitSet!(instruction, 22);
        let isLoad = isBitSet!(instruction, 20);
        let isUser = isBitSet!(instruction, 21) && !isBitSet!(instruction, 24);
        let rdIndex = (instruction >> 12) & 0xF;
        let address = self.ARM_getLoadStoreAddr(LoadStoreAddrModes::ImmediateOffset, 
                                                instruction, bus);

        match isLoad {
            true => {
                match isByte {
                    true => match isUser {
                        true => todo!("[ARM] Implement LDRBT"),
                        false => todo!("Implement LDRB")
                    }

                    false => match isUser {
                        true => todo!("[ARM] Implement LDRT"),
                        false => todo!("[ARM] Implement LDR")
                    }
                }
            }

            false => {
                match isByte {
                    true => match isUser {
                        true => todo!("[ARM] Implement STRBT"),
                        false => todo!("[ARM] Implement STRB")
                    }

                    false => match isUser {
                        true => todo!("[ARM] Implement STRT"),
                        false => self.ARM_STR(rdIndex, address, bus)
                    }
                }
            }
        }
    }
    
    fn ARM_getLoadStoreAddr (&mut self, addrMode: LoadStoreAddrModes, instruction: u32, bus: &mut Bus) -> u32 {
        let rnIndex = (instruction >> 16) & 0xF;
        let rdIndex = (instruction >> 12) & 0xF;
        let rn = self.getGPR(rnIndex);
        
        
        let mut address = rn;
        let mut offset = 0_u32;

        let addToBase = isBitSet!(instruction, 23);
        let preIndexing = isBitSet!(instruction, 24);
        let w = isBitSet!(instruction, 21);
        let mut shouldWriteBack = !(preIndexing && !w);
        let isLoad = isBitSet!(instruction, 20);

        if isLoad && rdIndex == rnIndex {
            shouldWriteBack = false;
        }

        match addrMode {
            LoadStoreAddrModes::ImmediateOffset => offset = instruction & 0xFFF,
            _ => todo!("Implemented ARM load store addr modes")
        }

        match addToBase {
            true => address += offset,
            false => address -= offset
        }

        if shouldWriteBack {
            if rdIndex != rnIndex {
                self.setGPR(rnIndex, address, bus)
            }

            else {
                todo!("LDR/STR with Rn == Rd\n")
            }
        }

        match preIndexing {
            true => address,
            false => rn
        }
    }

    fn ARM_STR(&mut self, rdIndex: u32, address: u32, bus: &mut Bus) {
        let mut source = self.getGPR(rdIndex);
        if rdIndex == 15 { source += 4; } // When storing r15, it's 3 steps ahead instead of 2
        bus.write32 (address & 0xFFFFFFFC, address); // STR forcibly word-aligns the addr
    }
}