
use async_trait::async_trait;
use serde_json::json;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::info;

use crate::connectors::Connector;
use crate::stream::producer::Producer;
use crate::stream::schema::{RawEvent, SourceKind};
use m0_common::time::now_ms;

#[derive(Debug, Clone)]
pub struct WebhookConnector {
    pub bind: String,
    pub market_id: String,
}

#[async_trait]
impl Connector for WebhookConnector {
    async fn run(&self, producer: Producer) -> anyhow::Result<()> {
        let listener = TcpListener::bind(&self.bind).await?;
        info!(bind=%self.bind, "webhook connector listening (very small HTTP parser)");

        loop {
            let (mut socket, _) = listener.accept().await?;
            let mut buf = vec![0u8; 16 * 1024];
            let n = socket.read(&mut buf).await?;
            let body = String::from_utf8_lossy(&buf[..n]);
            // naive: accept anything, emit a raw payload snapshot
            let ev = RawEvent {
                source: SourceKind::Webhook,
                market_id: self.market_id.clone(),
                observed_at_ms: now_ms(),
                payload: json!({ "kind": "webhook", "raw": body.to_string() }),
                dedupe_key: format!("webhook:{}:{}", self.market_id, now_ms()),
            };
            producer.send(ev).await.ok();

            let resp = "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\n\r\n{"ok":true}\n";
            socket.write_all(resp.as_bytes()).await?;
        }
    }
}
