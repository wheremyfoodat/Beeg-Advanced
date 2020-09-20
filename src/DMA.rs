use crate::bus::Bus;
use crate::io::DMACNT;

pub struct DMAChannel {
    pub wordCount: u16,
    pub sourceAddr: u32,
    pub destAddr: u32,
    pub controlReg: DMACNT
}

impl DMAChannel {
    pub fn new() -> DMAChannel {
        DMAChannel {
            wordCount: 0,
            sourceAddr: 0,
            destAddr: 0,
            controlReg: DMACNT(0)
        }
    }
}

const DMAOffsets: [i32; 4] = [4, -4, 0, 4];

impl Bus {
    pub fn fireDMA (&mut self, channel: u32) {
        let controlReg = &self.dmaChannels[channel as usize].controlReg;
        let mut source = self.dmaChannels[channel as usize].sourceAddr & 0x0FFFFFFF; // Top 4 bits of DMA source/dest are ignored
        let mut dest   = self.dmaChannels[channel as usize].destAddr & 0x0FFFFFFF;
        let wordCount = self.dmaChannels[channel as usize].wordCount;
        let destAddrControl = controlReg.getDestAddrControl();
        let srcAddrControl = controlReg.getSourceAddrControl();

        let is32Bit = controlReg.is32Bit() == 1;

        assert!(controlReg.getRepeat() == 0);
        assert!(controlReg.shouldFireIRQ() == 0);
        assert!(destAddrControl != 3);
        assert!(controlReg.getDMAStartTiming() == 0);
        assert!(wordCount < 0x4000 || channel == 3);
        assert!(wordCount != 0);

        println!("Firing DMA from channel {}. Word Count: {:04X}\nSource: {:08X}  Destination: {:08X}", channel, wordCount, source, dest);

        if is32Bit {
            for i in 0..wordCount {
                self.write32(dest, self.read32(source));
                dest += DMAOffsets[destAddrControl as usize] as u32;
                source += DMAOffsets[srcAddrControl as usize] as u32;
            }
        }
        
        else {
            for i in 0..wordCount {
                self.write16(dest, self.read16(source));
                dest += (DMAOffsets[destAddrControl as usize] >> 1) as u32;
                source += (DMAOffsets[srcAddrControl as usize] >> 1) as u32;
            }
        }
    }
}