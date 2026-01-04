
use clap::Parser;
use m0_common::{config::Config, logging};
use tracing::info;
use m0_core::pipeline::ingest::IngestRuntime;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    config: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    logging::init("m0-ingestd");

    let _cfg = Config::load_toml_file(&args.config).unwrap_or_default();
    let markets = vec!["NBA_LAL_BOS".to_string(), "POL_US_ELECTION".to_string(), "MACRO_CPI_US".to_string()];

    let mut ingest = IngestRuntime::start_simulated(&markets).await?;
    info!("ingest daemon running; printing raw events");

    while let Some(ev) = ingest.consumer.recv().await {
        info!(market_id=%ev.market_id, source=?ev.source, "raw_event");
    }

    Ok(())
}
