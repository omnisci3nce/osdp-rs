use deku::DekuError;

use crate::packet::Packet;
use std::error::Error;

pub mod card_data;
pub mod device_capabilities;
pub mod device_identification;
pub mod keypad_data;
pub mod poll;

use self::{
    device_capabilities::{DeviceCapabilitiesReport, DeviceCapabilitiesRequest},
    device_identification::{DeviceIDRequest, DeviceIdentification},
    keypad_data::KeypadData,
    markers::Command,
    poll::Poll,
};

/// Every type of OSDP message that is currently handled
#[allow(non_camel_case_types)]
pub enum Message {
    CMD_POLL(Poll),
    CMD_ID(DeviceIDRequest),
    CMD_CAP(DeviceCapabilitiesRequest),
    REPLY_PDID(DeviceIdentification),
    REPLY_PDCAP(DeviceCapabilitiesReport),
    REPLY_KEYPAD(KeypadData),
}

/// HACK: this will be changed its just to test out the [`Command`] default serialize implementation
#[derive(Debug)]
pub struct SerializationError;

// marker traits
pub(crate) mod markers {
    use super::SerializationError;
    use deku::DekuContainerWrite;
    use std::io::Write;

    pub trait Command: DekuContainerWrite {
        fn serialize(&self, mut buf: &mut [u8]) -> Result<(), SerializationError> {
            let output = self.to_bytes()?;
            Ok(buf.write_all(output.as_slice())?)
        }
    }
    pub trait Reply {}
}

impl From<DekuError> for SerializationError {
    fn from(_value: DekuError) -> Self {
        SerializationError
    }
}
impl From<std::io::Error> for SerializationError {
    fn from(_value: std::io::Error) -> Self {
        SerializationError
    }
}

// TODO: macro for cooercing to a internal error type

impl Message {
    pub fn serialize(&self, buf: &mut [u8]) {
        // TODO: return Result

        match self {
            Message::CMD_POLL(p) => p.serialize(buf).unwrap(),
            _ => unimplemented!()
            // Message::CMD_ID(_) => todo!(),
            // Message::CMD_CAP(_) => todo!(),
            // Message::REPLY_PDID(_) => todo!(),
            // Message::REPLY_PDCAP(_) => todo!(),
            // Message::REPLY_KEYPAD(_) => todo!(),
        }
    }
}

// TODO: completely overhaul this
pub fn from_packet(p: Packet) -> Result<Message, Box<dyn Error>> {
    let data_slice = &p.data[5..(p.data.len() - p.header.validation_len() as usize)];
    match p.header.msg_type {
        0x45 => Ok(Message::REPLY_PDID(
            DeviceIdentification::try_from(data_slice).unwrap(),
        )),
        // 0x46 => Ok(Message::REPLY_PDCAP(DeviceCapabilitiesReport::deserialise(
        //   data_slice,
        // ))),
        0x53 => Ok(Message::REPLY_KEYPAD(
            KeypadData::try_from(data_slice).unwrap(),
        )),
        _ => Err("Unknown or unimplemented msg type")?,
    }
}