use crate::logger::Logger;
use crate::state::{State, events};
use events::{ChangeLedState, EventHeader, EventType};

#[cfg(feature = "usb-io")]
#[macro_use]
pub mod embedded_usb;

pub mod device {
    pub enum ReadState {
        Full,
        Partial(usize),
        None
    }

    pub enum WriteState {
        Full,
        Partial(usize)
    }

    pub trait Read {
        fn read_into(&mut self, buf: &mut [u8]) -> Result<ReadState, &str>;
    }

    pub trait Write {
        fn write_from(&mut self, buf: &[u8]) -> Result<WriteState, &str>;
    }
}

pub fn parse_event_header(header: &EventHeader, device: &mut dyn device::Read) {
    use device::ReadState;
    let state = State::global_mut();

    match header.event_type_or_err() {
        Ok(EventType::ChangeLedState) => {
            let mut evt_bytes = ChangeLedState::new().into_bytes();
            match device.read_into(&mut evt_bytes) {
                Ok(ReadState::Full) => {
                    let evt = ChangeLedState::from_bytes(evt_bytes);
                    state.change_led_state(evt);
                },

                Ok(ReadState::Partial(bytes_read)) => {
                    Logger::warning_fmt(format_args!("Error reading change_led_state: {:?} bytes read instead of {:?}", bytes_read, evt_bytes.len()));
                }

                Ok(ReadState::None) => {
                    Logger::warning_fmt(format_args!("Error reading change_led_state: Event data missing"));
                }

                Err(err) => {
                    Logger::warning_fmt(format_args!("Error reading change_led_state: {:?}", err));
                }
            }
        },

        Ok(EventType::None) => (),

        Err(err) => {
            Logger::warning_fmt(format_args!("Invalid Event Header: {:?}", err));
        }
    }
}