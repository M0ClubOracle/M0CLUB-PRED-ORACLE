
use crate::M0Client;

pub async fn run() -> anyhow::Result<()> {
    let base = std::env::var("M0_API_BASE").unwrap_or_else(|_| "http://localhost:8080".into());
    let client = M0Client::new(&base)?;

    let health = client.health().await?;
    println!("health: {}", health);

    let markets = client.markets().await?;
    println!("markets: {}", serde_json::to_string_pretty(&markets)?);

    if let Some(m) = markets.first() {
        let p = client.latest_prediction(&m.market_id).await?;
        println!("latest: {}", serde_json::to_string_pretty(&p)?);
    }

    Ok(())
}
