
#![doc(html_logo_url = "https://git.io/JeGIp")]

//! The 'kenerl' binary.

#![feature(format_args_nl)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(trait_alias)]
#![no_main]
#![no_std]

mod bsp;
mod console;
mod cpu;
mod driver;

mod panic_wait;
mod print;
mod synchronization;



/// - only a single core must be active and running this function.
unsafe fn kernel_init() -> ! {
    use driver::interface::DriverManager;

    //println!("[0] Hello from rust!");
    for i in bsp::driver::driver_manager().all_device_drivers().iter() {
        if let Err(x) = i.init() {
            panic!("Error loading driver: {}: {}", i.compatible(), x);
        }
    }

    bsp::driver::driver_manager().post_device_driver_init();
    // println! is uable from her eon. 
    kernel_main()

    // println!("[1] Chars written: {}",
    //          bsp::console::console().chars_written());
    // println!("[2] Stopping here.");
    // cpu::wait_forever()
}


fn kernel_main() -> ! {
    use bsp::console::console;

    use console::interface::All;
    use driver::interface::DriverManager;

    println!(
        "[0] {} version {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"));

    println!("[1] Booting on: {}", bsp::board_name());

    unsafe {
        let base = 0x3F00_0000;
        let t: *const i32 = (base + 0x0020_1000) as *const i32;
        while core::ptr::read_volatile(t) & 0x10 != 0 {
            cpu::nop();
        }
        let f = core::ptr::read_volatile(t);
        println!("{}", f);
        
    }

    loop {
    }
}
