use std::collections::{BTreeSet, BTreeMap};

use anyhow::{anyhow, Result};

use crate::message::MessageType;
use crate::neighbor::Neighbor;

pub struct State {
    pub node_id: String,
    messages: BTreeSet<MessageType>,
    neighbors: BTreeMap<String, Neighbor>,
}

impl State {
    pub fn new(node_id: String, _node_ids: Vec<String>) -> Self {
        Self {
            node_id,
            messages: BTreeSet::new(),
            neighbors: BTreeMap::new(),
        }
    }

    pub fn set_topology(&mut self, topology: BTreeMap<String, Vec<String>>) -> Result<()> {
        self.neighbors = topology
            .get(&self.node_id)
            .ok_or(anyhow!("Topology for node not found!"))?
            .iter()
            .map(|n| (n.to_owned(), Neighbor::new(n.to_owned())))
            .collect();

        Ok(())
    }

    pub fn confirm_gossip(&mut self, from: &String, values: Vec<MessageType>) -> Result<()> {
        self.neighbors
            .get_mut(from)
            .ok_or(anyhow!("Missing neighbor, but got GossipOk from"))?
            .confirm_gossip(values);
        Ok(())
    }

    pub fn receive(&mut self, message: MessageType) {
        if self.messages.insert(message) {
            for neighbor in self.neighbors.values_mut() {
                neighbor.receive(message);
            }
        }
    }

    pub fn gossip_data(&self) -> BTreeMap<String, Vec<MessageType>> {
        self.neighbors.iter().filter_map(|(k, v)| {
            if v.to_gossip.is_empty() {
                None
            } else {
                Some((k.to_owned(), v.to_gossip.iter().copied().collect()))
            }
        }).collect()
    }

    pub fn seen(&self) -> Vec<MessageType> {
        self.messages.iter().copied().collect()
    }
}
