use std::fmt;

// A message must implement the following functions
pub trait Message {
  /** Returns Some for expected data in bytes and None when the data length is variable */
  fn data_length(&self) -> Option<i32>;
}

pub trait DataBlock {
  fn serialise(&self) -> Vec<u8>;
  fn deserialise(bytes: &[u8]) -> Self;
}

pub trait Command {}
pub trait Reply {}

/*
  Commands
  ========
*/

pub struct Poll {} // 0x60
pub struct ReaderLED {} // 0x69

impl Message for Poll {
  fn data_length(&self) -> Option<i32> {
    Some(0)
  }
}

pub struct DeviceIDReportRequest {}
impl DeviceIDReportRequest {
  fn serialise(&self) -> Vec<u8> {
    Vec::from([0x00])
  }
}

pub struct DeviceCapabilitiesRequest {}
impl DeviceCapabilitiesRequest {
  fn serialise(&self) -> Vec<u8> {
    Vec::from([0x00])
  }
}

/*
  Replies
  =======
*/

pub struct Ack {} // 0x40
pub struct Nack {} // 0x41

impl Message for Ack {
  fn data_length(&self) -> Option<i32> {
    Some(0)
  }
}

#[derive(Debug)]
pub struct DeviceIDReport {
  vendor_code: u32,
  model_no: u8,
  model_version: u8,
  serial_number: u32,
  firmware_version: String,
}

struct Capability {
  function_code: u8,
  compliance: u8,
  number_of: u8,
}
pub struct DeviceCapabilitiesReport {
  capabilities: Vec<Capability>,
}

impl fmt::Display for DeviceIDReport {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "Vendor Code: {}\nModel Number: {}\nModel Version: {}\nSerial Number: {}\nFirmware: {}\n",
      self.vendor_code,
      self.model_no,
      self.model_version,
      self.serial_number,
      self.firmware_version
    )
  }
}

impl DataBlock for DeviceIDReport {
  fn deserialise(bytes: &[u8]) -> Self {
    if bytes.len() != 12 {
      panic!("Expected data block length of 12");
    }
    let vendor_code = bytes[0] as u32 | ((bytes[1] as u32) << 8) | ((bytes[2] as u32) << 16);
    let serial = bytes[5] as u32
      | ((bytes[6] as u32) << 8)
      | ((bytes[7] as u32) << 16)
      | ((bytes[8] as u32) << 24);
    let firmware = format!("{}.{}.{}", bytes[9], bytes[10], bytes[11]);
    DeviceIDReport {
      vendor_code: vendor_code,
      model_no: bytes[3],
      model_version: bytes[4],
      serial_number: serial,
      firmware_version: firmware,
    }
  }

  fn serialise(&self) -> Vec<u8> {
    todo!("TODO");
  }
}
