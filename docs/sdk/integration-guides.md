
# SDK Integration Guides

This document provides integration guides for using M0Club data in real products.
It covers:
- integrating with trading UIs and dashboards
- integrating with on-chain programs that consume M0 outputs
- integrating with risk engines and alerting
- integrating with data warehouses
- integrating with backtesting and research pipelines
- integrating with custody/signing and compliance boundaries

This guide is written to be actionable and implementation-oriented.

Links:
- Website: https://m0club.com/
- X (Twitter): https://x.com/M0Clubonx

---

## 0. Integration Surfaces

M0Club exposes data through:
- REST API (JSON)
- WebSocket realtime (JSON)
- optional gRPC streams (if enabled)
- on-chain accounts (commit/reveal state, registry, outputs)
- SDK libraries (TypeScript, Rust, Python)

The recommended approach for most apps:
- REST for initial state + pagination
- WebSocket for updates
- SDK verification locally for integrity
- optionally fetch signer sets from chain for trust-minimized verification

---

## 1. Common Patterns

### 1.1 Trust-minimized client verification
Pattern:
1) Fetch latest bundle via API.
2) Fetch signer set pubkeys from on-chain registry.
3) Compute canonical bundle content hash.
4) Verify threshold signatures.
5) Only then display or act on outputs.

Use cases:
- DeFi products relying on oracle signals
- high-stakes alerts
- automated trading

### 1.2 Server-side verification and caching
Pattern:
1) A backend service verifies bundles and stores them in DB/cache.
2) Frontend fetches from trusted backend.
3) Backend exposes a simplified API with verification status.

Use cases:
- dashboards
- analytics and alerting
- rate-limited environments

### 1.3 Hybrid approach
- Frontend verifies occasionally or on critical actions.
- Backend verifies continuously for availability.

---

## 2. Integrating with Trading UIs

### 2.1 Goals
- show probabilities, confidence intervals, and risk flags
- show market history
- show source coverage and staleness
- allow users to filter markets by quality

### 2.2 UI data model
Recommended frontend state:
- `markets[]` (definitions)
- `latestByMarketId` (latest output + bundle hash)
- `qualityByMarketId` (flags, risk score)
- `historyByMarketId` (time series of p_scaled)

### 2.3 UI computation
Convert fixed-point to display format:
- `p_scaled / 1e9` -> 0..1
- display as percent with 2 decimals
- show CI band: `ci_low`..`ci_high`

### 2.4 Handling quality flags
Recommended UX:
- show a badge for risk level (LOW/MED/HIGH)
- show tooltips with reason codes
- hide or gray out markets with BLOCK status

### 2.5 Realtime updates
Use WebSocket subscription:
- subscribe to `market.latest` per market or per domain
- update local store
- re-render chart and badges

### 2.6 Example integration (TypeScript pseudo-code)
```ts
import { M0Client, M0RealtimeClient, verifyBundle } from "@m0club/sdk";

const client = new M0Client({ apiBase: process.env.M0_API_BASE! });
const rt = new M0RealtimeClient({ wsUrl: process.env.M0_WS_URL! });

const signerSetCache = new Map<string, string[]>();

async function ensureSignerPubkeys(signerSetId: string) {
  if (signerSetCache.has(signerSetId)) return signerSetCache.get(signerSetId)!;
  const set = await client.signers.getSignerSet(signerSetId);
  signerSetCache.set(signerSetId, set.pubkeys);
  return set.pubkeys;
}

async function onLatestUpdate(msg: any) {
  const latest = msg.data;
  const pubkeys = await ensureSignerPubkeys(latest.signer_set_id);
  const ok = verifyBundle({ bundle: latest.bundle, signerSetPubkeys: pubkeys });
  if (!ok) return; // do not display unverified data
  // store + render
}
```

---

## 3. Integrating with Alerting and Risk Engines

### 3.1 Alert types
Common alerts:
- probability jumps exceeding threshold
- confidence interval widening suddenly (risk increase)
- risk score increase
- staleness or low coverage
- divergence flags
- publish halt (no updates for interval)

### 3.2 Recommended backend
Use a small alerting service:
- subscribes via WebSocket
- verifies bundles
- stores latest values in Redis
- emits alerts to Slack/Discord/Webhooks

### 3.3 Example alert rule
- if `abs(p_now - p_prev) > 0.05` and `risk_score < 2500`
- trigger alert with CI band and reason codes

### 3.4 Debounce and noise controls
- require N consecutive updates
- use cooldown windows (e.g., 60s)
- adjust thresholds by market tier

### 3.5 Audit trail
Store:
- market_id, epoch_id, tick_index
- bundle_content_hash
- verification result
- rule id triggered
- timestamps

---

## 4. Integrating with Data Warehouses

### 4.1 Goals
- long-term analytics and model evaluation
- join with external datasets
- produce BI dashboards

### 4.2 Recommended ingestion
- a consumer service subscribes to updates
- writes normalized rows to:
  - Postgres (short term)
  - ClickHouse (time series)
  - object storage (parquet) for lakehouse

### 4.3 Schema recommendations
Tables:
- `market_outputs` (market_id, ts, p, ci_low, ci_high, risk, flags, hash)
- `bundles` (bundle_id, hash, signer_set_id, publish_epoch, created_at)
- `signatures` (bundle_hash, signer_pubkey, signature, verified)

Partition by:
- date (created_at)
- domain
- market_id (optional)

### 4.4 Data quality
- always store verification status
- store reason codes
- store source coverage metrics if provided

---

## 5. Integrating with On-Chain Programs

### 5.1 Two approaches

A) Direct on-chain consumption
- read oracle program accounts that store latest revealed outputs
- verify commit/reveal state on-chain (program does)
- use outputs in your program

B) Off-chain attestation + on-chain proof
- off-chain verifier checks bundle and signatures
- submits a proof/attestation transaction referencing bundle hash
- consumer program checks attestation program state

Approach A is simpler but depends on oracle program interface.
Approach B is flexible for custom logic.

### 5.2 Consumer program best practices
- validate freshness (epoch/tick constraints)
- validate market_id exists and is active
- validate risk thresholds (e.g., disallow high risk or stale flags)
- use bounded arithmetic and fixed-point math

### 5.3 Example consumer constraints (conceptual)
- require `now - observed_at_ms < MAX_STALENESS_MS`
- require `risk_score <= MAX_RISK`
- require `quality_flags & BLOCK_FLAGS == 0`

### 5.4 Testing consumer programs
- use localnet with M0 programs deployed
- use deterministic fixtures for oracle outputs
- simulate replay protection edge cases

---

## 6. Integrating with Custody and Secure Signing

### 6.1 Signer agents boundary
Do not run signer agents in the same namespace/node pool as general services.
Use:
- dedicated node pool
- strict NetworkPolicy
- separate service account

### 6.2 KMS/HSM integration
Preferred architecture:
- signer agent requests signatures from KMS
- never holds raw key material
- logs key usage to audit trail

### 6.3 Rotation readiness
- keep an emergency signer set prepared but inactive
- rehearse rotation in staging
- alert on signer anomalies

---

## 7. Integrating with Backtesting Pipelines

### 7.1 Goals
- evaluate model changes
- validate guardrails
- compare sources

### 7.2 Recommended workflow
- export time series from warehouse
- run backtest runner offline
- produce metrics:
  - Brier score
  - log loss
  - calibration curves
  - coverage stats
  - drift stats

### 7.3 A/B testing
- canary quant pods in production shadow mode
- compare canary vs baseline outputs
- promote only if metrics improve

---

## 8. Integrating with Compliance and Governance

This section is technical and operational, not legal advice.

Recommended controls:
- clear audit trails for publish events
- signed release artifacts (optional)
- access control for registry admin
- documented incident response
- data retention policies for raw feeds

---

## 9. Integration Checklists

### 9.1 Frontend checklist
- uses REST for market list + initial latest
- subscribes to WS for updates
- verifies bundles before display (recommended)
- shows confidence intervals and risk flags
- handles staleness gracefully

### 9.2 Backend consumer checklist
- verifies bundle hash and signatures
- stores outputs with verification flag
- emits alerts with cooldown
- maintains signer set cache from chain or API
- has retry logic and idempotency

### 9.3 On-chain integration checklist
- validates freshness
- checks market is active
- enforces risk thresholds
- includes comprehensive tests

---

## 10. Example Integration Project Layout

A typical integration service repo:
- `src/`
  - `consumer.ts` (WS consumer)
  - `verifier.ts` (bundle verification)
  - `store.ts` (DB/redis)
  - `alerts.ts` (rules)
  - `http.ts` (health endpoints)
- `Dockerfile`
- `helm/` or `k8s/`
- `README.md`

---

## 11. Troubleshooting

- Verification fails: confirm signer set pubkeys and schema version.
- Missing updates: verify WS endpoint and market is active.
- High latency: use regional RPC, reduce concurrency, scale consumer.
- Schema changes: handle schema_version negotiation and upgrade SDK.

---

## References

- Types: `docs/sdk/types.md`
- Examples: `docs/sdk/examples.md`
- Output format: `docs/protocol-spec/oracle-output-format.md`
- Bundle hashing: `docs/engine-spec/bundle-hashing.md`
- Key management: `docs/ops/key-management.md`
- Incident response: `docs/ops/incident-response.md`

---

## Links

- Website: https://m0club.com/
- X (Twitter): https://x.com/M0Clubonx
