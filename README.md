# YieldLadder

Time-locked USDC vaults on Soroban with auto-routed AMM yield.

[![audit](https://img.shields.io/badge/audit-pending-yellow.svg)](audit/)
[![version](https://img.shields.io/badge/protocol-v0.3.0-informational.svg)](CHANGELOG.md)

## Abstract

YieldLadder is a savings primitive on the Stellar network. A user deposits USDC into one of four time-locked vaults — Flex, 3-month, 6-month, or 12-month — and receives a non-transferable position representing their share. The protocol routes deposited USDC into a curated set of Stellar AMM liquidity pools, harvests trading fees on a fixed cadence, compounds harvested yield back into the position, and distributes yield to depositors proportional to share weight and lock duration.

The protocol is non-custodial and immutable per deployment. Yield is sourced exclusively from on-chain Stellar liquidity pools — no off-chain CeFi venues, no anchor lending, no rehypothecation. The only privileged role is the Strategist, who can propose new pool allocations subject to a 72-hour timelock and a hard-coded per-pool exposure cap.

## Vault tiers

| Tier | Lock duration | Lock multiplier | Early-exit fee | Min deposit |
|---|---|---|---|---|
| Flex | None | 1.00x | 0% | 1 USDC |
| L3 | 3 months | 1.05x | 0.50% | 50 USDC |
| L6 | 6 months | 1.15x | 1.25% | 100 USDC |
| L12 | 12 months | 1.40x | 3.00% | 250 USDC |

Lock multipliers apply at the share-weight level: a 100 USDC deposit into L12 receives 140 share-units, while the same deposit into Flex receives 100. Yield is distributed pro-rata against share-units, so longer-lock depositors capture a structurally larger fraction of harvested fees.

Early-exit fees are charged against principal at withdrawal time and redistributed to remaining depositors in the same tier. They are not retained by the protocol.

## Architecture

```
+------------------+       +-------------------+       +------------------+
|   Vault Router   | ----> |  Vault Contracts  | ----> |  Strategy Vault  |
|   (entrypoint)   |       |  Flex / L3 / L6 / |       |    (allocator)   |
+------------------+       |        L12        |       +------------------+
                           +-------------------+                |
                                                                v
                                                       +------------------+
                                                       |   Stellar AMM    |
                                                       |  liquidity pools |
                                                       |   (curated set)  |
                                                       +------------------+
                                                                |
                                                                v
                                                       +------------------+
                                                       |    Harvester     |
                                                       | (cron-triggered) |
                                                       +------------------+
```

The Vault Router is the user-facing contract; it enforces tier rules and mints position records. Each tier vault holds depositor balances and a reference to the Strategy Vault. The Strategy Vault holds the protocol's working capital and executes allocations across the pool set, subject to per-pool exposure caps.

The Harvester is a permissionless contract — anyone can call its `harvest()` entrypoint after the cooldown elapses. The caller receives a small bounty (10 bps of harvested yield) to incentivize timely calls without requiring a privileged keeper.

## Vault mechanics

### Deposit

#### Flow
1. User approves the Vault Router for the deposit amount in USDC.
2. User calls `deposit(tier, amount)`.
3. Router transfers USDC into the tier vault.
4. Router records `shares = amount * lock_multiplier` against the user's address.
5. Router records `lock_until = now + tier.duration` for L3, L6, L12.
6. Tier vault forwards USDC to the Strategy Vault for allocation.

#### Position records
Positions are non-transferable. A user's claim on the vault is identified by their address, not by holding a fungible token. Position transferability is omitted by design to prevent secondary-market trades from interacting with lock semantics in ways the protocol cannot enforce on-chain.

### Yield accrual

Yield accrues continuously at the AMM layer but is only realized at harvest time. Between harvests, a user's claim grows in proportion to the harvested-yield events that affect their tier, weighted by their share-units at the moment of each harvest.

The accrual function:

```
user_yield(t) = Σ over harvests h_i where t_deposit < h_i <= t:
    harvest_amount(h_i) * (user_shares / total_shares_at(h_i))
```

Share counts are tracked with a checkpoint mechanism so that mid-period deposits and withdrawals do not retroactively affect prior harvests.

### Withdrawal

#### At maturity
After `lock_until`, a user calls `withdraw(tier)` and receives principal plus accrued yield. No fee is charged. The position record is cleared.

#### Early exit
Available on L3, L6, L12. Before `lock_until`, a user can call `early_exit(tier)`. Principal minus the tier's exit fee is returned, along with all accrued yield. The exit fee is socialized across the remaining depositors in the same tier — `total_shares` does not change, so per-share value rises.

### Compounding

After each harvest, harvested USDC is deposited back into the Strategy Vault and re-allocated, increasing the total assets backing each tier's share-units. Compounding is implicit — users do not call a `compound` function, and the protocol charges no compounding fee.

## Yield sources

YieldLadder allocates exclusively to Stellar AMM liquidity pools that meet all of the following criteria:

- USDC is one of the two assets in the pool
- Counterparty asset is on the protocol allowlist (currently: XLM, EURC, AQUA)
- Pool has at least 30 days of trading history
- Pool has at least $250,000 TVL at allocation time
- Per-pool exposure is capped at 35% of Strategy Vault assets

The Strategist can propose adding pools or counterparty assets via the governance contract, subject to a 72-hour timelock during which the proposal can be vetoed by the Guardian Multisig.

Current allocations are published live at `https://yieldladder.dev/allocations` and on-chain via `Strategy.allocations()`.

## Risk model

### Smart contract risk

Protocol contracts are immutable per deployment — there is no upgrade path. Bugs found post-deployment can only be addressed by deploying a new version and offering migration. Each deployed version should be treated as the final form of that version.

Mitigation: external audit prior to mainnet release; formal verification of the Vault Router's lock and share accounting; continuous internal review.

### Impermanent loss

YieldLadder allocates to AMM pools, which exposes principal to impermanent loss when the prices of the two pool assets diverge. The protocol mitigates this by:

- Restricting the allowlist to pools where one side is USDC and the other is a comparatively low-volatility asset
- Capping per-pool exposure
- Tracking realized versus potential IL per pool and rebalancing when divergence exceeds a configured threshold

Impermanent loss is **not** absorbed by the protocol — it is reflected in share-unit value. Under sustained price divergence, vault assets can fall below initial deposit value.

### Stablecoin depeg

USDC on Stellar is issued by Circle and is regulated, but no stablecoin is risk-free. A material USDC depeg directly affects every YieldLadder position. The protocol does not hedge this risk. Capital allocated to YieldLadder should be capital the user can afford to see meaningfully diminished under such a scenario.

### Strategist risk

The Strategist role can propose new allocations but cannot withdraw user funds. The Strategist's actions are bounded by:

- The 72-hour timelock on allocation changes
- The Guardian Multisig veto
- Hard-coded per-pool exposure caps
- Hard-coded counterparty allowlist (changes require governance)

## Audits and security

### Audit status

| Auditor | Scope | Status | Report |
|---|---|---|---|
| Internal review | All contracts | Complete (2026-01) | `audit/internal-2026-01.md` |
| External audit firm | Vault Router, Strategy Vault | Engaged (Q2 2026) | Pending |
| Formal verification | Lock and share accounting | In progress | — |

### Bug bounty

A bug bounty program opens simultaneously with the external audit report. Critical findings (loss of funds): up to $50,000. High (lock or permission bypass): up to $15,000. Medium and low: scaled. Full terms at `security/bounty.md`.

### Disclosure

Security issues should be reported to `security@yieldladder.dev`, encrypted with PGP key fingerprint `D7F2 7A91 C4AB 8E03 5F12 9B6E 88AD 4F71 33CC 0BAE`. Do not open public issues for security findings.

## Contracts

| Contract | Purpose | Address (mainnet) |
|---|---|---|
| `VaultRouter` | User entrypoint | `CC...PENDING` |
| `VaultFlex` | Flex tier holdings | `CC...PENDING` |
| `VaultL3` | 3-month tier holdings | `CC...PENDING` |
| `VaultL6` | 6-month tier holdings | `CC...PENDING` |
| `VaultL12` | 12-month tier holdings | `CC...PENDING` |
| `StrategyVault` | Allocation execution | `CC...PENDING` |
| `Harvester` | Yield harvest trigger | `CC...PENDING` |
| `Governance` | Strategist proposals and timelock | `CC...PENDING` |

Mainnet addresses are published after audit completion. Testnet addresses are at `deployments/testnet.json`.

## SDK

A TypeScript SDK is provided for application integrations:

```typescript
import { YieldLadder } from '@yieldladder/sdk';

const yl = new YieldLadder({ network: 'mainnet', signer });

await yl.deposit({ tier: 'L6', amount: '500.00' });

const position = await yl.position(userAddress);
// { tier: 'L6', principal: '500.00', shares: '575',
//   accruedYield: '12.34', lockUntil: 1781856000 }

await yl.withdraw({ tier: 'L6' });    // throws if lock not expired
await yl.earlyExit({ tier: 'L6' });   // succeeds, charges 1.25% fee
```

A Next.js reference frontend lives in `app/`. The SDK targets `@stellar/stellar-sdk` v12+ with Soroban support.

## Glossary

**Vault.** A Soroban contract that holds depositor balances for a single tier and forwards working capital to the Strategy Vault.

**Tier.** A vault parameterization comprising lock duration, lock multiplier, early-exit fee, and minimum deposit.

**Share-unit.** A non-transferable accounting unit representing a depositor's claim on a tier vault. `shares = deposit_amount * lock_multiplier`.

**Lock multiplier.** A constant per tier (1.00x, 1.05x, 1.15x, 1.40x) applied at deposit time to weight a depositor's share claim.

**Harvest.** A permissionless transaction that claims accumulated trading fees from allocated AMM pools and routes them back into the Strategy Vault as compound yield.

**Strategist.** A role that can propose pool allocation changes, subject to timelock and Guardian veto. Cannot move user funds.

**Guardian Multisig.** A 4-of-7 multisig authorized to veto Strategist proposals during the 72-hour timelock window. Cannot move user funds.

**Strategy Vault.** The contract that holds aggregate working capital across all tiers and executes pool allocations.

**Position record.** A non-transferable record of a user's deposit, indexed by user address and tier.

**Allocation cap.** Hard-coded maximum fraction of Strategy Vault assets exposed to any single AMM pool. Currently 35%.

## License

Apache 2.0. See `LICENSE` for the full text.
