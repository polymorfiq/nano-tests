use arduino_nano33iot as bsp;
use bsp::hal;

use hal::usb::UsbBus;
use hal::pac::interrupt;

use usb_device::prelude::*;
use usb_device::bus::UsbBusAllocator;
use usbd_serial::SerialPort;

macro_rules! init_nano_33_iot {
    ($core:ident, $peripherals:ident, $clocks:ident, $pins:ident) => {
        unsafe {
            use usb_device;
            use usbd_serial;
            use usb_device::prelude::*;
            use usbd_serial::{SerialPort, USB_CLASS_CDC};
            use cortex_m::peripheral::NVIC;
            use hal::pac::interrupt;

            let bus_allocator = {
                $crate::logger::usb::USB_ALLOCATOR = Some(bsp::usb_allocator(
                    $peripherals.USB,
                    &mut $clocks,
                    &mut $peripherals.PM,
                    $pins.usb_dm,
                    $pins.usb_dp,
                ));
                $crate::logger::usb::USB_ALLOCATOR.as_ref().unwrap()
            };
            
            $crate::logger::usb::USB_SERIAL = Some(SerialPort::new(&bus_allocator));
            $crate::logger::usb::USB_BUS = Some(
                UsbDeviceBuilder::new(&bus_allocator, UsbVidPid(0x2222, 0x3333))
                    .manufacturer("Fake company")
                    .product("Serial port")
                    .serial_number("TEST")
                    .device_class(USB_CLASS_CDC)
                    .build(),
            );

            $core.NVIC.set_priority(interrupt::USB, 1);
            NVIC::unmask(interrupt::USB);

            logger::Logger::new()
        }
    };
}

impl super::Logger {
    pub const fn new() -> Self {
        Self {}
    }

    pub fn log(&mut self, message: &str) {
        self.log_bytes(message.as_bytes());
    }

    pub fn log_bytes(&mut self, message: &[u8]) {
        // Turn off interrupts so we don't fight with the interrupt
        cortex_m::interrupt::free(|_| unsafe {
            USB_BUS.as_mut().map(|_| {
                USB_SERIAL.as_mut().map(|serial| {
                    // Skip errors so we can continue the program
                    let _ = serial.write(message);
                });
            })
        });
    }
}

pub static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
pub static mut USB_BUS: Option<UsbDevice<UsbBus>> = None;
pub static mut USB_SERIAL: Option<SerialPort<UsbBus>> = None;

fn poll_usb() {
    unsafe {
        USB_BUS.as_mut().map(|usb_dev| {
            USB_SERIAL.as_mut().map(|serial| {
                usb_dev.poll(&mut [serial]);

                // Make the other side happy
                let mut buf = [0u8; 16];
                let _ = serial.read(&mut buf);
            });
        });
    };
}

#[interrupt]
fn USB() { poll_usb(); }