use core::panic::PanicInfo;
use log::error;

#[panic_handler]
#[cfg(not(debug_assertions))]
/// Panic by aborting.
fn panic(panic: &PanicInfo<'_>) -> ! {
    error!("{}", panic);
    core::intrinsics::abort()
}

#[panic_handler]
#[cfg(debug_assertions)]
/// Panic by halting using an infinite loop.
/// This is easier to debug than the release panic handler.
fn panic(panic: &PanicInfo<'_>) -> ! {
    error!("{}", panic);
    loop {}
}
