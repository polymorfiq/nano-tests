use modular_bitfield::prelude::*;
use super::EventType;

#[bitfield]
#[derive(Clone, Copy)]
pub struct EventHeader {
    #[bits = 8]
    pub event_type: EventType,
    pub size: B32
}