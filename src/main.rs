use osdp_rs::parser::Parser;
use osdp_rs::packet::Packet;
use osdp_rs::message::{Poll, Message};

fn main() {
  println!("osdp-rs");

  let mut parser = Parser::new(); // instantiate default parser state

  //                            SOM   ADDR  LEN   LEN   CTRL c(POLL) CRC1  CRC2
  let raw_bytes: Vec<u8> = vec![0x53, 0x80, 0x08, 0x00, 0x04,  0x60, 0x00, 0x00];

  for byte in raw_bytes {
    let completed_packet: Option<Packet> = parser.parse_byte(byte);
    match completed_packet {
      Some (p) => println!("Complete packet received: {:?}", p),
      None => ()
    }
    
  }
}
