'use client';

import { useTVL } from '@/hooks/useTVL';
import { useAPY } from '@/hooks/useAPY';
import styles from '@/app/page.module.css';

function formatTVL(n: number): string {
  if (n >= 1_000_000) return `$${(n / 1_000_000).toFixed(2)}M`;
  if (n >= 1_000) return `$${(n / 1_000).toFixed(0)}K`;
  return `$${n.toFixed(0)}`;
}

export function StatsBar() {
  const { total, isLoading: tvlLoading } = useTVL();
  const { best, isLoading: apyLoading } = useAPY();

  return (
    <section className={styles.stats}>
      <div className={styles.stat}>
        <span className={styles.statValue}>
          {tvlLoading ? '—' : formatTVL(total)}
        </span>
        <span className={styles.statLabel}>Total TVL</span>
      </div>
      <div className={styles.statDivider} />
      <div className={styles.stat}>
        <span className={styles.statValue}>
          {apyLoading ? '—' : `${best.toFixed(1)}%`}
        </span>
        <span className={styles.statLabel}>Best APY</span>
      </div>
      <div className={styles.statDivider} />
      <div className={styles.stat}>
        <span className={styles.statValue}>0%</span>
        <span className={styles.statLabel}>Protocol Fee</span>
      </div>
      <div className={styles.statDivider} />
      <div className={styles.stat}>
        <span className={styles.statValue}>35%</span>
        <span className={styles.statLabel}>Max Pool Exposure</span>
      </div>
    </section>
  );
}
