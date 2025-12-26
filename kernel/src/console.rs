use crate::sbi::console_putchar;
use core::fmt::{self, Write};

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

// 宏定义：print!
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::console::print(format_args!($($arg)*));
    });
}

// 宏定义：println!
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

// 新增：debug! 宏
// 输出格式：[DEBUG] file:line message
// 使用绿色 (\x1b[32m) 高亮 [DEBUG] 前缀
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => ({
        $crate::print!("\x1b[32m[DEBUG] {}:{}\x1b[0m {}\n", file!(), line!(), format_args!($($arg)*));
    });
}
