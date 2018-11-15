#![no_std]

#[macro_use]
extern crate cortex_m_semihosting;

use cortex_m_semihosting::debug;

#[no_mangle]
pub fn __test_start(ntests: usize) {
    hprintln!("running {} tests", ntests);
}

#[no_mangle]
pub fn __test_ignored(name: &str) {
    hprintln!("test {} ... ignored", name);
}

#[no_mangle]
pub fn __test_before_run(name: &str) {
    hprint!("test {} ... ", name);
}

#[no_mangle]
pub fn __test_panic_fmt(args: ::core::fmt::Arguments,
                        file: &'static str,
                        line: u32) {
    hprint!("\npanicked at '");
    cortex_m_semihosting::export::hstdout_fmt(args).unwrap();
    hprintln!("', {}:{}", file, line);
}

#[no_mangle]
pub fn __test_failed(_name: &str) {
    hprintln!("FAILED");
}

#[no_mangle]
pub fn __test_success(_name: &str) {
    hprintln!("OK");
}

#[no_mangle]
pub fn __test_summary(passed: usize, failed: usize, ignored: usize) {
    hprintln!("\ntest result: {}. {} passed; {} failed; {} ignored",
               if failed == 0 { "OK" } else { "FAILED" },
               passed,
               failed,
               ignored);

    debug::exit(debug::EXIT_SUCCESS);
}
