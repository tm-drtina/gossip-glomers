mod handler;
pub mod message;
pub mod state;

fn main() {
    let mut handler = handler::Handler::new(std::io::stdin().lock(), std::io::stdout().lock());
    while let Some(msg_res) = handler.read_msg() {
        msg_res.unwrap();
    }
}
