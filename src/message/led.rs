use super::markers::Command;
use deku::prelude::*;

/// osdp_LED - Control reader LEDs
#[derive(Debug, DekuRead, DekuWrite)]
pub struct ReaderLEDControl {
    reader_num: u8,
    led_num: u8,
    // temporary settings
    temp_control_code: TemporaryControlCode,
    temp_on_time: u8,
    temp_off_time: u8,
    temp_on_color: LEDColor,
    temp_off_color: LEDColor,
    temp_timer: u16,
    // permanent settings
    perm_control_code: TemporaryControlCode,
    perm_on_time: u8,
    perm_off_time: u8,
    perm_on_color: LEDColor,
    perm_off_color: LEDColor,
}

impl Command for ReaderLEDControl {
    #[inline]
    fn msg_byte(&self) -> u8 {
        0x69
    }
}

#[derive(Debug, Default, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum TemporaryControlCode {
    #[default]
    NoOp = 0x00,
    CancelTemp = 0x01,
    SetTemp = 0x02,
}

#[derive(Debug, Default, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum PermanentControlCode {
    #[default]
    NoOp = 0x00,
    SetPerm = 0x01,
}

#[derive(Debug, Default, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum LEDColor {
    #[default]
    Unlit = 0,
    Red = 1,
    Green = 2,
    Amber = 3,
    Blue = 4,
}
