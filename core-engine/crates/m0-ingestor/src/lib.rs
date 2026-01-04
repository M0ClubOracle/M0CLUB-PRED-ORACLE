
pub mod connectors;
pub mod dedupe;
pub mod error;
pub mod rate_limit;
pub mod retries;
pub mod stream;

use connectors::{Connector, solana::SolanaConnector, sports::SportsConnector, politics::PoliticsConnector, macro_::MacroConnector};
use stream::{Producer, Consumer};

pub fn default_simulated_connectors(markets: &[String]) -> Vec<Box<dyn Connector>> {
    // In a real system, connectors are configured per market domain.
    // Here we provide a simple deterministic mapping.
    let mut out: Vec<Box<dyn Connector>> = vec![];
    for (i, m) in markets.iter().enumerate() {
        match i % 4 {
            0 => out.push(Box::new(SolanaConnector { market_id: m.clone() })),
            1 => out.push(Box::new(SportsConnector { market_id: m.clone() })),
            2 => out.push(Box::new(PoliticsConnector { market_id: m.clone() })),
            _ => out.push(Box::new(MacroConnector { market_id: m.clone() })),
        }
    }
    out
}

pub fn stream_channel(buffer: usize) -> (Producer, Consumer) {
    stream::channel(buffer)
}
