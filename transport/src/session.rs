use std::collections::HashMap;
use uuid::Uuid;

use ipv8_core::packet::Packet;

#[derive(Debug)]
pub struct Session {
    pub id: Uuid,
    pub connections: HashMap<Uuid, ConnectionState>,
}

#[derive(Debug)]
pub struct ConnectionState {
    pub active: bool,
    pub last_packet: Option<Packet>,
}

impl Session {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            connections: HashMap::new(),
        }
    }

    pub fn create_connection(&mut self) -> Uuid {
        let id = Uuid::new_v4();
        self.connections.insert(
            id,
            ConnectionState {
                active: true,
                last_packet: None,
            },
        );
        id
    }

    pub fn close_connection(&mut self, id: &Uuid) {
        if let Some(conn) = self.connections.get_mut(id) {
            conn.active = false;
        }
    }
}
