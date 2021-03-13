use crate::data::Client;

use std::vec::Vec;

#[derive(Debug)]
pub enum ReplicationRole {
    Primary,
    Replica,
    Confused,
}

#[derive(Debug)]
pub struct ReplicationNode {
    client: Client,
}

#[derive(Debug)]
pub struct ReplicationConfig {
    role: ReplicationRole,
    replicas: Vec<ReplicationNode>
}

impl ReplicationNode {
    pub fn new(client: Client) -> Self {
        Self {
            client: client,
        }
    }
}

impl ReplicationConfig {
    pub fn new() -> Self {
        let role = ReplicationRole::Confused;
        let replicas = Vec::new();
        Self {
            role: role,
            replicas: replicas,
        }
    }

    pub fn add_replica_node(&mut self, client: Client) {
        let new_node = ReplicationNode::new(client);
        self.replicas.push(new_node);
    }
}