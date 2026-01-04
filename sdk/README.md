
# M0Club SDK

This directory contains:
- `types/` shared type definitions (JSON Schema + TS/Rust/Python)
- `ts/` TypeScript SDK
- `rust/` Rust SDK
- `python/` Python SDK

Quick local demo (requires services/api-gateway running):

TypeScript:
```bash
cd sdk/ts
npm install
npm run build
node dist/examples/basic.js
```

Rust:
```bash
cd sdk/rust
cargo test -q
```

Python:
```bash
cd sdk/python
python -m pip install -e .
python -m m0club.examples.basic
```
