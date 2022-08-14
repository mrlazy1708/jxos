#![no_std]
#![no_main]

mod sbi;

core::arch::global_asm! {"
    .section .bss.stack
    .globl stack
stack:
    .space 4096 * 16
"}

/* -------------------------------------------------------------------------- */
/*                                    MAIN                                    */
/* -------------------------------------------------------------------------- */

core::arch::global_asm! {"
    .section .text.entry
    .globl _start
_start:
    la sp, stack
    call main
"}

#[no_mangle]
pub extern "C" fn main() -> ! {
    println!("Hello, World!");
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
