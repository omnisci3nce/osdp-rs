use osdp_rs::crc::calc_checksum;
use osdp_rs::device::BusDevice;
use osdp_rs::message::{ DeviceIDReportRequest, DataBlock, from_packet};
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

  let device = BusDevice {
    address: 0x00,
  };

  // Send a packet for testing (requests device info)
  let db = DeviceIDReportRequest{};
  device.send(&mut port, &db);

  let mut read_buffer: [u8; 1] = [0];
  // port.write(&packet).expect("Write failed!");
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
                osdp_rs::message::MsgReply::ReplyPdId(d) => println!("{}", d),
 //               osdp_rs::message::Message::REPLY_PDCAP(d) => println!("{}", d),
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
