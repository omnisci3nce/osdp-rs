use strum::EnumDiscriminants;

/// spec defines this
pub const MAX_PACKET_SIZE: usize = 128;
/// max packet size minus header and checksum
pub const MAX_DATA_LEN: usize = MAX_PACKET_SIZE - 5 - 2;

/// Defines a type `ValidationType`
#[derive(Debug, Clone, EnumDiscriminants)]
#[strum_discriminants(name(ValidationType))]
pub enum PacketValidation {
    Checksum(u8),
    CRC(u16),
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Sqn {
    #[default]
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
}

#[derive(Debug, Default, Clone)]
pub struct PacketHeader {
    pub address: u8,
    pub length: u16,
    pub msg_ctrl_info: u8,
    pub msg_type: u8,
}

impl PacketHeader {
    pub fn has_sch(&self) -> bool {
        self.msg_ctrl_info & 0x08 != 0
    }

    pub fn validation_len(&self) -> u16 {
        match self.validation_type() {
            ValidationType::CRC => 2,
            ValidationType::Checksum => 1,
        }
    }

    pub fn validation_type(&self) -> ValidationType {
        let is_bit_set = self.msg_ctrl_info & 0x04 != 0;
        match is_bit_set {
            true => ValidationType::CRC,
            false => ValidationType::Checksum,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct MsgControl {
    sqn: u8,
    validation_type: u8,
    scb_presence: u8,
    // remaining 4 bits are deprecated as part of the spec and thus unused
}

/// An OSDP packet
#[derive(Debug, Clone)]
pub struct Packet {
    pub header: PacketHeader,
    pub data: heapless::Vec<u8, MAX_DATA_LEN>,
    pub integrity: PacketValidation,
}

impl Packet {
    pub fn new() -> Self {
        todo!()
    }
}
