
use std::env;
use std::io;
use std::time::Duration;

// use std::io::prelude::*;
// use serial::prelude::*;

fn main() {
    println!("Hello, world!");

    let ports = serialport::available_ports().expect("no ports found");
    for p in ports {
        println!("{}", p.port_name);
    }

    let mut port = serialport::new("/dev/ttyUSB0", 921_600)
        .timeout(Duration::from_millis(10)).open().expect("Failed to open port");
                 
    loop { 
        let mut serial_buf: Vec<u8> = vec![0; 32];
        match port.read(serial_buf.as_mut_slice()) {
            Ok(r) => {
                println!("Red :{}", r);
                let mut count = 0;
                for i in serial_buf.iter() {
                    if *i == 3 {
                        println!("Found a 3");
                        count += 1;
                        if count == 3 {
                            println!("Ready to start doing stuff.")
                        }
                    }
                    else {
                        count = 0;
                    }
                }
            },
            Err(f) => {
            }
        }
    }
    // println!("{}", String::from_utf8(serial_buf).unwrap());

    // for arg in env::args_os().skip(1) {
    //     let mut port = serial::open(&arg).unwrap();
    //     interact(&mut port).unwrap();
    // }
}

// fn interact<T: SerialPort>(port: &mut T) -> io::Result<()> {
//     port.reconfigure(&|settings| {
//         settings.set_baud_rate(serial::BaudOther(921_600));
//         settings.set_char_size(serial::Bits8);
//         settings.set_parity(serial::ParityNone);
//         settings.set_stop_bits(serial::Stop1);
//         settings.set_flow_control(serial::FlowNone);
//         Ok(())
//     });

//     port.set_timeout(Duration::from_millis(1000));

//     let mut buf: Vec<u8> = (0..255).collect();

//     port.read(&mut buf[..]);
//     println!("{}", String::from_utf8(buf).unwrap());
//     Ok(())
// }

