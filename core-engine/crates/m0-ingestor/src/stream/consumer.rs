
use tokio::sync::mpsc;
use super::schema::RawEvent;

pub struct Consumer {
    rx: mpsc::Receiver<RawEvent>,
}

impl Consumer {
    pub fn new(rx: mpsc::Receiver<RawEvent>) -> Self {
        Self { rx }
    }

    pub async fn recv(&mut self) -> Option<RawEvent> {
        self.rx.recv().await
    }
}
