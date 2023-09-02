use osdp_rs::{controller::Controller, device::BusDevice, peripheral::Peripheral};


fn main() {
  let acu = Controller::new();

  let known_device_1 = Peripheral::new();
  acu.register_device(device)
}