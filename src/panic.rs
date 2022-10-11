use crate::logger::LOGGER;

#[cfg(feature = "nice-panic")]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    unsafe {
        if let Some(s) = info.payload().downcast_ref::<&str>() {
            LOGGER.printf(format_args!("PANIC: {}\n", *s)).unwrap();
        } else if let Some(msg) = info.message() {
            LOGGER.printf(format_args!("PANIC: {}\n", *msg)).unwrap();
        } else {
            LOGGER.println("Panic occurred!");
        }
    }

    loop {}
}

#[cfg(not(feature = "nice-panic"))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    unsafe {
        if let Some(s) = info.payload().downcast_ref::<&str>() {
            LOGGER.printf(format_args!("PANIC: {}\n", *s)).unwrap();
        } else {
            LOGGER.print("Panic occurred!");
        }
    }

    loop {}
}