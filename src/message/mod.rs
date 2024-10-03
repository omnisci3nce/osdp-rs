use crate::packet::Packet;

pub mod card_data;
pub mod device_capabilities;
pub mod device_identification;
pub mod encryption;
pub mod keypad_data;
pub mod led;
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

    pub trait Command {
        fn msg_byte(&self) -> u8;
        fn serialize(&self, mut buf: &mut [u8]) -> Result<u16, SerializationError> {
            todo!()
            // let output = self.to_bytes()?;
            // buf.write_all(output.as_slice())?;
            // Ok(output.len() as u16)
        }
    }
    pub trait Reply {
        fn msg_byte(&self) -> u8;
    }
}

// impl From<std::io::Error> for SerializationError {
//     fn from(_value: std::io::Error) -> Self {
//         SerializationError
//     }
// }

// TODO: macro for cooercing to a internal error type

impl Message {
    pub fn msg_type(&self) -> u8 {
        match self {
            Message::CMD_ID(cmd) => cmd.msg_byte(),
            _ => todo!(),
        }
    }
    pub fn serialize(&self, buf: &mut [u8]) -> u16 {
        // TODO: return Result

        match self {
            Message::CMD_POLL(_) => {
                buf[0] = 0x00;
                1
            }
            Message::CMD_ID(p) => p.serialize(buf).unwrap(),
            _ => todo!(),
        }
    }
}

// TODO: completely overhaul this
// pub fn from_packet(p: Packet) -> Result<Message, Box<dyn Error>> {
//     let data_slice = &p.data[5..(p.data.len() - p.header.validation_len() as usize)];
//     match p.header.msg_type {
//         0x45 => Ok(Message::REPLY_PDID(
//             DeviceIdentification::try_from(data_slice).unwrap(),
//         )),
//         0x53 => Ok(Message::REPLY_KEYPAD(
//             KeypadData::try_from(data_slice).unwrap(),
//         )),
//         _ => Err("Unknown or unimplemented msg type")?,
//     }
// }
