use deku::prelude::*;

use super::markers::{Command, Reply};

#[derive(Debug, DekuRead, DekuWrite)]
struct Capability {
    function_code: u8,
    compliance: u8,
    number_of: u8,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct DeviceCapabilitiesRequest {} // TODO: this should serialise to 0x00

impl Command for DeviceCapabilitiesRequest {}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct DeviceCapabilitiesReport {
    // TODO: capabilities: Vec<Capability>,
}

impl Reply for DeviceCapabilitiesRequest {}
