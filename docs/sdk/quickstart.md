
# SDK Quickstart

This quickstart shows how to use the M0Club SDKs to:
- discover markets
- fetch latest oracle outputs
- verify bundle integrity (hash + signatures)
- subscribe to realtime updates (websocket)
- submit off-chain verification for auditing (optional)

This doc is written to be runnable with minimal changes.
Examples assume:
- you have access to an M0Club API endpoint
- you have the signer set public keys (from on-chain registry or published config)
- you have Node.js (for TypeScript) and/or Python installed

Links:
- Website: https://m0club.com/
- X (Twitter): https://x.com/M0Clubonx

---

## 0. Endpoints and Environment Variables

Set your endpoint(s):

- REST API: `M0_API_BASE`
- WebSocket: `M0_WS_URL`

Examples:
```bash
export M0_API_BASE="https://api.m0club.com"
export M0_WS_URL="wss://ws.m0club.com"
```

If you run locally:
```bash
export M0_API_BASE="http://127.0.0.1:8080"
export M0_WS_URL="ws://127.0.0.1:8090"
```

---

## 1. TypeScript SDK

### 1.1 Install
From your app folder:
```bash
npm init -y
npm install @m0club/sdk
```

If you use ESM:
```bash
npm pkg set type=module
```

### 1.2 Create a simple client

Create `index.ts`:
```ts
import { M0Client, verifyBundle } from "@m0club/sdk";

const apiBase = process.env.M0_API_BASE || "http://127.0.0.1:8080";

async function main() {
  const client = new M0Client({ apiBase });

  // 1) List markets
  const markets = await client.markets.list();
  console.log("Markets:", markets.slice(0, 5));

  // 2) Fetch latest output for a market
  const marketId = markets[0]?.market_id ?? "NBA_LAL_BOS";
  const latest = await client.markets.latest(marketId);
  console.log("Latest:", {
    market_id: latest.market_id,
    epoch_id: latest.epoch_id,
    tick_index: latest.tick_index,
    bundle_content_hash: latest.bundle_content_hash,
  });

  // 3) Verify bundle integrity
  // You need signer set pubkeys. Fetch from registry or config.
  const signerSet = await client.signers.getSignerSet(latest.signer_set_id);
  const ok = verifyBundle({
    bundle: latest.bundle,
    signerSetPubkeys: signerSet.pubkeys,
  });
  console.log("Bundle verified:", ok);

  // 4) Render probabilities
  for (const o of latest.bundle.markets[0].outcomes) {
    console.log(`${o.outcome_id}: p=${o.p_scaled} ci=[${o.ci_low_scaled},${o.ci_high_scaled}]`);
  }
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
```

Run:
```bash
npx tsx index.ts
```

Notes:
- `@m0club/sdk` is expected to ship a `verifyBundle` helper.
- If your SDK provides a different API, adapt the names, but keep the flow.

---

## 2. Python SDK

### 2.1 Install
```bash
python -m venv .venv
source .venv/bin/activate
pip install m0club-sdk
```

### 2.2 Fetch and verify latest output

Create `quickstart.py`:
```py
import os
from m0club_sdk import M0Client, verify_bundle

api_base = os.getenv("M0_API_BASE", "http://127.0.0.1:8080")

def main():
    client = M0Client(api_base=api_base)

    markets = client.markets_list()
    print("Markets:", markets[:5])

    market_id = markets[0]["market_id"] if markets else "NBA_LAL_BOS"
    latest = client.market_latest(market_id)

    print("Latest:", {
        "market_id": latest["market_id"],
        "epoch_id": latest["epoch_id"],
        "tick_index": latest["tick_index"],
        "bundle_content_hash": latest["bundle_content_hash"],
        "signer_set_id": latest["signer_set_id"],
    })

    signer_set = client.signer_set_get(latest["signer_set_id"])
    ok = verify_bundle(bundle=latest["bundle"], signer_set_pubkeys=signer_set["pubkeys"])
    print("Bundle verified:", ok)

    m = latest["bundle"]["markets"][0]
    for o in m["outcomes"]:
        print(f'{o["outcome_id"]}: p={o["p_scaled"]} ci=[{o["ci_low_scaled"]},{o["ci_high_scaled"]}]')

if __name__ == "__main__":
    main()
```

Run:
```bash
python quickstart.py
```

---

## 3. Bundle Verification Details

Bundle verification typically includes:
- canonical serialization
- bundle content hash computation (domain separated)
- signature message hash computation (includes signer_set_id, epoch_id, sequence)
- ed25519 signature verification against signer set pubkeys
- threshold check

If you need deeper details, see:
- `docs/engine-spec/bundle-hashing.md`
- `docs/protocol-spec/commit-reveal.md`
- `docs/protocol-spec/signer-set.md`
- `docs/protocol-spec/replay-protection.md`

---

## 4. WebSocket Subscriptions

### 4.1 TypeScript example
```ts
import { M0RealtimeClient } from "@m0club/sdk";

const wsUrl = process.env.M0_WS_URL || "ws://127.0.0.1:8090";

const rt = new M0RealtimeClient({ wsUrl });

rt.on("open", () => {
  rt.subscribe({ type: "market.latest", market_id: "NBA_LAL_BOS" });
});

rt.on("message", (msg) => {
  if (msg.type === "market.latest") {
    console.log("Update:", msg.data.epoch_id, msg.data.tick_index);
  }
});

rt.on("error", console.error);
rt.connect();
```

### 4.2 Python example
```py
import os
import json
import websocket

ws_url = os.getenv("M0_WS_URL", "ws://127.0.0.1:8090")

def on_open(ws):
    sub = {"type":"subscribe","channel":"market.latest","market_id":"NBA_LAL_BOS"}
    ws.send(json.dumps(sub))

def on_message(ws, message):
    print("Update:", message)

ws = websocket.WebSocketApp(ws_url, on_open=on_open, on_message=on_message)
ws.run_forever()
```

---

## 5. Localnet Quickstart (End-to-End)

If you are running the M0Club repo locally:

1) Start Solana local validator:
```bash
solana-test-validator --reset
```

2) Deploy programs:
```bash
cd programs
anchor build
anchor deploy
```

3) Start infra:
```bash
docker compose -f infrastructure/docker-compose.local.yml up -d
```

4) Start engine:
```bash
cd core-engine
cargo run -p m0-engine -- --profile local
```

5) Start API:
```bash
cd services/api-gateway
pnpm dev
```

6) Run SDK examples:
```bash
export M0_API_BASE="http://127.0.0.1:8080"
npx tsx index.ts
python quickstart.py
```

---

## 6. Troubleshooting

### 6.1 Bundle verification fails
- wrong signer set pubkeys
- wrong canonical hashing version
- corrupted bundle bytes in transport

Actions:
- fetch signer set from on-chain registry
- verify schema_version in bundle
- compare bundle_content_hash with expected hash

### 6.2 API returns stale outputs
- publishing paused
- submitter down
- guardrails blocking

Actions:
- check engine/submitter health
- check publish enabled flag
- check signer threshold and RPC errors

### 6.3 WebSocket connects but no updates
- market not publishing
- wrong channel name
- ws endpoint mismatch

Actions:
- verify market_id
- verify ws URL
- check realtime service logs

---

## 7. Reference

- Website: https://m0club.com/
- X (Twitter): https://x.com/M0Clubonx
