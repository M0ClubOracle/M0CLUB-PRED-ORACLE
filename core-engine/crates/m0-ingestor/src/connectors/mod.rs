
use async_trait::async_trait;
use crate::stream::producer::Producer;

pub mod solana;
pub mod sports;
pub mod politics;
pub mod macro_;
pub mod webhooks;

#[async_trait]
pub trait Connector: Send + Sync {
    async fn run(&self, producer: Producer) -> anyhow::Result<()>;
}
