use core::intrinsics;
use core::panic::PanicInfo;

#[lang = "start"]
extern "C" fn start<T>(main: fn() -> T, _argc: isize, _argv: *const *const u8) -> isize
where
    T: Termination,
{
    main();

    0
}

#[lang = "termination"]
pub trait Termination {
    fn report(self) -> i32;
}

impl Termination for () {
    fn report(self) -> i32 {
        0
    }
}


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
