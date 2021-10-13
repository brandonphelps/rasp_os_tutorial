
#![doc(html_logo_url = "https://git.io/JeGIp")]

//! The 'kenerl' binary.

#![feature(global_asm)]
#![no_main]
#![no_std]

mod bsp;
mod cpu;

mod panic_wait;

/// - only a single core must be active and running this function.
unsafe fn kernel_init() -> !{
       panic!()
}