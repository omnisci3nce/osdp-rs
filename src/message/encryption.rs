use super::markers::{Command, Reply};
use deku::prelude::*;

/// osdp_KEYSET - Transfer encryption key to peripheral device
#[derive(Debug, DekuWrite)]
pub struct EncryptionKeySet {
    key_type: u8,
    length: u8,
    data: Vec<u8>,
}

impl Command for EncryptionKeySet {
    #[inline]
    fn msg_byte(&self) -> u8 {
        0x75
    }
}
