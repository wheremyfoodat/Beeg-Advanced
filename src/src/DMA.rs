use crate::bus::Bus;
use crate::io::DMACNT;

#[derive(PartialEq)]
pub enum DMAChannelStatus {
    Inactive, // No DMA is enabled on this channel
    Immediate, // An instant DMA is currently being executed (Note: My immediate mode DMA is instant, so this is currently unused) 
    HBlank, // This channel will fire a DMA on the next HBlank
    VBlank, // This channel will fire a DMA on the next VBlank
    Special
}

pub struct DMAChannel {
    pub wordCount: u16,
    pub sourceAddr: u32,
    pub destAddr: u32,
    pub controlReg: DMACNT,

    pub status: DMAChannelStatus,
    pub repeatSrcAddr: u32,
    pub repeatDestAddr: u32
}

impl DMAChannel {
    pub fn new() -> DMAChannel {
        DMAChannel {
            wordCount: 0,
            sourceAddr: 0,
            destAddr: 0,
            controlReg: DMACNT(0),

            status: DMAChannelStatus::Inactive,
            repeatSrcAddr: 0,
            repeatDestAddr: 0
        }
    }
}

const DMAOffsets: [i32; 4] = [4, -4, 0, 4];

impl Bus {
    pub fn fireDMA (&mut self, channel: usize) {
        let controlReg = self.dmaChannels[channel].controlReg;
        let mut source: u32;
        let mut dest: u32;
        let mut wordCount = self.dmaChannels[channel].wordCount as u32;
        let destAddrControl = controlReg.getDestAddrControl() as usize;
        let srcAddrControl = controlReg.getSourceAddrControl() as usize;

        if !controlReg.getRepeat() {
            source = self.dmaChannels[channel].sourceAddr & 0x0FFFFFFF; // Top 4 bits of DMA source/dest are ignored
            dest = self.dmaChannels[channel].destAddr & 0x0FFFFFFF;
        }

        else {
            source = self.dmaChannels[channel].repeatSrcAddr & 0x0FFFFFFF;
            dest = self.dmaChannels[channel].repeatDestAddr & 0x0FFFFFFF;
        }

        assert!(srcAddrControl != 3);
        assert!(wordCount < 0x4000 || channel == 3);
        
        if wordCount == 0 { // If word count is 0, it gets set to 0x4000, or 0x10000 for DMA3
            wordCount = 0x4000;
            if channel == 3 { wordCount = 0x10000 }
        }

        //println!("Firing DMA from channel {}. Word Count: {:04X}\nSource: {:08X}  Destination: {:08X}", channel, wordCount, source, dest);

        if controlReg.is32Bit() { // If the transfer is 32 bit
            dest &= !3; // Align the dest and source addresses
            source &= !3;

            for i in 0..wordCount {
                self.write32(dest, self.read32(source));
                dest += DMAOffsets[destAddrControl] as u32;
                source += DMAOffsets[srcAddrControl] as u32;
            }
        }
        
        else {
            dest &= !1; // Align the dest and source addresses
            source &= !1;

            for i in 0..wordCount {
                self.write16(dest, self.read16(source));
                dest += (DMAOffsets[destAddrControl] >> 1) as u32;
                source += (DMAOffsets[srcAddrControl] >> 1) as u32;
            }
        }

        if controlReg.shouldFireIRQ() { // Request IRQ upon end of word count 
            self.dma_irq_requests |= (1 << 8 << channel);
        }

        if controlReg.getRepeat() && self.dmaChannels[channel].status != DMAChannelStatus::Immediate {
            if destAddrControl != 3 { // Dest addr control 3 is reload.
                self.dmaChannels[channel].repeatDestAddr = dest;
            }
            self.dmaChannels[channel].repeatSrcAddr = source;
        }

        else {
            self.dmaChannels[channel].status = DMAChannelStatus::Inactive;
            self.dmaChannels[channel].controlReg.DMAEnable(false);
        }
    }

    pub fn writeDMACNT32 (&mut self, channelNum: usize, val: u32) {
        self.dmaChannels[channelNum].wordCount = val as u16;
        self.writeDMACNTHigh(channelNum, (val >> 16) as u16);
    }

    pub fn writeDMACNTHigh (&mut self, channelNum: usize, val: u16) {
        self.dmaChannels[channelNum].controlReg.setRaw(val);
       // println!("Wrote {:04X} to DMA{}CNT!", val, channelNum);

        if (val >> 15) == 1 { // If enable bit is 1
            self.dmaChannels[channelNum].repeatDestAddr = self.dmaChannels[channelNum].destAddr;
            self.dmaChannels[channelNum].repeatSrcAddr = self.dmaChannels[channelNum].sourceAddr;

            match (val >> 12) & 0x3 {
                0 => self.fireDMA(channelNum),
                1 => self.dmaChannels[channelNum].status = DMAChannelStatus::VBlank,
                2 => self.dmaChannels[channelNum].status = DMAChannelStatus::HBlank,
                _ => self.dmaChannels[channelNum].status = DMAChannelStatus::Special
            }
        }

        else {
            self.dmaChannels[channelNum].status = DMAChannelStatus::Inactive;
        }
    }

    // Used for polling HBlank DMAs and VBlank DMAs
    pub fn pollDMAs (&mut self, status: DMAChannelStatus) {
        for i in 0..4 {
            if self.dmaChannels[i].status == status {
                self.fireDMA(i);
            }
        }
    }
}