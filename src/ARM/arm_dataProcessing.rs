use crate::ARM::arm;
use crate::bus::Bus;
use crate::cpu::CPU;
use crate::barrelShifter;
use crate::cpu::CPUModes;

#[macro_use]
use crate::isBitSet;

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

        match opcode {
            0   => todo!("[ARM] Implement AND\n"),
            1   => todo!("[ARM] Implement EOR\n"),
            2   => todo!("[ARM] Implement SUB\n"),
            3   => todo!("[ARM] Implement RSB\n"),
            4   => self.ARM_ADD(rdIndex, operand1, operand2, affectFlags, bus),
            5   => todo!("[ARM] Implement ADC\n"),
            6   => todo!("[ARM] Implement SBC\n"),
            7   => todo!("[ARM] Implement RSC\n"),
            8   => todo!("[ARM] Implement TST\n"),
            9   => todo!("[ARM] Implement TEQ\n"),
            10  => todo!("[ARM] Implement CMP\n"),
            11  => todo!("[ARM] Implement CMN\n"),
            12  => todo!("[ARM] Implement ORR\n"),
            13  => self.ARM_MOV(rdIndex, operand2, affectFlags, bus),
            14  => todo!("[ARM] Implement BIC\n"),
             _  => todo!("[ARM] Implement MVN\n")
        }

        if s && rdIndex == 15 {
            todo!("DP instruction with rd == 15 and s bit set")
        }
    }

    pub fn ARM_MOV(&mut self, rdIndex: u32, operand2: u32, affectFlags: bool, bus: &mut Bus) {
        self.setGPR(rdIndex, operand2, bus);
        if (affectFlags) {
            self.setSignAndZero(operand2);
        }
    }

    pub fn ARM_ADD(&mut self, rdIndex: u32, operand1: u32, operand2: u32, affectFlags: bool, bus: &mut Bus) {
        let res = operand1 as u64 + operand2 as u64;
        
        if affectFlags {
            self.cpsr.setCarry(((res >> 32) > 0) as u32);
            self.setSignAndZero(res as u32);
            self.cpsr.setOverflow(((operand1 >> 31) == (operand2 >> 31) && (operand1 >> 31) != (res as u32 >> 31)) as u32)
        }

        self.setGPR(rdIndex, res as u32, bus);
    }
}