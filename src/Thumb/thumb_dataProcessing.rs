use crate::bus::Bus;
use crate::cpu::CPU;
use crate::isBitSet;

impl CPU {
    pub fn Thumb_handleMoveImm (&mut self, bus: &mut Bus, instruction: u32) {
        let offset = instruction & 0xFF;
        let rdIndex = (instruction >> 8) & 7;

        self.gprs[rdIndex as usize] = offset;
        self.cpsr.setNegative(0);
        self.cpsr.setZero((offset == 0) as u32);
    }

    pub fn Thumb_handleAddImm (&mut self, bus: &mut Bus, instruction: u32) {
        let offset = instruction & 0xFF;
        let rdIndex = (instruction >> 8) & 7;
        let rd = self.gprs[rdIndex as usize];
        self.gprs[rdIndex as usize] = self._ADD(rd, offset, true);
    }

    pub fn Thumb_handleSubImm (&mut self, bus: &mut Bus, instruction: u32) {
        let offset = instruction & 0xFF;
        let rdIndex = (instruction >> 8) & 7;
        let rd = self.gprs[rdIndex as usize];
        self.gprs[rdIndex as usize] = self._SUB(rd, offset, true);
    }

    pub fn Thumb_handleCMPImm (&mut self, bus: &mut Bus, instruction: u32) {
        let offset = instruction & 0xFF;
        let rdIndex = (instruction >> 8) & 7;
        let rd = self.gprs[rdIndex as usize];
        self._CMP(rd, offset);    
    }

    pub fn Thumb_handleAddReg (&mut self, bus: &mut Bus, instruction: u32) {
        let rdIndex = instruction & 7;
        let rsIndex = (instruction >> 3) & 7;
        let rnIndex = (instruction >> 6) & 7;

        let rs = self.gprs[rsIndex as usize];
        let rn = self.gprs[rnIndex as usize];
        let res = self._ADD(rs, rn, true);
        self.gprs[rdIndex as usize] = res;
    }

    pub fn Thumb_handleSubReg (&mut self, bus: &mut Bus, instruction: u32) {
        let rdIndex = instruction & 7;
        let rsIndex = (instruction >> 3) & 7;
        let rnIndex = (instruction >> 6) & 7;

        let rs = self.gprs[rsIndex as usize];
        let rn = self.gprs[rnIndex as usize];
        let res = self._SUB(rs, rn, true);
        self.gprs[rdIndex as usize] = res;
    }

    pub fn Thumb_handleAddOffset (&mut self, bus: &mut Bus, instruction: u32) {
        let rdIndex = instruction & 7;
        let rsIndex = (instruction >> 3) & 7;
        let offset = (instruction >> 6) & 7;

        let rs = self.gprs[rsIndex as usize];
        let res = self._ADD(rs, offset, true);
        self.gprs[rdIndex as usize] = res;
    }

    pub fn Thumb_handleSubOffset (&mut self, bus: &mut Bus, instruction: u32) {
        let rdIndex = instruction & 7;
        let rsIndex = (instruction >> 3) & 7;
        let offset = (instruction >> 6) & 7;

        let rs = self.gprs[rsIndex as usize];
        let res = self._SUB(rs, offset, true);
        self.gprs[rdIndex as usize] = res;
    }

    pub fn Thumb_handleALU (&mut self, bus: &mut Bus, instruction: u32) {
        let rdIndex = (instruction & 0x7) as usize;
        let rsIndex = (instruction >> 3) & 0x7;
        let opcode = (instruction >> 6) & 0xF;

        let mut rs = self.gprs[rsIndex as usize];
        let rd = self.gprs[rdIndex];

        match opcode {
            0 => self.gprs[rdIndex] = self._AND(rd, rs, true),
            1 => self.gprs[rdIndex] = self._EOR(rd, rs, true),
            2 => { self.gprs[rdIndex] = self.LSL(rd, rs, true); self.setSignAndZero(self.gprs[rdIndex]) },
            3 => { 
                   // if rs == 0 {
                   //     rs = 32;
                   //}
                    self.gprs[rdIndex] = self.LSR(rd, rs, true); 
                    self.setSignAndZero(self.gprs[rdIndex]) 
            },
            4 => { 
               // if rs == 0 {
               //     rs = 32;
               // }
                self.gprs[rdIndex] = self.ASR(rd, rs, true); 
                self.setSignAndZero(self.gprs[rdIndex]) 
            },
            5 => self.gprs[rdIndex] = self._ADC(rd, rs, true, self.cpsr.getCarry()),
            6 => self.gprs[rdIndex] = self._SBC(rd, rs, true, self.cpsr.getCarry()),
            7 => {
                    self.gprs[rdIndex] = self.ROR(rd, rs, true);
                    self.setSignAndZero(self.gprs[rdIndex]) 
            },
            8 => self._TST(rd, rs),
            9 => self.gprs[rdIndex] = self._SUB(0, rs, true),
            10 => self._CMP(rd, rs),
            11 => self._CMN(rd, rs),
            12 => self.gprs[rdIndex] = self._ORR(rd, rs, true),
            13 => self.gprs[rdIndex] = self._MUL(rd, rs, true),
            14 => self.gprs[rdIndex] = self._BIC(rd, rs, true),
            _  => { // MVN
                self.gprs[rdIndex] = !rs;
                self.setSignAndZero(!rs);
            }
        }
    }

    pub fn Thumb_handleHighRegOp (&mut self, bus: &mut Bus, instruction: u32) {
        let rdIndex = (instruction & 0x7) | ((instruction >> 4) & 0b1000);
        let rsIndex = ((instruction >> 3) & 0x7) | ((instruction >> 3) & 0b1000);
        let op = (instruction >> 8) & 0x3;

        let rs = self.gprs[rsIndex as usize];
        let rd = self.gprs[rdIndex as usize];

        match op {
            0 => { // ADD
                let res = self._ADD(rd, rs, false);
                self.setGPR(rdIndex, res, bus)
            },
            1 => self._CMP(rd, rs), // CMP
            2 => self.setGPR(rdIndex, self.gprs[rsIndex as usize], bus), // MOV
            _ => self.Thumb_handleBX(rs, bus) // BX
        }
    }

    pub fn Thumb_handleAddSignedOffsetToSP (&mut self, bus: &mut Bus, instruction: u32) {
        let imm = (instruction & 0x7F) << 2; // The immediate is a signed amount of words to be added to the sp
        if isBitSet!(instruction, 7) {
            self.gprs[13] -= imm;
        }
        
        else {
            self.gprs[13] += imm;
        }

    }
}