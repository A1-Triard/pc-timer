#![feature(extern_types)]

#![deny(warnings)]

#![windows_subsystem="console"]
#![no_std]
#![no_main]

extern crate rlibc_ext;

mod no_std {
    #[panic_handler]
    fn panic_handler(info: &core::panic::PanicInfo) -> ! { panic_no_std::panic(info, b'P') }
}

extern {
    type PEB;
}

use dos_cp::println;
use pc_timer::Timer;

#[allow(non_snake_case)]
#[no_mangle]
extern "stdcall" fn mainCRTStartup(_: *const PEB) -> u64 {
    let timer = unsafe { Timer::new(125) };
    for _ in 0 .. 1000 {
        println!("{}", timer.ticks());
    }
    0
}
