
use async_trait::async_trait;
use rand::Rng;
use serde_json::json;
use tracing::info;

use crate::connectors::Connector;
use crate::stream::producer::Producer;
use crate::stream::schema::{RawEvent, SourceKind};
use m0_common::time::now_ms;

#[derive(Debug, Clone)]
pub struct SolanaConnector {
    pub market_id: String,
}

#[async_trait]
impl Connector for SolanaConnector {
    async fn run(&self, producer: Producer) -> anyhow::Result<()> {
        info!(market_id = %self.market_id, "solana connector started (simulated)");
        loop {
            let price: f64 = rand::thread_rng().gen_range(90.0..110.0);
            let ev = RawEvent {
                source: SourceKind::Solana,
                market_id: self.market_id.clone(),
                observed_at_ms: now_ms(),
                payload: json!({ "kind": "solana_price", "price": price }),
                dedupe_key: format!("solana:{}:{}", self.market_id, (price * 100.0) as i64),
            };
            producer.send(ev).await.ok();
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
    }
}
