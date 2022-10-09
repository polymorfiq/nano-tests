use arduino_nano33iot as bsp;
use bsp::hal;
use hal::usb::UsbBus;
use hal::pac::interrupt;
use usb_device::device::UsbDevice;
use usbd_serial::SerialPort;
use crate::io::event::event_queue::GLOBAL_QUEUE as GLOBAL_EVENT_QUEUE;
use crate::io::event::{EventHeader, EventType};
use crate::io::event::events::LogEvent;
use crate::logger::Logger;

fn poll_usb() {
    unsafe {
        super::BUS.as_mut().map(|usb_dev| {
            super::SERIAL.as_mut().map(|serial| {
                do_poll(usb_dev, serial)
            });
        });
    };
}

fn do_poll<'a>(usb_dev: &mut UsbDevice<'a, UsbBus>, serial: &mut SerialPort<'a, UsbBus>) {
    usb_dev.poll(&mut [serial]);

    let header = EventHeader::new();
    let mut buf = header.into_bytes();
    let (event_type, size) = match serial.read(&mut buf) {
        Ok(bytes_read) => {
            let header = EventHeader::from_bytes(buf);
            let mut logger = Logger::new();
            logger.fmtln(format_args!("RECEIVED MESSAGE: {:?} {:?} {:?} ({:?})", bytes_read, header.event_type_or_err(), header.size(), header.into_bytes())).unwrap();

            if header.event_type_or_err().is_ok() {
                (header.event_type(), header.size())
            } else {
                (EventType::None, 0)
            }
        }

        Err(usb_device::UsbError::WouldBlock) => {
            (EventType::None, 0)
        },

        Err(err) => {
            let mut logger = Logger::new();
            logger.fmtln(format_args!("Error reading header from USB: {:?}", err)).unwrap();
            (EventType::None, 0)
        }
    };

    match event_type {
        EventType::Log => {
            let event = LogEvent::pop();
            let size_read = event.message.len().min(size as usize);
            event.size = size_read;
            Logger::new().fmtln(format_args!("Reading {:?} bytes...", size_read)).unwrap();

            match serial.read(&mut event.message[0..size_read]) {
                Ok(_) => {
                    unsafe { GLOBAL_EVENT_QUEUE.as_mut().unwrap().push(event); }
                },

                Err(err) => {
                    let mut logger = Logger::new();
                    logger.fmtln(format_args!("Error reading event from USB: {:?}", err)).unwrap();
                }
            }
        },

        _ => ()
    }
}

#[interrupt]
fn USB() { poll_usb(); }