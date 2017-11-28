#![feature(lang_items)]
#![no_std]

#[no_mangle]
pub fn fib(x: i32) -> i32 {
    if x <= 2 {
      return 1;
    }

    return fib(x - 1) + fib(x - 2);
}


// needed for no_std
#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments,
                               _file: &'static str,
                               _line: u32,
                               _column: u32) -> ! {
                                   loop {}
}
