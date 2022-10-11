#![allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Logger {}

#[cfg(not(feature = "usb-logger"))]
#[macro_use]
mod noop;

#[cfg(feature = "usb-logger")]
#[macro_use]
pub mod usb;

impl Logger {
    pub const fn new() -> Self { Self {} }
    pub fn do_write(message: &str) {
        unsafe {
            LOGGER.log_message_fragments(&[message.as_bytes()]);
        }
    }

    pub fn debug_fmt(args: core::fmt::Arguments) {
        unsafe {
            core::fmt::write(&mut LOGGER, format_args!("Debug: {}\n", args)).unwrap();
        }
    }

    pub fn warning_fmt(args: core::fmt::Arguments) {
        unsafe {
            core::fmt::write(&mut LOGGER, format_args!("Warning: {}\n", args)).unwrap();
        }
    }

    pub fn print(&mut self, message: &str) {
        self.log_message_fragments(&[message.as_bytes()]);
    }

    pub fn println(&mut self, message: &str) {
        self.log_message_fragments(&[message.as_bytes(), "\n".as_bytes()]);
    }

    pub fn printf(&mut self, args: core::fmt::Arguments) -> core::fmt::Result {
        core::fmt::write(self, args)
    }
}

impl core::fmt::Write for Logger {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        self.log_message_fragment(s.as_bytes());
        Ok(())
    }
}

pub static mut LOGGER: Logger = Logger::new();