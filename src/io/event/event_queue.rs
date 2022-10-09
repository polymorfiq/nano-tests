use super::Event;
use super::MAX_QUEUED_EVENTS;

pub struct EventQueue<'a, const SIZE: usize> {
    pub(self) start_idx: usize,
    pub(self) end_idx: usize,
    pub(self) events: [&'a dyn Event; SIZE]
}

impl<'a, const SIZE: usize> EventQueue<'a, SIZE> {
    pub fn new() -> Self {
        Self {
            start_idx: 0,
            end_idx: 0,
            events: [&super::EMPTY_EVENT; SIZE]
        }
    }

    pub fn push(&mut self, event: &'a dyn Event) {
        self.events[self.end_idx] = event;
        self.end_idx = (self.end_idx + 1) % SIZE;
    }
}

impl<'a, const SIZE: usize> Iterator for EventQueue<'a, SIZE> {
    type Item = &'a dyn Event;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start_idx < self.end_idx {
            let resp = self.events[self.start_idx];
            self.start_idx = (self.start_idx + 1) % SIZE;
            Some(resp)
        } else {
            None
        }
    }
}

pub static mut GLOBAL_QUEUE: Option<EventQueue<MAX_QUEUED_EVENTS>> = None;
macro_rules! init_event_queue {
    () => {
        unsafe {
            use $crate::io::event::event_queue::GLOBAL_QUEUE;
            if GLOBAL_QUEUE.is_none() { GLOBAL_QUEUE = Some(EventQueue::new()); }
            GLOBAL_QUEUE.as_mut().unwrap()
        }
    }
}