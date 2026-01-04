
from __future__ import annotations
from m0club import M0Client

def main() -> None:
    c = M0Client()
    print("health:", c.health())
    markets = c.list_markets()
    print("markets:", markets)
    if markets:
        mid = markets[0]["market_id"]
        print("latest:", c.latest_prediction(mid))
    c.close()

if __name__ == "__main__":
    main()
