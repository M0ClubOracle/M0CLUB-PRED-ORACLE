
use async_trait::async_trait;
use rand::Rng;
use serde_json::json;
use tracing::info;

use crate::connectors::Connector;
use crate::stream::producer::Producer;
use crate::stream::schema::{RawEvent, SourceKind};
use m0_common::time::now_ms;

#[derive(Debug, Clone)]
pub struct SportsConnector {
    pub market_id: String,
}

#[async_trait]
impl Connector for SportsConnector {
    async fn run(&self, producer: Producer) -> anyhow::Result<()> {
        info!(market_id = %self.market_id, "sports connector started (simulated)");
        loop {
            let signal: f64 = rand::thread_rng().gen_range(0.0..1.0);
            let ev = RawEvent {
                source: SourceKind::Sports,
                market_id: self.market_id.clone(),
                observed_at_ms: now_ms(),
                payload: json!({ "kind": "sports_signal", "signal": signal }),
                dedupe_key: format!("sports:{}:{}", self.market_id, (signal * 1_000_000.0) as i64),
            };
            producer.send(ev).await.ok();
            tokio::time::sleep(std::time::Duration::from_millis(800)).await;
        }
    }
}
