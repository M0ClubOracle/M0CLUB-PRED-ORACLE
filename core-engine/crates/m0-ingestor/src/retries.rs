
use std::time::Duration;

pub async fn retry_linear<F, Fut, T, E>(mut f: F, attempts: usize, delay: Duration) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
{
    let mut i = 0;
    loop {
        match f().await {
            Ok(v) => return Ok(v),
            Err(e) => {
                i += 1;
                if i >= attempts {
                    return Err(e);
                }
                tokio::time::sleep(delay).await;
            }
        }
    }
}
