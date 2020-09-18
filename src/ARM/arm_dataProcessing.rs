use crate::bus::Bus;
use crate::cpu::CPU;

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
        let oldCarry = self.cpsr.getCarry();

        let affectFlags = s && rdIndex != 15;
        let operand2 = self.ROR(imm, rot_imm * 2, affectFlags);
        let opcode = (instruction >> 21) & 0xF;

        if s && rdIndex == 15 {
            self.setCPSR(self.spsr.getRaw());
            //println!("IRQ/SWI return")
        }

        self.executeDP(opcode, rdIndex, operand1, operand2, affectFlags, oldCarry, bus)
    }

    pub fn ARM_handleDataProcessingRegister (&mut self, bus: &mut Bus, instruction: u32) {
        self.gprs[15] += 4; // PC is 3 steps ahead instead of 2 in this type of instr.
                            // We stub it by making it go an extra step ahead during operand fetch

        let s = isBitSet!(instruction, 20);
        let rdIndex = (instruction >> 12) & 0xF; 
        let rnIndex = (instruction >> 16) & 0xF;
        let rmIndex = instruction & 0xF;    
        let oldCarry = self.cpsr.getCarry();

        let shift = (instruction >> 5) & 3;
        let shiftAmount = self.getGPR((instruction >> 8) & 0xF) & 0xFF;
        let opcode = (instruction >> 21) & 0xF;
        let affectFlags = s && rdIndex != 15;
        debug_assert!(!(s && rdIndex == 15));

        let mut rn = self.getGPR(rnIndex);
        let mut rm = self.getGPR(rmIndex);

        self.gprs[15] -= 4; // Undo what we did in the first line

        if s && rdIndex == 15 {
            self.setCPSR(self.spsr.getRaw());
            //println!("IRQ/SWI return")
        }

        match shift {
            0 => rm = self.LSL(rm, shiftAmount, affectFlags),
            1 => rm = self.LSR(rm, shiftAmount, affectFlags),
            2 => rm = self.ASR(rm, shiftAmount, affectFlags),
            _ => rm = self.ROR(rm, shiftAmount, affectFlags)
        }

        self.executeDP(opcode, rdIndex, rn, rm, affectFlags, oldCarry, bus)
    }

    pub fn ARM_handleDataProcessingImmShift (&mut self, bus: &mut Bus, instruction: u32) {
        let s = isBitSet!(instruction, 20);
        let rdIndex = (instruction >> 12) & 0xF; 
        let rnIndex = (instruction >> 16) & 0xF;
        let rmIndex = instruction & 0xF;    
        let oldCarry = self.cpsr.getCarry();

        let shift = (instruction >> 5) & 3;
        let mut shiftImm = (instruction >> 7) & 31;
        let opcode = (instruction >> 21) & 0xF;
        let affectFlags = s && rdIndex != 15;
       // debug_assert!(!(s && rdIndex == 15));

        let rn = self.getGPR(rnIndex);
        let mut rm = self.getGPR(rmIndex);

        if shiftImm == 0 && (shift == 1 || shift == 2) { // LSR #0 and ASR #0 become LSR #32 and ASR #32 instead
            shiftImm = 32;
        }

        if s && rdIndex == 15 {
            self.setCPSR(self.spsr.getRaw());
            //println!("IRQ/SWI return")
        }

        match shift {
            0 => rm = self.LSL(rm, shiftImm, affectFlags),
            1 => rm = self.LSR(rm, shiftImm, affectFlags),
            2 => rm = self.ASR(rm, shiftImm, affectFlags),
            _ => {
                if shiftImm != 0 {
                    rm = self.ROR(rm, shiftImm, affectFlags);
                }

                else {
                    rm = self.RRX(rm, affectFlags);
                }
            }
        }

        self.executeDP(opcode, rdIndex, rn, rm, affectFlags, oldCarry, bus)
    }

    pub fn executeDP (&mut self, opcode: u32, rdIndex: u32, operand1: u32, operand2: u32, affectFlags: bool, oldCarry: u32, bus: &mut Bus) {
        match opcode {
            0   => self.ARM_AND(rdIndex, operand1, operand2, affectFlags, bus),
            1   => self.ARM_EOR(rdIndex, operand1, operand2, affectFlags, bus),
            2   => self.ARM_SUB(rdIndex, operand1, operand2, affectFlags, bus),
            3   => self.ARM_SUB(rdIndex, operand2, operand1, affectFlags, bus),
            4   => self.ARM_ADD(rdIndex, operand1, operand2, affectFlags, bus),
            5   => self.ARM_ADC(rdIndex, operand1, operand2, affectFlags, oldCarry, bus),
            6   => self.ARM_SBC(rdIndex, operand1, operand2, affectFlags, oldCarry, bus),
            7   => self.ARM_SBC(rdIndex, operand2, operand1, affectFlags, oldCarry, bus),
            8   => self._TST(operand1, operand2),
            9   => self._TEQ(operand1, operand2),
            10  => self._CMP(operand1, operand2),
            11  => self._CMN(operand1, operand2),
            12  => self.ARM_ORR(rdIndex, operand1, operand2, affectFlags, bus),
            13  => self.ARM_MOV(rdIndex, operand2, affectFlags, bus),
            14  => self.ARM_BIC(rdIndex, operand1, operand2, affectFlags, bus),
             _  => self.ARM_MVN(rdIndex, operand2, affectFlags, bus)
        }
    }
    
    pub fn ARM_MOV(&mut self, rdIndex: u32, operand2: u32, affectFlags: bool, bus: &mut Bus) {
        self.setGPR(rdIndex, operand2, bus);
        if affectFlags {
            self.setSignAndZero(operand2);
        }
    }

    pub fn ARM_MVN(&mut self, rdIndex: u32, operand2: u32, affectFlags: bool, bus: &mut Bus) {
        let res = !operand2;
        self.setGPR(rdIndex, res, bus);
        if affectFlags {
            self.setSignAndZero(res);
        }
    }

    pub fn ARM_ADD(&mut self, rdIndex: u32, operand1: u32, operand2: u32, affectFlags: bool, bus: &mut Bus) {
        let res = self._ADD(operand1, operand2, affectFlags);
        self.setGPR(rdIndex, res , bus);
    }

    pub fn ARM_BIC(&mut self, rdIndex: u32, operand1: u32, operand2: u32, affectFlags: bool, bus: &mut Bus) {
        let res = self._BIC(operand1, operand2, affectFlags);
        self.setGPR(rdIndex, res , bus);
    }

    pub fn ARM_ADC(&mut self, rdIndex: u32, operand1: u32, operand2: u32, affectFlags: bool, oldCarry: u32, bus: &mut Bus) {
        let res = self._ADC(operand1, operand2, affectFlags, oldCarry);
        self.setGPR(rdIndex, res , bus);
    }

    pub fn ARM_SBC(&mut self, rdIndex: u32, operand1: u32, operand2: u32, affectFlags: bool, oldCarry: u32, bus: &mut Bus) {
        let res = self._SBC(operand1, operand2, affectFlags, oldCarry);
        self.setGPR(rdIndex, res , bus);
    }

    pub fn ARM_SUB(&mut self, rdIndex: u32, operand1: u32, operand2: u32, affectFlags: bool, bus: &mut Bus) {
        let res = self._SUB(operand1, operand2, affectFlags);
        self.setGPR(rdIndex, res , bus);
    }

    pub fn ARM_AND(&mut self, rdIndex: u32, operand1: u32, operand2: u32, affectFlags: bool, bus: &mut Bus) {
        let res = self._AND(operand1, operand2, affectFlags);
        self.setGPR(rdIndex, res, bus);
    }

    pub fn ARM_ORR(&mut self, rdIndex: u32, operand1: u32, operand2: u32, affectFlags: bool, bus: &mut Bus) {
        let res = self._ORR(operand1, operand2, affectFlags);
        self.setGPR(rdIndex, res, bus);
    }

    pub fn ARM_EOR(&mut self, rdIndex: u32, operand1: u32, operand2: u32, affectFlags: bool, bus: &mut Bus) {
        let res = self._EOR(operand1, operand2, affectFlags);
        self.setGPR(rdIndex, res, bus);
    }
}