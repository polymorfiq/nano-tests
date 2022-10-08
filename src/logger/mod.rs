#[derive(Clone, Copy)]
pub struct Logger {}

#[cfg(not(feature = "usb-logger"))]
#[macro_use]
mod noop;

#[cfg(feature = "usb-logger")]
#[macro_use]
pub mod usb;

impl Logger {
    pub fn print(&mut self, message: &str) {
        self.log_bytes(message.as_bytes());
    }

    pub fn println(&mut self, message: &str) {
        self.log_bytes(message.as_bytes());
        self.log_bytes("\n".as_bytes());
    }

    pub fn fmt(&mut self, args: core::fmt::Arguments) -> core::fmt::Result {
        core::fmt::write(self, args)
    }

    pub fn fmtln(&mut self, args: core::fmt::Arguments) -> core::fmt::Result {
        let resp = core::fmt::write(self, args);
        if resp.is_ok() { self.log_bytes("\n".as_bytes()); }
        resp
    }
}

impl core::fmt::Write for Logger {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        self.print(s);
        Ok(())
    }
}