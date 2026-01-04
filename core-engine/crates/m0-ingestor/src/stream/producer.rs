
use tokio::sync::mpsc;
use super::schema::RawEvent;

#[derive(Clone)]
pub struct Producer {
    tx: mpsc::Sender<RawEvent>,
}

impl Producer {
    pub fn new(tx: mpsc::Sender<RawEvent>) -> Self {
        Self { tx }
    }

    pub async fn send(&self, ev: RawEvent) -> Result<(), mpsc::error::SendError<RawEvent>> {
        self.tx.send(ev).await
    }
}
