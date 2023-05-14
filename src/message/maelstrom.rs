use std::io::Write;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::message::envelope::Envelope;
use crate::message::payload::Payload;

#[derive(Debug, Serialize, Deserialize)]
pub struct MaelstromMessage {
    pub(super) src: String,
    pub(super) dest: String,
    pub(super) body: Body,
}

impl MaelstromMessage {
    pub fn write_output(&self, writer: &mut impl Write) -> Result<()> {
        serde_json::to_writer(&mut *writer, self)?;
        writer.write_all(b"\n")?;
        Ok(())
    }
}

impl From<Envelope> for MaelstromMessage {
    fn from(envelope: Envelope) -> Self {
        Self {
            src: envelope.header.src,
            dest: envelope.header.dst,
            body: Body {
                msg_id: envelope.header.id,
                in_reply_to: envelope.header.in_reply_to,
                payload: envelope.payload,
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct Body {
    pub(super) msg_id: Option<usize>,
    pub(super) in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub(super) payload: Payload,
}
