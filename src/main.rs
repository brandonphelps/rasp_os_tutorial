
#![doc(html_logo_url = "https://git.io/JeGIp")]

//! The 'kenerl' binary.

#![feature(format_args_nl)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(trait_alias)]
#![no_main]
#![no_std]

mod bsp;
mod console;
mod cpu;

mod panic_wait;
mod print;
mod synchronization;

/// - only a single core must be active and running this function.
unsafe fn kernel_init() -> ! {
    use console::interface::Statistics;

    println!("[0] Hello from rust!");

    println!("[1] Chars written: {}",
             bsp::console::console().chars_written());
    println!("[2] Stopping here.");
    cpu::wait_forever()
}
