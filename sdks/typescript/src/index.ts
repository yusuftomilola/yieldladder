export type { Tier, Network, YieldLadderOptions, Position } from './types';
import type { Tier, YieldLadderOptions, Position } from './types';

export class YieldLadder {
  constructor(_options: YieldLadderOptions) {
    // Full implementation in subsequent commits.
  }

  async deposit(_params: { tier: Tier; amount: string }): Promise<void> {
    throw new Error('Not implemented');
  }

  async withdraw(_params: { tier: Tier }): Promise<void> {
    throw new Error('Not implemented');
  }

  async earlyExit(_params: { tier: Tier }): Promise<void> {
    throw new Error('Not implemented');
  }

  async position(_address: string): Promise<Position> {
    throw new Error('Not implemented');
  }
}
