use bitfield::*;
use crate::ARM::*;
use crate::bus::*;

bitfield!{
    pub struct PSR(u32);
    pub getMode,       setMode:       4, 0;
    pub isThumb,       setThumbState: 5, 5;
    pub getFIQDisable, setFIQDisable: 6, 6;
    pub getIRQDisable, setIRQDisable: 7, 7;
    pub reserved,      setReserved:   8, 27;
    pub getOverflow,   setOverflow:   28, 28;
    pub getCarry,      setCarry:      29, 29;
    pub getZero,       setZero:       30, 30;
    pub getNegative,   setNegative:   31, 31;
}

pub struct CPU {
    pub gprs: [u32; 16],
    pub cpsr: PSR,
    pub spsr: PSR,
    pub pipeline: [u32; 3],
    pub armLUT: [fn(&mut CPU, &mut Bus, u32); 4096], 
    pub thumbLUT: [fn(&mut CPU, &mut Bus, u32); 1024]
}

pub enum CPUModes {
    ARM,
    Thumb
}


impl CPU {
    pub fn new() -> CPU {
        CPU {
            gprs: [0; 16],
            cpsr: PSR(0x000000D3),
            spsr: PSR(0),
            pipeline: [0; 3],
            armLUT:   [Self::ARM_handleUndefined; 4096],
            thumbLUT: [Self::ARM_handleUndefined; 1024]
        }
    }

    pub fn init(&mut self, bus: &Bus) {
        self.gprs[15] = 0x8000000;
        self.refillPipeline(bus);
        self.populateARMLut();
    }

    pub fn getGPR(&mut self, gpr: u32) -> u32 {
        self.gprs[gpr as usize]
    } 

    pub fn setGPR(&mut self, gpr: u32, val: u32, bus: &mut Bus) {
        match gpr {
            15 => {
                if self.isInARMState() { self.gprs[15] = (val - 4) & !3 }
                else {self.gprs[15] = (val - 2) & !1}
                self.refillPipeline(bus);
            }
            _ => self.gprs[gpr as usize] = val
        }
    }

    pub fn isInARMState(&self) -> bool {
        return self.cpsr.isThumb() == 0
    }

    pub fn step (&mut self, bus: &mut Bus) {
        if self.isInARMState() {
            self.executeARMInstruction(bus, self.pipeline[0]);
        }

        else {
            todo!("Implement THUMB!\n");
        }

        self.advancePipeline(bus)
    }

    pub fn advancePipeline(&mut self, bus: &Bus) {
        let mode = match self.isInARMState() {
            true => CPUModes::ARM,
            false => CPUModes::Thumb
        };

        self.pipeline[0] = self.pipeline[1];
        self.pipeline[1] = self.pipeline[2];
        
        match mode {
            CPUModes::ARM => {
                self.gprs[15] += 4;
                self.pipeline[2] = bus.read32(self.getGPR(15));
            },

            CPUModes::Thumb => {
                self.gprs[15] += 2;
                self.pipeline[2] = bus.read16(self.getGPR(15)) as u32;
            }
        }
    }

    pub fn refillPipeline (&mut self, bus: &Bus) {
        if self.isInARMState() {
            self.pipeline[0] = bus.read32(self.gprs[15]);
            self.pipeline[1] = bus.read32(self.gprs[15] + 4);
            self.pipeline[2] = bus.read32(self.gprs[15] + 8);
            self.gprs[15] += 8;
        }

        else {
            self.pipeline[0] = bus.read16(self.gprs[15]) as u32;
            self.pipeline[1] = bus.read16 (self.gprs[15] + 2) as u32;
            self.pipeline[2] = bus.read16 (self.gprs[15] + 4) as u32;
            self.gprs[15] += 4;
        }
    }

    pub fn isConditionTrue (&self, condition: u32) -> bool {
        match condition {
            0 => self.cpsr.getZero()  == 1, // EQ
            1 => self.cpsr.getZero()  == 0, // NE
            2 => self.cpsr.getCarry() == 1, // CS
            3 => self.cpsr.getCarry() == 0, // CC
            4 => self.cpsr.getNegative() == 1, // MI
            5 => self.cpsr.getNegative() == 0, // PL
            6 => self.cpsr.getOverflow() == 1, // VS
            7 => self.cpsr.getOverflow() == 0, // VC
            8 => self.cpsr.getCarry() == 1 && self.cpsr.getZero() == 0, // HI!
            9 => self.cpsr.getCarry() == 0 && self.cpsr.getZero() == 1, // LO
            10 => self.cpsr.getNegative() == self.cpsr.getOverflow(),   // GE 
            11 => self.cpsr.getNegative() != self.cpsr.getOverflow(),   // LT
            12 => self.cpsr.getZero() == 0 && (self.cpsr.getNegative() == self.cpsr.getOverflow()), // GT
            13 => self.cpsr.getZero() == 1 || (self.cpsr.getNegative() != self.cpsr.getOverflow()), // LE
            14 => true, // AL
            _  => panic!("CONDITION CODE NV!\n")
        }
    }

    pub fn setSignAndZero (&mut self, val: u32) {
        self.cpsr.setZero((val == 0) as u32);
        self.cpsr.setNegative(val >> 31);
    }
}