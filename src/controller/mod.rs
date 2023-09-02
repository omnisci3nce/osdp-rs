//! brainstorm
//! -----------
//! a controller needs to know what devices exist on the OSDP loop a.k.a a `BusDevice`
//! controllers can use more expensive data structures
//! controller should loop through the peripheral devices on the bus and poll them

use std::collections::HashMap;

use crate::device::BusDevice;

type DeviceID = u32;

pub struct Controller {
    devices: HashMap<DeviceID, BusDevice>,
}

impl Controller {
    pub fn register_device() {}
}
