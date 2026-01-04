
use crate::runtime::metrics::RuntimeMetrics;
use crate::pipeline::health::HealthStatus;
use tokio::sync::Mutex;
use std::sync::Arc;

#[derive(Clone)]
pub struct WorkerState {
    pub metrics: Arc<Mutex<RuntimeMetrics>>,
}

impl WorkerState {
    pub fn new() -> Self {
        Self { metrics: Arc::new(Mutex::new(RuntimeMetrics::default())) }
    }

    pub async fn inc_tick(&self) {
        let mut m = self.metrics.lock().await;
        m.ticks += 1;
    }

    pub async fn inc_bundle(&self) {
        let mut m = self.metrics.lock().await;
        m.bundles_emitted += 1;
    }

    pub async fn health(&self) -> HealthStatus {
        HealthStatus::Ok
    }
}
