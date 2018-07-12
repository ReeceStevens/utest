use core::intrinsics;
use core::panic::PanicInfo;

#[panic_implementation]
fn panic(info: &PanicInfo) -> ! {
    unsafe {
        #[allow(improper_ctypes)]
        extern "Rust" {
            fn __test_panic_fmt(args: ::core::fmt::Arguments,
                                file: &str,
                                line: u32);
        }

        if let Some(location) = info.location() {
            __test_panic_fmt(*info.message().unwrap(), location.file(), location.line());
        }

        intrinsics::abort()
    }
}
