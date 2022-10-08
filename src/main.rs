#![no_std]
#![no_main]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![cfg_attr(feature = "nice-panic", feature(panic_info_message))]

use core::panic::PanicInfo;
use cortex_m::asm::delay as cycle_delay;

use arduino_nano33iot as bsp;
use bsp::hal;
use hal::prelude::*;
use hal::clock::GenericClockController;
use hal::pac::{CorePeripherals, Peripherals};

#[macro_use]
mod logger;

static mut FAKE: Option<bool> = None;
static mut LOGGER: Option<logger::Logger> = None;

#[bsp::entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let pins = bsp::Pins::new(peripherals.PORT);
    let mut led: bsp::Led = pins.led_sck.into();

    let mut logger = init_nano_33_iot!(core, peripherals, clocks, pins);
    unsafe { LOGGER = Some(logger); }

    // Flash the LED in a spin loop to demonstrate that USB is
    // entirely interrupt driven.
    loop {
        cycle_delay(5 * 1024 * 1024);
        led.toggle().unwrap();

        logger.println("This is a log line");
        logger.fmtln(format_args!("This is a formatted line! {:?}", "abc")).unwrap();
        unsafe { FAKE.expect("THIS IS A PANIC!!!") };
    }
}

#[cfg(feature = "nice-panic")]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe {
        match LOGGER.as_mut() {
            Some(logger) => {
                if let Some(s) = info.payload().downcast_ref::<&str>() {
                    logger.print("PANIC: ");
                    logger.println(s);
                } else if let Some(msg) = info.message() {
                    logger.print("PANIC: ");
                    logger.fmtln(*msg).unwrap();
                } else {
                    logger.println("Panic occurred!");
                }
            },

            None => ()
        }
    }

    loop {}
}

#[cfg(not(feature = "nice-panic"))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe {
        match LOGGER.as_mut() {
            Some(logger) => {
                if let Some(s) = info.payload().downcast_ref::<&str>() {
                    logger.print("PANIC: ");
                    logger.print(s);
                } else {
                    logger.print("Panic occurred!");
                }
            },

            None => ()
        }
    }

    loop {}
}