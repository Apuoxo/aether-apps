#![no_std]
#![no_main]

// Native executable entry point will be connected once the real Aether
// userspace ABI is implemented and runtime-tested. Do not guess syscalls.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop { unsafe { core::arch::asm!("hlt"); } }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop { unsafe { core::arch::asm!("hlt"); } }
}
