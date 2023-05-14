use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use ulid::Ulid;

pub type MessageType = usize;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Payload {
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
    Echo {
        echo: String,
    },
    EchoOk {
        echo: String,
    },
    Generate,
    GenerateOk {
        id: Ulid,
    },
    Broadcast {
        message: MessageType,
    },
    BroadcastOk,
    Read,
    ReadOk {
        messages: HashSet<MessageType>,
    },
    Topology {
        topology: HashMap<String, Vec<String>>,
    },
    TopologyOk,
}
