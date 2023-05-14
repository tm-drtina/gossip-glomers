use std::io::{Read, Write};

use anyhow::Result;
use serde_json::de::IoRead;
use serde_json::{Deserializer, StreamDeserializer};

use crate::message::{Envelope, Request, Response};

pub struct Handler<'a, R: Read, W: Write> {
    reader: StreamDeserializer<'a, IoRead<R>, Envelope<Request>>,
    writer: W,
}

impl<'a, R: Read, W: Write> Handler<'a, R, W> {
    pub fn new(reader: R, writer: W) -> Self {
        let reader = Deserializer::from_reader(reader).into_iter();

        Self { reader, writer }
    }

    pub fn read_msg(&mut self) -> Option<Result<()>> {
        match self.reader.next() {
            None => None,
            Some(Err(err)) => Some(Err(err.into())),
            Some(Ok(msg)) => Some(self.handle_msg(msg)),
        }
    }

    fn handle_msg(&mut self, msg: Envelope<Request>) -> Result<()> {
        match msg.body {
            Request::Init {
                msg_id,
                node_id: _,
                node_ids: _,
            } => {
                let res = Response::InitOk {
                    in_reply_to: msg_id,
                };
                let envelope = Envelope::reply_to(msg.header, res);
                self.write_output(&envelope)?;
            }
            Request::Echo { msg_id, echo } => {
                let res = Response::EchoOk {
                    msg_id: 1,
                    in_reply_to: msg_id,
                    echo,
                };
                let envelope = Envelope::reply_to(msg.header, res);
                self.write_output(&envelope)?;
            }
        }
        Ok(())
    }

    fn write_output(&mut self, value: &Envelope<Response>) -> Result<()> {
        serde_json::to_writer(&mut self.writer, value)?;
        self.writer.write(b"\n")?;
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
