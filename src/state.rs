use std::collections::HashSet;

use crate::message::MessageType;

pub struct State {
    seen: HashSet<MessageType>,
}

impl State {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            seen: HashSet::new(),
        }
    }

    pub fn receive(&mut self, message: MessageType) {
        self.seen.insert(message);
    }

    pub fn seen(&self) -> HashSet<MessageType> {
        self.seen.clone()
    }
}
