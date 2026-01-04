
# Calibration Specification (M0-CORE)

This document specifies calibration for M0Club (M0-CORE).
Calibration transforms raw model probabilities into better-calibrated probabilities aligned with empirical outcomes.
It also defines how calibration artifacts are versioned, stored, deployed, and verified.

This spec is implementation-oriented and maps to components in:
- `core-engine/m0-quant` (online calibration application)
- `services/backtest` (offline training and evaluation)
- `core-engine/m0-common` (deterministic math and encoding)
- `services/api-gateway` (exposing calibration metadata)
- `sdk/*` (optional verifier for calibration metadata)

---

## 1. Goals

- Improve reliability of predicted probabilities across markets/domains.
- Provide deterministic calibration transforms for production.
- Version and audit calibration artifacts.
- Support per-market calibration overrides.
- Integrate calibration with drift detection and risk scoring.

Non-goals:
- Guaranteeing prediction accuracy for all domains.
- Replacing domain-specific model design.
- Publishing proprietary raw datasets.

---

## 2. Terminology

- **Raw probability**: model output before calibration.
- **Calibrated probability**: probability after calibration transform.
- **Calibration artifact**: the parameters defining the transform.
- **Calibration version**: a monotonically increasing version number for an artifact.
- **Training window**: historical period used to fit calibration.
- **Evaluation window**: period used to test calibration quality.

---

## 3. Calibration Placement in Pipeline

### 3.1 Online path
1) Model produces raw distribution (p_raw).
2) Calibration transform applies to p_raw to produce p_cal.
3) Bundler writes p_cal into oracle output.
4) Metadata includes calibration references for auditability.

### 3.2 Offline path
1) Feature store + outcomes store produce labeled dataset.
2) Model is run on historical features to obtain p_raw.
3) Calibration artifact is trained on (p_raw, y).
4) Artifact evaluated and approved.
5) Artifact deployed to model registry and applied online.

---

## 4. Supported Calibration Methods

M0-CORE supports multiple calibration methods. The chosen method must be explicit in the artifact.

### 4.1 Temperature scaling (categorical)
Applies a temperature to logits before softmax.

Pros:
- simple and stable
- works for multi-class

Cons:
- limited flexibility

### 4.2 Platt scaling (binary)
A logistic transform:
- `p_cal = sigmoid(a * logit(p_raw) + b)`

Pros:
- strong baseline for binary

Cons:
- assumes logistic shape

### 4.3 Isotonic regression (binary or per-class)
A monotonic piecewise-constant function mapping p_raw to p_cal.

Pros:
- flexible and non-parametric

Cons:
- can overfit, requires constraints and binning

### 4.4 Histogram binning
Map p_raw into bins and replace with observed frequency per bin.

Pros:
- simple and interpretable
Cons:
- coarse, requires sufficient data

Recommended v1 defaults:
- Binary markets: Platt scaling (with fallback to histogram)
- Multi-class: Temperature scaling (with optional per-class isotonic if data supports)

---

## 5. Calibration Artifacts

### 5.1 Artifact identity
Calibration artifacts are referenced by:
- `calibration_id` (string, stable)
- `calibration_version` (u32)
- `method` (enum)
- `model_id` and `model_version` compatibility
- `feature_schema_version` compatibility
- `artifact_hash` (sha256 of canonical encoded artifact bytes)

### 5.2 Artifact format (canonical)
Artifacts must be encoded deterministically.

Recommended representation:
- a canonical JSON form for human inspection
- a canonical binary encoding for hashing and runtime loading

Fields (logical):
- `calibration_id`
- `calibration_version`
- `method`
- `domain`
- `market_id` (optional; if absent, applies to all markets using model)
- `trained_on_start_ms`
- `trained_on_end_ms`
- `eval_on_start_ms`
- `eval_on_end_ms`
- `data_requirements` metadata
- `params` (method-specific)
- `metrics` (brier, logloss, ece, nll improvements)
- `created_at_ms`
- `created_by` (optional identifier)

### 5.3 Method-specific params
#### Platt scaling
- `a` (fixed-point or f64 encoded deterministically)
- `b`

Recommended:
- store as fixed-point with scale `1e9` to avoid floating drift
- define stable rounding for conversions

#### Temperature scaling
- `temperature` (fixed-point)
- optional per-class temperatures (advanced)

#### Isotonic regression
- `knot_x[]` (sorted unique p_raw values or bin centers)
- `knot_y[]` (monotonic mapped outputs)
- method must enforce monotonicity

#### Histogram binning
- `bin_edges[]`
- `bin_values[]`
- optionally counts per bin for diagnostics

---

## 6. Deterministic Application Rules

Calibration application MUST be deterministic.

### 6.1 Fixed-point probability representation
Input probabilities are fixed-point:
- `p_scaled` with scale `P = 1e9`

Calibration must operate in a way that yields deterministic p_scaled outputs.

Approaches:
- convert p_scaled to f64 using a deterministic conversion and apply transform with controlled rounding
- or implement transforms fully in fixed-point integer math where possible

Recommended v1:
- for Platt scaling and temperature scaling, use stable f64 math + explicit rounding to nearest-even, with test vectors.
- for isotonic/histogram, use integer comparisons and fixed-point outputs directly.

### 6.2 Bounds
After calibration:
- `0 <= p_cal <= 1`
- for multi-class, sum of p_cal equals 1 (within rounding), then canonical normalization enforces exact sum.

### 6.3 Stability requirements
- identical p_raw and artifact produce identical p_cal
- artifacts are immutable once deployed
- any artifact changes require new version and new hash

---

## 7. Multi-Class Calibration

### 7.1 Per-class vs joint calibration
Options:
- joint calibration on logits with temperature scaling (recommended)
- per-class calibration transforms (advanced)

Recommended v1:
- use temperature scaling applied to logits produced by the model.
If the model only emits probabilities, you can:
- reconstruct pseudo-logits via `log(p)` with safeguards
- or apply per-class isotonic on probabilities (requires enough data)

### 7.2 Softmax determinism
Softmax can be sensitive to floating math.
To ensure stability:
- clamp logits to a reasonable range
- subtract max logit before exp
- use deterministic math functions
- round final probabilities to fixed-point with stable rounding

---

## 8. Calibration Selection and Routing

### 8.1 Selection rules
At runtime, the engine selects a calibration artifact using:
1) market-specific override (if configured)
2) model-level default calibration artifact
3) domain-level fallback
4) no calibration (identity transform)

Selection must be deterministic and logged.

### 8.2 Compatibility checks
The engine must verify:
- artifact supports the model_id/version
- artifact supports the feature_schema_version
- artifact method matches expected output type (binary vs multi-class)

If incompatible:
- mark quality flags
- increase risk score
- optionally fall back to identity transform

---

## 9. Storage and Deployment

### 9.1 Artifact store
Calibration artifacts should be stored in:
- a versioned object store (S3/GCS)
- and indexed in Postgres/registry for discovery
- optionally embedded in the container image for stable deployments

### 9.2 Hash verification
At runtime:
- load artifact bytes
- compute sha256
- compare to expected artifact_hash from registry/config
- refuse to apply if mismatch

### 9.3 Rollout strategy
- canary deploy calibration updates per market or percentage of traffic
- monitor metrics and drift
- rollback to prior version if degradation detected

---

## 10. Metrics and Evaluation

Calibration evaluation metrics:
- Brier score (lower better)
- Log loss / NLL
- Expected calibration error (ECE)
- Maximum calibration error (MCE)
- Reliability diagrams
- Sharpness and entropy

Artifacts should store:
- baseline metrics (pre-calibration)
- post-calibration metrics
- delta improvements

Minimum data requirements:
- minimum N samples (domain dependent)
- minimum coverage over probability bins
- exclude periods with known data outages

---

## 11. Drift and Recalibration Policy

Calibration drifts over time.

Recalibration triggers:
- ECE increases beyond threshold
- Brier/logloss regression beyond threshold
- feature distribution drift signals
- regime change events

Policy:
- schedule recalibration periodic jobs (weekly/monthly)
- trigger ad-hoc recalibration on alerts
- keep historical artifacts for audit

---

## 12. On-Chain and Bundle Metadata

Oracle bundles may include calibration references without revealing proprietary details.

Recommended metadata fields:
- `calibration_id`
- `calibration_version`
- `calibration_hash`
- `trained_on_end_ms` (optional)
- `calibration_flags` (e.g., applied, fallback, incompatible)

This allows consumers to:
- audit which calibration was used
- detect unexpected changes

---

## 13. Test Plan

### 13.1 Test vectors
For each method:
- define input p_scaled values
- define expected output p_scaled
- define artifact bytes and hash
- store under `m0-common/test-vectors/calibration/`

### 13.2 Unit tests
- artifact parsing and hash verification
- deterministic application for a wide set of inputs
- monotonicity checks for isotonic
- normalization and bounds checks

### 13.3 Integration tests
- backtest job produces artifact
- artifact deployed to registry
- online engine loads and applies artifact
- bundle includes correct metadata
- localnet end-to-end publish with calibrated probabilities

---

## 14. Implementation Guidance

- Keep calibration artifacts small and immutable.
- Use explicit rounding and clamping rules.
- Always log calibration selection decisions.
- Increase risk score when calibration is missing or incompatible.
- Provide dashboards for calibration health and drift metrics.

---

## Links

- Website: https://m0club.com/
- X (Twitter): https://x.com/M0Clubonx
