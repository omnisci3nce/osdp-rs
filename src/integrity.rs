pub(crate) fn calc_checksum(msg: &[u8]) -> u8 {
    let mut sum: u32 = 0;
    for x in msg {
        sum += *x as u32;
    }
    let twos_complement = sum.wrapping_neg();
    let lsb = twos_complement & 0b11111111;
    lsb as u8
}

// TODO: calc_crc
