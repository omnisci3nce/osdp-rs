//! brainstorm
//! -----------
//! a controller needs to know what devices exist on the OSDP loop a.k.a a `BusDevice`
//! controllers can use more expensive data structures
//! controller should loop through the peripheral devices on the bus and poll them

use std::collections::HashMap;

use crate::{device::BusDevice, errors::OSDPError};

/// Controller has address of 0x00
#[derive(Default)]
pub struct Controller {
    /// address -> device
    devices: HashMap<u8, BusDevice>,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            devices: HashMap::new(),
        }
    }

    pub fn register_device(mut self, device: BusDevice) -> Result<(), OSDPError> {
        if self.devices.contains_key(&device.address) {
            return Err(OSDPError::AddressInUse);
        }
        self.devices.insert(device.address, device);
        Ok(())
    }
}
