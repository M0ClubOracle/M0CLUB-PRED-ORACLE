
from __future__ import annotations
import os
import httpx
from typing import Any, Dict, List, Optional

class M0Client:
    def __init__(self, base_url: str | None = None, api_key: str | None = None, timeout: float = 10.0):
        self.base_url = (base_url or os.getenv("M0_API_BASE") or "http://localhost:8080").rstrip("/")
        self.api_key = api_key or os.getenv("M0_API_KEY")
        self._client = httpx.Client(timeout=timeout, headers=self._headers())

    def _headers(self) -> Dict[str, str]:
        h = {"accept": "application/json"}
        if self.api_key:
            h["authorization"] = f"Bearer {self.api_key}"
        return h

    def _get(self, path: str) -> Any:
        url = f"{self.base_url}{path}"
        r = self._client.get(url)
        r.raise_for_status()
        return r.json()

    def health(self) -> Any:
        return self._get("/health")

    def list_markets(self) -> List[Dict[str, Any]]:
        return self._get("/markets")

    def list_epochs(self) -> List[Dict[str, Any]]:
        return self._get("/epochs")

    def latest_prediction(self, market_id: str) -> Dict[str, Any]:
        return self._get(f"/predictions/{market_id}/latest")

    def close(self) -> None:
        self._client.close()
