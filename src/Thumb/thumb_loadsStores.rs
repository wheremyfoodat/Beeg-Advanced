use crate::bus::Bus;
use crate::cpu::CPU;
use crate::barrelShifter;
use crate::isBitSet;

impl CPU {
    pub fn Thumb_handleSPRelativeLoad (&mut self, bus: &mut Bus, instruction: u32) {
        //let imm = (instruction & 0xFF) << 2;
        //let sp = self.gprs[13];
        //let rdIndex = (instruction >> 8) & 0x7;

        //self.gprs[rdIndex as usize] = bus.read32(sp + imm);
        todo!("[THUMB] SP relative load!\n");
    }

    pub fn Thumb_handlePCRelativeLoad (&mut self, bus: &mut Bus, instruction: u32) {
        let imm = (instruction & 0xFF) << 2;
        let rdIndex = (instruction >> 8) & 0x7;
        let addr = (self.gprs[15] & !2) + imm;

        let mut val = bus.read32(addr & !3);
        val = self.ROR(val, 8 * (addr & 0x3), false);

        self.gprs[rdIndex as usize] = val;
    }

    pub fn Thumb_handleSTMIA (&mut self, bus: &mut Bus, instruction: u32) {
        let rlist = instruction & 0xFF;
        let rbIndex = (instruction >> 8) & 7;
        let mut sp = self.gprs[rbIndex as usize];

        debug_assert!(rlist != 0);
        debug_assert!(!isBitSet!(instruction, rbIndex));

        for x in 0..8 {
            if isBitSet!(rlist, x) {
                //println!("Pushing r{} to addr {:08X} (SP: r{})", x, sp, rbIndex);
                bus.write32(sp, self.gprs[x as usize]);
                sp += 4;
            }
        }

        self.gprs[rbIndex as usize] = sp;
    }
}