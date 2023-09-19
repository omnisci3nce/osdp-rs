use std::{error::Error, io, time::Duration};

use osdp_rs::{
    controller::{Controller, ControllerOptions},
    message::{device_identification::DeviceIDRequest, from_packet},
    packet::Packet,
    parser::Parser,
};

fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let mut parser = Parser::new();

    // parse some random bytes
    parser.parse_byte(0x53);
    parser.parse_byte(0xAA);
    parser.parse_byte(0x05);
    parser.parse_byte(0xFF);
    parser.parse_byte(0x06);
    parser.parse_byte(0x07);

    println!("osdp-rs");

    let msg = osdp_rs::message::Message::CMD_ID(DeviceIDRequest {});

    let builder = serialport::new("/dev/ttyUSB0", 9600).timeout(Duration::from_millis(20));
    println!("{:?}", &builder);
    let mut port = builder.open().expect("Failed to open port");
    let mut acu = Controller::new(&mut port, ControllerOptions::default());
    acu.enqueue_cmd(0x01, msg);

    let _ = acu.send_next().unwrap();

    let mut read_buffer: [u8; 1] = [0];
    loop {
        match acu.port.read(&mut read_buffer) {
            Ok(_bytes) => {
                let byte = read_buffer[0]; // the byte we just read in
                let maybe_completed_packet: Option<Packet> = parser.parse_byte(byte);
                if let Some(p) = maybe_completed_packet {
                    println!("Complete packet received: {:?}", p);
                    // Attempt to deserialise it
                    let msg = from_packet(p);
                    match msg {
                        Ok(msg) => match msg {
                            osdp_rs::message::Message::REPLY_PDID(d) => println!("{:#?}", d),
                            osdp_rs::message::Message::REPLY_PDCAP(d) => println!("{}", d),
                            osdp_rs::message::Message::REPLY_KEYPAD(d) => println!("{:?}", d),
                            _ => (),
                        },
                        Err(_e) => panic!("Error deserialising packet to message!"),
                    };
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
        }
    }
}
