use crate::bus::Bus;
use crate::cpu::CPU;
use crate::barrelShifter;

impl CPU {
    pub fn Thumb_handleLSL (&mut self, bus: &mut Bus, instruction: u32) {
        let offset = (instruction >> 6) & 0x1F;
        let rsIndex = (instruction >> 3) & 0x7;
        let rdIndex = instruction & 0x7;

        let rs = self.gprs[rsIndex as usize];
        let res = self.LSL(rs, offset, true);
        self.setSignAndZero(res);
        self.gprs[rdIndex as usize] = res;
    }
    
    pub fn Thumb_handleLSR (&mut self, bus: &mut Bus, instruction: u32) {
        let offset = (instruction >> 6) & 0x1F;
        let rsIndex = (instruction >> 3) & 0x7;
        let rdIndex = instruction & 0x7;

        let rs = self.gprs[rsIndex as usize];
        let res = self.LSR(rs, offset, true);
        self.setSignAndZero(res);
        self.gprs[rdIndex as usize] = res;
    }

    pub fn Thumb_handleASR (&mut self, bus: &mut Bus, instruction: u32) {
        let offset = (instruction >> 6) & 0x1F;
        let rsIndex = (instruction >> 3) & 0x7;
        let rdIndex = instruction & 0x7;

        let rs = self.gprs[rsIndex as usize];
        let res = self.ASR(rs, offset, true);
        self.setSignAndZero(res);
        self.gprs[rdIndex as usize] = res;
    }
}