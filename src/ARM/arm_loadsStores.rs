use crate::bus::Bus;
use crate::cpu::CPU;
use crate::isBitSet;

// TODO: Clean this up
// A case could be made for making a specific handler for each type of LDR/STRH to make the LUT faster
// But the code would become a ton of copy paste

enum LoadStoreAddrModes {
    ImmediateOffset,
    RegisterOffset
}

impl CPU {
    pub fn ARM_handleLoadStoreImm(&mut self, bus: &mut Bus, instruction: u32) {
        let isByte = isBitSet!(instruction, 22);
        let isLoad = isBitSet!(instruction, 20);
        let isUser = isBitSet!(instruction, 21) && !isBitSet!(instruction, 24);
        let rdIndex = (instruction >> 12) & 0xF;
        let address = self.ARM_getLoadStoreAddr(LoadStoreAddrModes::ImmediateOffset, 
                                                instruction, bus);

        self.ARM_matchLoadStoreType(isByte, isLoad, isUser, rdIndex, address, bus)
    }

    pub fn ARM_handleLoadStoreWithShift (&mut self, bus: &mut Bus, instruction: u32) {
        let isByte = isBitSet!(instruction, 22);
        let isLoad = isBitSet!(instruction, 20);
        let isUser = isBitSet!(instruction, 21) && !isBitSet!(instruction, 24);
        let rdIndex = (instruction >> 12) & 0xF;
        let address = self.ARM_getLoadStoreAddr(LoadStoreAddrModes::RegisterOffset, 
                                                instruction, bus);
        self.ARM_matchLoadStoreType(isByte, isLoad, isUser, rdIndex, address, bus)
    }

    pub fn ARM_matchLoadStoreType (&mut self, isByte: bool, isLoad: bool, isUser: bool, rdIndex: u32, address: u32, bus: &mut Bus) {
        match isLoad {
            true => {
                match isByte {
                    true => match isUser {
                        true => todo!("[ARM] Implement LDRBT"),
                        false => self.ARM_LDRB(rdIndex, address, bus)
                    }

                    false => match isUser {
                        true => todo!("[ARM] Implement LDRT\n"),
                        false => self.ARM_LDR(rdIndex, address, bus)
                    }
                }
            }

            false => {
                match isByte {
                    true => match isUser {
                        true => todo!("[ARM] Implement STRBT\n"),
                        false => self.ARM_STRB(rdIndex, address, bus)
                    }

                    false => match isUser {
                        true => todo!("[ARM] Implement STRT\n"),
                        false => self.ARM_STR(rdIndex, address, bus)
                    }
                }
            }
        }
    }

    
    pub fn ARM_handleMiscLoadStores (&mut self, bus: &mut Bus, instruction: u32) {
        let mut address: u32;
        let isLoad = isBitSet!(instruction, 20);
        let signExtend = isBitSet!(instruction, 6);
        let isHalfword = isBitSet!(instruction, 5);
        let rdIndex = (instruction >> 12) & 0xF;
        debug_assert!(rdIndex != 15);

        if isBitSet!(instruction, 22) { // immediate offset
            address = self.ARM_getMiscLoadStoreAddr(LoadStoreAddrModes::ImmediateOffset, instruction, bus);
        }

        else {
            address = self.ARM_getMiscLoadStoreAddr(LoadStoreAddrModes::RegisterOffset, instruction, bus);
        }

        if isLoad {
            if isHalfword {
                if signExtend {
                    self.ARM_LDRSH(rdIndex, address, bus);
                }

                else {
                    self.ARM_LDRH(rdIndex, address, bus);
                }
            }

            else {
                if signExtend {
                    self.ARM_LDRSB(rdIndex, address, bus);
                }

                else {
                    panic!("Invalid ARM misc load at addr {:08X}", self.gprs[15] - 8);
                }
            }
        }

        else {
            if isHalfword {
                if !signExtend {
                    self.ARM_STRH(rdIndex, address, bus);
                }

                else {
                    todo!("STRD")
                }
            }

            else {
                if signExtend {
                    todo!("LDRD")
                }

                else {
                    panic!("Invalid ARM misc load at addr {:08X}", self.gprs[15] - 8);
                }
            }
        }
    }
    
    fn ARM_getLoadStoreAddr (&mut self, addrMode: LoadStoreAddrModes, instruction: u32, bus: &mut Bus) -> u32 {
        let rnIndex = (instruction >> 16) & 0xF;
        let rdIndex = (instruction >> 12) & 0xF;
        let rn = self.getGPR(rnIndex);
        
        
        let mut address = rn;
        let mut offset: u32;

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
            _ => {
                let rm = self.gprs[(instruction & 0xF) as usize];
                let shift = (instruction >> 5) & 3;
                let mut shift_amount = (instruction >> 7) & 31;

                if shift_amount == 0 && (shift == 1 || shift == 2) { // LSR #0 and ASR #0 become LSR #32 and ASR #32 respectively
                    shift_amount = 32;
                }

                match shift {
                    0 => offset = self.LSL(rm, shift_amount, false),
                    1 => offset = self.LSR(rm, shift_amount, false),
                    2 => offset = self.ASR(rm, shift_amount, false),
                    _ => {
                        if shift_amount == 0 { offset = self.RRX(rm, false) }
                        else { offset = self.ROR(rm, shift_amount, false) }
                    }
                }
            }
        }

        match addToBase {
            true => address = address.wrapping_add(offset),
            false => address = address.wrapping_sub(offset)
        }

        if shouldWriteBack {
            if rdIndex != rnIndex {
                self.setGPR(rnIndex, address, bus)
            }

            else {
                todo!("[ARM] LDR/STR with Rn == Rd\n")
            }
        }

        match preIndexing {
            true => address,
            false => rn
        }
    }

    fn ARM_getMiscLoadStoreAddr (&mut self, addrMode: LoadStoreAddrModes, instruction: u32, bus: &mut Bus) -> u32 {
        let rnIndex = (instruction >> 16) & 0xF;
        let rdIndex = (instruction >> 12) & 0xF;
        let rn = self.getGPR(rnIndex);

        let mut offset: u32;
        let mut address = rn;

        let addToBase = isBitSet!(instruction, 23);
        let preIndexing = isBitSet!(instruction, 24);
        let w = isBitSet!(instruction, 21);
        let mut shouldWriteBack = !(preIndexing && !w);
        let isLoad = isBitSet!(instruction, 20);

        if isLoad && rdIndex == rnIndex {
            shouldWriteBack = false;
        }

        debug_assert!(!(rdIndex == rnIndex && shouldWriteBack && !isLoad));

        match addrMode {
            LoadStoreAddrModes::ImmediateOffset => {
                offset = (instruction & 0xF) | ((instruction >> 4) & 0xF0);
            }
            LoadStoreAddrModes::RegisterOffset => {
                offset = self.getGPR(instruction & 0xF);
            }

            _ => panic!("Invalid addr mode for misc load/stores")
        }

        match addToBase {
            true => address += offset,
            false => address -= offset
        }

        if shouldWriteBack {
            self.setGPR(rnIndex, address, bus);
        }

        match preIndexing {
            true => address,
            false => rn
        }
    }

    fn ARM_LDR(&mut self, rdIndex: u32, address: u32, bus: &mut Bus) {
        let mut val = bus.read32 (address & !3);
        val = self.ROR(val, 8 * (address & 3), false);
        self.setGPR(rdIndex, val, bus);
    }

    fn ARM_LDRH(&mut self, rdIndex: u32, address: u32, bus: &mut Bus) {
        let val = bus.read16(address);
        self.setGPR(rdIndex, val as u32, bus);
    }

    fn ARM_LDRSH(&mut self, rdIndex: u32, address: u32, bus: &mut Bus) {
        let mut val = bus.read16(address) as u32;
        if (val >> 15) != 0 {
            val |= 0xFFFF0000;
        }

        self.setGPR(rdIndex, val, bus);
    }

    fn ARM_LDRB(&mut self, rdIndex: u32, address: u32, bus: &mut Bus) {
        self.setGPR(rdIndex, bus.read8(address) as u32, bus);
    }

    fn ARM_LDRSB(&mut self, rdIndex: u32, address: u32, bus: &mut Bus) {
        let mut val = bus.read8(address) as u32;
        if (val >> 7) != 0 {
            val |= 0xFFFFFF00;
        }

        self.setGPR(rdIndex, val, bus);
    }

    fn ARM_STR(&mut self, rdIndex: u32, address: u32, bus: &mut Bus) {
        let mut source = self.getGPR(rdIndex);
        if rdIndex == 15 { source += 4; } // When storing r15, it's 3 steps ahead instead of 2
        bus.write32 (address & 0xFFFFFFFC, source); // STR forcibly word-aligns the addr
    }

    fn ARM_STRH(&mut self, rdIndex: u32, address: u32, bus: &mut Bus) {
        let source = self.getGPR(rdIndex) as u16;
        bus.write16(address, source);
    }

    fn ARM_STRB (&mut self, rdIndex: u32, address: u32, bus: &mut Bus) {
        let source = self.getGPR(rdIndex) as u8;
        bus.write8(address, source)
    }
}