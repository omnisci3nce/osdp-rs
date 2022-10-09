/*
  Header
  Byte 1: SOM  - Start of Message
  Byte 2: ADDR - Address of PD
  Byte 3: LEN_LSB - Packet Length Least Significant Byte
  Byte 4: LEN_MSG - Packet Length Most Significant Byte
  Byte 5: CTRL - Message Control Information
*/

#[derive(Debug)]
enum ParserState {
  Header,
  // SCB,
  Data,
  // MAC,
  // Validation
}

// if 4th bit in CTRL set, we have SCB otherwise skip to Data

pub struct Parser {
  state: ParserState,
  buffer: Vec<u8>
}

impl Parser {
  pub fn new() -> Parser {
    Parser {
      state: ParserState::Header,
      buffer: vec![],
    }
  }

  fn transition(&mut self, target: ParserState) {
    println!("[PARSER] Transition to {:?} state", target);
    self.state = target;
  }

  pub fn parse_byte(&mut self, byte: u8) {
    println!("[PARSER] Parse byte {:#02x}", byte);
    // Push byte into buffer
    self.buffer.push(byte);

    // Depending on what state the parser is in we will take different actions
    match self.state {
      ParserState::Header => {
        if self.buffer.len() >= 5 {
          self.transition(ParserState::Data)
        }
      },
      _ => {
        dbg!("TEST!");
      }
    }
  }
}
