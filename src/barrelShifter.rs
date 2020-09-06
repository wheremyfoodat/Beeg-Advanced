use crate::cpu::CPU;
#[macro_use]
use crate::isBitSet;

impl CPU {
    pub fn ROR (&mut self, number: u32, mut amount: u32, affectFlags: bool) -> u32 {
        amount &= 31;
        let res = number.rotate_right(amount); 

        if affectFlags {
            self.cpsr.setCarry(isBitSet!(res, 31) as u32);
        }

        res
    }
}