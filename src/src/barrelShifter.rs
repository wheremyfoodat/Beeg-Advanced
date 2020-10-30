use crate::cpu::CPU;

impl CPU {
    #[inline(always)]
    pub fn ROR (&mut self, number: u32, amount: u32, affectFlags: bool) -> u32 {
        let res = number.rotate_right(amount); 

        if affectFlags && amount != 0 {
            self.cpsr.setCarry(res >> 31);
        }

        res
    }

    #[inline(always)]
    pub fn LSL (&mut self, number: u32, amount: u32, affectFlags: bool) -> u32 {
        let res: u32;

        if amount < 32 {
            res = number << amount;

            if affectFlags && amount != 0 {
                self.cpsr.setCarry((number >> (32 - amount)) & 1);
            }
        }

        else {
            res = 0;
            if affectFlags {
                if amount == 32 {
                    self.cpsr.setCarry(number & 1)
                }
                else {
                    self.cpsr.setCarry(0);
                }
            }
        }

        res
    }

    #[inline(always)]
    pub fn LSR (&mut self, number: u32, amount: u32, affectFlags: bool) -> u32 {
        let res: u32;

        if amount < 32 {
            res = number >> amount;

            if affectFlags && amount != 0 {
                self.cpsr.setCarry((number >> (amount-1)) & 1);
            }
        }

        else {
            res = 0;
            if affectFlags {
                if amount == 32 {
                    self.cpsr.setCarry(number >> 31);
                }
                else {
                    self.cpsr.setCarry(0);
                }
            }
        }

        res
    }

    #[inline(always)]
    pub fn ASR (&mut self, number: u32, amount: u32, affectFlags: bool) -> u32 {
       //debug_assert!(amount < 32 && amount != 0);
        let res: u32;

        if amount < 32 {
            res = ((number as i32) >> amount) as u32;
            if affectFlags && amount != 0 {
                self.cpsr.setCarry((number >> (amount-1)) & 1);
            }
        }

        else { // if shift amount > 31 => shift amount = 31, set carry to MSB (TOOD: confirm?)
            res = ((number as i32) >> 31) as u32;
            if affectFlags {
                self.cpsr.setCarry(res & 1);
            }
        }

        res
    }

    #[inline(always)]
    pub fn RRX (&mut self, number: u32, affectFlags: bool) -> u32 {
        let res = (number >> 1) | (self.cpsr.getCarry() << 31);
        if affectFlags {
            self.cpsr.setCarry(number & 1);
        }

        res
    }
}