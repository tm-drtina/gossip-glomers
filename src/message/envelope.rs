use std::io::Write;

use anyhow::Result;

use crate::message::maelstrom::MaelstromMessage;
use crate::message::payload::Payload;

pub struct Header {
    pub src: String,
    pub dst: String,
    pub id: Option<usize>,
    pub in_reply_to: Option<usize>,
}

impl Header {
    pub fn reply(self, new_id: Option<usize>) -> Self {
        Self {
            src: self.dst,
            dst: self.src,
            id: new_id,
            in_reply_to: self.id,
        }
    }

    pub fn with_payload(self, payload: Payload) -> Envelope {
        Envelope {
            header: self,
            payload,
        }
    }
}

pub struct Envelope {
    pub header: Header,
    pub payload: Payload,
}

impl Envelope {
    pub fn write_output(self, writer: &mut impl Write) -> Result<()> {
        MaelstromMessage::from(self).write_output(writer)
    }
}

impl From<MaelstromMessage> for Envelope {
    fn from(msg: MaelstromMessage) -> Self {
        Self {
            header: Header {
                src: msg.src,
                dst: msg.dest,
                id: msg.body.msg_id,
                in_reply_to: msg.body.in_reply_to,
            },
            payload: msg.body.payload,
        }
    }
}
