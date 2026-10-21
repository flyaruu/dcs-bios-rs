use std::net::UdpSocket;

use dcs_bios_rs_generated::av8bna::{Av8bnaBuilder, JettModeValue};

fn main() {
    let mut latest_value = None;

    let mut callback = |value: JettModeValue| {
        println!("JETT: {:?}", value);
        latest_value = Some(value);
    };

    let mut builder = Av8bnaBuilder::<2, 256>::new();
    builder.register_jett_mode(&mut callback).unwrap();
    let mut av8bna = builder.build();

    // Make sure that DCS is configure to push data to the IP of my device:
    let socket = UdpSocket::bind("0.0.0.0:5010").expect("couldn't bind to address");
    let mut buf = [0; 256];
    loop {
        let (number_of_bytes, _src_addr) = socket.recv_from(&mut buf).expect("Didn't receive data");
        let filled_buf = &buf[..number_of_bytes];
        av8bna.feed(filled_buf).expect("invalid DCS-BIOS data");
    }
}
