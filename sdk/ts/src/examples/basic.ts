
import { M0Client } from "../client";

async function main() {
  const client = new M0Client({
    baseUrl: process.env.M0_API_BASE ?? "http://localhost:8080"
  });

  const health = await client.health();
  console.log("health:", health);

  const markets = await client.listMarkets();
  console.log("markets:", markets);

  if (markets.length > 0) {
    const p = await client.latestPrediction(markets[0].market_id);
    console.log("latest:", p);
  }
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
