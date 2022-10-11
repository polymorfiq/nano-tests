use crate::io::embedded_usb::EmbeddedUsb;

macro_rules! initialize_logger {
    ($core:ident, $peripherals:ident, $clocks:ident, $pins:ident) => {
        {
            init_usb_io!($core, $peripherals, $clocks, $pins);
            unsafe { &mut $crate::logger::LOGGER }
        }
    };
}

impl super::Logger {
    pub fn log_message_fragment(&self, message: &[u8]) {
        let mut usb = EmbeddedUsb::maybe_new();
        if let Some(device) = usb.as_mut() {
            device.do_write_from(message).unwrap();
        }
    }
    
    pub fn log_message_fragments(&self, fragments: &[&[u8]]) {
        let mut usb = EmbeddedUsb::maybe_new();
        if let Some(device) = usb.as_mut() {
            for fragment in fragments {
                device.do_write_from(fragment).unwrap();
            }
        }
    }
}