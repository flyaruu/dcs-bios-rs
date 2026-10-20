use std::net::UdpSocket;

use dcs_bios_rs_generated::av8bna::{Av8bnaBuilder, HudBright, JettMode};

fn main() {
    // let mut on_mode = |value| {
    //     println!("JETT: {}", value);
    // };
    let mut latest_value = 0;

    let mut callback = |value| {
        println!("JETT: {}", value);
        latest_value = value;
    };

    let mut callback2 = |value| {
        println!("BRIGHT: {}", value);
    };
    let mut builder = Av8bnaBuilder::<2, 256>::new();
    builder
        .register_jett_mode(JettMode::new(), &mut callback)
        .unwrap()
        .register_hud_bright(HudBright::new(), &mut callback2)
        .unwrap();
    let mut av8bna = builder.build();

    let socket = UdpSocket::bind("0.0.0.0:5010").expect("couldn't bind to address");
    let mut buf = [0; 256];
    println!("Socket bound");
    loop {
        // print!(".");
        let (number_of_bytes, _src_addr) = socket.recv_from(&mut buf).expect("Didn't receive data");
        let filled_buf = &buf[..number_of_bytes];
        // println!("\nCount: {}", number_of_bytes);
        av8bna.feed(filled_buf).expect("invalid DCS-BIOS data");
    }
    // .build();
}
