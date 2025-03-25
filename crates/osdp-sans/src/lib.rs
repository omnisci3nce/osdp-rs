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
pub struct Bus {}

impl Bus {
    pub fn feed(&self, _data: &[u8]) {}
    pub fn take_packet(&self) -> Option<Packet> {
        todo!()
    }
}
