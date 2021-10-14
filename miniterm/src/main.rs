
use std::env;
use std::io;
use std::time::Duration;

use std::io::prelude::*;
use serial::prelude::*;

fn main() {

    println!("Hello, world!");

    for arg in env::args_os().skip(1) {
        let mut port = serial::open(&arg).unwrap();
        interact(&mut port).unwrap();
    }
}

fn interact<T: SerialPort>(port: &mut T) -> io::Result<()> {
    port.reconfigure(&|settings| {
        settings.set_baud_rate(serial::BaudOther(921_600));
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    });

    port.set_timeout(Duration::from_millis(1000));

    let mut buf: Vec<u8> = (0..255).collect();

    port.read(&mut buf[..]);
    println!("{}", String::from_utf8(buf).unwrap());
    Ok(())
}

