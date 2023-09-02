use deku::prelude::*;

const MAX_PACKET_SIZE: usize = 128; // spec defines this
const MAX_DATA_LEN: usize = MAX_PACKET_SIZE - 5 - 2; // max packet size minus header and checksum

/* A packet is constructed like [PacketHeader - PacketDataBlock - PacketValidation] */
pub struct Packet {
    pub header: PacketHeader,
    pub data: [u8; MAX_DATA_LEN],
    pub checksum: PacketValidation,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct PacketHeader {
    pub address: u8,
    pub length: u16,
    pub msg_ctrl_info: u8,
    pub msg_type: u8,
}

#[derive(Debug, Clone, Copy)]
pub enum PacketValidation {
    Checksum(u8),
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
