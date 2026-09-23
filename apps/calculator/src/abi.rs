//! Experimental Aether userspace ABI. Numbers mirror the kernel syscall table.
pub const SYS_POLL_KEY: u64 = 10;
pub const SYS_DRAW_TEXT: u64 = 11;
pub const SYS_FILL_RECT: u64 = 12;
pub const SYS_YIELD: u64 = 13;
pub const SYS_EXIT: u64 = 60;

#[repr(C)]
pub struct KeyEvent {
    pub key: u8,
    pub pressed: u8,
    pub hid_code: u8,
    pub modifiers: u8,
}

#[inline(always)]
pub fn poll_key(ev: &mut KeyEvent) -> u64 {
    let ret: u64;
    unsafe { core::arch::asm!("int 0x80", in("rax") SYS_POLL_KEY, in("rdi") ev as *mut _ as u64, lateout("rax") ret, options(nostack)); }
    ret
}

#[inline(always)]
pub fn draw_text(x: usize, y: usize, text: *const u8, color: u32) -> u64 {
    let ret: u64;
    unsafe { core::arch::asm!("int 0x80", in("rax") SYS_DRAW_TEXT, in("rdi") x as u64, in("rsi") y as u64, in("rdx") text as u64, in("r10") color as u64, lateout("rax") ret, options(nostack)); }
    ret
}

#[inline(always)]
pub fn fill_rect(x: usize, y: usize, w: usize, h: usize, color: u32) -> u64 {
    let ret: u64;
    unsafe { core::arch::asm!("int 0x80", in("rax") SYS_FILL_RECT, in("rdi") x as u64, in("rsi") y as u64, in("rdx") w as u64, in("r10") h as u64, in("r8") color as u64, lateout("rax") ret, options(nostack)); }
    ret
}

#[inline(always)]
pub fn yield_now() -> u64 {
    let ret: u64;
    unsafe { core::arch::asm!("int 0x80", in("rax") SYS_YIELD, lateout("rax") ret, options(nostack)); }
    ret
}

pub fn exit(_code: u64) -> ! {
    unsafe { core::arch::asm!("int 0x80", in("rax") SYS_EXIT, options(noreturn)); }
}
