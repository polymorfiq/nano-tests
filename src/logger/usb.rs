macro_rules! initialize_logger {
    ($core:ident, $peripherals:ident, $clocks:ident, $pins:ident) => {
        {
            init_usb_io!($core, $peripherals, $clocks, $pins);
            unsafe { &mut $crate::logger::LOGGER }
        }
    };
}

impl super::Logger {
    pub fn log_bytes(&self, message: &[u8]) {
        unsafe { crate::io::usb::DEVICE.write(message) }
    }
}