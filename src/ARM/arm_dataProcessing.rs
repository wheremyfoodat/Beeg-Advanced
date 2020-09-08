use crate::ARM::arm;
use crate::bus::Bus;
use crate::cpu::CPU;
use crate::barrelShifter;
use crate::cpu::CPUModes;

#[macro_use]
use crate::isBitSet;

// Todo: clean this up
// Every addressing mode has its own handler so as not to have to decode instructions at runtime
// It looks like shit

impl CPU {
    pub fn ARM_handleDataProcessingImm(&mut self, bus: &mut Bus, instruction: u32) {
        let imm = instruction & 0xFF;
        let rot_imm = (instruction >> 8) & 0xF;
        let operand1 = self.getGPR((instruction >> 16) & 0xF);

        let rdIndex = (instruction >> 12) & 0xF;
        let s = isBitSet!(instruction, 20);

        let affectFlags = s && rdIndex != 15;
        let operand2 = self.ROR(imm, rot_imm * 2, affectFlags);
        let opcode = (instruction >> 21) & 0xF;

        debug_assert!(!(s && rdIndex == 15));

        match opcode {
            0   => self.ARM_AND(rdIndex, operand1, operand2, affectFlags, bus),
            1   => todo!("[ARM] Implement EOR\n"),
            2   => todo!("[ARM] Implement SUB\n"),
            3   => todo!("[ARM] Implement RSB\n"),
            4   => self.ARM_ADD(rdIndex, operand1, operand2, affectFlags, bus),
            5   => todo!("[ARM] Implement ADC\n"),
            6   => todo!("[ARM] Implement SBC\n"),
            7   => todo!("[ARM] Implement RSC\n"),
            8   => todo!("[ARM] Implement TST\n"),
            9   => todo!("[ARM] Implement TEQ\n"),
            10  => self._CMP(operand1, operand2),
            11  => todo!("[ARM] Implement CMN\n"),
            12  => todo!("[ARM] Implement ORR\n"),
            13  => self.ARM_MOV(rdIndex, operand2, affectFlags, bus),
            14  => todo!("[ARM] Implement BIC\n"),
             _  => todo!("[ARM] Implement MVN\n")
        }
    }

    pub fn ARM_handleDataProcessingRegister (&mut self, bus: &mut Bus, instruction: u32) {
        let s = isBitSet!(instruction, 20);
        let rdIndex = (instruction >> 12) & 0xF; 
        let rnIndex = (instruction >> 16) & 0xF;
        let rmIndex = instruction & 0xF;    

        let shift = (instruction >> 5) & 3;
        let shiftAmount = self.getGPR((instruction >> 8) & 0xF) & 0xFF;
        let opcode = (instruction >> 21) & 0xF;
        let affectFlags = s && rdIndex != 15;
        debug_assert!(!(s && rdIndex == 15));

        let rn = self.getGPR(rnIndex);
        let mut rm = self.getGPR(rmIndex);

        match shift {
            0 => rm = self.LSL(rm, shiftAmount, affectFlags),
            1 => rm = self.LSR(rm, shiftAmount, affectFlags),
            2 => rm = self.ASR(rm, shiftAmount, affectFlags),
            _ => rm = self.ROR(rm, shiftAmount, affectFlags)
        }

        match opcode {
            0   => self.ARM_AND(rdIndex, rn, rm, affectFlags, bus),
            1   => todo!("[ARM] Implement EOR\n"),
            2   => todo!("[ARM] Implement SUB\n"),
            3   => todo!("[ARM] Implement RSB\n"),
            4   => self.ARM_ADD(rdIndex, rn, rm, affectFlags, bus),
            5   => todo!("[ARM] Implement ADC\n"),
            6   => todo!("[ARM] Implement SBC\n"),
            7   => todo!("[ARM] Implement RSC\n"),
            8   => todo!("[ARM] Implement TST\n"),
            9   => todo!("[ARM] Implement TEQ\n"),
            10  => self._CMP(rn, rm),
            11  => todo!("[ARM] Implement CMN\n"),
            12  => todo!("[ARM] Implement ORR\n"),
            13  => self.ARM_MOV(rdIndex, rm, affectFlags, bus),
            14  => todo!("[ARM] Implement BIC\n"),
             _  => todo!("[ARM] Implement MVN\n")
        }
    }

    pub fn ARM_handleDataProcessingImmShift (&mut self, bus: &mut Bus, instruction: u32) {
        let s = isBitSet!(instruction, 20);
        let rdIndex = (instruction >> 12) & 0xF; 
        let rnIndex = (instruction >> 16) & 0xF;
        let rmIndex = instruction & 0xF;    

        let shift = (instruction >> 5) & 3;
        let shiftImm = (instruction >> 7) & 31;
        let opcode = (instruction >> 21) & 0xF;
        let affectFlags = s && rdIndex != 15;
        debug_assert!(!(s && rdIndex == 15));

        let rn = self.getGPR(rnIndex);
        let mut rm = self.getGPR(rmIndex);

        match shift {
            0 => rm = self.LSL(rm, shiftImm, affectFlags),
            1 => rm = self.LSR(rm, shiftImm, affectFlags),
            2 => rm = self.ASR(rm, shiftImm, affectFlags),
            _ => {
                if shiftImm != 0 {
                    rm = self.ROR(rm, shiftImm, affectFlags);
                }

                else {
                    todo!("[ARM] Implement RRX\n");
                }
            }
        }

        match opcode {
            0   => self.ARM_AND(rdIndex, rn, rm, affectFlags, bus),
            1   => todo!("[ARM] Implement EOR\n"),
            2   => todo!("[ARM] Implement SUB\n"),
            3   => todo!("[ARM] Implement RSB\n"),
            4   => self.ARM_ADD(rdIndex, rn, rm, affectFlags, bus),
            5   => todo!("[ARM] Implement ADC\n"),
            6   => todo!("[ARM] Implement SBC\n"),
            7   => todo!("[ARM] Implement RSC\n"),
            8   => todo!("[ARM] Implement TST\n"),
            9   => todo!("[ARM] Implement TEQ\n"),
            10  => self._CMP(rn, rm),
            11  => todo!("[ARM] Implement CMN\n"),
            12  => todo!("[ARM] Implement ORR\n"),
            13  => self.ARM_MOV(rdIndex, rm, affectFlags, bus),
            14  => todo!("[ARM] Implement BIC\n"),
             _  => todo!("[ARM] Implement MVN\n")
        }
    }
    
    pub fn ARM_MOV(&mut self, rdIndex: u32, operand2: u32, affectFlags: bool, bus: &mut Bus) {
        self.setGPR(rdIndex, operand2, bus);
        if affectFlags {
            self.setSignAndZero(operand2);
        }
    }

    pub fn ARM_ADD(&mut self, rdIndex: u32, operand1: u32, operand2: u32, affectFlags: bool, bus: &mut Bus) {
        let res = self._ADD(operand1, operand2, affectFlags);
        self.setGPR(rdIndex, res , bus);
    }

    pub fn ARM_AND(&mut self, rdIndex: u32, operand1: u32, operand2: u32, affectFlags: bool, bus: &mut Bus) {
        let res = self._AND(operand1, operand2, affectFlags);
        self.setGPR(rdIndex, res, bus);
    }
}