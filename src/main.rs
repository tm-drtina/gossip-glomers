use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

mod event;
mod handler;
mod message;
mod neighbor;
mod parser;
mod state;

use anyhow::Result;

const GOSSIP_PERIOD: Duration = Duration::from_millis(80);

fn main() -> Result<()> {
    let (tx, rx) = channel();
    let tx_parser = tx.clone();

    let jh_parser = thread::spawn(move || {
        let parser = parser::Parser::new(std::io::stdin().lock(), tx_parser);
        parser.processing_loop()
    });

    let jh_handler = thread::spawn(move || {
        let handler = handler::Handler::new(rx, std::io::stdout().lock());
        handler.processing_loop()
    });

    let jh_timer = thread::spawn(move || loop {
        thread::sleep(GOSSIP_PERIOD);
        if tx.send(event::Event::Timer).is_err() {
            break;
        }
    });

    jh_parser.join().expect("Parser thread panicked")?;
    jh_timer.join().expect("Timer thread panicked");
    jh_handler.join().expect("Handler thread panicked")?;

    Ok(())
}
