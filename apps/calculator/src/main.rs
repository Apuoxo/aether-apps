#![no_std]
#![no_main]

#[path = "lib.rs"]
mod calculator;
mod abi;
mod app;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    app::run()
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    abi::exit(1)
}
