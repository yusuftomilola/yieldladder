'use client';

import { useState, useEffect } from 'react';

export interface PoolAllocation {
  id: string;
  pair: string;
  allocatedUSDC: number;
  strategyPct: number;
  capHeadroom: number;
  feeAPY30d: number;
  ilPct: number;
}

export interface AllocationsData {
  pools: PoolAllocation[];
  totalAllocated: number;
  isLoading: boolean;
  error: string | null;
}

export function useAllocations(): AllocationsData {
  const [data, setData] = useState<AllocationsData>({
    pools: [],
    totalAllocated: 0,
    isLoading: true,
    error: null,
  });

  useEffect(() => {
    // TODO(GF-12): Replace with StrategyVault.allocations() Soroban RPC call
    const pools: PoolAllocation[] = [
      { id: 'xlm-usdc', pair: 'XLM/USDC', allocatedUSDC: 215_000, strategyPct: 28.0, capHeadroom: 7.0, feeAPY30d: 4.2, ilPct: 1.3 },
      { id: 'eurc-usdc', pair: 'EURC/USDC', allocatedUSDC: 190_000, strategyPct: 24.7, capHeadroom: 10.3, feeAPY30d: 3.8, ilPct: 0.4 },
      { id: 'aqua-usdc', pair: 'AQUA/USDC', allocatedUSDC: 160_000, strategyPct: 20.8, capHeadroom: 14.2, feeAPY30d: 6.1, ilPct: 2.7 },
      { id: 'ybtc-usdc', pair: 'yBTC/USDC', allocatedUSDC: 120_000, strategyPct: 15.6, capHeadroom: 19.4, feeAPY30d: 5.3, ilPct: 0.8 },
      { id: 'yeth-usdc', pair: 'yETH/USDC', allocatedUSDC: 83_500, strategyPct: 10.9, capHeadroom: 24.1, feeAPY30d: 4.9, ilPct: 0.6 },
    ];
    const totalAllocated = pools.reduce((sum, p) => sum + p.allocatedUSDC, 0);
    setData({ pools, totalAllocated, isLoading: false, error: null });
  }, []);

  return data;
}
