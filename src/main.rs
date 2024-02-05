#![no_std]
#![no_main]
use core::panic::PanicInfo;
use rust_os::prelude::*;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    write!(VGA_WRITER.lock(), "Eef freef\nSbubby\n{}", 8.0 / 3.15).unwrap();

    loop {}
}
