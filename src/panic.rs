use crate::logger::LOGGER;

#[cfg(feature = "nice-panic")]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    unsafe {
        if let Some(s) = info.payload().downcast_ref::<&str>() {
            LOGGER.print("PANIC: ");
            LOGGER.println(s);
        } else if let Some(msg) = info.message() {
            LOGGER.print("PANIC: ");
            LOGGER.fmtln(*msg).unwrap();
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
            LOGGER.print("PANIC: ");
            LOGGER.print(s);
        } else {
            LOGGER.print("Panic occurred!");
        }
    }

    loop {}
}