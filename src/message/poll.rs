use deku::prelude::*;

use super::markers::Command;

/// osdp_POLL - Poll
#[derive(Debug, Clone, Copy, PartialEq, DekuRead, DekuWrite)]
pub struct Poll {}

impl Command for Poll {
    #[inline]
    fn msg_byte(&self) -> u8 {
        0x60
    }
}
