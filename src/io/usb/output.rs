use super::{BUS, SERIAL};

impl super::Device {
    pub fn write(&mut self, bytes: &[u8]) {
        // Turn off interrupts so we don't fight with the interrupt
        cortex_m::interrupt::free(|_| unsafe {
            BUS.as_mut().map(|_| {
                SERIAL.as_mut().map(|serial| {
                    let _ = serial.write(bytes);
                });
            })
        });
    }
}


impl core::fmt::Write for super::Device {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        self.write(s.as_bytes());
        Ok(())
    }
}