use super::markers::Command;

/// osdp_KEYSET - Transfer encryption key to peripheral device
#[derive(Debug)]
pub struct EncryptionKeySet {
    key_type: u8,
    length: u8,
    data: heapless::Vec<u8, 32>, // TODO: research correct max key length
}

impl Command for EncryptionKeySet {
    #[inline]
    fn msg_byte(&self) -> u8 {
        0x75
    }
}
