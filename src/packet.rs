// A message must implement the following functions
pub trait Message {
  fn data_length(&self) -> Option<i32>;
}

#[derive(Debug)]
pub struct Packet{}

pub struct ACK {}

impl Message for ACK {
  /** Returns Some for expected data in bytes and None when the data length is variable */
  fn data_length(&self) -> Option<i32> {
    Some(0)
  }
}