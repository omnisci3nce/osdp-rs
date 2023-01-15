# osdp-rs

*Very work-in-progress* implementation of the OSDP specification in Rust.

More information on OSDP can be found on the official SIA site: [open-supervised-device-protocol](https://www.securityindustry.org/industry-standards/open-supervised-device-protocol/)


## Ideas

- Investigate [deku](https://github.com/sharksforarms/deku) for serialisation/deserialisation
- REPL
- TUI

## Example output

```rust
let device = BusDevice { address: 0x00 };
// Send a packet for testing (requests device info)
let datablock = DeviceIDReportRequest{};
device.send(&mut port, &datablock);
```

```
[PARSER] Accumulated all data bytes
[PARSER] Transition to Validation state
[PARSER] Parse byte 0x0
[PARSER] Parse byte 0xc9
[PARSER] Finished receiving packet
[PARSER] Transition to Done state
Complete packet received: Packet { address: 128, length: 19, msg_ctrl_info: 0, buffer: [128, 19, 0, 0, 69, 0, 6, 142, 0, 0, 50, 55, 54, 55, 1, 161, 0, 201], msg_type: 69 }
DeviceIDReport (0x45):
  Vendor Code: 9307648
  Model Number: 0
  Model Version: 0
  Serial Number: 926299954
  Firmware: 1.161.0
```
