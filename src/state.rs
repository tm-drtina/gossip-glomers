use std::collections::{HashMap, HashSet};

use crate::message::MessageType;

pub struct State {
    pub node_id: String,
    node_ids: Vec<String>,
    seen: HashSet<MessageType>,
    neighbors: Option<Vec<String>>,
}

impl State {
    pub fn new(node_id: String, node_ids: Vec<String>) -> Self {
        Self {
            node_id,
            node_ids,
            seen: HashSet::new(),
            neighbors: None,
        }
    }

    pub fn set_topology(&mut self, topology: HashMap<String, Vec<String>>) {
        self.neighbors = topology.get(&self.node_id).cloned();
    }

    pub fn receive(&mut self, from: &str, message: MessageType) -> Vec<(String, MessageType)> {
        if self.seen.insert(message) {
            // Only redistribute newly seen messages
            self.neighbors
                .as_ref()
                .unwrap_or(&self.node_ids)
                .iter()
                .filter(|node| *node != &self.node_id)
                .filter(|node| *node != from)
                .map(|node| (node.clone(), message))
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn seen(&self) -> HashSet<MessageType> {
        self.seen.clone()
    }
}
