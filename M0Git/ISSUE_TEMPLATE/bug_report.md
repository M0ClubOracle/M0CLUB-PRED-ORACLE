[bug_report.md](https://github.com/user-attachments/files/24420670/bug_report.md)
---
name: Bug report
about: Report a reproducible bug in M0Club
title: "[Bug]: "
labels: ["bug", "triage"]
assignees: []
---

> Please fill out this template as completely as possible.
> Incomplete reports may be closed if we cannot reproduce the issue.

## Summary

A clear and concise description of what the bug is.

## Impact

Describe the impact and severity:
- Who is affected (developers, operators, users)?
- What breaks (API, engine pipeline, indexer, on-chain program, SDK)?
- Is there data loss, security risk, or downtime?

## Environment

**Component**
- [ ] programs (Anchor / Solana)
- [ ] core-engine (M0-CORE)
- [ ] services (api-gateway / indexer / realtime / jobs)
- [ ] sdk (ts / rust / python)
- [ ] infra (docker / k8s / terraform)
- [ ] docs / tooling

**Runtime**
- OS: (e.g. Ubuntu 22.04, macOS 14, Windows 11)
- CPU/Arch: (e.g. x86_64, arm64)
- Rust: (run `rustc -V`)
- Cargo: (run `cargo -V`)
- Solana CLI: (run `solana --version`)
- Anchor: (run `anchor --version`)
- Node: (run `node -v`)
- npm/pnpm/yarn: (run `npm -v` etc.)
- Python: (run `python -V`)
- Docker: (run `docker --version`)
- Kubernetes (if relevant): (run `kubectl version --client`)

**Network / Cluster**
- Solana cluster: (localnet / devnet / testnet / mainnet)
- RPC endpoint: (redact credentials)
- If localnet: output of `solana-test-validator --version`

## Version / Commit

- M0Club version/tag: (e.g. v1.2.3)
- Git commit: (run `git rev-parse HEAD`)
- Branch: (e.g. main)
- If using Docker: image tag/digest

## Reproduction Steps

Provide exact steps to reproduce the behavior.

1. Step 1
2. Step 2
3. Step 3

### Minimal Repro (Preferred)

If possible, provide:
- a minimal config file snippet
- a minimal command sequence
- a minimal dataset fixture (or describe the input)

## Expected Behavior

What you expected to happen.

## Actual Behavior

What actually happened.

## Logs / Output

Paste relevant logs. If large, attach a file.

### Engine logs
<details>
<summary>engine logs</summary>

```text
PASTE LOGS HERE
```

</details>

### Program logs (Anchor / Solana)
<details>
<summary>program logs</summary>

```text
PASTE LOGS HERE
```

</details>

### Service logs (api-gateway / indexer / realtime / jobs)
<details>
<summary>service logs</summary>

```text
PASTE LOGS HERE
```

</details>

### SDK logs (ts / rust / python)
<details>
<summary>sdk logs</summary>

```text
PASTE LOGS HERE
```

</details>

## Screenshots / Videos

If applicable, add screenshots or a short video.

## On-chain Details (if relevant)

- Transaction signature(s):
- Program ID(s):
- Accounts involved (redact secrets):
- IDL version:
- Slot / block time (if known):

## Data Integrity / Consistency (if relevant)

- Is the issue deterministic or intermittent?
- Approximate frequency:
- Any evidence of missing events, duplicate events, out-of-order events, or reorg-related behavior?

## Security Considerations

- [ ] I confirm this is **NOT** a security vulnerability disclosure.
- [ ] I suspect this may be security-related and will report privately instead.

If you suspect a vulnerability, **do not** file a public issue.
Follow `SECURITY.md` for responsible disclosure.

## Additional Context

Add any other context that may help:
- recent changes
- config differences
- traffic/load patterns
- related issues/PRs
- links to dashboards/metrics (redact secrets)

## Checklist

- [ ] I searched existing issues and did not find a duplicate
- [ ] I can reproduce this on the latest main (or I provided the exact commit/tag)
- [ ] I included logs and environment details
- [ ] I provided minimal reproduction steps
