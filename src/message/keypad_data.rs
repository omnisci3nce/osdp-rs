use deku::prelude::*;

/// KeypadDataReport
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct KeypadData {
    /// which reader this is for
    reader_num: u8,
    /// how many keypad digits will follow
    digit_count: u8,
    /// byte 2-N
    #[deku(count = "digit_count")]
    digits: Vec<u8>,
}
