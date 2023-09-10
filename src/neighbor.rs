use std::collections::BTreeSet;

use crate::message::MessageType;

pub struct Neighbor {
    pub node_id: String,
    confirmed: BTreeSet<MessageType>,
    pub to_gossip: BTreeSet<MessageType>,
}

impl Neighbor {
    pub fn new(node_id: String) -> Self {
        Self {
            node_id,
            confirmed: BTreeSet::new(),
            to_gossip: BTreeSet::new(),
        }
    }

    pub fn receive(&mut self, message: MessageType) {
        if self.confirmed.contains(&message) {
            return;
        }
        self.to_gossip.insert(message);
    }

    pub fn confirm_gossip(&mut self, messages: Vec<MessageType>) {
        for message in messages {
            if self.confirmed.contains(&message) {
                continue;
            }
            self.to_gossip.remove(&message);
            self.confirmed.insert(message);
        }
    }
}
