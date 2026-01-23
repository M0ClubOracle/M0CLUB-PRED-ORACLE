
# M0CORE On-Chain Programs

This folder contains Anchor-based Solana programs used by M0Club.

Programs:
- m0-oracle     Commit-reveal oracle output publishing and on-chain replay protection
- m0-registry   Market registry and metadata publication
- m0-fee-router Fee routing configuration (token routing skeleton)
- m0-governance Minimal governance + timelock skeleton

Notes:
- Program IDs in Anchor.toml are placeholders. Replace them after `anchor keys list`.
- Some verification logic is intentionally minimal in this skeleton and must be hardened for production.
