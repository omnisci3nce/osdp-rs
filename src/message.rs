use crate::packet::Packet;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{self, Display};

// A message must implement the following functions
/*pub trait Message {
  /** Returns Some for expected data in bytes and None when the data length is variable */
  fn data_length(&self) -> Option<i32>;
}*/

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

#[derive(Debug)]
pub struct DeviceIDReport {
  vendor_code: u32,
  model_no: u8,
  model_version: u8,
  serial_number: u32,
  firmware_version: String,
}

impl fmt::Display for DeviceIDReport {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "DeviceIDReport (0x45):\n  Vendor Code: {}\n  Model Number: {}\n  Model Version: {}\n  Serial Number: {}\n  Firmware: {}\n",
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

#[derive(Debug)]
struct Capability {
  function_code: u8,
  compliance: u8,
  number_of: u8,
}
#[derive(Debug)]
pub struct DeviceCapabilitiesReport {
  capabilities: Vec<Capability>,
}

impl Display for DeviceCapabilitiesReport {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut pretty_function_code = HashMap::new();
    pretty_function_code.insert(1, String::from("Contact Status Monitoring"));
    pretty_function_code.insert(2, String::from("Output Control"));
    pretty_function_code.insert(3, String::from("Card Data Format"));
    pretty_function_code.insert(4, String::from("Reader LED Control"));
    pretty_function_code.insert(5, String::from("Reader Audible Output"));
    pretty_function_code.insert(6, String::from("Reader Text Output"));
    pretty_function_code.insert(7, String::from("Time Keeping"));
    pretty_function_code.insert(8, String::from("Check Character Support"));
    pretty_function_code.insert(9, String::from("Communication Security"));
    pretty_function_code.insert(10, String::from("Receive BufferSize"));
    pretty_function_code.insert(11, String::from("Largest Combined Message Size"));
    pretty_function_code.insert(12, String::from("Smart Card Support"));

    for ele in &self.capabilities {
      write!(
        f,
        "  Function: {}  Compliance: {}\n",
        pretty_function_code
          .get(&ele.function_code)
          .unwrap_or(&String::from(ele.function_code.to_string())),
        ele.compliance
      )?;
    }
    Ok(())
  }
}

impl DataBlock for DeviceCapabilitiesReport {
  fn deserialise(bytes: &[u8]) -> Self {
    if bytes.len() % 3 != 0 {
      panic!("data block must be multiple of 3 byte structure.");
    }
    let r = bytes.chunks(3).map(|b| Capability {
      function_code: b[0],
      compliance: b[1],
      number_of: b[2],
    });
    DeviceCapabilitiesReport {
      capabilities: r.collect(),
    }
  }

  fn serialise(&self) -> Vec<u8> {
    todo!("TODO");
  }
}

pub enum Message {
  CMD_POLL(Poll),
  CMD_ID(DeviceIDReportRequest),
  CMD_CAP(DeviceCapabilitiesRequest),

  REPLY_ACK(Ack),
  REPLY_NAK(Nack),
  REPLY_PDID(DeviceIDReport),
  REPLY_PDCAP(DeviceCapabilitiesReport),
}

pub fn from_packet(p: Packet) -> Result<Message, Box<dyn Error>> {
  match p.msg_type {
    0x45 => Ok(Message::REPLY_PDID(DeviceIDReport::deserialise(
      &p.buffer[5..(p.buffer.len() - p.validation_len() as usize)],
    ))),
    0x46 => Ok(Message::REPLY_PDCAP(DeviceCapabilitiesReport::deserialise(
      &p.buffer[5..(p.buffer.len() - p.validation_len() as usize)],
    ))),
    _ => Err("Unknown type")?,
  }
}
