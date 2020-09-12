use bitfield::*;
use crate::bus::*;

bitfield!{
    pub struct PSR(u32);
    pub getRaw,        setRaw:        31, 0;
    pub getMode,       setMode:       4, 0;
    pub isThumb,       setThumbState: 5, 5;
    pub getFIQDisable, setFIQDisable: 6, 6;
    pub getIRQDisable, setIRQDisable: 7, 7;
    pub reserved,      setReserved:   27, 8;
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
    pub thumbLUT: [fn(&mut CPU, &mut Bus, u32); 1024],

    //For r8-r12: banks[0] is a backup of the universal reg, while banks[1] is a backup of the FIQ reg

    pub r8_banks: [u32; 2],
    pub r9_banks: [u32; 2],
    pub r10_banks: [u32; 2],
    pub r11_banks: [u32; 2],
    pub r12_banks: [u32; 2],

    // 0:Sys/User  1: FIQ  2: SVC  3: ABT  4: IRQ  5: UND

    pub r13_banks: [u32; 6],
    pub r14_banks: [u32; 6],
    pub spsr_banks: [PSR; 5],
}

pub enum CPUStates {
    ARM,
    Thumb
}

pub enum CPUModes {
    User_mode = 0x10,
    FIQ_mode,
    IRQ_mode,
    SVC_mode,
    ABT_mode  = 0x17,
    UND_mode  = 0x1B,
    System_mode = 0x1F
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            gprs: [0; 16],
            cpsr: PSR(0x1F),
            spsr: PSR(0),
            pipeline: [0; 3],
            armLUT:   [Self::ARM_handleUndefined; 4096],
            thumbLUT: [Self::ARM_handleUndefined; 1024],

            r8_banks: [0; 2],
            r9_banks: [0; 2],
            r10_banks: [0; 2],
            r11_banks: [0; 2],
            r12_banks: [0; 2],
        
            r13_banks: [0; 6],
            r14_banks: [0; 6],
            spsr_banks: [PSR(0), PSR(0), PSR(0), PSR(0), PSR(0)],
        }
    }

    pub fn init(&mut self, bus: &Bus) {
   /*   For skipping BIOS:
        self.setCPSR(0x6000001F);
        self.gprs[0] = 0x08000000;
        self.gprs[1] = 0x000000EA;
        self.gprs[13] = 0x03007F00;
        self.r13_banks[2] = 0x03007FE0; // SP SVC
        self.r13_banks[4] = 0x03007FA0; // SP IRQ

        self.gprs[15] = 0x08000000;
    */
        self.gprs[13] = 0x03007F00;
        self.gprs[15] = 0;
        self.refillPipeline(bus);
        self.populateARMLut();
        self.populateThumbLUT();
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
        self.cpsr.isThumb() == 0
    }

    pub fn step (&mut self, bus: &mut Bus) {
        if self.isInARMState() {
            self.executeARMInstruction(bus, self.pipeline[0]);
        }

        else {
            self.executeThumbInstruction(bus, self.pipeline[0]);
        }

        self.advancePipeline(bus)
    }

    pub fn advancePipeline(&mut self, bus: &Bus) {
        let mode = match self.isInARMState() {
            true => CPUStates::ARM,
            false => CPUStates::Thumb
        };

        self.pipeline[0] = self.pipeline[1];
        self.pipeline[1] = self.pipeline[2];
        
        match mode {
            CPUStates::ARM => {
                self.gprs[15] += 4;
                self.pipeline[2] = bus.read32(self.getGPR(15));
            },

            CPUStates::Thumb => {
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
            self.pipeline[1] = bus.read16(self.gprs[15] + 2) as u32;
            self.pipeline[2] = bus.read16(self.gprs[15] + 4) as u32;
            self.gprs[15] += 4;
        }
    }

    pub fn setCPSR(&mut self, val: u32) {
        self.changeMode(val & 0x1F);
        self.cpsr.setRaw(val);
    }

    pub fn cpuModeToArrayIndex(mode: u32) -> usize {
        match mode {
            0x10 | 0x1F => 0, // User/System
            0x11 => 1, // FIQ
            0x12 => 2, // IRQ
            0x13 => 3, // SVC
            0x17 => 4, // ABT
            0x1B => 5, // UND
            _ => panic!("Invalid CPU mode!\n")
        }
    }

    pub fn changeMode (&mut self, newMode: u32) {
        let currentMode = self.cpsr.getMode();
        self.cpsr.setMode(newMode);
        
        if currentMode == newMode {
            return;
        }

        match currentMode { // store r8-r12
            0x11 => { // FIQ mode
                self.r8_banks[1] = self.gprs[8];
                self.r9_banks[1] = self.gprs[9];
                self.r10_banks[1] = self.gprs[10];
                self.r11_banks[1] = self.gprs[11];
                self.r12_banks[1] = self.gprs[12];
            },

            _ => {
                self.r8_banks[0] = self.gprs[8];
                self.r9_banks[0] = self.gprs[9];
                self.r10_banks[0] = self.gprs[10];
                self.r11_banks[0] = self.gprs[11];
                self.r12_banks[0] = self.gprs[12];
            }
        }

        match currentMode { // store r13, 14, spsr
            0x10 | 0x1F => { // user and system mode 
                self.r13_banks[0] = self.gprs[13];
                self.r14_banks[0] = self.gprs[14];
            }

            _ => { 
                let currentModeindex = CPU::cpuModeToArrayIndex(currentMode);
                self.r13_banks[currentModeindex] = self.gprs[13];
                self.r14_banks[currentModeindex] = self.gprs[14];
                self.spsr_banks[currentModeindex-1].setRaw(self.spsr.getRaw());
            }
        }

        match newMode { // fetch new r8-r12
            0x11 => { // FIQ mode
                self.gprs[8] = self.r8_banks[1];
                self.gprs[9] = self.r9_banks[1];
                self.gprs[10] = self.r10_banks[1];
                self.gprs[11] = self.r11_banks[1];
                self.gprs[12] = self.r12_banks[1];
            }

            _ => { // rest of the modes
                self.gprs[8] = self.r8_banks[0];
                self.gprs[9] = self.r9_banks[0];
                self.gprs[10] = self.r10_banks[0];
                self.gprs[11] = self.r11_banks[0];
                self.gprs[12] = self.r12_banks[0];
            }
        }

        match newMode { // fetch new r13, r14, spsr
            0x10 | 0x1F => { // User/System
                self.gprs[13] = self.r13_banks[0];
                self.gprs[14] = self.r14_banks[0];
            }

            _ => { // rest of the modes
                let newModeIndex = CPU::cpuModeToArrayIndex(newMode);
                self.gprs[13] = self.r13_banks[newModeIndex];
                self.gprs[14] = self.r14_banks[newModeIndex];
                self.spsr.setRaw(self.spsr_banks[newModeIndex-1].getRaw())
            }
        }
    }

    pub fn isConditionTrue (&self, condition: u32) -> bool {
        match condition {
            14 => true, // AL
            0 => self.cpsr.getZero()  == 1, // EQ
            1 => self.cpsr.getZero()  == 0, // NE
            2 => self.cpsr.getCarry() == 1, // CS
            3 => self.cpsr.getCarry() == 0, // CC
            4 => self.cpsr.getNegative() == 1, // MI
            5 => self.cpsr.getNegative() == 0, // PL
            6 => self.cpsr.getOverflow() == 1, // VS
            7 => self.cpsr.getOverflow() == 0, // VC
            8 => self.cpsr.getCarry() == 1 && self.cpsr.getZero() == 0, // HI!
            9 => self.cpsr.getCarry() == 0 || self.cpsr.getZero() == 1, // LO
            10 => self.cpsr.getNegative() == self.cpsr.getOverflow(),   // GE 
            11 => self.cpsr.getNegative() != self.cpsr.getOverflow(),   // LT
            12 => self.cpsr.getZero() == 0 && (self.cpsr.getNegative() == self.cpsr.getOverflow()), // GT
            13 => self.cpsr.getZero() == 1 || (self.cpsr.getNegative() != self.cpsr.getOverflow()), // LE
            _  => { panic!("CONDITION CODE NV!\n") }
        }
    }

    pub fn setSignAndZero (&mut self, val: u32) {
        self.cpsr.setZero((val == 0) as u32);
        self.cpsr.setNegative(val >> 31);
    }

    pub fn logState (&mut self) {
        for i in 0..8 {
            println!("r{}: {:08X} r{}: {:08X}", i * 2, self.getGPR(i * 2), i *2 + 1, self.getGPR(i * 2 + 1));
        }

        println!("CPSR: {:08X}\nSPSR: {:08X}", self.cpsr.getRaw(), self.spsr.getRaw());
        println!("Negative: {} Zero: {}", self.cpsr.getNegative(), self.cpsr.getZero());
        println!("Carry: {} Overflow: {}", self.cpsr.getCarry(), self.cpsr.getOverflow());
        println!("Thumb: {}\n", self.cpsr.isThumb());
/*        
        let mut pc = self.gprs[15];
        if self.isInARMState() {pc -= 4}
        else {pc -= 2};

        self.log.push_str(format!("{:08X} {:08X} {:08X} {:08X} {:08X} {:08X} {:08X} {:08X} {:08X} {:08X} {:08X} {:08X} {:08X} {:08X} {:08X} {:08X} {:08X} cpsr: {:08X}\n",
                    self.gprs[0], self.gprs[1], self.gprs[0], self.gprs[2], self.gprs[3], self.gprs[4], self.gprs[5],
                    self.gprs[6], self.gprs[7], self.gprs[8], self.gprs[9], self.gprs[10], self.gprs[11], self.gprs[12],
                    self.gprs[13], self.gprs[14], pc, self.cpsr.getRaw()).as_str())
*/
    }

    pub fn _ADD(&mut self, operand1: u32, operand2: u32, affectFlags: bool) -> u32 {
        let res = operand1 as u64 + operand2 as u64;
        
        if affectFlags {
            self.cpsr.setCarry((res > 0xFFFFFFFF) as u32);
            self.setSignAndZero(res as u32);
            self.cpsr.setOverflow(((operand1 >> 31) == (operand2 >> 31) && (operand1 >> 31) != (res as u32 >> 31)) as u32)
        }

        res as u32
    }

    pub fn _ADC(&mut self, operand1: u32, operand2: u32, affectFlags: bool, carry: u32) -> u32 {
        let res = operand1 as u64 + operand2 as u64 + carry as u64;

        if affectFlags {
            self.cpsr.setCarry((res > 0xFFFFFFFF) as u32);
            self.setSignAndZero(res as u32);
            self.cpsr.setOverflow(((operand1 >> 31) == (operand2 >> 31) && (operand1 >> 31) != (res as u32 >> 31)) as u32)
        }

        res as u32
    }

    pub fn _SBC(&mut self, operand1: u32, operand2: u32, affectFlags: bool, carry: u32) -> u32 {
        let subtrahend = operand2 as u64 - carry as u64 + 1_u64;
        let res = (operand1 as u64).wrapping_sub(subtrahend);

        if affectFlags {
            self.cpsr.setCarry((subtrahend <= operand1 as u64) as u32);
            self.setSignAndZero(res as u32);
            self.cpsr.setOverflow(((operand1 >> 31) != (operand2 >> 31) && (operand2 >> 31) == (res as u32 >> 31)) as u32)
        }

        res as u32
    }

    pub fn _AND(&mut self, operand1: u32, operand2: u32, affectFlags: bool) -> u32 {
        let res = operand1 & operand2;

        if affectFlags {
            self.setSignAndZero(res);
        }

        res
    }

    pub fn _ORR(&mut self, operand1: u32, operand2: u32, affectFlags: bool) -> u32 {
        let res = operand1 | operand2;

        if affectFlags {
            self.setSignAndZero(res);
        }

        res
    }

    pub fn _EOR(&mut self, operand1: u32, operand2: u32, affectFlags: bool) -> u32 {
        let res = operand1 ^ operand2;

        if affectFlags {
            self.setSignAndZero(res);
        }

        res
    }

    pub fn _SUB(&mut self, operand1: u32, operand2: u32, affectFlags: bool) -> u32 {
        let res = operand1.wrapping_sub(operand2);

        if affectFlags {
            self.cpsr.setCarry((res <= operand1) as u32);
            self.setSignAndZero(res);
            self.cpsr.setOverflow(((operand1 >> 31) != (operand2 >> 31) && (operand2 >> 31) == (res >> 31)) as u32);
        }

        res
    }

    pub fn _BIC(&mut self, operand1: u32, operand2: u32, affectFlags: bool) -> u32 {
        let res = operand1 & !operand2;
        if affectFlags { 
            self.setSignAndZero(res) 
        }
        
        res
    }

    pub fn _CMP(&mut self, operand1: u32, operand2: u32) {
        let res = operand1.wrapping_sub(operand2);
        self.cpsr.setCarry((res <= operand1) as u32);
        self.setSignAndZero(res);
        self.cpsr.setOverflow(((operand1 >> 31) != (operand2 >> 31) && (operand2 >> 31) == (res >> 31)) as u32);
    }

    pub fn _TEQ(&mut self, operand1: u32, operand2: u32) {
        self.setSignAndZero(operand1 ^ operand2)
    }

    pub fn _TST(&mut self, operand1: u32, operand2: u32) {
        self.setSignAndZero(operand1 & operand2)
    }

    pub fn _MUL(&mut self, operand1: u32, operand2: u32, affectFlags: bool) -> u32 {
        let res = operand1.wrapping_mul(operand2);
        if affectFlags {
            self.setSignAndZero(res);
        }

        res
    }
}