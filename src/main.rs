/*
Header
Byte 1: SOM  - Start of Message
Byte 2: ADDR - Address of PD
Byte 3: LEN_LSB - Packet Length Least Significant Byte
Byte 4: LEN_MSG - Packet Length Most Significant Byte
Byte 5: CTRL - Message Control Information
*/

enum ParserState {
  Header,
  SCB,
  Data,
  MAC,
  Validation(i32), // 1 or 2 bytes of validation (chksum vs crc)
}

// if 4th bit in CTRL set, we have SCB otherwise skip to Data

struct Parser {
  state: ParserState,
  buffer: Vec<u8>,
}

impl Parser {
  fn new() -> Parser {
    let b = vec![];
    let p = Parser {
      state: ParserState::Header,
      buffer: b,
    };
    p
  }

  fn parse_byte(&mut self, byte: u8) {
    // Push byte into buffer
    self.buffer.push(byte);

    // Depending on what state the parser is in we will take different actions
    match self.state {
      ParserState::Header => {
        if self.buffer.len() >= 5 {
          self.state = ParserState::Data;
        } else {
          println!("HELLO");
        }
      }
      _ => {
        dbg!("TEST!");
      }
    }
  }
}

fn main() {
  println!("osdp-rs");

  let mut parser = Parser::new(); // instantiate default parser state

  //                            SOM   ADDR  LEN   LEN   CTRL
  let raw_bytes: Vec<u8> = vec![0x53, 0x80, 0x14, 0x00, 0x04];

  for byte in raw_bytes {
    parser.parse_byte(byte);
  }
}
