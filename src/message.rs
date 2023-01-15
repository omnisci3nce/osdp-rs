use crate::packet::Packet;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{self, Display};

pub trait DataBlock {
  fn serialise(&self) -> Vec<u8>;
  fn deserialise(bytes: &[u8]) -> Self where Self: Sized;
}

/* Commands
   ======== */

pub struct Poll {} // 0x60
pub struct ReaderLED {} // 0x69

pub struct DeviceIDReportRequest {}
impl DataBlock for DeviceIDReportRequest {
  fn deserialise(_bytes: &[u8]) -> Self {
    DeviceIDReportRequest {}
  }

  fn serialise(&self) -> Vec<u8> {
    Vec::from([0x00])
  }
}

pub struct DeviceCapabilitiesRequest {}
impl DataBlock for DeviceCapabilitiesRequest {
  fn deserialise(_bytes: &[u8]) -> Self {
    DeviceCapabilitiesRequest {}
  }

  fn serialise(&self) -> Vec<u8> {
    Vec::from([0x00])
  }
}

/* Replies
   ======= */

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

const fn capability_code_to_str(code: u8) -> Option<&'static str> {
  match code {
    1 => Some("Contact Status Monitoring"),
    2 => Some("Output Control"),
    3 => Some("Card Data Format"),
    4 => Some("Reader LED Control"),
    5 => Some("Reader Audible Output"),
    6 => Some("Reader Text Output"),
    7 => Some("Time Keeping"),
    8 => Some("Check Character Support"),
    9 => Some("Communication Security"),
    10 => Some("Receive BufferSize"),
    11 => Some("Largest Combined Message Size"),
    12 => Some("Smart Card Support"),
    _ => None,
  }
}

impl Display for DeviceCapabilitiesReport {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for ele in &self.capabilities {
      write!(
        f,
        "  Function: {}  Compliance: {}\n",
        capability_code_to_str(ele.function_code)
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

pub enum MsgType {
  CMD_POLL,
  CMD_ID,
  CMD_CAP,

  REPLY_ACK,
  REPLY_NAK,
  REPLY_PDID,
  REPLY_PDCAP,
}
/*
pub struct Message {
  msg_type: MsgType,
  data: &dyn DataBlock
}

impl Message {
  pub fn serialise(&self) -> Vec<u8> {
    self.data.serialise()
  }

  pub fn msg_byte(&self) -> u8 {
    match self.msg_type {
      _ => panic!("arrr")
    }
  }
}
*/

pub enum MsgReply {
  ReplyPdId(DeviceIDReport),
}

pub fn from_packet(p: Packet) -> Result<MsgReply, Box<dyn Error>> {
  match p.msg_type {
    0x45 => Ok(MsgReply::ReplyPdId(DeviceIDReport::deserialise(
      &p.buffer[5..(p.buffer.len() - p.validation_len() as usize)],
    ))),
//    0x46 => Ok(Box::new(DeviceCapabilitiesReport::deserialise(
//     &p.buffer[5..(p.buffer.len() - p.validation_len() as usize)],
//  ))),
    _ => Err("Unknown type")?,
  }
}
