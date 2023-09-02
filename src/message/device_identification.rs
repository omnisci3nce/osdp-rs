use deku::prelude::*;

use super::markers::{Command, Reply};

#[derive(Debug, DekuRead, DekuWrite)]
pub struct DeviceIDRequest {}

impl Command for DeviceIDRequest {}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct DeviceIdentification {
    #[deku(bits = "24")]
    vendor_code: u32,
    model_num: u8,
    model_version: u8,
    serial_number: u32,
    firmware_major: u8,
    firmware_minor: u8,
    firmware_build: u8,
}

// impl fmt::Display for DeviceIDReport {
//   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//     write!(
//       f,
//       "DeviceIDReport (0x45):\n  Vendor Code: {}\n  Model Number: {}\n  Model Version: {}\n  Serial Number: {}\n  Firmware: {}\n",
//       self.vendor_code,
//       self.model_no,
//       self.model_version,
//       self.serial_number,
//       self.firmware_version
//     )
//   }
// }

impl Reply for DeviceIdentification {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_device_ident() {
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

        let test_device = DeviceIdentification::try_from(test_data).unwrap();
        dbg!(test_device);
    }
}
