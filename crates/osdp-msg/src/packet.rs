use strum::EnumDiscriminants;

/// Defines a type `ValidationType`
#[derive(Debug, Clone, EnumDiscriminants)]
#[strum_discriminants(name(ValidationType))]
pub enum PacketValidation {
    Checksum(u8),
    CRC(u16),
}

pub struct Packet {}
