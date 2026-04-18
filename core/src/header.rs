use crate::address::IPv8Address;

#[derive(Debug, Clone)]
pub struct Header {
    pub version: u8,
    pub source: IPv8Address,
    pub destination: IPv8Address,
    pub payload_length: u32,
    pub hop_limit: u8,
    pub flow_label: u32,
    pub flags: u16,
}

impl Header {
    pub fn new(source: IPv8Address, destination: IPv8Address) -> Self {
        Self {
            version: 8,
            source,
            destination,
            payload_length: 0,
            hop_limit: 64,
            flow_label: 0,
            flags: 0,
        }
    }
}
