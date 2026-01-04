
# Auto-curated shared types for M0Club SDKs.

from __future__ import annotations
from dataclasses import dataclass
from typing import Dict, Tuple, Any

MarketId = str
EpochId = int
ConfidenceInterval = Tuple[float, float]

@dataclass
class OutcomeProb:
    p: float
    ci: ConfidenceInterval

PredictionPayload = Dict[str, OutcomeProb]

@dataclass
class Prediction:
    market_id: MarketId
    epoch_id: EpochId
    outcomes: Dict[str, Any]  # keep Any for flexible JSON payloads

@dataclass
class Market:
    market_id: MarketId
    domain: str
    status: str

@dataclass
class Epoch:
    epoch_id: EpochId
    market_id: MarketId
    state: str
