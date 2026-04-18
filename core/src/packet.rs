use crate::header::Header;

#[derive(Debug, Clone)]
pub struct Packet {
    pub header: Header,
    pub payload: Vec<u8>,
    pub checksum: Option<u32>,
}

impl Packet {
    pub fn new(header: Header, payload: Vec<u8>) -> Self {
        let mut packet = Self {
            header,
            payload,
            checksum: None,
        };

        packet.header.payload_length = packet.payload.len() as u32;
        packet
    }
}
