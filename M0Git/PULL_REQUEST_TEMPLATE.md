[PULL_REQUEST_TEMPLATE.md](https://github.com/user-attachments/files/24420694/PULL_REQUEST_TEMPLATE.md)
# Pull Request

> Please fill out this template as completely as possible.
> PRs that are easy to review get merged faster.

## Summary 

Describe what this PR changes and why.

## Type of Change

Select all that apply:

- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Performance improvement
- [ ] Refactor
- [ ] Documentation update
- [ ] Test improvement
- [ ] Build/CI change
- [ ] Security fix
- [ ] Operational / Infra change

## Scope / Area

Select all that apply:

- [ ] programs (Anchor / Solana)
- [ ] core-engine (M0-CORE)
- [ ] services (api-gateway / indexer / realtime / jobs)
- [ ] sdk (ts / rust / python)
- [ ] infra (docker / k8s / terraform)
- [ ] docs / tooling
- [ ] security / compliance
- [ ] observability / metrics

## Motivation / Context

Why is this change needed? What problem does it solve?

## Design / Approach

Explain the approach and any key decisions. Include tradeoffs where relevant.

## Changes

List the main changes in this PR:

- Change 1
- Change 2
- Change 3

## API / Contract Changes (if applicable)

- Public API changes:
- On-chain account changes / IDL changes:
- Config changes:
- Backward compatibility notes:

## Testing

Describe how you tested this change.

- [ ] Unit tests
- [ ] Integration tests
- [ ] E2E tests
- [ ] Localnet tests (Anchor / Solana)
- [ ] Load tests
- [ ] Fuzz tests
- [ ] Manual verification

### Test Commands / Output

```text
# Paste relevant commands and outputs
```

## Performance / Reliability (if applicable)

- Expected latency impact:
- Expected throughput impact:
- Memory/CPU impact:
- Failure modes considered:
- Rollback plan:

## Security Considerations

- [ ] No sensitive secrets are included
- [ ] Key management changes reviewed (if applicable)
- [ ] Input validation / replay protection considered (if applicable)
- [ ] Rate limiting / abuse prevention considered (if applicable)

### Threat Model Notes (optional)

```text
# Describe any adversarial or abuse scenarios considered
```

## Operational Notes (if applicable)

- Deployment steps:
- Migrations required:
- Feature flags:
- Monitoring/alerting updates:
- Runbook updates:

## Documentation

- [ ] README updated (if needed)
- [ ] Docs updated (if needed)
- [ ] Changelog entry added (if needed)

## Linked Issues

Link related issues (use keywords like: Fixes #123)

- Fixes #
- Related #

## Checklist

- [ ] I ran formatting/linting locally (fmt/clippy/eslint/ruff)
- [ ] I added/updated tests where appropriate
- [ ] I updated docs where appropriate
- [ ] I verified backward compatibility (or documented breaking changes)
- [ ] I verified CI is green
