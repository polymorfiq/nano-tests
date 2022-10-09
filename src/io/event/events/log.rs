use super::super::{EventType, MAX_QUEUED_EVENTS};

const MAX_MESSAGE_SIZE: usize = 128;
static mut LOG_BUFFER: LogEventBuffer<MAX_QUEUED_EVENTS> = LogEventBuffer::new();

#[derive(Clone, Copy)]
pub struct LogEvent {
    pub size: usize,
    pub message: [u8; MAX_MESSAGE_SIZE]
}

impl super::super::Event for LogEvent {
    fn event_type(&self) -> EventType { EventType::Log }
    fn as_any(&self) -> &dyn core::any::Any { self }
}

impl LogEvent {
    pub const fn new() -> Self {
        Self{size: 0, message: [0u8; MAX_MESSAGE_SIZE]}
    }

    pub fn pop() -> &'static mut Self {
        let evt = unsafe { LOG_BUFFER.pop() };
        evt.size = 0;
        evt.message = [0u8; MAX_MESSAGE_SIZE];
        evt
    }
}

pub struct LogEventBuffer<const SIZE: usize> {
    idx: usize,
    buffer: [LogEvent; MAX_QUEUED_EVENTS]
}
impl<const SIZE: usize> LogEventBuffer<SIZE> {
    pub const fn new() -> Self {
        Self {
            idx: 0,
            buffer: [LogEvent::new(); MAX_QUEUED_EVENTS]
        }
    }

    pub fn pop(&mut self) -> &mut LogEvent {
        let resp = &mut self.buffer[self.idx];
        self.idx = (self.idx + 1) % SIZE;
        resp
    }
}