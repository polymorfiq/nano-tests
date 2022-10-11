use modular_bitfield::prelude::*;

#[bitfield]
pub struct ChangeLedState {
    #[bits = 8]
    pub next_state: PossibleStates
}

#[derive(BitfieldSpecifier, Debug)]
#[bits = 8]
pub enum PossibleStates {
    None,
    LedOn = 101,
    LedOff = 102
}