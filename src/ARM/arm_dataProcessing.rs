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
        let rn = self.getGPR((instruction >> 12) & 0xF);

        let rdIndex = (instruction >> 16) & 0xF;
        let s = isBitSet!(instruction, 20);

        let affectFlags = s && rdIndex != 15;
        let operand2 = self.ROR(imm, rot_imm * 2, affectFlags);
        let opcode = (instruction >> 21) & 0xF;

        match opcode {
            13 => self.ARM_MOV(rdIndex, operand2, affectFlags, bus),
            _  => todo!("Unimplemented DP instruciton\n")
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
}