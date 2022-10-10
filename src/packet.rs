#[derive(Debug, Default, Clone, Copy)]
pub struct Packet {
  pub address: u8,
  pub length: u16,
  pub msg_ctrl_info: u8,

  pub msg_type: u8,
}

pub enum ValidationType {
  Checksum,
  CRC16,
}

impl Packet {
  pub fn has_sch(&self) -> bool {
    let scb_bit_set = self.msg_ctrl_info & 0x08 != 0;
    scb_bit_set
  }

  pub fn validation_len(&self) -> u16 {
    match self.validation_type() {
      ValidationType::CRC16 => 2,
      ValidationType::Checksum => 1,
    }
  }

  pub fn validation_type(&self) -> ValidationType {
    let bit_set = self.msg_ctrl_info & 0x04 != 0;
    match bit_set {
      true => ValidationType::CRC16,
      false => ValidationType::Checksum,
    }
  }
}
