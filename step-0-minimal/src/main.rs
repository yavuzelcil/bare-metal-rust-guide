#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn Reset() -> ! {
    // Şimdilik hiç bir şey yapma, sonsuz döngü
    loop {}
}
