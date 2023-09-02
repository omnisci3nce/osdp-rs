//! brainstorm
//! -----------
//! a controller needs to know what devices exist on the OSDP loop a.k.a a `BusDevice`
//! controllers can use more expensive data structures
//! controller should loop through the peripheral devices on the bus and poll them

use std::collections::{HashMap, VecDeque};

use crate::{device::BusDevice, errors::OSDPError, message::Message};

/// Controller
#[derive(Default)]
pub struct Controller {
    /// Map from address -> device
    devices: HashMap<u8, BusDevice>,
    /// Message queue
    msg_queue: VecDeque<Message>,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Register a peripheral device
    pub fn register_pd(mut self, device: BusDevice) -> Result<(), OSDPError> {
        if self.devices.contains_key(&device.address) {
            return Err(OSDPError::AddressInUse);
        }
        self.devices.insert(device.address, device);
        Ok(())
    }

    pub fn enqueue_cmd(mut self, msg: Message) {
        self.msg_queue.push_back(msg);
    }

    /// Enqueues a command and also takes a closure to call when the peripheral device has responded
    pub fn enqueue_recv_cmd<F>(self, _msg: Message, _callback: F)
    where
        F: FnOnce(Message),
    {
        todo!()
    }

    pub fn start_loop() {} // TODO
}
