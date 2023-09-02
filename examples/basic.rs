use osdp_rs::{controller::Controller, device::BusDevice, parser::Parser, peripheral::Peripheral};

fn main() {
    let acu = Controller::new();

    let known_device_1 = BusDevice { address: 0x01 };
    acu.register_device(known_device_1).unwrap();

    let mut parser = Parser::new();

    // parse some random bytes
    parser.parse_byte(0xFF);
    parser.parse_byte(0xAA);
    parser.parse_byte(0x05);
}
