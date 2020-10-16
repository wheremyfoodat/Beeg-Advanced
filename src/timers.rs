use crate::io::TMCNT;
use crate::bus::Bus;
use crate::scheduler::EventTypes;

const TIMER_THRESHOLDS: [u64; 4] = [1, 64, 256, 1024];

#[inline(always)]
pub fn timerNumToSchedulerEvent (timer_num: usize) -> EventTypes {
    match timer_num {
        0 => EventTypes::Timer0Overflow,
        1 => EventTypes::Timer1Overflow,
        2 => EventTypes::Timer2Overflow,
        _ => EventTypes::Timer3Overflow
    }
}

pub struct Timers {
    pub timer_values: [u16; 4],
    pub reload_values: [u16; 4],
    pub control_regs: [TMCNT; 4],
    pub starting_timestamps: [u64; 4],
    pub timer_interrupt_requests: u16
}

impl Timers {
    pub fn new () -> Timers {
        Timers {
            timer_values: [0; 4],
            reload_values: [0; 4],
            control_regs: [TMCNT(0), TMCNT(0), TMCNT(0), TMCNT(0)],
            starting_timestamps: [0; 4],
            timer_interrupt_requests: 0
        }
    }
}

impl Bus {
    pub fn writeTMCNT16 (&mut self, timer_num: usize, value: u16) {
        let old_reg = self.timers.control_regs[timer_num];
        let timer_overflow_event = timerNumToSchedulerEvent(timer_num);
        let new_reg = self.timers.control_regs[timer_num];

        self.scheduler.removeFirstEventByType(timer_overflow_event);
        self.timers.control_regs[timer_num].setRaw(value);

        if old_reg.isEnabled() && !new_reg.isEnabled() { // If the timer was on and it was turned off
            let time_passed = self.scheduler.currentTimestamp - self.timers.starting_timestamps[timer_num];
            self.timers.timer_values[timer_num] += (time_passed / TIMER_THRESHOLDS[old_reg.getFreq() as usize]) as u16;
        }

        else if new_reg.isEnabled() && !new_reg.isCascading() {
            let increments_until_overflow = 0x10000 as u64 - self.timers.timer_values[timer_num] as u64;
            let new_frequency = TIMER_THRESHOLDS[new_reg.getFreq() as usize];
            let cycles_until_overflow = self.scheduler.currentTimestamp + (increments_until_overflow * new_frequency);
            self.scheduler.pushEvent(timer_overflow_event, cycles_until_overflow);
            self.timers.starting_timestamps[timer_num] = self.scheduler.currentTimestamp;
        }
    }

    #[inline(always)]
    pub fn readTimer (&self, timer_num: usize) -> u16 {
        if self.timers.control_regs[timer_num].isCascading() {
            return self.timers.timer_values[timer_num]
        }

        let threshold = TIMER_THRESHOLDS[self.timers.control_regs[timer_num].getFreq() as usize];
        let time_passed = self.timers.starting_timestamps[timer_num] - self.scheduler.currentTimestamp;
        self.timers.timer_values[timer_num] + (time_passed / threshold) as u16
    }

    #[inline(always)]
    pub fn timer_overflow_callback (&mut self, timer_num: usize) {
        let control_reg = self.timers.control_regs[timer_num];
        let reload_value = self.timers.reload_values[timer_num];
        self.timers.timer_values[timer_num] = reload_value; // Load reload value into counter

        if control_reg.fireIRQ() {
            self.timers.timer_interrupt_requests |= 1 << (timer_num + 3);
            self.scheduler.pushEvent(EventTypes::PollInterrupts, 0);
        }
        
        if timer_num != 3 { // Check if the next timer is cascading
            let next_control_reg = self.timers.control_regs[timer_num + 1];
            if next_control_reg.isCascading() {
                self.timers.timer_values[timer_num + 1] += 1;
                if self.timers.timer_values[timer_num + 1] == 0 { // If the cascading timer also overflowed
                    self.timer_overflow_callback(timer_num + 1);
                }
            }
        }
    }
}