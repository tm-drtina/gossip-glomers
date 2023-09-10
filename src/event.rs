use crate::message::Envelope;


pub enum Event {
    Message(Envelope),
    Timer,
}
