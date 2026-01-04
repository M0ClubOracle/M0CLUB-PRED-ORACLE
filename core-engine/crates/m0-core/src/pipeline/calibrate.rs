
use m0_quant::calibration::isotonic::isotonic_calibrate;
use m0_quant::ProbabilityPoint;

pub fn calibrate(points: &mut [ProbabilityPoint]) {
    for p in points {
        p.p = isotonic_calibrate(p.p);
        p.ci_low = isotonic_calibrate(p.ci_low);
        p.ci_high = isotonic_calibrate(p.ci_high);
    }
}
