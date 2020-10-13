extern crate staticvec;
use crate::staticvec::StaticVec;

const MAX_NUM_OF_EVENTS: usize = 16; // The max amount of events that can be on the scheduler at a time
 
#[derive(Copy, Clone, PartialEq)]
pub enum EventTypes {
  VBlank,
  HBlank,
  EndOfLine,
  Timer0Overflow,
  Timer1Overflow,
  Timer2Overflow,
  Timer3Overflow,
  PollInterrupts,
  Panic
}
 
#[derive(Copy, Clone)]
pub struct Event {
  pub eventType: EventTypes, // Type of the event
  pub endTimestamp: u64   // The timestamp at which the event will be fired
}
 
impl Event {
  pub fn new (eventType: EventTypes, endTimestamp: u64) -> Event {
    Event {
      eventType,
      endTimestamp
    }
  }
}
 
pub struct Scheduler {
  pub eventList: StaticVec::<Event, MAX_NUM_OF_EVENTS>,
  pub currentTimestamp: u64
}
 
impl Scheduler {
  pub fn new() -> Scheduler {
    let mut scheduler = Scheduler {
      eventList: StaticVec::<Event, MAX_NUM_OF_EVENTS>::new(),
      currentTimestamp: 0
    };
 
    scheduler.eventList.insert(0, Event::new(EventTypes::Panic, u64::MAX)); // 1 event that constantly stays on the scheduler
    scheduler
  }
 
  pub fn pushEvent(&mut self, eventType: EventTypes, endTimestamp: u64) {
    for i in 0..self.eventList.len() {
      if endTimestamp <= self.eventList[i].endTimestamp {
        self.eventList.insert(i, Event::new(eventType, endTimestamp));
        return;
      }
    }
  }
 
  pub fn getNearestEvent(&self) -> Event {
    self.eventList[0]
  }
 
  pub fn removeEvent(&mut self) {
    unsafe {
      self.eventList.remove(0);
    }
  }
 
  pub fn removeFirstEventByType (&mut self, eventType: EventTypes) { // Todo: Use StaticVec.remove_item here
    for i in 0..self.eventList.len() {
      if self.eventList[i].eventType == eventType {
        self.eventList.remove(i);
        return;
      }
    }
  }
}
