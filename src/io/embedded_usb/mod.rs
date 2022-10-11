use arduino_nano33iot as bsp;
use bsp::hal;
use hal::usb::UsbBus;

use usb_device::prelude::*;
use usb_device::bus::UsbBusAllocator;
use usbd_serial::{SerialPort, USB_CLASS_CDC};
use crate::io::{EventHeader, parse_event_header};

pub mod input;
pub mod output;

pub static mut ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
static mut BUS: Option<UsbDevice<UsbBus>> = None;
static mut SERIAL: Option<SerialPort<UsbBus>> = None;

pub struct EmbeddedUsb<'a> {
    bus: &'a mut UsbDevice<'static, UsbBus>,
    serial: &'a mut SerialPort<'static, UsbBus>,
}

impl<'a> EmbeddedUsb<'a> {
    pub fn maybe_new() -> Option<Self> {
        let maybe_bus = unsafe { BUS.as_mut() };
        let maybe_serial = unsafe { SERIAL.as_mut() };

        match (maybe_bus, maybe_serial) {
            (Some(bus), Some(serial)) => {
                Some(Self {bus: bus, serial: serial})
            },

            (_, _) => None
        }
    }

    pub fn parse_event_header(&mut self, header: &EventHeader) {
        parse_event_header(header, self);
    }

    pub fn allocate_device(allocator: &'static UsbBusAllocator<UsbBus>) {
        unsafe {
            SERIAL = Some(SerialPort::new(allocator));
            BUS = Some(
                UsbDeviceBuilder::new(allocator, UsbVidPid(0x2222, 0x3333))
                    .manufacturer("Fake company")
                    .product("Serial port")
                    .serial_number("-nano-device-")
                    .device_class(USB_CLASS_CDC)
                    .build(),
            );
        }
    }
}

macro_rules! init_usb_io {
    ($core:ident, $peripherals:ident, $clocks:ident, $pins:ident) => {
        if unsafe { &$crate::io::embedded_usb::ALLOCATOR }.is_none() {
            unsafe {
                use cortex_m::peripheral::NVIC;
                use hal::pac::interrupt;
                use $crate::io::embedded_usb::{ALLOCATOR, EmbeddedUsb};

                let bus_allocator = {
                    ALLOCATOR = Some(bsp::usb_allocator(
                        $peripherals.USB,
                        &mut $clocks,
                        &mut $peripherals.PM,
                        $pins.usb_dm,
                        $pins.usb_dp,
                    ));
                    ALLOCATOR.as_ref().unwrap()
                };

                EmbeddedUsb::allocate_device(bus_allocator);

                $core.NVIC.set_priority(interrupt::USB, 1);
                NVIC::unmask(interrupt::USB);
            }
        }
    };
}