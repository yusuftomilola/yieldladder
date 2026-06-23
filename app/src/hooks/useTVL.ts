'use client';

import { useState, useEffect } from 'react';

export interface TierTVL {
  tier: 'flex' | 'l3' | 'l6' | 'l12';
  label: string;
  tvl: number;
}

export interface TVLData {
  total: number;
  byTier: TierTVL[];
  isLoading: boolean;
  error: string | null;
}

export function useTVL(): TVLData {
  const [data, setData] = useState<TVLData>({
    total: 0,
    byTier: [],
    isLoading: true,
    error: null,
  });

  useEffect(() => {
    // TODO(GF-12): Replace with Soroban RPC calls once data service layer is implemented
    const tiers: TierTVL[] = [
      { tier: 'flex', label: 'Flex', tvl: 125_000 },
      { tier: 'l3', label: 'L3', tvl: 87_500 },
      { tier: 'l6', label: 'L6', tvl: 210_000 },
      { tier: 'l12', label: 'L12', tvl: 340_000 },
    ];
    const total = tiers.reduce((sum, t) => sum + t.tvl, 0);
    setData({ total, byTier: tiers, isLoading: false, error: null });
  }, []);

  return data;
}
