use crate::*;
use core::panic::PanicInfo;

#[panic_handler]
fn dummy_panic(info: &PanicInfo) -> ! {
    eprintln!("{info}");
    serial_println!("{info}");
    crate::hlt_loop();
}
