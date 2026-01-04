
# Commit-Reveal Publishing (Protocol Spec)

This document specifies the commit-reveal protocol used by M0Club to publish oracle outputs on Solana.

Objectives:
- Prevent copying/front-running of full oracle payloads before publication finality.
- Provide deterministic verifiability across engine, services, SDKs, and on-chain programs.
- Support replay protection, signer set policies, and optional Merkle proof reveals.
- Define account model, instruction flows, and validation rules.

This spec is implementation-oriented and intended to map directly to Anchor program logic.

---

## 1. Overview

The commit-reveal protocol publishes oracle outputs in two phases:

1) **Commit**
- Publishes a commitment (`bundle_hash` or `merkle_root`) to an oracle bundle.
- Minimal on-chain data and stable identifiers.

2) **Reveal**
- Publishes the bundle bytes or a proof/chunk that matches the commitment.
- Includes signature metadata proving authorization.

3) **Finalize** (optional but recommended)
- Records the canonical revealed output for an epoch.
- Establishes the authoritative result for consumers.

---

## 2. Inputs and Outputs

### 2.1 Inputs
- `market_id`
- `epoch_id`
- `bundle_hash` (32 bytes)
- `signer_set_id`
- `sequence` (replay context)
- `signatures[]` (ed25519 over derived message)
- `reveal_payload` (bundle bytes or merkle proof)

### 2.2 Outputs
On-chain state:
- Commitment record for `(market_id, epoch_id)`
- Reveal record keyed by `(market_id, epoch_id, bundle_hash)`
- Optional finalized record for `(market_id, epoch_id)`

Indexer and services:
- Stream commit/reveal/finalize events for consumption
- Reconcile expected vs published updates

---

## 3. Roles and Permissions

### 3.1 Authorities
Recommended roles:
- Registry admin: creates markets and updates parameters
- Guardian: can pause markets and trigger emergency actions
- Program upgrade authority: controlled separately (timelock/multisig)

### 3.2 Signers
Oracle outputs are authenticated by a signer set.
On-chain program verifies that:
- signatures are valid
- pubkeys belong to the authorized signer set
- minimum signature threshold is met

---

## 4. Identifiers and PDAs

### 4.1 Market PDA derivation
Market accounts should be derived from a stable hash of MarketId rather than raw string seeds.

Recommended:
- `market_seed = sha256(market_id_bytes)` (32 bytes)
- `MarketPDA = PDA("market", market_seed)`

### 4.2 Epoch PDA derivation
Epoch is uniquely defined by `(market, epoch_id)`.

Recommended:
- `EpochPDA = PDA("epoch", market_pda, epoch_id_le_bytes)`

### 4.3 Commitment PDA derivation
Commitment is unique per epoch, with optional multiple commits allowed by sequence policy.

Option A (single active commitment per epoch):
- `CommitPDA = PDA("commit", epoch_pda)`

Option B (multiple commits per epoch by sequence):
- `CommitPDA = PDA("commit", epoch_pda, sequence_le_bytes)`

Recommended default:
- Option A for simplicity unless multi-commit is required.

### 4.4 Reveal PDA derivation
Reveal should be idempotent for a given commitment.

Recommended:
- `RevealPDA = PDA("reveal", epoch_pda, bundle_hash)`

This allows:
- safe retries
- unique reveal per commitment hash

### 4.5 Finalization PDA derivation
Finalized state is unique per epoch.

Recommended:
- `FinalPDA = PDA("final", epoch_pda)`

---

## 5. Account Model (Conceptual)

Exact layouts are implementation-specific. This section defines conceptual fields.

### 5.1 MarketAccount
- `market_id` (bytes)
- `state` (ACTIVE/PAUSED/etc.)
- `schema_version`
- `publish_params` (epoch_window_ms, cadence, reveal_mode, caps)
- `signer_policy` (signer_set_id, min_sigs)
- `authority` and `guardian` references

### 5.2 EpochAccount
- `market` reference
- `epoch_id`
- `window_start_ms`
- `window_end_ms`
- `status` (OPEN, REVEALED, FINALIZED)
- `last_commit_slot` (optional)
- `created_at_ms` (optional)

### 5.3 CommitmentAccount
- `market` reference
- `epoch_id`
- `bundle_hash`
- `signer_set_id`
- `sequence`
- `committer` pubkey (submitter)
- `commit_slot`
- `commit_time_ms` (optional)
- `status` (COMMITTED, REVEALED, INVALIDATED)

### 5.4 RevealAccount
- `market` reference
- `epoch_id`
- `bundle_hash`
- `reveal_mode`
- `payload_hash` (optional: hash of payload bytes)
- `payload_len`
- `signatures[]` metadata (or hash of signatures)
- `reveal_slot`
- `reveal_time_ms` (optional)

Storage guidance:
- Keep on-chain payload minimal.
- Prefer storing:
  - full payload only when small
  - otherwise store merkle root and verify proofs
  - store only hashes if payload too large and rely on off-chain availability (depending on protocol choice)

### 5.5 FinalizationAccount
- `market` reference
- `epoch_id`
- `final_bundle_hash`
- `final_reveal_pda`
- `finalized_slot`
- `finalized_time_ms` (optional)

---

## 6. Instruction Flow

### 6.1 Commit instruction: `commit_oracle_update`

Inputs:
- `market_id` (or Market PDA)
- `epoch_id`
- `bundle_hash`
- `signer_set_id`
- `sequence`

Accounts:
- market account (registry)
- epoch account (init if needed)
- commitment account (init or update)
- payer/committer
- system program

Checks:
1) Market is ACTIVE.
2) Epoch bounds are valid for current time with allowed skew.
3) Commit is not stale:
   - `now_ms - window_end_ms <= max_commit_staleness_ms`
4) Replay protection:
   - `(signer_set_id, sequence)` not already used for this market/epoch policy.
5) Commitment uniqueness policy:
   - if single commit per epoch, reject if already committed (unless overwrite allowed by policy).
6) Write commitment fields and emit `CommitEvent`.

Notes:
- Commit does not include payload or signatures.
- Commit can include a short memo or metadata if needed.

### 6.2 Reveal instruction: `reveal_oracle_update`

Inputs:
- `bundle_hash`
- `signer_set_id`
- `sequence`
- `reveal_payload` (bytes) OR `(merkle_root, proof, leaf)` depending on mode
- `signatures[]`

Accounts:
- market account
- epoch account
- commitment account (must exist)
- reveal account (init if needed)
- system program
- ed25519 verification instruction sysvar (if using native verification)

Checks:
1) Commitment exists and matches `(market, epoch, bundle_hash, signer_set_id, sequence)`.
2) Reveal timing constraints:
   - within `max_reveal_delay_ms` relative to commit time (or within window).
3) Payload validity:
   - if `FULL_BUNDLE`: payload bytes decode and canonical hash == bundle_hash.
   - if `MERKLE_PROOF`: proof verifies membership; root matches commitment.
   - enforce `max_reveal_bytes` and `bundle_item_cap`.
4) Signature checks:
   - signature count >= min_signatures
   - each signature valid over required message (bundle_hash + replay context)
   - signer pubkeys are authorized in signer set
   - no duplicate signer pubkeys counted twice
5) Write reveal fields and emit `RevealEvent`.
6) Mark commitment status REVEALED and epoch status REVEALED when appropriate.

Notes:
- On Solana, signature verification is typically performed via the ed25519 program instruction preflight.
- Anchor can validate that an ed25519 instruction was included with expected message/pubkey/signature.

### 6.3 Finalize instruction: `finalize_epoch`

Inputs:
- `epoch_id` and reference to reveal
- optionally the final bundle hash to assert equality

Accounts:
- market account
- epoch account
- reveal account (must exist)
- finalization account (init if needed)
- authority (optional requirement)

Checks:
1) Epoch is REVEALED (or commitment + reveal exist).
2) Finalization delay satisfied:
   - `now_ms >= window_end_ms + finalization_delay_ms`
3) Reveal matches commitment and signatures threshold (already validated in reveal).
4) Write finalization fields and emit `FinalizeEvent`.
5) Mark epoch status FINALIZED.

Notes:
- Some markets may allow immediate finalization after reveal.
- Governance can require authority signature for finalization, or allow permissionless finalization.

---

## 7. Signature Message

All signers sign a derived message that binds:
- bundle_hash
- signer_set_id
- sequence
- optional market_id and epoch_id context

Recommended message:
`msg = sha256("M0CLUB_ORACLE_V1" || market_id_bytes || epoch_id_bytes || bundle_hash || signer_set_id_le || sequence_le)`

Requirements:
- Domain separation prefix MUST be included.
- market_id and epoch_id SHOULD be included to prevent cross-market replay.
- signer_set_id and sequence MUST be included for replay protection.

---

## 8. Replay Protection

Replay protection ensures an attacker cannot reuse old signatures or payloads.

Recommended mechanisms:
- Maintain a sequence counter per signer set (global) or per market.
- Include `(signer_set_id, sequence)` in signature message.
- On-chain program stores last used sequence or a bitmap/window.

Policies:
- `GLOBAL`: one global monotonic sequence per signer set.
- `PER_MARKET`: monotonic per (signer_set_id, market).
- `PER_EPOCH`: monotonic per (signer_set_id, market, epoch) (weakest).

Recommended default:
- GLOBAL or PER_MARKET.

---

## 9. Reveal Modes

### 9.1 FULL_BUNDLE
Reveal payload is the full canonical bundle bytes.
Validation:
- Deserialize bundle
- Ensure canonical re-serialization bytes match exactly (optional)
- Compute hash and compare to commitment

Pros:
- Simple verification
Cons:
- Payload size limited

### 9.2 MERKLE_PROOF
Commitment is a Merkle root.
Reveal payload includes:
- leaf bytes for one item (or chunk)
- merkle proof path

Validation:
- Verify proof and root matches commitment
- Optionally verify chunking scheme

Pros:
- Scales to large bundles
Cons:
- Complexity, proof format standardization needed

---

## 10. Events (Indexer)

Emit structured events for:
- CommitEvent
  - market_id, epoch_id, bundle_hash, signer_set_id, sequence, slot
- RevealEvent
  - market_id, epoch_id, bundle_hash, payload_len, slot
- FinalizeEvent
  - market_id, epoch_id, final_bundle_hash, slot

Indexer stores:
- raw events
- derived status tables
- reconciliation state

---

## 11. Failure Modes and Handling

### 11.1 Commit succeeded, reveal missing
- jobs service detects missing reveal after max delay
- engine resubmits reveal with same bundle_hash
- if permanently missing, market can be paused or rolled forward depending on policy

### 11.2 Reveal invalid (hash mismatch)
- reject reveal
- engine must produce a new bundle and commit again (new hash)

### 11.3 Signature threshold not met
- reject reveal
- engine must re-collect signatures or rotate signer set

### 11.4 Stale commits
- reject commit
- engine should publish for current epoch only

---

## 12. Localnet Test Plan (Suggested)

To validate correctness end-to-end:
1) Create a market in registry and set ACTIVE.
2) Compute current epoch bounds.
3) Produce a deterministic test bundle with fixed probabilities.
4) Compute bundle_hash.
5) Sign bundle_hash with test keys.
6) Submit commit.
7) Submit reveal with payload.
8) Finalize epoch.
9) Verify on-chain state and indexer outputs.
10) Verify SDK can fetch and validate signature/hash locally.

---

## Links

- Website: https://m0club.com/
- X (Twitter): https://x.com/M0Clubonx
