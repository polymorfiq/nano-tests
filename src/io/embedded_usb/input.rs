use arduino_nano33iot as bsp;
use bsp::hal::pac::interrupt;
use crate::state::events::EventHeader;
use crate::io::device;
use crate::logger::Logger;
use super::EmbeddedUsb;

impl<'a> EmbeddedUsb<'a> {
    fn poll_next_event_header(&mut self) {
        self.bus.poll(&mut [self.serial]);
        self.do_read_header()
    }

    fn do_read_header(&mut self) {
        let mut buf = EventHeader::new().into_bytes();

        match self.do_read_into(&mut buf) {
            Ok(device::ReadState::Full) => {
                let header = EventHeader::from_bytes(buf);
                self.parse_event_header(&header);
            }

            Ok(device::ReadState::Partial(bytes_read)) => {
                Logger::warning_fmt(format_args!("USB - Header size wrong: {:?} instead of {:?}\n", bytes_read, buf.len()));
            }

            Ok(device::ReadState::None) => (),

            Err(_) => ()
        };
    }

    fn do_read_into(&mut self, buf: &mut [u8]) -> Result<device::ReadState, &str> {
        match self.serial.read(buf) {
            Ok(bytes_read) => {
                if bytes_read == buf.len() {
                    Ok(device::ReadState::Full)
                } else {
                    Ok(device::ReadState::Partial(bytes_read))
                }
            },
            Err(usb_device::UsbError::WouldBlock) => Ok(device::ReadState::None),
            Err(err) => {
                Logger::warning_fmt(format_args!("USB - read_into error: {:?}\n", err));
                Err("USB Read Error!")
            }
        }
    }
}

impl<'a> device::Read for EmbeddedUsb<'a> {
    fn read_into(&mut self, buf: &mut [u8]) -> Result<device::ReadState, &str> {
        self.do_read_into(buf)
    }
}

#[interrupt]
fn USB() {
    let mut maybe_device = EmbeddedUsb::maybe_new();

    if let Some(device) = maybe_device.as_mut() {
        device.poll_next_event_header();
    }
}