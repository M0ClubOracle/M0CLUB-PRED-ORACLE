
import { getJson } from "./utils/http";
import type { Market, Epoch, Prediction } from "./types";

export interface M0ClientOptions {
  baseUrl?: string; // default: http://localhost:8080
  apiKey?: string;  // optional
}

export class M0Client {
  readonly baseUrl: string;
  readonly apiKey?: string;

  constructor(opts: M0ClientOptions = {}) {
    this.baseUrl = (opts.baseUrl ?? "http://localhost:8080").replace(/\/$/, "");
    this.apiKey = opts.apiKey;
  }

  private headers(): Record<string, string> {
    const h: Record<string, string> = { "accept": "application/json" };
    if (this.apiKey) h["authorization"] = `Bearer ${this.apiKey}`;
    return h;
  }

  async health(): Promise<{ ok: boolean }> {
    return getJson(`${this.baseUrl}/health`, this.headers());
  }

  async listMarkets(): Promise<Market[]> {
    return getJson(`${this.baseUrl}/markets`, this.headers());
  }

  async listEpochs(): Promise<Epoch[]> {
    return getJson(`${this.baseUrl}/epochs`, this.headers());
  }

  async latestPrediction(marketId: string): Promise<Prediction> {
    return getJson(`${this.baseUrl}/predictions/${encodeURIComponent(marketId)}/latest`, this.headers());
  }
}
