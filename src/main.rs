#![no_std]
#![no_main]

// [ZION_FS] Module Registration
mod council {
    pub mod scribe;
}
mod law {
    pub mod commandments;
}

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // ALEPH: The Start of the Flame
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
