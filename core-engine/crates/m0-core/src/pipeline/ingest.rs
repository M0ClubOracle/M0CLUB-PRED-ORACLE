
use m0_ingestor::{default_simulated_connectors, stream_channel};
use m0_ingestor::connectors::Connector;
use m0_ingestor::stream::consumer::Consumer;
use tracing::info;

pub struct IngestRuntime {
    pub consumer: Consumer,
}

impl IngestRuntime {
    pub async fn start_simulated(markets: &[String]) -> anyhow::Result<Self> {
        let (producer, consumer) = stream_channel(1024);
        let connectors: Vec<Box<dyn Connector>> = default_simulated_connectors(markets);

        for c in connectors {
            let p = producer.clone();
            tokio::spawn(async move {
                let _ = c.run(p).await;
            });
        }

        info!("ingest runtime started (simulated connectors)");
        Ok(Self { consumer })
    }
}
