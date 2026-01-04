
use m0_ingestor::stream_channel;

#[tokio::test]
async fn channel_works() {
    let (_p, mut c) = stream_channel(4);
    // no sender usage here; just ensure types compile.
    assert!(c.recv().await.is_none());
}
