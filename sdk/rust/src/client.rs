
use crate::types::{Market, Epoch, Prediction};
use crate::utils::http::get_json;

#[derive(Clone)]
pub struct M0Client {
    http: reqwest::Client,
    base_url: url::Url,
    api_key: Option<String>,
}

impl M0Client {
    pub fn new(base_url: &str) -> anyhow::Result<Self> {
        let base_url = url::Url::parse(base_url)?;
        Ok(Self {
            http: reqwest::Client::new(),
            base_url,
            api_key: None,
        })
    }

    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    fn url(&self, path: &str) -> String {
        self.base_url.join(path).expect("valid join").to_string()
    }

    pub async fn health(&self) -> anyhow::Result<serde_json::Value> {
        get_json(&self.http, &self.url("/health"), self.api_key.as_deref()).await
    }

    pub async fn markets(&self) -> anyhow::Result<Vec<Market>> {
        get_json(&self.http, &self.url("/markets"), self.api_key.as_deref()).await
    }

    pub async fn epochs(&self) -> anyhow::Result<Vec<Epoch>> {
        get_json(&self.http, &self.url("/epochs"), self.api_key.as_deref()).await
    }

    pub async fn latest_prediction(&self, market_id: &str) -> anyhow::Result<Prediction> {
        let p = format!("/predictions/{}/latest", urlencoding::encode(market_id));
        get_json(&self.http, &self.url(&p), self.api_key.as_deref()).await
    }
}
