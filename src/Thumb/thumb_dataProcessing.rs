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

    pub fn Thumb_handleSubImm (&mut self, bus: &mut Bus, instruction: u32) {
        let offset = instruction & 0xFF;
        let rdIndex = (instruction >> 8) & 7;
        let rd = self.gprs[rdIndex as usize];
        self.gprs[rdIndex as usize] = self._SUB(rd, offset, true);
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
        let rdIndex = instruction & 0x7;
        let rsIndex = (instruction >> 3) & 0x7;
        let opcode = (instruction >> 6) & 0xF;

        let rs = self.gprs[rsIndex as usize];
        let rd = self.gprs[rdIndex as usize];

        match opcode {
            0 => todo!("[THUMB] Implement AND!\n"),
            1 => todo!("[THUMB] Implement EOR!\n"),
            2 => todo!("[THUMB] Implement LSL!\n"),
            3 => todo!("[THUMB] Implement LSR!\n"),
            4 => todo!("[THUMB] Implement ASR!\n"),
            5 => todo!("[THUMB] Implement ADC!\n"),
            6 => todo!("[THUMB] Implement SBC!\n"),
            7 => todo!("[THUMB] Implement ROR!\n"),
            8 => todo!("[THUMB] Implement TST!\n"),
            9 => todo!("[THUMB] Implement NEG!\n"),
            10 => todo!("[THUMB] Implement CMP!\n"),
            11 => todo!("[THUMB] Implement CMN!\n"),
            12 => todo!("[THUMB] Implement ORR!\n"),
            13 => todo!("[THUMB] Implement MUL!\n"),
            14 => self.gprs[rdIndex as usize] = self._BIC(rd, rs, true),
            _  => todo!("[THUMB] Implement MVN!\n")
        }
    }

    pub fn Thumb_handleHighRegOp (&mut self, bus: &mut Bus, instruction: u32) {
        let h1 = isBitSet!(instruction, 7);
        let h2 = isBitSet!(instruction, 6);

        let mut rdIndex = instruction & 0x7;
        let mut rsIndex = (instruction >> 3) & 0x7;
        let op = (instruction >> 8) & 0x3;

        if h1 {rdIndex += 8}
        if h2 {rsIndex += 8}

        let rs = self.gprs[rsIndex as usize];
        let rd = self.gprs[rdIndex as usize];

        match op {
            0 => todo!("[THUMB] Implement high reg ADD"),
            1 => todo!("[THUMB] Implement high reg CMP"),
            2 => todo!("[THUMB] Implement high reg MOV"),
            _ => self.Thumb_handleBX(rs, bus)
        }
    }
}