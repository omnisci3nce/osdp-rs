use crate::packet::Packet;
use std::{error::Error, io::Write};

pub mod card_data;
pub mod device_capabilities;
pub mod device_identification;
pub mod keypad_data;
pub mod poll;

use self::{
    device_capabilities::{DeviceCapabilitiesReport, DeviceCapabilitiesRequest},
    device_identification::{DeviceIDRequest, DeviceIdentification},
    keypad_data::KeypadData,
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

// marker traits
pub(crate) mod markers {
    pub trait Command {}
    pub trait Reply {}
}

impl Message {
    pub fn serialize(&self, mut buf: &mut [u8]) {
        // TODO: return Result

        match self {
            Message::CMD_POLL(p) => {
                let output: Vec<u8> = (*p).try_into().unwrap();
                buf.write_all(output.as_slice()).unwrap()
            },
            _ => unimplemented!()
            // Message::CMD_ID(_) => todo!(),
            // Message::CMD_CAP(_) => todo!(),
            // Message::REPLY_PDID(_) => todo!(),
            // Message::REPLY_PDCAP(_) => todo!(),
            // Message::REPLY_KEYPAD(_) => todo!(),
        }
    }
}

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
