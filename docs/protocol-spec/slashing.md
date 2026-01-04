
# Slashing and Accountability (Protocol Spec)

This document specifies slashing and accountability mechanisms for M0Club.
Slashing is an optional module that penalizes misbehavior (integrity violations, missed reveals, replay violations, etc.) by reducing stake bonded by signer operators or service operators.

Because M0Club is an oracle analytics publisher, slashing is primarily an accountability mechanism for:
- signer operators (who attest to bundle hashes)
- submitter operators (who commit/reveal on time)
- optionally data providers (if they post bonds in a registry)

This spec is designed to be compatible with Solana account constraints and can be implemented as a dedicated program (recommended) or integrated into registry/oracle programs.

---

## 1. Goals

- Provide economic accountability for signer sets and operators.
- Penalize provable protocol violations.
- Incentivize liveness (commit/reveal within deadlines).
- Make penalty actions auditable and governed.

Non-goals:
- Guaranteeing correctness of real-world events.
- Enforcing subjective "prediction quality" beyond defined integrity and policy rules.

---

## 2. Concepts

### 2.1 Bond / Stake
A bond is a token or SOL deposit locked as collateral by an operator.

Bond types:
- **Signer bond**: posted by each signer operator.
- **Submitter bond**: posted by the publishing operator.
- **Market bond** (optional): posted by a market creator or data provider.

### 2.2 Slashable events
A slashable event is an on-chain provable violation that triggers a penalty.

### 2.3 Slashing module
A program that:
- manages bond vaults
- accepts evidence submissions
- executes penalties via governance/committee
- routes slashed funds according to policy

---

## 3. Slashable Offenses

This section defines offenses that can be objectively proven.

### 3.1 Integrity offenses
- **Invalid signature**: a signer attests but signature does not verify.
  - Note: on-chain verification should prevent acceptance; offense applies when evidence shows signer attempted to sign invalid messages or signed conflicting messages.

- **Double-signing**: signer signs two different bundle_hash values for the same (market_id, epoch_id, sequence) context.

- **Replay violation**: signer signs with reused sequence in violation of policy (GLOBAL/PER_MARKET).

### 3.2 Liveness offenses
- **Commit without reveal**: commit is published but reveal does not arrive within `max_reveal_delay_ms`.
- **Repeated missed epochs**: systematic liveness failures beyond tolerated thresholds.

### 3.3 Policy offenses
- **Publishing while paused**: attempt to publish for a paused market.
- **Schema violation**: attempt to publish payloads violating schema rules (detected by validation layer).

### 3.4 Non-slashable (informational)
- poor predictive accuracy
- domain source outages beyond operator control (unless policy includes negligence rules)
- model drift (unless explicitly tied to integrity violations)

---

## 4. Evidence and Proofs

Slashing MUST be evidence-driven.

### 4.1 Evidence types
- Signed messages (pubkey, msg hash, signature bytes)
- Commit transactions and reveal transactions (tx signatures)
- On-chain accounts snapshots (commitment, reveal, finalization)
- Registry config at the time of event
- Indexer proofs (optional supplemental)

### 4.2 Evidence hash and storage
On-chain slashing accounts should store:
- `evidence_hash = sha256(evidence_bytes)`
- optional `evidence_uri` for large evidence blobs (IPFS/Arweave)

### 4.3 Verifiability
Any slashing action must be reproducible:
- observers can recompute hashes
- observers can verify signatures
- observers can verify conflicting messages for double-signing

---

## 5. Bonding Model

### 5.1 Bond assets
Bonds may be:
- SOL
- SPL token (e.g., a governance token)
- a stable SPL token

v1 recommendation:
- support SOL bonds first for simplicity

### 5.2 Bond vaults
Bonds are held in vault accounts controlled by the slashing program.
Vault derivation:
- `BondVaultPDA = PDA("bond_vault", operator_pubkey, bond_asset_mint)`

### 5.3 Bond states
- `LOCKED`
- `UNLOCKING` (timelock)
- `UNLOCKED`
- `SLASHED` (partially or fully)

Unbonding should include a delay to allow disputes/slashing to be applied.

---

## 6. Slashing Parameters

Each signer set and/or market can define:
- `slash_bps` per offense type
- `max_slash_amount`
- `liveness_grace_count` (allowed missed reveals per window)
- `unbonding_period_ms`
- `reward_bps_to_reporter` (optional)

Parameters should be stored in registry or slashing config accounts.

---

## 7. Slashing Workflow

### 7.1 Reporting (permissionless)
Anyone can open a slashing case by submitting evidence.

Instruction: `open_slash_case`
- inputs: offender pubkey, offense type, market_id, epoch_id, evidence_hash, optional uri
- outputs: SlashCase account in `OPEN` state
- optional bond required to reduce spam

### 7.2 Review (committee/governance)
A designated role transitions case to `UNDER_REVIEW` and then `ACCEPTED` or `REJECTED`.

Instruction: `set_slash_case_state`
- requires committee/admin

### 7.3 Execution (committee/governance)
When accepted:
- compute penalty based on config
- transfer funds from bond vault to penalty destinations

Instruction: `execute_slash`
- requires committee/admin
- writes immutable record of executed amount and destinations

### 7.4 Appeals (optional)
A separate dispute workflow can allow appeals:
- open appeal within `appeal_window_ms`
- governance decision final

v1 recommendation:
- keep appeals off-chain or as simple dispute accounts until needed

---

## 8. On-chain Data Model (Conceptual)

### 8.1 OperatorBondAccount
- operator pubkey
- bond asset mint (or SOL marker)
- amount
- state
- unbonding_start_time (optional)
- last_updated_slot

### 8.2 SlashCaseAccount
- case_id
- offender pubkey
- offense type enum
- market_id, epoch_id
- target bundle_hash (optional)
- evidence_hash
- evidence_uri (optional)
- reporter pubkey
- opened_slot
- state
- resolution metadata

### 8.3 SlashExecutionRecord
- case_id
- executed_by pubkey
- amount_slashed
- destinations[] (treasury, insurance, reporter reward)
- executed_slot

---

## 9. Destinations and Routing

Slashed funds can be routed to:
- protocol treasury
- insurance fund
- reporter reward
- buyback/burn vault (optional)
- affected consumer compensation vault (advanced)

Recommended v1 routing:
- 70% treasury
- 20% insurance
- 10% reporter reward (optional)

These percentages should be configurable and governed.

---

## 10. Offense Definitions and Penalty Suggestions

This section provides default penalty suggestions (tunable by governance):

- Double-signing: 5000 bps (50%) up to max cap
- Replay violation: 2000 bps (20%)
- Missed reveal: 500 bps (5%) per occurrence, escalates after grace count
- Publishing while paused (attempt): 1000 bps (10%)
- Schema violation (attempt): 500 bps (5%)

Notes:
- Attempts may be penalized only if provable and malicious.
- Repeated liveness failures should be penalized progressively.

---

## 11. Integration with Commit-Reveal

Slashing should integrate with commit-reveal indexing:
- indexer detects missed reveal: commit exists, reveal missing after max delay
- indexer opens a slashing report (off-chain) or publishes an on-chain case (if permissioned)
- evidence includes commit tx signature and registry params

For double-signing:
- evidence includes two signatures by same pubkey over different bundle_hash for same context

---

## 12. Safety Controls

- Only committee/admin can execute slashes.
- Timelock can be applied to execution to allow oversight.
- Slashing module should emit events for transparency.
- Bond withdrawal requires unbonding delay.

---

## 13. Implementation Guidance

- Prefer a dedicated slashing program to isolate logic and upgrades.
- Keep on-chain evidence minimal (hash + pointers).
- Provide SDK helpers to build evidence bundles and compute evidence_hash.
- Provide localnet tests:
  - bond deposit/withdraw with unbonding
  - open slash case
  - accept and execute slash
  - verify vault balances and routing
- Ensure re-entrancy and CPI safety in transfers.

---

## 14. Test Plan Guidance

### 14.1 Unit tests
- bond vault PDA derivation
- offense enum mapping and penalty calc
- state transitions and permission checks

### 14.2 Integration tests (localnet)
- deposit bond
- open case with evidence
- accept case
- execute slash
- verify balances and events
- attempt unauthorized execution (should fail)

---

## Links

- Website: https://m0club.com/
- X (Twitter): https://x.com/M0Clubonx
