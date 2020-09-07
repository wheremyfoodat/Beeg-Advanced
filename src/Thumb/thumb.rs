use crate::bus::Bus;
use crate::cpu::CPU;
use crate::cpu::CPUModes;
use crate::Thumb::*;
use crate::isBitSet;

impl CPU {
    pub fn executeThumbInstruction (&mut self, bus: &mut Bus, instruction: u32) {
        if self.gprs[1] == 0xFD000300 {
            panic!("Bad")
        }

        let lutIndex = (instruction >> 8) as usize;
        self.thumbLUT[lutIndex](self, bus, instruction);
    }

    pub fn populateThumbLUT(&mut self) {
        for x in 0..1024 {
            if (x >> 3) == 0 { // MOV LSL
                self.thumbLUT[x] = Self::Thumb_handleLSL;
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

            else if (x >> 3) == 0b00111 { // subs rd, #imm
                self.thumbLUT[x] = Self::Thumb_handleSubImm;
            }

            else if (x >> 3) == 0b11000 { // stmia rb! {rlist}
                self.thumbLUT[x] = Self::Thumb_handleSTMIA;
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