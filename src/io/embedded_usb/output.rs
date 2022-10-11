use crate::logger::Logger;
use crate::io::device;

impl<'a> super::EmbeddedUsb<'a> {
    pub fn do_write_from(&mut self, bytes: &[u8]) -> Result<device::WriteState, &str> {
        cortex_m::interrupt::free(|_|
            match self.serial.write(bytes) {
                Ok(bytes_written) => {
                    if bytes_written == bytes.len() {
                        Ok(device::WriteState::Full)
                    } else {
                        Logger::warning_fmt(format_args!("Did not fully write to USB: {:?} written instead of {:?}\n", bytes_written, bytes.len()));
                        Ok(device::WriteState::Partial(bytes_written))
                    }
                }

                Err(err) => {
                    Logger::warning_fmt(format_args!("Error reading header from USB: {:?}\n", err));
                    Err("USB Write Error: Unknown error!")
                }
            }
        )
    }
}

impl<'a> device::Write for super::EmbeddedUsb<'a> {
    fn write_from(&mut self, bytes: &[u8]) -> Result<device::WriteState, &str> {
        self.do_write_from(bytes)
    }
}