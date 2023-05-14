use std::io::{Read, Write};

use anyhow::{anyhow, bail, Result};
use serde_json::de::IoRead;
use serde_json::{Deserializer, StreamDeserializer};

use crate::message::{Envelope, Header, MaelstromMessage, Payload};
use crate::state::State;

pub struct Handler<'a, R: Read, W: Write> {
    reader: StreamDeserializer<'a, IoRead<R>, MaelstromMessage>,
    writer: W,
    id_generator: ulid::Generator,
    state: Option<State>,
    next_id: usize,
}

impl<'a, R: Read, W: Write> Handler<'a, R, W> {
    pub fn new(reader: R, writer: W) -> Self {
        let reader = Deserializer::from_reader(reader).into_iter();
        let id_generator = ulid::Generator::default();

        Self {
            reader,
            writer,
            id_generator,
            state: None,
            next_id: 1,
        }
    }

    pub fn read_msg(&mut self) -> Option<Result<()>> {
        match self.reader.next() {
            None => None,
            Some(Err(err)) => Some(Err(err.into())),
            Some(Ok(msg)) => Some(self.handle_msg(msg.into())),
        }
    }

    fn get_id(&mut self) -> Option<usize> {
        let id = self.next_id;
        self.next_id += 1;
        Some(id)
    }

    fn header_to(&mut self, recipient: String) -> Result<Header> {
        Ok(Header {
            src: self.get_state()?.node_id.clone(),
            dst: recipient,
            id: self.get_id(),
            in_reply_to: None,
        })
    }

    fn get_state(&self) -> Result<&State> {
        self.state
            .as_ref()
            .ok_or(anyhow!("state must be initialized"))
    }
    fn get_state_mut(&mut self) -> Result<&mut State> {
        self.state
            .as_mut()
            .ok_or(anyhow!("state must be initialized"))
    }

    fn handle_msg(&mut self, envelope: Envelope) -> Result<()> {
        let Envelope { header, payload } = envelope;

        match payload {
            Payload::Init { node_id, node_ids } => {
                self.state = Some(State::new(node_id, node_ids));
                header
                    .reply(self.get_id())
                    .with_payload(Payload::InitOk)
                    .write_output(&mut self.writer)?;
            }
            Payload::Echo { echo } => {
                header
                    .reply(self.get_id())
                    .with_payload(Payload::EchoOk { echo })
                    .write_output(&mut self.writer)?;
            }
            Payload::Generate => {
                let id = self.id_generator.generate()?;
                header
                    .reply(self.get_id())
                    .with_payload(Payload::GenerateOk { id })
                    .write_output(&mut self.writer)?;
            }
            Payload::Broadcast { message } => {
                let distribute = self.get_state_mut()?.receive(&header.src, message);

                header
                    .reply(self.get_id())
                    .with_payload(Payload::BroadcastOk)
                    .write_output(&mut self.writer)?;

                for (recipient, message) in distribute {
                    self.header_to(recipient)?
                        .with_payload(Payload::Broadcast { message })
                        .write_output(&mut self.writer)?;
                }
            }
            Payload::Read => {
                let messages = self.get_state()?.seen();

                header
                    .reply(self.get_id())
                    .with_payload(Payload::ReadOk { messages })
                    .write_output(&mut self.writer)?;
            }
            Payload::Topology { topology } => {
                self.state
                    .as_mut()
                    .ok_or(anyhow!("State must be initalized"))?
                    .set_topology(topology);

                header
                    .reply(self.get_id())
                    .with_payload(Payload::TopologyOk)
                    .write_output(&mut self.writer)?;
            }
            Payload::BroadcastOk => {
                // TODO: mark as delivered
            }
            Payload::ReadOk { messages } => {
                let _ = messages;
                todo!();
            }
            Payload::InitOk { .. } => bail!("Did not expect InitOk message"),
            Payload::TopologyOk { .. } => bail!("Did not expect TopologyOk message"),
            Payload::EchoOk { .. } => bail!("Did not expect EchoOk message"),
            Payload::GenerateOk { .. } => bail!("Did not expect GenerateOk message"),
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::assert_eq;

    use crate::handler::Handler;

    use anyhow::Result;

    #[test]
    fn test_init() -> Result<()> {
        let src = "c1";
        let dst = "n1";
        let req_id = 1;

        let input = format!(
            r#"{{"src": "{src}", "dest": "{dst}", "body": {{"type": "init", "msg_id": {req_id}, "node_id":  "n3", "node_ids": ["n1", "n2", "n3"]}}}}
"#
        );
        let mut output = Vec::new();
        let mut handler = Handler::new(input.as_bytes(), &mut output);

        let res = handler.read_msg();
        assert!(res.is_some());
        let _ = res.unwrap()?;
        assert!(handler.read_msg().is_none());

        assert_eq!(
            std::str::from_utf8(&output)?,
            format!(
                r#"{{"src":"{dst}","dest":"{src}","body":{{"type":"init_ok","in_reply_to":{req_id}}}}}
"#
            )
        );

        Ok(())
    }

    #[test]
    fn test_echo() -> Result<()> {
        let src = "c1";
        let dst = "n1";
        let req_id = 1;
        let res_id = 1;
        let echo = "Please echo 35";

        let input = format!(
            r#"{{"src": "{src}", "dest": "{dst}", "body": {{"type": "init", "msg_id": {req_id}, "node_id":  "n3", "node_ids": ["n1", "n2", "n3"]}}}}
{{"src": "{src}", "dest": "{dst}", "body": {{"type": "echo", "msg_id": {req_id}, "echo": "{echo}"}}}}
"#
        );
        let mut output = Vec::new();
        let mut handler = Handler::new(input.as_bytes(), &mut output);

        for _ in 0..2 {
            let res = handler.read_msg();
            assert!(res.is_some());
            let _ = res.unwrap()?;
        }
        assert!(handler.read_msg().is_none());

        assert_eq!(
            std::str::from_utf8(&output)?,
            format!(
                r#"{{"src":"{dst}","dest":"{src}","body":{{"type":"init_ok","in_reply_to":{req_id}}}}}
{{"src":"{dst}","dest":"{src}","body":{{"type":"echo_ok","msg_id":{res_id},"in_reply_to":{req_id},"echo":"{echo}"}}}}
"#
            )
        );

        Ok(())
    }
}
