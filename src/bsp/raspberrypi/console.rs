

use crate::{console, synchronization, synchronization::NullLock};
use core::fmt;

pub fn console() -> &'static impl console::interface::All {
    &super::PL011_UART
}


pub unsafe fn panic_console_out() -> impl fmt::Write {
    let mut panic_gpio = device_driver::PanicGPIO::new(memory::map::mmio::GPIO_START);
    let mut panic_uart = device_driver::PanicUart::new(memory::map::mmio::PL011_UART_START);

    panic_gpio.map_pl011_uart();
    panic_uart.init();
    panic_uart    
}

// OS interface code

use synchronization::interface::Mutex;

impl console::interface::Write for QEMUOutput {
    fn write_char(&self, c: char) {
        self.write_fmt(format_args!("{}", c));
    }

    fn flush(&self) {
        // do nothing. 
    }

     fn write_fmt(&self, args: core::fmt::Arguments) -> fmt::Result {
        // fully qualified syntax
	self.inner.lock(|inner| fmt::Write::write_fmt(inner, args))
     }
}

impl console::interface::Statistics for QEMUOutput {

     fn chars_written(&self) -> usize {
       self.inner.lock(|inner| inner.chars_written)
     }
}

impl console::interface::Read for QEMUOutput {
    fn clear_rx(&self)  {
        todo!()
    }
}
