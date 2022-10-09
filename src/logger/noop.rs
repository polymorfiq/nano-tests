macro_rules! init_nano_33_iot {
    ($core:ident, $peripherals:ident, $clocks:ident, $pins:ident) => {
        $crate::logger::Logger::new()
    }
}

impl super::Logger {
    pub fn log_bytes(&mut self, message: &[u8]) {}
}