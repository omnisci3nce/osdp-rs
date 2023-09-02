use deku::prelude::*;

use super::markers::Command;

#[derive(Debug, Clone, Copy, PartialEq, DekuRead, DekuWrite)]
pub struct Poll {}

impl Command for Poll {
    #[inline]
    fn msg_byte(&self) -> u8 {
        0x00
    } // FIXME: update byte
}
