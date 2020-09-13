use crate::cpu::CPU;
use crate::bus::Bus;

impl CPU {
    pub fn pollInterrupts (&mut self, bus: &mut Bus) {
        if self.cpsr.getIRQDisable() == 0 && bus.ime && ((bus.ie & bus.ppu.interruptFlags as u16) != 0) { // TODO: Handle writes to IF and misc interrupts
            let cpsr = self.cpsr.getRaw();
            let lr: u32;

            if self.isInARMState() {
                lr = self.gprs[15] - 4;
                //println!("Firing ARM mode interrupt. Current instr address {:08X}\n Return address: {:08X}", self.gprs[15]-8, lr)
            }

            else {
                lr = self.gprs[15] + 2;
                println!("Firing Thumb mode interrupt. Current instr address {:08X}\n Return address: {:08X}", self.gprs[15]-4, lr);
                //std::thread::sleep_ms(10000);
                todo!("Interrupts in Thumb mode!")
            }

            self.changeMode(0x12); // Enter IRQ mode
            self.spsr.setRaw(cpsr); // Copy previous CPSR to current mode SPSR
            self.cpsr.setThumbState(0); // Enter ARM mode
            self.cpsr.setIRQDisable(1); // Disable IRQs
            self.gprs[14] = lr; // Set return address
            self.gprs[15] = 0x18;
            self.refillPipeline(bus);
        }
    }
}