use std::io::Write;
use std::sync::mpsc::Receiver;

use anyhow::{anyhow, bail, Result};

use crate::event::Event;
use crate::message::{Envelope, Header, Payload};
use crate::state::State;

pub struct Handler<W: Write> {
    receiver: Receiver<Event>,
    writer: W,
    id_generator: ulid::Generator,
    state: Option<State>,
    next_id: usize,
}

impl<W: Write> Handler<W> {
    pub fn new(receiver: Receiver<Event>, writer: W) -> Self {
        let id_generator = ulid::Generator::default();

        Self {
            receiver,
            writer,
            id_generator,
            state: None,
            next_id: 1,
        }
    }

    pub fn processing_loop(mut self) -> Result<()> {
        while let Ok(event) = self.receiver.recv() {
            match event {
                Event::Message(envelope) => self.handle_msg(envelope)?,
                Event::Timer => self.handle_timer()?,
            }
        }
        Ok(())
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

    fn handle_timer(&mut self) -> Result<()> {
        for (recipient, data) in self.get_state()?.gossip_data() {
            self.header_to(recipient)?
                .with_payload(Payload::Gossip { seen: data.clone() })
                .write_output(&mut self.writer)?;
        }
        Ok(())
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
                self.get_state_mut()?.receive(message);

                header
                    .reply(self.get_id())
                    .with_payload(Payload::BroadcastOk)
                    .write_output(&mut self.writer)?;
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
                    .set_topology(topology)?;

                header
                    .reply(self.get_id())
                    .with_payload(Payload::TopologyOk)
                    .write_output(&mut self.writer)?;
            }
            Payload::Gossip { seen } => {
                for message in &seen {
                    self.get_state_mut()?.receive(*message);
                }

                header
                    .reply(self.get_id())
                    .with_payload(Payload::GossipOk { seen })
                    .write_output(&mut self.writer)?;
            }
            Payload::GossipOk { seen } => {
                self.get_state_mut()?.confirm_gossip(&header.src, seen)?;
            }
            Payload::BroadcastOk => bail!("Did not expect BroadcastOk message"),
            Payload::ReadOk { .. } => bail!("Did not expect ReadOk message"),
            Payload::InitOk => bail!("Did not expect InitOk message"),
            Payload::TopologyOk => bail!("Did not expect TopologyOk message"),
            Payload::EchoOk { .. } => bail!("Did not expect EchoOk message"),
            Payload::GenerateOk { .. } => bail!("Did not expect GenerateOk message"),
        }
        Ok(())
    }
}
