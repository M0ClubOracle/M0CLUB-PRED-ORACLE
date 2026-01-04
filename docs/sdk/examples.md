
# SDK Examples

This document provides runnable example code for the M0Club SDKs.
It includes:
- REST API usage
- bundle verification
- realtime subscriptions
- on-chain fetch + verification (Solana RPC)
- auditing and persistence examples

Examples are written in:
- TypeScript (Node.js 18+)
- Python (3.10+)

Assumptions:
- You have an M0Club API endpoint (or local dev API)
- You can obtain signer sets (from the API or on-chain registry)
- You have Node.js and Python installed

Links:
- Website: https://m0club.com/
- X (Twitter): https://x.com/M0Clubonx

---

## 0. Common Setup

Set environment variables:
```bash
export M0_API_BASE="http://127.0.0.1:8080"
export M0_WS_URL="ws://127.0.0.1:8090"
export SOLANA_RPC_URL="http://127.0.0.1:8899"
```

If using production:
```bash
export M0_API_BASE="https://api.m0club.com"
export M0_WS_URL="wss://ws.m0club.com"
export SOLANA_RPC_URL="https://api.mainnet-beta.solana.com"
```

---

## 1. TypeScript Examples

### 1.1 Project setup
```bash
mkdir m0-sdk-examples && cd m0-sdk-examples
npm init -y
npm install @m0club/sdk @solana/web3.js
npm install -D tsx typescript
```

Create `tsconfig.json`:
```json
{
  "compilerOptions": {
    "target": "ES2022",
    "module": "ESNext",
    "moduleResolution": "Bundler",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true
  }
}
```

---

### 1.2 List markets (REST)
Create `01_list_markets.ts`:
```ts
import { M0Client } from "@m0club/sdk";

const apiBase = process.env.M0_API_BASE || "http://127.0.0.1:8080";

async function main() {
  const client = new M0Client({ apiBase });
  const markets = await client.markets.list();

  console.log("Market count:", markets.length);
  for (const m of markets.slice(0, 20)) {
    console.log("-", m.market_id, m.domain, m.tier_policy, "cadence_ms=", m.cadence_ms);
  }
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
```

Run:
```bash
npx tsx 01_list_markets.ts
```

---

### 1.3 Fetch latest market output and verify bundle
Create `02_latest_and_verify.ts`:
```ts
import { M0Client, verifyBundle } from "@m0club/sdk";

const apiBase = process.env.M0_API_BASE || "http://127.0.0.1:8080";

async function main() {
  const client = new M0Client({ apiBase });

  const marketId = process.argv[2] || "NBA_LAL_BOS";
  const latest = await client.markets.latest(marketId);

  console.log("Latest:", {
    market_id: latest.market_id,
    epoch_id: latest.epoch_id,
    tick_index: latest.tick_index,
    signer_set_id: latest.signer_set_id,
    bundle_content_hash: latest.bundle_content_hash,
  });

  const signerSet = await client.signers.getSignerSet(latest.signer_set_id);

  const ok = verifyBundle({
    bundle: latest.bundle,
    signerSetPubkeys: signerSet.pubkeys,
  });

  console.log("Verified:", ok);

  const m = latest.bundle.markets.find((x) => x.market_id === marketId) ?? latest.bundle.markets[0];
  console.log("Outcomes for", m.market_id);
  for (const o of m.outcomes) {
    console.log(`- ${o.outcome_id} p=${o.p_scaled} ci=[${o.ci_low_scaled},${o.ci_high_scaled}]`);
  }
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
```

Run:
```bash
npx tsx 02_latest_and_verify.ts NBA_LAL_BOS
```

---

### 1.4 Subscribe to realtime updates (WebSocket)
Create `03_realtime_subscribe.ts`:
```ts
import { M0RealtimeClient } from "@m0club/sdk";

const wsUrl = process.env.M0_WS_URL || "ws://127.0.0.1:8090";
const marketId = process.argv[2] || "NBA_LAL_BOS";

async function main() {
  const rt = new M0RealtimeClient({ wsUrl });

  rt.on("open", () => {
    console.log("connected");
    rt.subscribe({ type: "market.latest", market_id: marketId });
  });

  rt.on("message", (msg: any) => {
    if (msg.type === "market.latest") {
      const d = msg.data;
      console.log("update", d.market_id, "epoch", d.epoch_id, "tick", d.tick_index, "hash", d.bundle_content_hash);
    }
  });

  rt.on("error", (e: any) => console.error("ws error", e));
  rt.on("close", () => console.log("closed"));

  rt.connect();
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
```

Run:
```bash
npx tsx 03_realtime_subscribe.ts NBA_LAL_BOS
```

---

### 1.5 Fetch signer set from chain (Solana RPC) and verify
This example assumes the M0 registry program exposes signer set accounts.
You need:
- registry program id
- account derivation rules (PDA seeds)

Create `04_chain_signer_set_verify.ts`:
```ts
import { Connection, PublicKey } from "@solana/web3.js";
import { M0Client, verifyBundle } from "@m0club/sdk";

const apiBase = process.env.M0_API_BASE || "http://127.0.0.1:8080";
const rpcUrl = process.env.SOLANA_RPC_URL || "http://127.0.0.1:8899";
const registryProgramId = process.env.M0_REGISTRY_PROGRAM_ID || "";

function assertEnv(name: string, v: string) {
  if (!v) throw new Error(`missing env: ${name}`);
}

async function fetchSignerSetFromChain(conn: Connection, signerSetId: string, programId: PublicKey) {
  // Conceptual PDA. Replace seeds with your program’s PDA scheme.
  const seed = Buffer.from("signer_set");
  const id = Buffer.alloc(8);
  id.writeBigUInt64LE(BigInt(signerSetId));
  const [pda] = PublicKey.findProgramAddressSync([seed, id], programId);

  const acc = await conn.getAccountInfo(pda);
  if (!acc) throw new Error("signer set account not found");

  // In a real implementation, decode with Anchor IDL or Borsh.
  // Here we assume the SDK exposes a decoder helper.
  // return decodeSignerSet(acc.data);

  throw new Error("implement signer set decoding using IDL/Borsh for your program");
}

async function main() {
  assertEnv("M0_REGISTRY_PROGRAM_ID", registryProgramId);

  const client = new M0Client({ apiBase });
  const conn = new Connection(rpcUrl, "confirmed");

  const marketId = process.argv[2] || "NBA_LAL_BOS";
  const latest = await client.markets.latest(marketId);

  const signerSet = await fetchSignerSetFromChain(conn, latest.signer_set_id, new PublicKey(registryProgramId));

  const ok = verifyBundle({ bundle: latest.bundle, signerSetPubkeys: signerSet.pubkeys });
  console.log("Verified:", ok);
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
```

Notes:
- This file includes an explicit placeholder for account decoding, because decoding depends on your program IDL.
- In the real repo, you should provide an SDK helper that decodes registry accounts from IDL.

---

### 1.6 Persist verified outputs to a local SQLite file
Create `05_persist_sqlite.ts`:
```ts
import fs from "node:fs";
import path from "node:path";
import Database from "better-sqlite3";
import { M0Client, verifyBundle } from "@m0club/sdk";

const apiBase = process.env.M0_API_BASE || "http://127.0.0.1:8080";
const dbPath = process.env.M0_SQLITE_PATH || path.join(process.cwd(), "m0_audit.sqlite");

function initDb(db: Database.Database) {
  db.exec(`
    create table if not exists market_latest (
      market_id text not null,
      epoch_id text not null,
      tick_index integer not null,
      bundle_content_hash text not null,
      signer_set_id text not null,
      verified integer not null,
      payload_json text not null,
      inserted_at_ms integer not null,
      primary key (market_id, epoch_id, tick_index)
    );
  `);
}

async function main() {
  const client = new M0Client({ apiBase });
  const marketId = process.argv[2] || "NBA_LAL_BOS";

  const latest = await client.markets.latest(marketId);
  const signerSet = await client.signers.getSignerSet(latest.signer_set_id);

  const verified = verifyBundle({ bundle: latest.bundle, signerSetPubkeys: signerSet.pubkeys });

  const db = new Database(dbPath);
  initDb(db);

  const stmt = db.prepare(`
    insert or replace into market_latest
    (market_id, epoch_id, tick_index, bundle_content_hash, signer_set_id, verified, payload_json, inserted_at_ms)
    values (?, ?, ?, ?, ?, ?, ?, ?)
  `);

  const insertedAt = Date.now();
  stmt.run(
    latest.market_id,
    latest.epoch_id,
    latest.tick_index,
    latest.bundle_content_hash,
    latest.signer_set_id,
    verified ? 1 : 0,
    JSON.stringify(latest),
    insertedAt
  );

  console.log("saved:", dbPath, "verified:", verified);
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
```

Install sqlite dependency:
```bash
npm install better-sqlite3
```

Run:
```bash
npx tsx 05_persist_sqlite.ts NBA_LAL_BOS
```

---

## 2. Python Examples

### 2.1 Project setup
```bash
mkdir m0-py-examples && cd m0-py-examples
python -m venv .venv
source .venv/bin/activate
pip install m0club-sdk requests websocket-client
```

---

### 2.2 List markets (REST)
Create `01_list_markets.py`:
```py
import os
from m0club_sdk import M0Client

api_base = os.getenv("M0_API_BASE", "http://127.0.0.1:8080")

def main():
    client = M0Client(api_base=api_base)
    markets = client.markets_list()
    print("Market count:", len(markets))
    for m in markets[:20]:
        print("-", m["market_id"], m.get("domain"), m.get("tier_policy"), "cadence_ms=", m.get("cadence_ms"))

if __name__ == "__main__":
    main()
```

Run:
```bash
python 01_list_markets.py
```

---

### 2.3 Fetch latest and verify
Create `02_latest_and_verify.py`:
```py
import os
from m0club_sdk import M0Client, verify_bundle

api_base = os.getenv("M0_API_BASE", "http://127.0.0.1:8080")

def main():
    market_id = os.getenv("M0_MARKET_ID", "NBA_LAL_BOS")
    client = M0Client(api_base=api_base)

    latest = client.market_latest(market_id)
    signer_set = client.signer_set_get(latest["signer_set_id"])

    ok = verify_bundle(bundle=latest["bundle"], signer_set_pubkeys=signer_set["pubkeys"])
    print("Verified:", ok)

    b = latest["bundle"]
    m = next((x for x in b["markets"] if x["market_id"] == market_id), b["markets"][0])
    for o in m["outcomes"]:
        print(o["outcome_id"], o["p_scaled"], o["ci_low_scaled"], o["ci_high_scaled"])

if __name__ == "__main__":
    main()
```

---

### 2.4 Realtime subscription (WebSocket)
Create `03_realtime_subscribe.py`:
```py
import os
import json
import websocket

ws_url = os.getenv("M0_WS_URL", "ws://127.0.0.1:8090")
market_id = os.getenv("M0_MARKET_ID", "NBA_LAL_BOS")

def on_open(ws):
    sub = {"type":"subscribe","channel":"market.latest","market_id":market_id}
    ws.send(json.dumps(sub))
    print("subscribed", market_id)

def on_message(ws, message):
    print("message", message)

def on_error(ws, error):
    print("error", error)

def on_close(ws, close_status_code, close_msg):
    print("closed", close_status_code, close_msg)

ws = websocket.WebSocketApp(
    ws_url,
    on_open=on_open,
    on_message=on_message,
    on_error=on_error,
    on_close=on_close
)
ws.run_forever()
```

---

### 2.5 Persist verified outputs to CSV
Create `04_persist_csv.py`:
```py
import os
import csv
import time
from m0club_sdk import M0Client, verify_bundle

api_base = os.getenv("M0_API_BASE", "http://127.0.0.1:8080")
out_path = os.getenv("M0_CSV_PATH", "m0_audit.csv")
market_id = os.getenv("M0_MARKET_ID", "NBA_LAL_BOS")

def main():
    client = M0Client(api_base=api_base)
    latest = client.market_latest(market_id)
    signer_set = client.signer_set_get(latest["signer_set_id"])

    ok = verify_bundle(bundle=latest["bundle"], signer_set_pubkeys=signer_set["pubkeys"])

    row = {
        "ts": int(time.time() * 1000),
        "market_id": latest["market_id"],
        "epoch_id": latest["epoch_id"],
        "tick_index": latest["tick_index"],
        "bundle_content_hash": latest["bundle_content_hash"],
        "signer_set_id": latest["signer_set_id"],
        "verified": 1 if ok else 0
    }

    write_header = not os.path.exists(out_path)
    with open(out_path, "a", newline="", encoding="utf-8") as f:
        w = csv.DictWriter(f, fieldnames=list(row.keys()))
        if write_header:
            w.writeheader()
        w.writerow(row)

    print("saved:", out_path, "verified:", ok)

if __name__ == "__main__":
    main()
```

---

## 3. Auditing Notes

When building audits:
- always verify `bundle_content_hash`
- always verify signatures against the on-chain signer set
- store evidence hashes and tx signatures
- record the API endpoint and response headers (optional)

---

## 4. Troubleshooting

- If verification fails, ensure you are using the correct signer set pubkeys.
- If realtime does not send updates, ensure the market is active and publishing.
- If local dev, confirm the API gateway and engine are running.

---

## Reference

- Types: `docs/sdk/types.md`
- Quickstart: `docs/sdk/quickstart.md`
- Bundle hashing: `docs/engine-spec/bundle-hashing.md`
- Signer sets: `docs/protocol-spec/signer-set.md`
- Commit-reveal: `docs/protocol-spec/commit-reveal.md`
