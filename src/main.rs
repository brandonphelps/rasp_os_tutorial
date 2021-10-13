
#![doc(html_logo_url = "https://git.io/JeGIp")]

//! The 'kenerl' binary.

#![feature(asm)]
#![feature(global_asm)]
#![no_main]
#![no_std]

mod bsp;

mod panic_wait;