
use crate::dataset::Dataset;
use crate::metrics::{compute, Metrics};

pub fn run(ds: &Dataset) -> Metrics {
    compute(ds)
}
