#[derive(Debug, Default, Clone, Copy)]
pub struct Packet{
  pub address: u8,
  pub length: u16,
  pub msg_ctrl_info: u8,

  pub msg_type: u8,
}

impl Packet {
  pub fn has_sch(&self) -> bool {
    let scb_bit_set = self.msg_ctrl_info & 0x08 != 0;
    scb_bit_set
  }

  // TODO(Josh): get this based on bits in CTRL byte in header
  pub fn validation_len(&self) -> u16 { 2 } // for now just spit out 2
}