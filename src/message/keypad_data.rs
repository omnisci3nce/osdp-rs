use super::markers::Reply;
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

impl Reply for KeypadData {}

impl KeypadData {
    /// return the keypad digits as an ascii string
    fn ascii(&self) -> &str {
        std::str::from_utf8(&self.digits).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_keypad_data() {
        let test_data: &[u8] = [
            0x01, // byte 0 - reader
            0x05, // byte 1 - digits
            0x68, 0x65, 0x6C, 0x6C, 0x6F, // "hello"
        ]
        .as_ref();
        let test_keypad = KeypadData::try_from(test_data).unwrap();
        dbg!(&test_keypad);
        println!("{}", test_keypad.ascii());
    }
}
