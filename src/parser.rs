use std::io::Read;
use std::sync::mpsc::Sender;

use anyhow::{Context, Result};
use serde_json::de::IoRead;
use serde_json::{Deserializer, StreamDeserializer};

use crate::event::Event;
use crate::message::MaelstromMessage;

pub struct Parser<'a, R: Read> {
    reader: StreamDeserializer<'a, IoRead<R>, MaelstromMessage>,
    sender: Sender<Event>,
}

impl<'a, R: Read> Parser<'a, R> {
    pub fn new(reader: R, sender: Sender<Event>) -> Self {
        let reader = Deserializer::from_reader(reader).into_iter();

        Self { reader, sender }
    }

    pub fn processing_loop(mut self) -> Result<()> {
        loop {
            match self.reader.next() {
                None => return Ok(()),
                Some(Err(err)) => return Err(err.into()),
                Some(Ok(msg)) => self
                    .sender
                    .send(Event::Message(msg.into()))
                    .context("Sending parsed message to queue")?,
            }
        }
    }
}
