
# Market Registry (Protocol Spec)

This document specifies the market registry design used by M0Club.
The registry is the source of truth for market metadata, schemas, signer policies, and operational parameters that govern how oracle updates are produced and published.

The registry can be implemented:
- On-chain (recommended) via an Anchor program (e.g., `m0-registry`)
- Off-chain (supplemental) as an indexed cache derived from on-chain state

This spec covers:
- Market identifiers and naming rules
- Market schemas and outcome definitions
- Lifecycle states
- Configuration parameters (cadence, epoch windows, fees)
- Authority and governance model
- Signer set references
- SDK and API access patterns

---

## 1. Goals

- Provide a stable market identity for consumers.
- Allow domain-specific schemas while maintaining a unified interface.
- Support safe lifecycle management (create, activate, pause, deprecate).
- Bind markets to signer set policies and publish parameters.
- Enable deterministic, verifiable oracle publishing.

Non-goals:
- Storing raw proprietary data sources on-chain.
- Providing a full historical analytics store (handled by services/indexer).

---

## 2. Market Identity

### 2.1 MarketId
A MarketId is a stable identifier used across:
- engine modeling
- oracle bundles
- on-chain accounts
- SDK and API queries

Recommended format:
- Uppercase ASCII
- Components separated by `_`
- Max length: 64 bytes
- Allowed charset: `A-Z`, `0-9`, `_`

Examples:
- `SPORTS_NBA_LAL_BOS_2026_01_04`
- `POLITICS_US_PRES_2028`
- `MACRO_US_CPI_YOY_2026_02`
- `MARKETS_SOL_BTC_REGIME_1H`

### 2.2 Canonical normalization
Registry MUST store MarketId in canonical form:
- Trim whitespace
- Uppercase
- Reject non-ASCII characters
- Reject IDs exceeding max length

---

## 3. Market Domains

M0Club models multiple domains. Domain influences:
- schema type and validation rules
- suggested cadence and epoch windows
- integration and display semantics

Recommended domain enum:
- `SPORTS`
- `POLITICS`
- `MACRO`
- `MARKETS`
- `ONCHAIN`

The domain is metadata. It MUST NOT change for an existing MarketId.

---

## 4. Outcome Schemas

### 4.1 Schema types
A market has a schema type that defines the set of outcomes and how to interpret them.

Recommended schema types:
- `BINARY` (YES/NO)
- `TRINARY` (HOME/AWAY/DRAW or 3-way)
- `MULTI` (N outcomes, fixed)
- `RANGE_BUCKETS` (binned numeric outcome)
- `ORDERED` (ranked outcomes)
- `CONTINUOUS_PROXY` (continuous value represented by buckets)

The schema type is versioned:
- `schema_version` (u16)
- `schema_type` (enum)

### 4.2 Outcome definitions
For fixed outcome schemas, the registry stores outcomes:

Outcome fields:
- `outcome_id` (ASCII, uppercase)
- `label` (short display name, ASCII recommended)
- `description` (optional)
- `is_active` (bool)
- `sort_key` (u16) for stable ordering

Rules:
- `outcome_id` max length 64 bytes
- Must be unique per market
- Stored in stable canonical ordering by `sort_key` then `outcome_id`

### 4.3 Outcome stability
Once a market is active:
- outcomes MUST NOT be reordered without a new schema version
- outcome_id MUST NOT change
- new outcomes require schema version bump and consumer migration plan

---

## 5. Market Lifecycle

### 5.1 States
Recommended state machine:
- `DRAFT` (created, not active, not publishable)
- `ACTIVE` (publishable)
- `PAUSED` (temporarily disabled; commits/reveals rejected)
- `DEPRECATED` (no new epochs; historical only)
- `ARCHIVED` (hidden by default; historical only)

### 5.2 Transitions
Allowed transitions:
- DRAFT -> ACTIVE
- ACTIVE -> PAUSED
- PAUSED -> ACTIVE
- ACTIVE -> DEPRECATED
- DEPRECATED -> ARCHIVED
- PAUSED -> DEPRECATED

The registry program MUST enforce state transitions.

---

## 6. Publish Parameters

Markets have parameters that control on-chain publishing behavior.

### 6.1 Epoch window configuration
Fields:
- `epoch_window_ms` (u64): length of an epoch window
- `publish_cadence_ms` (u64): expected update cadence
- `finalization_delay_ms` (u64): minimum delay before finalization is allowed
- `max_commit_staleness_ms` (u64): reject commits older than this
- `max_reveal_delay_ms` (u64): reveal must occur within this time

Guidance:
- `publish_cadence_ms` <= `epoch_window_ms`
- `finalization_delay_ms` <= `epoch_window_ms` (typical)
- staleness thresholds prevent replay/stale publishing

### 6.2 Payload constraints
Fields:
- `reveal_mode` (enum): `FULL_BUNDLE` or `MERKLE_PROOF`
- `max_reveal_bytes` (u32): safety cap for reveal payload
- `bundle_item_cap` (u32): maximum items per bundle for this market

### 6.3 Fee policy (optional)
If the protocol uses fees:
- `fee_bps` (u16): basis points
- `fee_vault` (pubkey)
- `fee_router` (pubkey or program id)

Fees should be handled in a dedicated program when possible.

---

## 7. Signer Policies

### 7.1 Signer set reference
Each market references a signer policy:
- `signer_set_id` (u32)
- `min_signatures` (u8)
- `sig_scheme` (enum, default ed25519)
- `sequence_policy` (enum): `GLOBAL` or `PER_MARKET`

### 7.2 Verification requirements
On-chain oracle program MUST verify:
- signature pubkeys belong to signer set
- signature count >= min_signatures
- signature message includes bundle_hash and replay context
- sequence/nonces are not reused

### 7.3 Rotation
Signer sets should be rotated by governance/authority.
Market can be migrated to a new signer_set_id with a timelock policy.

---

## 8. Authority and Governance

### 8.1 Roles
Recommended roles:
- `admin_authority`: create markets, update configs, rotate signer sets
- `guardian_authority`: emergency pause, disable publishing
- `upgrade_authority`: program upgrade (should be separate or timelocked)

### 8.2 Safety controls
- Timelock for high-impact changes
- Multi-sig recommended for admin authority
- Optional emergency pause with constrained scope

### 8.3 Audit trail
All changes should emit events/logs:
- market created
- market activated/paused/deprecated
- schema version updated
- signer set rotated
- publish params changed

Indexer should capture these events.

---

## 9. On-chain Data Model (Conceptual)

This section describes recommended on-chain account structures. Exact layout is implementation-specific.

### 9.1 RegistryRoot
- protocol version
- admin authority pubkey
- guardian authority pubkey
- registry nonce
- signer set index pointer

### 9.2 MarketAccount
- market_id (bytes)
- domain (enum)
- state (enum)
- schema_type (enum)
- schema_version (u16)
- outcomes (vector)
- publish params (epoch window, cadence, reveal mode)
- signer policy ref (signer_set_id, min_sigs)
- created_at, updated_at timestamps

### 9.3 SignerSetAccount
- signer_set_id
- pubkeys[]
- min_signatures default
- created_at, updated_at
- sequence counter (if global)

### 9.4 Events
Emit structured logs for:
- MarketCreated
- MarketStateChanged
- MarketParamsUpdated
- MarketSchemaUpdated
- SignerSetRotated

---

## 10. SDK and API Access

### 10.1 Queries
SDKs and API should support:
- list markets (filters by domain/state)
- get market by id
- get outcomes and schema
- get current publish params
- get signer policy and public keys

### 10.2 Caching strategy
Services may cache registry state:
- in-memory LRU for hot markets
- Redis cache for distributed instances
- invalidation based on on-chain slot updates

### 10.3 Compatibility requirements
SDKs MUST:
- validate MarketId canonical rules
- expose schema type and outcomes
- expose publish parameters used for local verification

---

## 11. Example Market Definition (Debug JSON)

```json
{
  "market_id": "SPORTS_NBA_LAL_BOS_2026_01_04",
  "domain": "SPORTS",
  "state": "ACTIVE",
  "schema_type": "TRINARY",
  "schema_version": 1,
  "outcomes": [
    { "outcome_id": "HOME_WIN", "label": "Home", "description": "Home team wins", "is_active": true, "sort_key": 1 },
    { "outcome_id": "AWAY_WIN", "label": "Away", "description": "Away team wins", "is_active": true, "sort_key": 2 },
    { "outcome_id": "DRAW", "label": "Draw", "description": "Match ends in draw", "is_active": true, "sort_key": 3 }
  ],
  "publish_params": {
    "epoch_window_ms": 3600000,
    "publish_cadence_ms": 30000,
    "finalization_delay_ms": 120000,
    "max_commit_staleness_ms": 600000,
    "max_reveal_delay_ms": 600000,
    "reveal_mode": "FULL_BUNDLE",
    "max_reveal_bytes": 9000,
    "bundle_item_cap": 64
  },
  "signer_policy": {
    "signer_set_id": 1,
    "min_signatures": 2,
    "sig_scheme": "ED25519",
    "sequence_policy": "PER_MARKET"
  }
}
```

---

## 12. Implementation Notes

- Prefer fixed-size upper bounds for account size predictability.
- Use PDA derivation with MarketId hash to avoid storing long IDs in seeds.
- Store full MarketId in account data; derive PDA from `sha256(market_id)`.
- Consider migration tooling for schema and signer updates.
- Keep registry and oracle programs separate for upgrade safety.

---

## Links

- Website: https://m0club.com/
- X (Twitter): https://x.com/M0Clubonx
