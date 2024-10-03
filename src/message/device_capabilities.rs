// use std::fmt::{self, Display};

use super::markers::{Command, Reply};

#[derive(Debug)]
struct Capability {
    function_code: u8,
    compliance: u8,
    number_of: u8,
}

/// osdp_CAP - PD Capabilities Request
#[derive(Debug, PartialEq)]
pub struct DeviceCapabilitiesRequest {} // TODO: this should serialise to 0x00

impl Command for DeviceCapabilitiesRequest {
    #[inline]
    fn msg_byte(&self) -> u8 {
        0x62
    }
}

#[derive(Debug)]
pub struct DeviceCapabilitiesReport {
    capabilities: heapless::Vec<Capability, 32>, // TODO: research max number of capabilities
}

impl Reply for DeviceCapabilitiesRequest {
    fn msg_byte(&self) -> u8 {
        0x46
    }
}

const fn capability_code_to_str(code: u8) -> Option<&'static str> {
    match code {
        1 => Some("Contact Status Monitoring"),
        2 => Some("Output Control"),
        3 => Some("Card Data Format"),
        4 => Some("Reader LED Control"),
        5 => Some("Reader Audible Output"),
        6 => Some("Reader Text Output"),
        7 => Some("Time Keeping"),
        8 => Some("Check Character Support"),
        9 => Some("Communication Security"),
        10 => Some("Receive BufferSize"),
        11 => Some("Largest Combined Message Size"),
        12 => Some("Smart Card Support"),
        _ => None,
    }
}

#[cfg(feature = "std")]
impl Display for DeviceCapabilitiesReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for ele in &self.capabilities {
            writeln!(
                f,
                "  Function: {}  Compliance: {}",
                capability_code_to_str(ele.function_code).unwrap_or(&ele.function_code.to_string()),
                ele.compliance
            )?;
        }
        Ok(())
    }
}
