
# Dispute Resolution and Corrections (Protocol Spec)

This document specifies dispute resolution and correction mechanisms for M0Club oracle outputs.
Because M0Club publishes probabilistic analytics rather than binary resolutions, disputes are typically about integrity, correctness, and policy compliance rather than a single “true outcome.”

This spec defines:
- What can be disputed
- Roles and permissions
- Dispute lifecycle and on-chain flows
- Correction policies (retractions, superseding epochs, patch bundles)
- Evidence requirements and auditability
- Operational safeguards and limits

---

## 1. Goals

- Provide a structured way to challenge and correct published oracle outputs.
- Ensure corrections are transparent, verifiable, and auditable on-chain.
- Minimize disruption to downstream consumers while preserving safety.
- Provide emergency controls for compromised signer sets or corrupted pipelines.

Non-goals:
- Providing a full legal arbitration system.
- Guaranteeing the correctness of external sources beyond documented policies.
- Resolving prediction-market payouts (handled by consumer protocols).

---

## 2. What Can Be Disputed

Disputes are allowed for:

### 2.1 Integrity disputes
- A reveal payload does not match the committed bundle hash.
- Signatures are invalid or below threshold.
- Replay protection violated (sequence reuse).
- Signer set mismatch or unauthorized signer used.

### 2.2 Availability disputes
- Commit exists but reveal never arrives within required deadlines.
- Finalization does not occur within policy bounds.

### 2.3 Policy compliance disputes
- Market was paused but commits/reveals still occurred.
- Publish cadence or epoch rules violated.
- Payload size or schema rules violated.
- Registry parameters not respected.

### 2.4 Data quality disclosures (non-binding disputes)
- Divergent sources.
- Missing coverage.
- Suspected manipulation flags.
These are typically handled as quality flags and risk scoring rather than hard disputes, but can trigger investigation.

---

## 3. Roles and Authorities

### 3.1 Actors
- **Reporter**: anyone opening a dispute.
- **Guardian**: emergency authority capable of pausing markets and triggering emergency corrections.
- **Registry Admin**: authority managing market parameters and signer sets.
- **Dispute Committee** (optional): a governance-controlled role set for dispute evaluation.

### 3.2 Permissions
- Anyone can submit a dispute report account (permissionless).
- Only guardian/admin/committee can:
  - mark a dispute as accepted/rejected on-chain
  - trigger retractions
  - rotate signer sets (through registry governance)
  - finalize a correction status

This split allows public reporting with controlled action execution.

---

## 4. Dispute Lifecycle

### 4.1 States
Recommended dispute state machine:
- `OPEN`
- `UNDER_REVIEW`
- `ACCEPTED`
- `REJECTED`
- `RESOLVED`

### 4.2 Timeouts
Registry should define:
- `dispute_window_ms`: how long disputes can be opened after an epoch is finalized
- `review_timeout_ms`: how long an open dispute can remain unreviewed before escalation

Suggested defaults:
- dispute_window_ms: 24h to 7d depending on market domain
- review_timeout_ms: 6h to 24h

---

## 5. On-chain Data Model (Conceptual)

This spec defines conceptual accounts. Actual implementation may differ.

### 5.1 DisputeAccount
Fields:
- `market_id`
- `epoch_id`
- `bundle_hash` (target)
- `reason_code` (enum)
- `reporter` pubkey
- `opened_slot`
- `opened_time_ms` (optional)
- `state` (enum)
- `evidence_hash` (32 bytes)
- `notes_uri` (optional)
- `resolution` (optional struct)
- `bond_lamports` (optional anti-spam)

### 5.2 Resolution struct
- `resolver` pubkey
- `resolved_slot`
- `decision` (ACCEPTED/REJECTED)
- `action` (NONE/RETRACT/SUPERSEDE/PATCH)
- `action_ref` (pubkey or hash)
- `message` (short ASCII)

### 5.3 CorrectionRecordAccount (optional)
Records correction actions:
- `market_id`
- `epoch_id`
- `target_bundle_hash`
- `correction_type`
- `new_bundle_hash` (if superseding/patch)
- `created_slot`
- `created_by` pubkey

---

## 6. Dispute Reasons (Reason Codes)

Suggested enum:
- `INTEGRITY_HASH_MISMATCH`
- `SIGNATURE_INVALID`
- `SIGNATURE_THRESHOLD_NOT_MET`
- `REPLAY_VIOLATION`
- `SIGNER_SET_MISMATCH`
- `REVEAL_MISSING`
- `FINALIZATION_DELAYED`
- `MARKET_PAUSED_VIOLATION`
- `SCHEMA_VIOLATION`
- `PARAMETER_VIOLATION`
- `OTHER`

---

## 7. Evidence Requirements

Disputes should include evidence that can be verified.

### 7.1 Evidence hash
DisputeAccount stores:
- `evidence_hash = sha256(evidence_bytes)`

Evidence may include:
- serialized reveal payload bytes
- bundle hash recomputation proof
- signature verification logs
- indexer snapshots (commit/reveal tx signatures)
- registry state at publication time
- external source pointers (for data-quality investigation)

### 7.2 Notes URI (optional)
If evidence is large, include:
- `notes_uri` pointing to an immutable storage location
- e.g., IPFS/Arweave or a signed HTTPS artifact

On-chain stores only the hash for verifiability.

---

## 8. Correction Policies

M0Club supports three correction classes.

### 8.1 Retraction (invalidate without replacement)
Used when:
- signer set compromised
- payload invalid or malicious
- integrity failure confirmed

Action:
- mark epoch output as retracted
- consumers must treat retracted outputs as invalid
- downstream protocols decide how to react

On-chain representation:
- `FinalizationAccount` gains a `status = RETRACTED`
- `CorrectionRecord` references the retraction

Constraints:
- Retraction should be rare and gated by guardian/admin.
- Retractions should emit high-severity events.

### 8.2 Supersede (replace with a new finalized output)
Used when:
- payload was correctable (e.g., wrong bundle published due to pipeline bug)
- integrity preserved but content needs correction under policy

Action:
- publish a new bundle with a new hash for the same epoch
- finalize as `SUPERSEDED_BY = new_hash`
- old hash remains auditable but not canonical

On-chain representation:
- store `canonical_bundle_hash` and optional `prior_bundle_hash`
- store a `supersedes` chain pointer to preserve history

Constraints:
- must preserve deterministic epoch identity
- must include an explicit correction reason and evidence reference
- must require threshold signatures under an active signer set

### 8.3 Patch (partial correction via patch bundle)
Used when:
- only a subset of items in a multi-market bundle is wrong
- Merkle mode or chunking allows targeted corrections

Action:
- publish a patch bundle referencing target hash and patch type
- on-chain records patch relationship
- consumers apply patch rules deterministically

Patch rules must be explicitly defined:
- which fields can change (e.g., p_scaled, ci bounds, risk_score)
- whether patch replaces or adjusts values

Recommended v1:
- Avoid patch complexity unless necessary.
- Prefer superseding the full epoch output.

---

## 9. Dispute and Correction Instructions (Conceptual)

### 9.1 `open_dispute`
Permissionless.
Creates a DisputeAccount.

Inputs:
- market_id, epoch_id, bundle_hash
- reason_code
- evidence_hash
- optional notes_uri
- optional bond

Checks:
- epoch exists and is within dispute window
- dispute for same target does not already exist (or allow multiple reporters)

### 9.2 `set_dispute_state`
Guardian/admin/committee only.
Transitions dispute to UNDER_REVIEW/ACCEPTED/REJECTED.

### 9.3 `apply_retraction`
Guardian/admin only.
Marks epoch output as RETRACTED and emits events.

### 9.4 `apply_supersede`
Admin/committee + signature threshold required.
Publishes a new commit/reveal (new hash) and finalizes it as canonical,
then links old output as superseded.

### 9.5 `close_dispute`
Marks dispute as RESOLVED and records final resolution metadata.

---

## 10. Consumer Guidance

Consumers should implement:
- Fetch canonical output for (market, epoch).
- Detect and handle status:
  - FINALIZED
  - SUPERSEDED
  - RETRACTED
- Subscribe to correction events via indexer/realtime.
- Cache invalidation rules:
  - invalidate caches on supersede or retraction
- Prefer outputs with:
  - valid signatures
  - no integrity flags
  - no retraction status

---

## 11. Anti-Spam Considerations

Disputes can be spammed if permissionless.

Mitigations:
- Optional bond (refundable if accepted, slashed if rejected)
- Rate limiting at service layer for dispute submission tools
- Allow only one open dispute per (market, epoch, bundle_hash) on-chain

Recommended v1:
- Provide bond option but allow zero-bond in early phases.
- Services should rate limit and require signed requests for public endpoints.

---

## 12. Incident Response Integration

When severe integrity failures occur:
1) Guardian pauses affected markets.
2) Rotate signer set if compromise suspected.
3) Publish incident report with evidence hash.
4) Decide on retraction or supersede action.
5) Resume markets with updated policies and monitoring.

Production operations should define:
- escalation paths
- runbooks
- alert thresholds (missing reveal, replay violation, signature failures)

---

## 13. Events (Indexer)

Emit and index:
- DisputeOpened
- DisputeStateChanged
- OutputRetracted
- OutputSuperseded
- OutputPatched (if supported)
- DisputeResolved

Events should include:
- market_id, epoch_id, target_bundle_hash
- action refs
- resolver identity (pubkey)
- evidence_hash

---

## 14. Test Plan Guidance

Localnet tests should include:
- opening a dispute within window
- rejecting invalid evidence formats
- enforcing permission checks on state transitions
- applying retraction and verifying canonical output status
- applying supersede and verifying that consumers see the new canonical hash
- verifying event emission and indexer ingestion

---

## Links

- Website: https://m0club.com/
- X (Twitter): https://x.com/M0Clubonx
