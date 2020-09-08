use crate::bus::Bus;
use crate::cpu::CPU;
use crate::cpu::CPUModes;
use crate::Thumb::*;
use crate::isBitSet;

impl CPU {
    pub fn executeThumbInstruction (&mut self, bus: &mut Bus, instruction: u32) {
        //if (self.gprs[15] == 0x80041E4+4) {
        //    self.logState();
        //    panic!("Breakpoint")
        //}

        let lutIndex = (instruction >> 8) as usize;
        self.thumbLUT[lutIndex](self, bus, instruction);
    }

    pub fn populateThumbLUT(&mut self) { // this LUT has a ton of specialized headers. this is to minimize decoding as much as possible during runtime and improve speed
        for x in 0..1024 {
            if (x >> 3) == 0 { // LSL rd, rs, #offset
                self.thumbLUT[x] = Self::Thumb_handleLSL;
            }

            else if (x >> 3) == 0b00010 {
                self.thumbLUT[x] = Self::Thumb_handleASR;
            }
            
            else if (x >> 3) == 0b10011 { // SP-relative load
                self.thumbLUT[x] = Self::Thumb_handleSPRelativeLoad;
            }

            else if (x >> 2) == 0b010001 { // TODO: possibly skip the opcode decoding by having separate handlers?
                self.thumbLUT[x] = Self::Thumb_handleHighRegOp;
            }

            else if (x >> 3) == 0b00100 { // movs rd, #imm
                self.thumbLUT[x] = Self::Thumb_handleMoveImm;
            }

            else if (x >> 3) == 0b00110 { // adds rd, #imm
                self.thumbLUT[x] = Self::Thumb_handleAddImm;
            }

            else if (x >> 3) == 0b00111 { // subs rd, #imm
                self.thumbLUT[x] = Self::Thumb_handleSubImm;
            }
            
            else if (x >> 3) == 0b01100 { // str rd, [rb, #imm]
                self.thumbLUT[x] = Self::Thumb_handleStoreImmOffset;
            }

            else if (x >> 3) == 0b11000 { // stmia rb! {rlist}
                self.thumbLUT[x] = Self::Thumb_handleSTMIA;
            }

            else if (x >> 3) == 0b11001 { // ldmia rb! {rlist}
                self.thumbLUT[x] = Self::Thumb_handleLDMIA;
            }

            else if (x >> 1) == 0b1011010 { // push (TODO: add separate handler for R=1 and R=0?)
                self.thumbLUT[x] = Self::Thumb_handlePUSH;
            }

            else if (x >> 1) == 0b1011110 { // pop (TODO: add separate handler for R=1 and R=0?)
                self.thumbLUT[x] = Self::Thumb_handlePOP;
            }

            else if (x >> 4) == 0b1101 { // Conditional branches
                self.thumbLUT[x] = Self::Thumb_handleConditionalBranch;
            }

            else if (x >> 3) == 0b01001 { // PC-relative load
                self.thumbLUT[x] = Self::Thumb_handlePCRelativeLoad;
            }

            else if (x >> 3) == 0b11110 { // First part of the 2-instruction long branch with link
                self.thumbLUT[x] = Self::Thumb_handleBL1;
            }

            else if (x >> 3) == 0b11111 { // Second part of the 2-instruction long branch with link
                self.thumbLUT[x] = Self::Thumb_handleBL2;
            }

            else if (x >> 1) == 0b0001100 { // adds rd, rs, rn
                self.thumbLUT[x] = Self::Thumb_handleAddReg
            }

            else if (x >> 1) == 0b0001110 { // adds rd, rs, #offset
                self.thumbLUT[x] = Self::Thumb_handleAddOffset;
            }

            else if (x >> 1) == 0b0001101 { // subs rd, rs, rn
                self.thumbLUT[x] = Self::Thumb_handleSubReg;
            }

            else if (x >> 1) == 0b0001111 { // subs rd, rs, #offset
                self.thumbLUT[x] = Self::Thumb_handleSubOffset;
            }

            else if (x >> 2) == 0b010000 { // ALU operations
                self.thumbLUT[x] = Self::Thumb_handleALU;
            }

            else {
                self.thumbLUT[x] = Self::Thumb_handleUndefined;
            }
        }
    }

    pub fn Thumb_handleUndefined (&mut self, bus: &mut Bus, instruction: u32) {
        self.logState();
        let lutIndex = instruction >> 8;
        println!("LUT index: {:#08b}", lutIndex);
        println!("In binary: {:#016b}", instruction);
        panic!("[THUMB] Undefined or unimplemented instruction {:04X} at PC: {:08X}\n", instruction, self.getGPR(15)-4)
    }
}