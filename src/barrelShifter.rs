use crate::cpu::CPU;
#[macro_use]
use crate::isBitSet;

impl CPU {
    pub fn ROR (&mut self, number: u32, mut amount: u32, affectFlags: bool) -> u32 {
        amount &= 31;
        let res = number.rotate_right(amount); 

        if affectFlags && amount != 0 {
            self.cpsr.setCarry(isBitSet!(res, 31) as u32);
        }

        res
    }

    pub fn LSL (&mut self, number: u32, mut amount: u32, affectFlags: bool) -> u32 {
        println!("LSL executed! In case of a bug, revisit this!\n");

        amount &= 31;
        let res = number << amount;

        if affectFlags && amount != 0 {
            self.cpsr.setCarry(isBitSet!(number, 32-amount) as u32);
        }

        res
    }
}