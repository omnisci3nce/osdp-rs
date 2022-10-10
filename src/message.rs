// A message must implement the following functions
pub trait Message {
  /** Returns Some for expected data in bytes and None when the data length is variable */
  fn data_length(&self) -> Option<i32>;
}

pub trait Command {}
pub trait Reply {}


/*
  Basic Commands
  ==============
*/

pub struct Poll {}          // 0x60
pub struct ReaderLED {}     // 0x69


impl Message for Poll {
  fn data_length(&self) -> Option<i32> {
    Some(0)
  }
}


/*
  Basic Replies
  ==============
*/

pub struct Ack {}           // 0x40
pub struct Nack {}          // 0x41

impl Message for Ack {
  fn data_length(&self) -> Option<i32> {
    Some(0)
  }
}
