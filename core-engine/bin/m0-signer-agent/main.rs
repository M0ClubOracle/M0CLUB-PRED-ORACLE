
use clap::Parser;
use m0_common::{config::Config, logging};
use m0_signer::{keyring::local::LocalKey, tx_submit::submit_tx_simulated};
use tracing::info;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    config: String,

    #[arg(long, default_value = "devnet")]
    cluster: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    logging::init("m0-signer-agent");

    let _cfg = Config::load_toml_file(&args.config).unwrap_or_default();
    let key = LocalKey::load_from_env()?;
    info!(secret_prefix=%hex::encode(&key.secret[..4]), "loaded signer key (ephemeral skeleton)");

    let sig = submit_tx_simulated(&args.cluster, b"payload").await?;
    info!(tx_sig=%sig, "submitted simulated tx");
    Ok(())
}
