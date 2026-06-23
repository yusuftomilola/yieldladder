'use client';

import { useState, useEffect } from 'react';

export type APYTier = 'flex' | 'l3' | 'l6' | 'l12';

export interface TierAPY {
  tier: APYTier;
  label: string;
  current: number;
  sevenDay: number;
  thirtyDay: number;
}

export interface APYData {
  byTier: TierAPY[];
  best: number;
  isLoading: boolean;
  error: string | null;
}

export function useAPY(): APYData {
  const [data, setData] = useState<APYData>({
    byTier: [],
    best: 0,
    isLoading: true,
    error: null,
  });

  useEffect(() => {
    // TODO(GF-12): Replace with real APY calculations from harvest event history
    const tiers: TierAPY[] = [
      { tier: 'flex', label: 'Flex', current: 4.2, sevenDay: 3.8, thirtyDay: 4.1 },
      { tier: 'l3', label: 'L3', current: 5.6, sevenDay: 5.2, thirtyDay: 5.4 },
      { tier: 'l6', label: 'L6', current: 7.8, sevenDay: 7.3, thirtyDay: 7.6 },
      { tier: 'l12', label: 'L12', current: 11.2, sevenDay: 10.8, thirtyDay: 11.0 },
    ];
    const best = Math.max(...tiers.map((t) => t.current));
    setData({ byTier: tiers, best, isLoading: false, error: null });
  }, []);

  return data;
}
