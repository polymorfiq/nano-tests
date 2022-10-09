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
use io::event::{EventType, EventQueue, events};
use events::LogEvent;

#[macro_use]
mod logger;
use logger::Logger;

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
    let mut led: bsp::Led = pins.led_sck.into();

    let logger = initialize_logger!(core, peripherals, clocks, pins);
    let event_queue = init_event_queue!();

    loop {
        parse_event_queue(logger, event_queue);

        cycle_delay(5 * 1024 * 1024);
        led.toggle().unwrap();
    }
}

fn parse_event_queue<'a, const SIZE: usize>(logger: &mut Logger, queue: &mut EventQueue<'a, SIZE>) {
    for event in queue {
        match event.event_type() {
            EventType::Log => {
                let event: &LogEvent = event.as_any().downcast_ref::<LogEvent>().expect("Could not parse LogEvent");
                let str_data = core::str::from_utf8(&event.message[0..event.size]).expect("Could not convert log event message to string");

                logger.fmtln(format_args!("Received LOG event of size {:?}... {:?}", event.size, str_data)).unwrap();
            }

            EventType::None => ()
        }
    }
}