use core::fmt::Write;

struct Stdout;
impl Write for Stdout {
    fn write_str(&mut self, string: &str) -> core::fmt::Result {
        for char in string.chars() {
            super::legacy::console_putchar(char as usize);
        }
        Ok(())
    }
}

#[allow(unused)]
pub fn print(args: core::fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! printk {
    ($fmt: literal $(, $arg: expr)*) => {
        $crate::sbi::console::print(format_args!(concat!("[KERNEL]: ", $fmt, "\n") $(, $arg)*));
    }
}
