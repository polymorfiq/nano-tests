pub mod structures;

pub mod events;
pub use events::change_led_state::PossibleStates as LedStates;
use events::change_led_state::ChangeLedState;

static mut STATE: State = State::new();
pub struct State {
    pub led_state: LedStates
}

impl State {
    pub fn global() -> &'static Self { unsafe { &STATE } }
    pub fn global_mut() -> &'static mut Self { unsafe { &mut STATE } }
    pub const fn new() -> Self {
        Self {
            led_state: LedStates::None
        }
    }

    pub fn change_led_state(&mut self, cmd: ChangeLedState) {
        if let Ok(new_state) = cmd.next_state_or_err() {
            self.led_state = new_state;
        }
    }
}