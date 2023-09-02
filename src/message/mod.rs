pub mod poll;
pub mod device_report;
pub mod keypad_data;
pub mod card_data;

use std::error::Error;

use crate::packet::Packet;

use self::{poll::Poll, device_report::DeviceIdentification, keypad_data::KeypadData};

/// Every type of OSDP message that is currently handled
#[allow(non_camel_case_types)]
pub enum Message {
  CMD_POLL(Poll),
//   CMD_ID(DeviceIDReportRequest),
//   CMD_CAP(DeviceCapabilitiesRequest),
  REPLY_PDID(DeviceIdentification),
//   REPLY_PDCAP(DeviceCapabilitiesReport),
  REPLY_KEYPAD(KeypadData),
}

// marker traits
pub(crate) mod markers {
    pub trait Command {}
    pub trait Reply {}
}

pub fn from_packet(p: Packet) -> Result<Message, Box<dyn Error>> {
  let data_slice = &p.buffer[5..(p.buffer.len() - p.validation_len() as usize)];
  match p.msg_type {
    0x45 => Ok(Message::REPLY_PDID(DeviceIdentification::try_from(data_slice).unwrap())),
    // 0x46 => Ok(Message::REPLY_PDCAP(DeviceCapabilitiesReport::deserialise(
    //   data_slice,
    // ))),
    0x53 => Ok(Message::REPLY_KEYPAD(KeypadData::try_from(
      data_slice,
    ).unwrap())),
    _ => Err("Unknown or unimplemented msg type")?,
  }
}