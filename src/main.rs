mod handler;
pub mod message;

fn main() {
    let mut handler = handler::Handler::new(std::io::stdin().lock(), std::io::stdout().lock());
    while let Some(msg_res) = handler.read_msg() {
        msg_res.unwrap();
    }
}
