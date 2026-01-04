
use clap::Parser;
use m0_backtest::dataset::{Dataset, DatasetRow};
use m0_backtest::runner::run;
use m0_backtest::reports::render_text;

#[derive(Parser, Debug)]
struct Args {}

fn main() -> anyhow::Result<()> {
    let _ = Args::parse();

    let ds = Dataset {
        rows: vec![
            DatasetRow { market_id: "DEMO".into(), observed_at_ms: 1, outcome: true, p: 0.62 },
            DatasetRow { market_id: "DEMO".into(), observed_at_ms: 2, outcome: false, p: 0.40 },
            DatasetRow { market_id: "DEMO".into(), observed_at_ms: 3, outcome: true, p: 0.70 },
        ]
    };

    let m = run(&ds);
    print!("{}", render_text(&m));
    Ok(())
}
