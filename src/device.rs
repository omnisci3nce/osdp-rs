use serialport::SerialPort;
use std::io::Error;

pub struct BusDevice {
    pub address: u8,
}

impl BusDevice {
    pub fn send(&self, _port: &mut Box<dyn SerialPort>) -> Result<(), Error> {
        // let data = msg.serialise();
        // let len: u16 = 5 + 1 + data.len() as u16 + 1;
        // let len_lsb = (len & 0xFF) as u8; // least significant byte
        // let len_msb = ((len >> 8) & 0xFF) as u8; // most significant byte

        // let mut packet = Vec::with_capacity(len as usize);
        // packet.push(0x53);
        // packet.push(self.address);
        // packet.push(len_lsb);
        // packet.push(len_msb);
        // packet.push(0x00);
        // packet.push(msg.msg_byte());
        // for b in data {
        //   packet.push(b)
        // }
        // let chksum = calc_checksum(&packet);
        // packet.push(chksum);

        // let _len = port.write(&packet)?;

        Ok(())
    }
}
