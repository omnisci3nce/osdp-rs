//! brainstorm
//! -----------
//! a controller needs to know what devices exist on the OSDP loop a.k.a a `BusDevice`
//! controllers can use more expensive data structures
//! controller should loop through the peripheral devices on the bus and poll them

use std::collections::{HashMap, VecDeque};

use deku::DekuContainerWrite;
use serialport::SerialPort;

use crate::{
    device::BusDevice,
    errors::OSDPError,
    message::{markers::Reply, Message},
    packet::{Packet, RequestID, Sqn, ValidationType},
};

#[allow(unused)]
const ADDRESS: u8 = 0x00;

/// Controller
pub struct Controller<'p> {
    pub port: &'p mut Box<dyn SerialPort>,
    /// Map from address -> device
    devices: HashMap<u8, BusDevice>,
    /// Message queue
    msg_queue: VecDeque<(u8, Message)>,
    /// Storage for replies
    request_map: HashMap<RequestID, Option<Box<dyn Reply>>>,
    /// Monotonic increasing unique identifier for command - responses
    next_request_id: RequestID,
    /// OSDP Message sequence number. Zero should only be used for communication startup or recovery after comms loss.
    next_sequence_num: Sqn,
    options: ControllerOptions,
}

impl<'p> Controller<'p> {
    pub fn new(port: &'p mut Box<dyn SerialPort>, options: ControllerOptions) -> Controller {
        Self {
            port,
            options,
            devices: HashMap::new(),
            msg_queue: VecDeque::new(),
            request_map: HashMap::new(),
            next_request_id: 0,
            next_sequence_num: Sqn::Zero,
        }
    }

    /// Register a peripheral device
    pub fn register_pd(&mut self, device: BusDevice) -> Result<(), OSDPError> {
        if self.devices.contains_key(&device.address) {
            return Err(OSDPError::AddressInUse);
        }
        self.devices.insert(device.address, device);
        Ok(())
    }

    pub fn enqueue_cmd(&mut self, address: u8, msg: Message) {
        self.msg_queue.push_back((address, msg));
    }

    pub fn send_next(&mut self) -> Result<RequestID, OSDPError> {
        if let Some((addr, msg)) = self.msg_queue.pop_front() {
            // construct packet
            let p = Packet::construct_from_msg(
                addr,
                self.next_sequence_num,
                self.options.validation_type,
                &msg,
            );
            let packet_bytes = p.to_bytes().unwrap();
            let req_id = self.next_request_id;
            self.next_request_id += 1;
            self.request_map.insert(req_id, None); // waiting for reply
            let _ = (*self.port).write(&packet_bytes); // TODO: handle errors
            Ok(req_id)
        } else {
            // TODO: Poll next device
            Ok(0)
        }
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

#[derive(Default)]
pub struct ControllerOptions {
    validation_type: ValidationType,
}
