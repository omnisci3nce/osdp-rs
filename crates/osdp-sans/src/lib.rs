#![no_std]

use osdp_msg::packet::Packet;

pub mod types;

pub const TX_BUFFER_SIZE: usize = 512;
pub const RX_BUFFER_SIZE: usize = 512;

#[derive(Debug)]
pub enum BusError {}

pub trait BusTransport {
    fn send(&mut self, data: &[u8]) -> Result<(), BusError>;
    fn receive(
        &mut self,
        buffer: &mut heapless::Vec<u8, RX_BUFFER_SIZE>,
    ) -> Result<usize, BusError>;
}

#[derive(Default)]
pub struct Bus {
    outbound_pkt: Option<Packet>,
}

impl Bus {
    /// Pushes data into the bus. The OSDP bus then handles parsing packets and
    /// updating state.
    pub fn feed(&self, _data: &[u8]) {}

    /// Returns Some if there is a packet waiting to be sent back over the bus.
    /// Note that this model assumes that by default there is only a single packet that will be
    /// queued for every [`Bus::feed`] invocation.
    pub fn take_packet(&mut self) -> Option<Packet> {
        self.outbound_pkt.take()
    }
}
