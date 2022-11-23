#![feature(asm_const)]
#![feature(naked_functions)]

#![deny(warnings)]
#![doc(test(attr(deny(warnings))))]
#![doc(test(attr(allow(dead_code))))]
#![doc(test(attr(allow(unused_variables))))]

#![no_std]

#[cfg(not(target_os="dos"))]
mod stub;

#[cfg(not(target_os="dos"))]
use stub::*;

#[cfg(target_os="dos")]
mod dos;

#[cfg(target_os="dos")]
use dos::*;

pub struct Timer(());

impl Timer {
    /// # Safety
    ///
    /// This function may not be called while another [`Timer`] instance is alive.
    /// Also, it should be guaranteed that it is executing on an effectively single-core processor.
    pub unsafe fn new(frequency: u16) -> Timer {
        init(frequency);
        Timer(())
    }

    pub fn ticks(&self) -> u64 {
        unsafe {
            ticks()
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        unsafe {
            done();
        }
    }
}
