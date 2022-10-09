use arduino_nano33iot as bsp;
use bsp::hal;

use hal::usb::UsbBus;

use usb_device::prelude::*;
use usb_device::bus::UsbBusAllocator;
use usbd_serial::SerialPort;

pub mod input;
pub mod output;

pub static mut ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
pub static mut BUS: Option<UsbDevice<UsbBus>> = None;
pub static mut SERIAL: Option<SerialPort<UsbBus>> = None;
pub static mut DEVICE: Device = Device::new();

pub struct Device {}
impl Device {
    pub const fn new() -> Self {
        Self{}
    }
}

macro_rules! init_usb_io {
    ($core:ident, $peripherals:ident, $clocks:ident, $pins:ident) => {
        if unsafe { &$crate::io::usb::ALLOCATOR }.is_none() {
            unsafe {
                use usb_device;
                use usbd_serial;
                use usb_device::prelude::*;
                use usbd_serial::{SerialPort, USB_CLASS_CDC};
                use cortex_m::peripheral::NVIC;
                use hal::pac::interrupt;

                let bus_allocator = {
                    $crate::io::usb::ALLOCATOR = Some(bsp::usb_allocator(
                        $peripherals.USB,
                        &mut $clocks,
                        &mut $peripherals.PM,
                        $pins.usb_dm,
                        $pins.usb_dp,
                    ));
                    $crate::io::usb::ALLOCATOR.as_ref().unwrap()
                };
                
                $crate::io::usb::SERIAL = Some(SerialPort::new(&bus_allocator));
                $crate::io::usb::BUS = Some(
                    UsbDeviceBuilder::new(&bus_allocator, UsbVidPid(0x2222, 0x3333))
                        .manufacturer("Fake company")
                        .product("Serial port")
                        .serial_number("TEST")
                        .device_class(USB_CLASS_CDC)
                        .build(),
                );

                $core.NVIC.set_priority(interrupt::USB, 1);
                NVIC::unmask(interrupt::USB);
            }
        }
    };
}