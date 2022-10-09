use osdp_rs::parser::Parser;

fn main() {
  println!("osdp-rs");

  let mut parser = Parser::new(); // instantiate default parser state

  //                            SOM   ADDR  LEN   LEN   CTRL
  let raw_bytes: Vec<u8> = vec![0x53, 0x80, 0x14, 0x00, 0x04];

  for byte in raw_bytes {
    parser.parse_byte(byte);
  }
}
