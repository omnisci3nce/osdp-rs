use deku::prelude::*;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
struct DeviceReport {
  #[deku(bits = "24")]
  vendor_code: u32,
  model_num: u8,
  model_version: u8,
  serial_number: u32,
  firmware_major: u8,
  firmware_minor: u8,
  firmware_build: u8,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
struct KeypadReport {
  /// which reader this is for
  reader_num: u8,
  /// how many keypad digits will follow
  digit_count: u8,
  /// byte 2-N
  #[deku(count = "digit_count")]
  digits: Vec<u8>
}

impl KeypadReport {
  /// return the keypad digits as an ascii string
  fn ascii(&self) -> &str {
    std::str::from_utf8(&self.digits).unwrap()
  }
}

#[cfg(test)]
mod tests {
    use crate::experiment::KeypadReport;

    use super::DeviceReport;

  #[test]
  fn check() {
    let test_data: &[u8] = [
        0x01, // byte 0 - vendor 1st
        0x02, // byte 1 - vendor 2nd
        0x03, // byte 2 - vendor 3rd
        0xFF, // byte 3 - model number
        0x08, // byte 4 - model version
        0xCD, // byte 5 - serial LSB
        0x01, // byte 6 
        0x02, // byte 7
        0xBE, // byte 8 - serial MSB
        0x0A, // byte 9 - fw maj
        0x0B, // byte 10 - fw min
        0x0C, // byte 11 - fw patch
    ]
    .as_ref();

    let test_device = DeviceReport::try_from(test_data).unwrap();
    dbg!(test_device);

    let test_data: &[u8] = [
      0x01, // byte 0 - reader
      0x05, // byte 1 - digits
      0x68, 0x65, 0x6C, 0x6C, 0x6F // "hello"
    ].as_ref();
    let test_keypad = KeypadReport::try_from(test_data).unwrap();
    dbg!(&test_keypad);
    println!("{}", test_keypad.ascii());
  }
}