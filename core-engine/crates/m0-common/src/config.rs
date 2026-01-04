
use serde::{Deserialize, Serialize};
use std::path::Path;
use crate::error::M0Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub env: String,

    // API endpoints for internal services (engine, ingest, signer).
    pub http: HttpConfig,

    // Storage (feature store, offsets, etc.)
    pub storage: StorageConfig,

    // Engine cadence and defaults
    pub engine: EngineConfig,

    // Signer settings
    pub signer: SignerConfig,

    // Telemetry
    pub telemetry: TelemetryConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpConfig {
    pub bind_addr: String,
    pub public_base_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub mode: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineConfig {
    pub tick_ms: u64,
    pub max_markets_per_tick: usize,
    pub schema_version: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignerConfig {
    pub keyring: String,
    pub threshold: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryConfig {
    pub enabled: bool,
    pub service_name: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            env: "dev".into(),
            http: HttpConfig {
                bind_addr: "127.0.0.1:8080".into(),
                public_base_url: "http://127.0.0.1:8080".into(),
            },
            storage: StorageConfig {
                mode: "local".into(),
                path: ".m0data".into(),
            },
            engine: EngineConfig {
                tick_ms: 1000,
                max_markets_per_tick: 32,
                schema_version: 1,
            },
            signer: SignerConfig {
                keyring: "local".into(),
                threshold: 1,
            },
            telemetry: TelemetryConfig {
                enabled: false,
                service_name: "m0".into(),
            },
        }
    }
}

impl Config {
    pub fn load_toml_file(path: impl AsRef<Path>) -> Result<Self, M0Error> {
        let s = std::fs::read_to_string(&path).map_err(|e| M0Error::Io(e.to_string()))?;
        toml::from_str(&s).map_err(|e| M0Error::Config(e.to_string()))
    }
}
