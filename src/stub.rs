use core::hint::unreachable_unchecked;

#[allow(unused_variables)]
pub unsafe fn init(frequency: u16) {
    panic!("cfg(target_os=\"dos\")")
}

pub unsafe fn done() {
    unreachable_unchecked()
}

pub unsafe fn ticks() -> u64 {
    unreachable_unchecked()
}
