/*
  Header
  Byte 1: SOM     - Start of Message
  Byte 2: ADDR    - Address of PD
  Byte 3: LEN_LSB - Packet Length Least Significant Byte
  Byte 4: LEN_MSG - Packet Length Most Significant Byte
  Byte 5: CTRL    - Message Control Information
*/

use crate::packet::{Packet, MAX_PACKET_SIZE};

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq)]
enum ParserState {
    WaitingSOM,
    Header,
    SCB,
    Data,
    // MAC,
    Validation,
    Done,
}

// if 4th bit in CTRL set, we have SCB otherwise skip to Data

pub struct Parser {
    state: ParserState,
    buffer: [u8; MAX_PACKET_SIZE], // Idea: create a fixed size array type that has similar API to Vec?
    current_idx: usize,
    temp_packet: Packet,
    expected_data_len: u16,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            state: ParserState::WaitingSOM,
            buffer: [0; MAX_PACKET_SIZE],
            current_idx: 0,
            temp_packet: Packet::default(),
            expected_data_len: 0,
        }
    }

    fn transition(&mut self, target: ParserState) {
        println!("[PARSER] Transition to {:?} state", target);
        self.state = target;
    }

    pub fn parse_byte(&mut self, byte: u8) -> Option<Packet> {
        println!("[PARSER] Parse byte {:#02x}", byte);

        if self.state != ParserState::WaitingSOM {
            self.buffer[self.current_idx] = byte; // Push byte into buffer no matter what state we're in
        }

        // Depending on what state the parser is in we will take different actions
        match self.state {
            ParserState::WaitingSOM => {
                if byte == 0x53 {
                    println!("Found SOM");
                    self.transition(ParserState::Header)
                }
            }
            ParserState::Header => {
                if self.buffer.len() == 4 {
                    println!("[PARSER] Accumulated whole packet header");

                    self.temp_packet.header.address = self.buffer[0];
                    let len_lsb = self.buffer[1];
                    let len_msb = self.buffer[2];
                    let len = ((len_msb as u16) << 8) | len_lsb as u16;
                    println!("[PARSER] Expecting packet of length: {}", len);
                    self.temp_packet.header.length = len;
                    self.temp_packet.header.msg_ctrl_info = self.buffer[3];

                    //                           header
                    println!("Len: {:?}", len);
                    println!(
                        "Validation len: {:?}",
                        self.temp_packet.header.validation_len()
                    );
                    self.expected_data_len = len - 6 - self.temp_packet.header.validation_len();
                    println!("expected data len: {:?}", self.expected_data_len);

                    if self.temp_packet.header.has_sch() {
                        self.transition(ParserState::SCB)
                    } else {
                        println!("[PARSER] Skip SCB");
                        self.transition(ParserState::Data)
                    }
                }
            }
            ParserState::Data => {
                // In Data state we just accumulate data
                if self.buffer.len()
                    == (4 + 1 + self.expected_data_len - self.temp_packet.header.validation_len())
                        .into()
                {
                    println!("[PARSER] Accumulated all data bytes");
                    // self.temp_packet.buffer = self.buffer.clone();
                    self.temp_packet.header.msg_type = self.buffer[4];
                    self.transition(ParserState::Validation);
                }
            }
            ParserState::Validation => {
                if self.buffer.len()
                    == (4 + 1 + self.expected_data_len + self.temp_packet.header.validation_len())
                        .into()
                {
                    println!("[PARSER] Finished receiving packet");
                    self.transition(ParserState::Done);
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
                // self.temp_packet.buffer = self.buffer.clone();
                let p = Some(self.temp_packet.clone());
                // reset the parser
                self.reset_parser();
                p
            }
            _ => None,
        }
    }

    fn reset_parser(&mut self) {
        self.state = ParserState::WaitingSOM;
        self.buffer.iter_mut().for_each(|i| *i = 0); // clear buffer
        self.temp_packet = Packet::default();
        self.expected_data_len = 0;
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}
