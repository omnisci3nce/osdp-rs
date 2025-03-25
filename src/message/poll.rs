use super::markers::Command;

/// osdp_POLL - Poll
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Poll {}

impl Command for Poll {
    #[inline]
    fn msg_byte(&self) -> u8 {
        0x60
    }
}

/// osdp_ACK - General acknowledge
#[derive(Debug)]
pub struct Ack {}

/// osdp_NAK - Negative acknowledge
#[derive(Debug)]
pub struct Nack {}
