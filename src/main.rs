#![no_std]
#![no_main]
#![cfg_attr(feature = "nice-panic", feature(panic_info_message))]

use cortex_m::asm::delay as cycle_delay;

use arduino_nano33iot as bsp;
use bsp::hal;
use hal::prelude::*;
use hal::clock::GenericClockController;
use hal::pac::{CorePeripherals, Peripherals};

#[macro_use]
mod io;

#[macro_use]
mod logger;

mod state;
use state::{State, LedStates};

mod panic;

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
    initialize_logger!(core, peripherals, clocks, pins);

    let mut led: bsp::Led = pins.led_sck.into();
    let state = State::global();
    loop {
        match state.led_state {
            LedStates::LedOn => led.set_high().unwrap(),
            _ => led.set_low().unwrap()
        }

        cycle_delay(5 * 1024 * 1024);
    }
}