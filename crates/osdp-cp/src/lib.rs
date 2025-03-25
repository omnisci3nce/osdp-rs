use heapless::LinearMap;
use osdp_sans::{types::Address, Bus, BusTransport};

pub const MAX_BUS_DEVICES: usize = 32;

pub struct PD;

pub struct Controller {
    devices: LinearMap<Address, PD, MAX_BUS_DEVICES>,
    _bus: Bus,
    _desired_poll_rate: u32,
    _transport: Box<dyn BusTransport>,
}

struct DummyTransport {}

impl BusTransport for DummyTransport {
    fn send(&mut self, _data: &[u8]) -> Result<(), osdp_sans::BusError> {
        todo!()
    }

    fn receive(
        &mut self,
        _buffer: &mut heapless::Vec<u8, 512>,
    ) -> Result<usize, osdp_sans::BusError> {
        todo!()
    }
}

impl Controller {
    pub fn new(transport: Box<dyn BusTransport>) -> Self {
        Self {
            devices: LinearMap::new(),
            _bus: Bus::default(),
            _desired_poll_rate: 50, // 1 second default poll rate
            _transport: transport,
        }
    }

    pub fn register_pd(&mut self, addr: Address, peripheral: PD) -> bool {
        self.devices.insert(addr, peripheral).ok().is_some() // TODO: define custom error type
    }

    /// Takes data from the transport and feeds it to the underlying sans-io [Bus],
    /// then if any data is queued to be sent back out send it.
    pub fn tick(&mut self) {
        let mut buf = heapless::Vec::new();
        // copy data from transport into the buffer
        self._transport.receive(&mut buf).unwrap();
        self._bus.feed(&buf);
        // the bus will now potentially have a packet to send back over the transport
        if let Some(_packet) = self._bus.take_packet() {
            // serialise to bytes
            let dummy_bytes = [0xFA, 0x04, 0x01, 0x00, 0xFF];
            self._transport.send(&dummy_bytes).unwrap();
        }
    }

    #[cfg(feature = "tokio")]
    pub async fn async_run(&mut self) {
        // TODO: implement an async loop that runs and passes responses back to an event handler
        //       closure
    }
}
