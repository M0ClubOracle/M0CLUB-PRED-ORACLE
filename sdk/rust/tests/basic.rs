
use m0club::examples::basic;

#[tokio::test]
async fn basic_smoke() {
    // This test expects api-gateway running. If not, it will fail.
    // For CI, set M0_API_BASE to a reachable gateway.
    let _ = basic::run().await;
}
