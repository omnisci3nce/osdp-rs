use std::vec;

use deku::prelude::*;

use crate::{integrity::calc_checksum, message::Message};

/// spec defines this
pub const MAX_PACKET_SIZE: usize = 128;
/// max packet size minus header and checksum
pub const MAX_DATA_LEN: usize = MAX_PACKET_SIZE - 5 - 2;

/// An OSDP packet
#[derive(Debug, Clone, DekuWrite, Default)]
pub struct Packet {
    pub header: PacketHeader,
    // pub data: [u8; MAX_DATA_LEN],
    pub data: Vec<u8>,
    pub checksum: PacketValidation,
}

#[derive(Debug, Default, Clone, DekuRead, DekuWrite)]
pub struct PacketHeader {
    pub address: u8,
    pub length: u16,
    pub msg_ctrl_info: u8,
    pub msg_type: u8,
}

#[derive(Debug, Clone, Copy, DekuWrite)]
#[deku(type = "u8")]
pub enum PacketValidation {
    #[deku(id = "1")]
    Checksum(u8),
    #[deku(id = "2")]
    CRC(u16),
}

pub enum ValidationType {
    Checksum,
    CRC16,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct MsgControlByte {
    #[deku(bits = "2")]
    sqn: u8,
    #[deku(bits = "1")]
    validation_type: u8,
    #[deku(bits = "1")]
    scb_presence: u8,
    // remaining 4 bits are deprecated as part of the spec and thus unused
}

impl PacketHeader {
    pub fn has_sch(&self) -> bool {
        self.msg_ctrl_info & 0x08 != 0
    }

    pub fn validation_len(&self) -> u16 {
        match self.validation_type() {
            ValidationType::CRC16 => 2,
            ValidationType::Checksum => 1,
        }
    }

    pub fn validation_type(&self) -> ValidationType {
        let is_bit_set = self.msg_ctrl_info & 0x04 != 0;
        match is_bit_set {
            true => ValidationType::CRC16,
            false => ValidationType::Checksum,
        }
    }
}

impl Packet {
    pub fn construct_from_msg(address: u8, msg: &Message) -> Self {
        // let mut data = [0; MAX_DATA_LEN];
        let mut data: Vec<u8> = vec![];
        let len = msg.serialize(&mut data);

        let checksum = PacketValidation::Checksum(calc_checksum(&data));
        let header = PacketHeader {
            address,
            length: 5 + len + 2,
            msg_ctrl_info: 0x00, // FIXME
            msg_type: msg.msg_type(),
        };
        Packet {
            header,
            data,
            checksum,
        }
    }
}

impl Default for PacketValidation {
    fn default() -> Self {
        PacketValidation::Checksum(0)
    }
}
