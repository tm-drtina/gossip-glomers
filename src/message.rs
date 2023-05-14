use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use ulid::Ulid;

pub type MessageType = usize;

#[derive(Debug, Serialize, Deserialize)]
pub struct Envelope<B> {
    #[serde(flatten)]
    pub header: Header,
    pub body: B,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    pub src: String,
    #[serde(rename = "dest")]
    pub dst: String,
}

impl Header {
    fn reply(self) -> Self {
        Self {
            src: self.dst,
            dst: self.src,
        }
    }
}

impl Envelope<Response> {
    pub fn reply_to(header: Header, reply: Response) -> Envelope<Response> {
        Envelope {
            header: header.reply(),
            body: reply,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Request {
    Init {
        msg_id: usize,
        node_id: String,
        node_ids: Vec<String>,
    },
    Echo {
        msg_id: usize,
        echo: String,
    },
    Generate {
        msg_id: usize,
    },
    Broadcast {
        msg_id: usize,
        message: MessageType,
    },
    Read {
        msg_id: usize,
    },
    Topology {
        msg_id: usize,
        topology: HashMap<String, HashSet<String>>,
    },
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Response {
    InitOk {
        in_reply_to: usize,
    },
    EchoOk {
        msg_id: usize,
        in_reply_to: usize,
        echo: String,
    },
    GenerateOk {
        msg_id: usize,
        in_reply_to: usize,
        id: Ulid,
    },
    BroadcastOk {
        msg_id: usize,
        in_reply_to: usize,
    },
    ReadOk {
        msg_id: usize,
        in_reply_to: usize,
        messages: HashSet<MessageType>,
    },
    TopologyOk {
        msg_id: usize,
        in_reply_to: usize,
    },
}
