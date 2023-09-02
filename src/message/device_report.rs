use deku::prelude::*;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
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
