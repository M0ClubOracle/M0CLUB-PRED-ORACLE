
use anyhow::Context;

pub async fn get_json<T: serde::de::DeserializeOwned>(client: &reqwest::Client, url: &str, bearer: Option<&str>) -> anyhow::Result<T> {
    let mut req = client.get(url).header("accept", "application/json");
    if let Some(b) = bearer {
        req = req.header("authorization", format!("Bearer {b}"));
    }
    let res = req.send().await.context("request failed")?;
    let status = res.status();
    if !status.is_success() {
        let body = res.text().await.unwrap_or_default();
        anyhow::bail!("HTTP {status}: {body}");
    }
    Ok(res.json::<T>().await.context("decode json failed")?)
}
