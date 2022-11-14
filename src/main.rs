use osdp_rs::crc::calc_checksum;
use osdp_rs::message::from_packet;
use osdp_rs::packet::Packet;
use osdp_rs::parser::Parser;
use std::io;
use std::time::Duration;

fn main() {
  println!("osdp-rs");

  let mut parser = Parser::new(); // instantiate default parser state

  let builder = serialport::new("/dev/ttyUSB0", 9600).timeout(Duration::from_millis(20));
  println!("{:?}", &builder);
  let mut port = builder.open().expect("Failed to open port");

  // Create a packet for testing (requests device info)
  // -------------------------------------------------
  let mut packet = Vec::with_capacity(20);

  // create header       SOM   ADDR  LEN -----------|  CTRL
  let len: u16 = (5 + 1 + 1 + 1);
  let len_lsb = (len & 0xFF) as u8;
  let len_msb = ((len >> 8) & 0xFF) as u8;
  let header: [u8; 5] = [0x53, 0x00, len_lsb, len_msb, 0x00];
  for b in header {
    packet.push(b);
  }
  // command type
  packet.push(0x62);
  // data block
  let data_block = Vec::from([0x00]);
  for b in data_block {
    packet.push(b);
  }
  // validation
  let chksum = calc_checksum(&packet);
  packet.push(chksum);
  // -------------------------------------------------

  let mut read_buffer: [u8; 1] = [0];
  port.write(&packet).expect("Write failed!");
  loop {
    match port.read(&mut read_buffer) {
      Ok(bytes) => {
        let byte = read_buffer[0]; // the byte we just read in
        let maybe_completed_packet: Option<Packet> = parser.parse_byte(byte);
        match maybe_completed_packet {
          Some(p) => {
            println!("Complete packet received: {:?}", p);
            // Attempt to deserialise it
            let msg = from_packet(p);
            // TODO: Remove a lot of this match nesting
            match msg {
              Ok(msg) => match msg {
                osdp_rs::message::Message::REPLY_PDID(d) => println!("{}", d),
                osdp_rs::message::Message::REPLY_PDCAP(d) => println!("{}", d),
                _ => (),
              },
              Err(_e) => panic!("Error deserialising packet to message!"),
            };
          }
          None => (),
        }
      }
      Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
      Err(e) => eprintln!("{:?}", e),
    }
  }
}
