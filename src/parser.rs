/*
  Header
  Byte 1: SOM     - Start of Message
  Byte 2: ADDR    - Address of PD
  Byte 3: LEN_LSB - Packet Length Least Significant Byte
  Byte 4: LEN_MSG - Packet Length Most Significant Byte
  Byte 5: CTRL    - Message Control Information
*/

use crate::packet::Packet;

#[derive(Debug)]
enum ParserState {
  Header,
  SCB,
  Data,
  // MAC,
  // Validation,
  Done
}

// if 4th bit in CTRL set, we have SCB otherwise skip to Data

pub struct Parser {
  state: ParserState,
  buffer: Vec<u8>,
  msg_type: Option<u8>
}

impl Parser {
  pub fn new() -> Parser {
    Parser {
      state: ParserState::Header,
      buffer: vec![],
      msg_type: None
    }
  }

  fn transition(&mut self, target: ParserState) {
    println!("[PARSER] Transition to {:?} state", target);
    self.state = target;
  }

  pub fn parse_byte(&mut self, byte: u8) -> Option<Packet> {
    println!("[PARSER] Parse byte {:#02x}", byte);
    self.buffer.push(byte); // Push byte into buffer no matter what state we're in

    // Depending on what state the parser is in we will take different actions
    match self.state {
      ParserState::Header => {
        if self.buffer.len() >= 5 {
          let scb_bit_set = self.buffer[4] & 0x08 != 0;
          if scb_bit_set {
            self.transition(ParserState::SCB)
          } else {
            self.transition(ParserState::Data)
          }
        }
      },
      ParserState::Data => {
        // In Data state we just accumulate data
        match self.msg_type {
          None => {
            self.msg_type = Some(byte);
          },
          Some(_) => {
            self.transition(ParserState::Done)
          }
        }
        
      }
      _ => {
        dbg!("TEST!");
      }
    };

    // If we've parsed a full message then we return it
    match self.state {
      ParserState::Done => {
        // save the packet
        let p = Some(Packet{});
        // reset the parser
        self.reset_parser();
        p
      },
      _ => None
    }
  }

  fn reset_parser(&mut self) {
    self.state = ParserState::Header;
    self.msg_type = None;
    self.buffer.clear();
  }
}

/*
  platform-agnostic
  -----------------
  parser
  packet

  platform-specific
  -----------------
  tokio ? on PC
  busy-loop?
*/

/*
  Overall control flow - pseudocode

  populate fixed sized header
  
  if SCB bit then we populate scb other wise skip to data
  
  data is either fixed-length or variable
  if fixed length we just look for X number of bytes
  else if variable we continue reading and extending expected data length
  based on data in message

  validation
  calc checksum or crc
*/