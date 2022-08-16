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
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::sbi::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::sbi::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
