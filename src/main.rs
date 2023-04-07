use osdp_rs::device::BusDevice;
use osdp_rs::message::{from_packet, Poll};
use osdp_rs::packet::Packet;
use osdp_rs::parser::Parser;
use std::error::Error;
use std::io;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
  println!("osdp-rs");

  let mut parser = Parser::new(); // instantiate default parser state

  let builder = serialport::new("/dev/ttyUSB0", 9600).timeout(Duration::from_millis(20));
  println!("{:?}", &builder);
  let mut port = builder.open().expect("Failed to open port");

  let device = BusDevice { address: 0x00 };

  // Send a packet for testing (requests device info)
  let db = Poll {};
  device.send(&mut port, &db)?;

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
              osdp_rs::message::Message::REPLY_PDID(d) => println!("{}", d),
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
