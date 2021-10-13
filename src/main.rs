
#![doc(html_logo_url = "https://git.io/JeGIp")]

//! The 'kenerl' binary.

#![feature(format_args_nl)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![no_main]
#![no_std]

mod bsp;
mod console;
mod cpu;

mod panic_wait;
mod print;

/// - only a single core must be active and running this function.
unsafe fn kernel_init() -> ! {
       println!("[0] Hello from rust!");
       panic!("Stopping here.")
}