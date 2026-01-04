
/* Auto-curated shared types for M0Club SDKs. */

export type Domain = "politics" | "sports" | "macro" | "crypto" | "markets" | "unknown";

export type MarketId = string;
export type EpochId = number;

export type ConfidenceInterval = [number, number];

export interface OutcomeProb {
  p: number;           // probability (0..1)
  ci: ConfidenceInterval; // confidence interval
}

export type PredictionPayload = Record<string, OutcomeProb>;

export interface Prediction {
  market_id: MarketId;
  epoch_id: EpochId;
  outcomes: PredictionPayload;
}

export interface Market {
  market_id: MarketId;
  domain: Domain;
  status: string;
}

export interface Epoch {
  epoch_id: EpochId;
  market_id: MarketId;
  state: string;
}
