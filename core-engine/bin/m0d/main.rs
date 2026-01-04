
use clap::Parser;
use m0_common::{config::Config, logging};
use tracing::{info, warn};
use m0_core::pipeline::{ingest::IngestRuntime, normalize::normalize_event, feature::make_features, model::predict_two_outcome, calibrate::calibrate, bundle::build_bundle};
use m0_signer::{commit::commit_hash, replay_protection::ReplayState, reveal::signature_message};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    config: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    logging::init("m0d");

    let cfg = Config::load_toml_file(&args.config).unwrap_or_default();
    info!(env=%cfg.env, "engine starting");

    let markets = vec![
        "NBA_LAL_BOS".to_string(),
        "POL_US_ELECTION".to_string(),
    ];

    let mut ingest = IngestRuntime::start_simulated(&markets).await?;
    let mut replay = ReplayState::default();

    let mut tick: u32 = 0;
    let mut interval = m0_core::runtime::scheduler::tick_interval(cfg.engine.tick_ms);

    loop {
        interval.tick().await;
        tick = tick.wrapping_add(1);

        // Consume up to N events per tick (simple).
        let mut processed = 0usize;
        while processed < cfg.engine.max_markets_per_tick {
            match ingest.consumer.recv().await {
                Some(raw) => {
                    let canon = match normalize_event(&raw) {
                        Ok(v) => v,
                        Err(e) => { warn!(error=%e, "normalize failed"); continue; }
                    };
                    let _feature_row = make_features(&canon);

                    // Very simplified: generate a two-outcome prediction
                    let mut probs = predict_two_outcome("A", "B", 1500.0, 1550.0, 200);
                    calibrate(&mut probs);

                    let sequence = replay.next()?;
                    let (bundle, bundle_bytes, content_hash) = build_bundle(cfg.engine.schema_version, 1, 1, &canon.market_id, 1, tick, sequence, 0, &probs)?;

                    // Commit/reveal message construction (client side)
                    let salt = [7u8; 32];
                    let commit = commit_hash(&content_hash, &salt);
                    let sig_msg = signature_message(&content_hash, bundle.signer_set_id, bundle.publish_epoch_id, sequence);

                    info!(
                        market_id=%canon.market_id,
                        tick_index=tick,
                        sequence=sequence,
                        commit_hex=%hex::encode(commit),
                        bundle_hash_hex=%hex::encode(content_hash),
                        sigmsg_hex=%hex::encode(sig_msg),
                        bundle_json_len=bundle_bytes.len(),
                        "bundle prepared (simulated)"
                    );
                }
                None => break,
            }
            processed += 1;
        }
    }
}
