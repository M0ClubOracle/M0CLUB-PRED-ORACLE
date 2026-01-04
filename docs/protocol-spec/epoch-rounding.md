
# Epoch Rounding and Time Windows (Protocol Spec)

This document defines how M0Club computes epoch identifiers and time windows for markets, including rounding rules, boundary conditions, and replay safety constraints.

The goal is to ensure that:
- Every component (engine, services, SDKs, on-chain programs) derives the same epoch boundaries.
- Epoch IDs are deterministic and stable.
- Updates are accepted only for valid windows (anti-replay / anti-stale).
- Markets can choose different cadences and window sizes without ambiguity.

---

## 1. Definitions

- **epoch_window_ms**: Duration of an epoch window (e.g., 1 hour).
- **publish_cadence_ms**: Expected update cadence within an epoch (e.g., 30 seconds).
- **window_start_ms**: Inclusive start timestamp (ms) for an epoch.
- **window_end_ms**: Exclusive end timestamp (ms) for an epoch.
- **epoch_index**: A monotonically increasing integer derived from time and a fixed origin.
- **epoch_id**: The identifier used on-chain, derived from epoch_index or from market-specific composition.
- **origin_ms**: A fixed reference timestamp used for epoch_index derivation.

---

## 2. Design Constraints

1) Determinism across languages:
- Avoid floating timestamps.
- Avoid locale/timezone conversion.
- Use integer milliseconds since UNIX epoch.

2) Stable boundaries:
- Define inclusive/exclusive boundaries explicitly.
- Ensure no overlap and no gaps.

3) Replay and staleness protection:
- Reject commits for windows far in the past.
- Enforce reveal timing relative to commits and epoch windows.

4) Multi-cadence compatibility:
- Markets can use distinct epoch_window_ms and publish_cadence_ms.

---

## 3. Time Reference and Origin

### 3.1 Time reference
All epoch computations MUST use UNIX epoch time in milliseconds (UTC).

### 3.2 origin_ms
All markets SHOULD share a global origin for epoch indices:
- `origin_ms = 0` (UNIX epoch) is the simplest.
- Alternatively, use a fixed constant like `origin_ms = 1700000000000` if desired.
- The origin MUST be treated as a protocol constant and must not change.

Recommended:
- `origin_ms = 0`

---

## 4. Epoch Index Computation

Given:
- `t_ms`: current timestamp in ms
- `epoch_window_ms`: window size in ms
- `origin_ms`: fixed constant

Compute:
- `delta = max(0, t_ms - origin_ms)`
- `epoch_index = floor(delta / epoch_window_ms)`

Window bounds:
- `window_start_ms = origin_ms + epoch_index * epoch_window_ms`
- `window_end_ms = window_start_ms + epoch_window_ms`

Boundary rule:
- `window_start_ms <= t_ms < window_end_ms`

---

## 5. Epoch ID

M0Club supports two approaches:

### 5.1 Numeric epoch_id (recommended)
Use `epoch_id = epoch_index` as `u64`.

Pros:
- Simple and compact on-chain.
- Stable and monotonic.
- Easy for SDKs.

Cons:
- Not globally unique across different window sizes unless combined with market_id (which it is in practice).

### 5.2 Composite epoch_id (optional)
Use a derived id:
- `epoch_id = sha256_u64(market_id || window_start_ms || window_end_ms)`
or
- store `(market_id, window_start_ms)` as the epoch key

Pros:
- Encodes window bounds into identity.

Cons:
- More complex and less human readable.
- Hash-based ids complicate debugging.

Recommended default:
- Use numeric `epoch_id = epoch_index` combined with `market_id` to uniquely identify an epoch.

---

## 6. Publish Cadence and Sub-Intervals

### 6.1 publish cadence
Within an epoch window, outputs may be produced at a cadence `publish_cadence_ms`.

Implementations MUST ensure that publish ticks are deterministic:
- `tick_index = floor((t_ms - window_start_ms) / publish_cadence_ms)`
- `tick_start_ms = window_start_ms + tick_index * publish_cadence_ms`
- `tick_end_ms = min(tick_start_ms + publish_cadence_ms, window_end_ms)`

This allows:
- engine to produce updates aligned to boundaries
- services to schedule expected update windows
- SDKs to validate freshness

### 6.2 Valid cadence constraints
Registry should enforce:
- `publish_cadence_ms > 0`
- `epoch_window_ms > 0`
- `publish_cadence_ms <= epoch_window_ms`
- `epoch_window_ms % publish_cadence_ms == 0` (recommended but optional)

If not divisible:
- the last tick in a window may be shorter
- this must be handled consistently by all components

Recommended:
- require divisibility for simplicity

---

## 7. Acceptance Rules (Anti-Stale / Anti-Replay)

These rules should be enforced by the on-chain program and mirrored by off-chain submitters.

### 7.1 Commit acceptance
A commit targets:
- `market_id`
- `epoch_id`
- `bundle_hash`
- `commit_ts_ms` (optional on-chain, required off-chain)
- `sequence` replay context

The program SHOULD reject commits if:
- market is not ACTIVE
- commit is for an epoch too far in the past:
  - `now_ms - window_end_ms > max_commit_staleness_ms`
- commit is for an epoch too far in the future:
  - `window_start_ms > now_ms + future_skew_ms`

Recommended parameters:
- `future_skew_ms = 15_000` (15 seconds)
- `max_commit_staleness_ms` configurable per market

### 7.2 Reveal acceptance
Reveal must match an existing commit for `(market_id, epoch_id)`.

The program SHOULD reject reveals if:
- reveal occurs after `max_reveal_delay_ms` relative to commit time
- reveal does not match `bundle_hash`
- reveal sequence/replay context invalid
- reveal payload invalid per reveal_mode constraints

Recommended:
- `max_reveal_delay_ms` configurable per market
- use idempotent reveal accounts keyed by (market_id, epoch_id, bundle_hash)

### 7.3 Finalization acceptance
Finalization should occur when:
- sufficient time has passed since window_end_ms and/or commit
- reveal exists and is valid
- optional quorum/min signatures satisfied

Recommended:
- `now_ms >= window_end_ms + finalization_delay_ms`

---

## 8. Clock Skew and Time Source

### 8.1 Engine time source
Off-chain components should use a monotonic clock for scheduling but must use UNIX ms for epoch derivation.

Recommended:
- derive t_ms from system time synchronized by NTP
- track offset and alert if drift > threshold (e.g., 1s)

### 8.2 On-chain time source
Solana provides block time/clock sysvar. Programs can use:
- slot time approximations
- Clock sysvar unix_timestamp (seconds)

If using on-chain unix_timestamp:
- convert to ms by `unix_timestamp * 1000`
- accept minor variance due to block time granularity

To avoid strict dependence on on-chain time:
- enforce staleness checks based on relative ordering and configured bounds rather than exact ms equality

---

## 9. Examples

### 9.1 Example 1: 1-hour windows
- origin_ms = 0
- epoch_window_ms = 3_600_000

If t_ms corresponds to 2026-01-04 12:34:56 UTC:
- epoch_index = floor(t_ms / 3_600_000)
- window_start_ms = epoch_index * 3_600_000
- window_end_ms = window_start_ms + 3_600_000

### 9.2 Example 2: 5-minute cadence within 1-hour window
- publish_cadence_ms = 300_000
- tick_index = floor((t_ms - window_start_ms) / 300_000)
- tick_start_ms = window_start_ms + tick_index * 300_000

---

## 10. Reference Pseudocode

```text
function epoch_bounds(t_ms, origin_ms, epoch_window_ms):
  assert epoch_window_ms > 0
  delta = max(0, t_ms - origin_ms)
  epoch_index = floor(delta / epoch_window_ms)
  start = origin_ms + epoch_index * epoch_window_ms
  end = start + epoch_window_ms
  return (epoch_index, start, end)

function tick_bounds(t_ms, window_start_ms, window_end_ms, publish_cadence_ms):
  assert publish_cadence_ms > 0
  if t_ms < window_start_ms: t_ms = window_start_ms
  if t_ms >= window_end_ms: t_ms = window_end_ms - 1
  tick_index = floor((t_ms - window_start_ms) / publish_cadence_ms)
  tick_start = window_start_ms + tick_index * publish_cadence_ms
  tick_end = min(tick_start + publish_cadence_ms, window_end_ms)
  return (tick_index, tick_start, tick_end)
```

---

## 11. Implementation Guidance

- The registry should publish `origin_ms`, `epoch_window_ms`, and `publish_cadence_ms` per market.
- SDKs should include helper functions to compute epoch_id and bounds.
- Engine and submitter should record:
  - produced_at_ms
  - targeted epoch_id
  - commit slot / tx signature
  - reveal slot / tx signature
- Indexer should reconcile by `(market_id, epoch_id)`.

---

## Links

- Website: https://m0club.com/
- X (Twitter): https://x.com/M0Clubonx
