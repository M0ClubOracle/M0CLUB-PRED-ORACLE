
// Telemetry skeleton.
// If you enable OpenTelemetry exports later, keep this module as the single integration point.

#[derive(Debug, Clone)]
pub struct TelemetryConfig {
    pub enabled: bool,
    pub service_name: String,
}

impl TelemetryConfig {
    pub fn disabled(service_name: impl Into<String>) -> Self {
        Self { enabled: false, service_name: service_name.into() }
    }
}
