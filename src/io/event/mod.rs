use modular_bitfield::prelude::*;

mod event;
pub use event::Event;

mod event_header;
pub use event_header::EventHeader;

#[macro_use]
pub mod event_queue;
pub use event_queue::EventQueue;

pub mod events;
pub use events::NoopEvent;

#[derive(BitfieldSpecifier, Debug)]
#[bits = 8]
pub enum EventType {
    None,
    Log = 076 as isize // 'L' in ASCII
}

pub const EMPTY_EVENT: NoopEvent = NoopEvent{};
pub const MAX_QUEUED_EVENTS: usize = 20;