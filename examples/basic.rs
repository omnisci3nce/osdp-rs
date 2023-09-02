use std::{error::Error, io, time::Duration};

use deku::DekuContainerWrite;
use osdp_rs::{
    controller::Controller,
    device::BusDevice,
    message::{device_identification::DeviceIDRequest, from_packet},
    packet::{Packet, ValidationType},
    parser::Parser,
};

fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let acu = Controller::new();

    let known_device_1 = BusDevice { address: 0x01 };
    acu.register_pd(known_device_1).unwrap();

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
    let packet = Packet::construct_from_msg(0x00, ValidationType::Checksum, &msg);
    let buf: Vec<u8> = packet.to_bytes()?;
    dbg!(&buf);

    let builder = serialport::new("/dev/ttyUSB0", 9600).timeout(Duration::from_millis(20));
    println!("{:?}", &builder);
    let mut port = builder.open().expect("Failed to open port");

    let _ = port.write(&buf)?;

    let mut read_buffer: [u8; 1] = [0];
    loop {
        match port.read(&mut read_buffer) {
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
                            // osdp_rs::message::Message::REPLY_PDCAP(d) => println!("{}", d),
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
