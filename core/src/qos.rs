#[derive(Debug, Clone, Copy)]
pub enum Priority {
    Low,
    Normal,
    High,
    Critical,
}

#[derive(Debug)]
pub struct QoS {
    pub priority: Priority,
    pub flow_label: u32,
}

impl QoS {
    pub fn new(priority: Priority) -> Self {
        let flow_label = rand::random::<u32>() & 0xFFFFF; // 20-bit like IPv6
        Self {
            priority,
            flow_label,
        }
    }
}
