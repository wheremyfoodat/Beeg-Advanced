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

    pub fn LSR (&mut self, number: u32, mut amount: u32, affectFlags: bool) -> u32 {
        println!("LSR executed! In case of a bug, revisit this!\n");
        amount &= 31;
        let res = number >> amount;

        if affectFlags && amount != 0 {
            self.cpsr.setCarry(isBitSet!(number, amount-1) as u32);
        }

        res
    }



    pub fn ASR (&mut self, number: u32, mut amount: u32, affectFlags: bool) -> u32 {
        let mut res = 0_u32;

        if (amount < 32) {
            res = ((number as i32) >> amount) as u32;
            if affectFlags && amount != 0 {
                self.cpsr.setCarry((number >> (amount-1)) & 1);
            }
        }

        else { // if shift amount > 31 => shift amount = 31, set carry to MSB (TOOD: confirm?)
            res = ((number as i32) >> 31) as u32;
            if (affectFlags) {
                self.cpsr.setCarry(res & 1);
            }
        }

        res
    }
}