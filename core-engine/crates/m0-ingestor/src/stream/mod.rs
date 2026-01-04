
pub mod consumer;
pub mod offsets;
pub mod producer;
pub mod schema;

use tokio::sync::mpsc;
use schema::RawEvent;

pub fn channel(buffer: usize) -> (producer::Producer, consumer::Consumer) {
    let (tx, rx) = mpsc::channel::<RawEvent>(buffer);
    (producer::Producer::new(tx), consumer::Consumer::new(rx))
}
