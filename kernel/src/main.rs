#![no_std]
#![no_main]

mod mm;
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
/*                                    INIT                                    */
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
    crate::printk!("Hello, World!");

    #[cfg(feature = "test")]
    test::main();

    use sbi::system_reset::*;
    reset(Type::Shutdown, Reason::NoReason)
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    if let Some(location) = info.location() {
        crate::printk!("kernel panicked at {}:{}", location.file(), location.line());
    } else {
        crate::printk!("kernel panicked");
    }

    use sbi::system_reset::*;
    reset(Type::Shutdown, Reason::SystemFailure)
}
