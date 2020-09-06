use crate::bus::Bus;
use crate::cpu::CPU;
use crate::barrelShifter;
use crate::isBitSet;

impl CPU {
    pub fn ARM_handlePSRTransfer(&mut self, bus: &mut Bus, instruction: u32) {
        if isBitSet!(instruction, 21) { // MSR
            let isPrivileged = self.cpsr.getMode() != 0x10;
            let setSPSR = isBitSet!(instruction, 22);
            let mut operand = 0_u32;
            let mut mask = 0_u32;

            if isBitSet!(instruction, 25) {
                let imm = instruction & 0xFF;
                let rotate_imm = (instruction >> 8) & 0xF;
                operand = self.ROR(imm, rotate_imm * 2, false);
            }

            else {
                operand = self.getGPR(instruction & 0xF);
            }

            if isBitSet!(instruction, 16) && isPrivileged {mask |= 0xFF}
            if isBitSet!(instruction, 17) && isPrivileged {mask |= 0xFF00}
            if isBitSet!(instruction, 18) && isPrivileged {mask |= 0xFF0000}
            if isBitSet!(instruction, 19) {
                if isPrivileged {mask |= 0xFF000000}
                else {mask |= 0x0F000000}
            }

            operand &= mask;
            if setSPSR {
                debug_assert!(self.cpsr.getMode() != 0x10 && self.cpsr.getMode() != 0x1F);
                self.spsr.setRaw((self.spsr.getRaw() & !mask) | operand);
            }

            else {
                self.setCPSR((self.cpsr.getRaw() & !mask) | operand)
            }
        }

        else {
            todo!("Implement MRS\n");
        }
    }
}