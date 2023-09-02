platform-agnostic
-----------------
parser
packet

platform-specific
-----------------
tokio ? on PC
busy-loop?

Overall control flow - pseudocode


Use phantom data for a peripheral device state

- `register_device()` -> device is now registered but we havent check its identification / capabilities
- next poll loops we will send PDID and PDCAP
- change its state to `<Identified>` ?