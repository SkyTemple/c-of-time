use core::panic::PanicInfo;
use log::error;

#[panic_handler]
/// Panic by logging the panic and then calling the game's
/// built-in function for hanging it.
fn panic(panic: &PanicInfo<'_>) -> ! {
    error!("{}", panic);
    unsafe { WaitForever() }
}

extern "C" {
    #[allow(clashing_extern_declarations)]
    pub(crate) fn WaitForever() -> !;
}
