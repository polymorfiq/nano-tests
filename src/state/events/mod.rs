use modular_bitfield::prelude::*;

pub mod change_led_state;
pub use change_led_state::ChangeLedState;

#[derive(BitfieldSpecifier, Debug)]
#[bits = 8]
pub enum EventType {
    None,
    ChangeLedState = 1 as isize
}

#[bitfield]
#[derive(Clone, Copy)]
pub struct EventHeader {
    #[bits = 8]
    pub event_type: EventType
}