#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Address(pub u8);

/// Represents a device on the RS485 bus and any information
/// we have about it.
pub struct BusDevice {
    address: Address,
}
