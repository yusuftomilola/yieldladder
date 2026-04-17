# Internal Security Review — 2026-01

**Scope:** All eight Soroban contracts (`VaultRouter`, `VaultFlex`, `VaultL3`, `VaultL6`,
`VaultL12`, `StrategyVault`, `Harvester`, `Governance`).

**Reviewers:** Core team (two engineers).

**Period:** 2026-01-06 to 2026-01-17.

**Status:** Complete.

---

## Summary

No critical or high-severity issues were identified in the internal review period.
Three medium-severity findings and two low-severity findings were raised and resolved
before this report was filed.

---

## Findings

### M-01 — Share checkpoint gap on same-block deposit and harvest

**Severity:** Medium  
**Status:** Resolved (commit `a3f1c2d`)

If a deposit and a harvest transaction landed in the same ledger, the checkpoint
mechanism credited the depositor with yield they had not yet contributed capital
for. Fixed by recording the checkpoint at `deposit_ledger + 1`.

### M-02 — Early-exit fee precision loss on small principal amounts

**Severity:** Medium  
**Status:** Resolved (commit `b7e4a91`)

Integer division in the exit-fee calculation silently rounded to zero for deposits
below approximately 5 USDC, effectively waiving the fee. Fixed by using scaled
fixed-point arithmetic (7 decimal places, matching Stellar's native token precision).

### M-03 — Governance timelock did not reset on proposal amendment

**Severity:** Medium  
**Status:** Resolved (commit `c2d8f05`)

A Strategist could amend a proposal one second before the timelock expired,
extending effective review time to near-zero. Fixed by treating any amendment as
a new proposal that restarts the 72-hour clock.

### L-01 — Missing bounds check on per-pool allocation cap

**Severity:** Low  
**Status:** Resolved (commit `d9a3b12`)

The exposure cap was enforced at proposal time but not at execution time.
A second concurrent proposal could collectively push a single pool above 35%.
Added a re-check at execution.

### L-02 — Harvester cooldown stored as u64 timestamp, not ledger sequence

**Severity:** Low  
**Status:** Resolved (commit `e5c7f44`)

Ledger close times on Stellar are not monotonically guaranteed under all
network conditions. Switched to ledger-sequence comparison for cooldown enforcement.

---

## Recommendations for external audit

1. Formal verification of the share-checkpoint invariant under concurrent deposits.
2. Fuzz testing of the exit-fee fixed-point arithmetic.
3. Review of Governance execution path for re-entrancy under Soroban's host-function model.
