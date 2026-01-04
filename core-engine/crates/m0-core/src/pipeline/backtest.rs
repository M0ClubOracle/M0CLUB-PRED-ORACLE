
use m0_backtest::dataset::{Dataset, DatasetRow};
use m0_backtest::runner::run;

pub fn quick_backtest(p: f64) -> m0_backtest::metrics::Metrics {
    // toy dataset
    let ds = Dataset {
        rows: vec![
            DatasetRow { market_id: "DEMO".into(), observed_at_ms: 0, outcome: true, p },
            DatasetRow { market_id: "DEMO".into(), observed_at_ms: 1, outcome: false, p },
        ]
    };
    run(&ds)
}
