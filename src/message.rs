// A message must implement the following functions
pub trait Message {
  fn data_length(&self) -> Option<i32>;
}

pub trait Command {}
pub trait Reply {}


/*
  Basic Commands
  ==============
*/
pub struct Poll {}
pub struct ReaderLED {}


impl Message for Poll {
  fn data_length(&self) -> Option<i32> {
    Some(0)
  }
}

impl Message for ReaderLED {
  fn data_length(&self) -> Option<i32> {
    Some(0)
  }
}




/*
  Basic Replies
  ==============
*/

pub struct ACK {}

impl Message for ACK {
  /** Returns Some for expected data in bytes and None when the data length is variable */
  fn data_length(&self) -> Option<i32> {
    Some(0)
  }
}
