#![no_std]
#![no_main]

mod sbi;

#[cfg(feature = "test")]
#[path = "../test/mod.rs"]
mod test;

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

    #[cfg(feature = "test")]
    test::run_test();

    use sbi::system_reset::*;
    reset(Type::Shutdown, Reason::NoReason)
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!("kernel panicked at {}:{}", location.file(), location.line());
    } else {
        println!("kernel panicked");
    }

    use sbi::system_reset::*;
    reset(Type::Shutdown, Reason::SystemFailure)
}
