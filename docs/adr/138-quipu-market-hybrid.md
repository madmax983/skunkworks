# 138. Quipu-Market Hybrid

Date: 2026-06-28

## Status

Accepted

## Context

The Splice Surgeon created a new experimental hybrid called `quipu-market` (Physical Knotted Market Ledger). This cross explores mapping ephemeral financial markets to permanent physical ledgers by crossing the discrete 2D order book grid of `crates/market-sim` (where Bids and Asks collide) with the discrete knotted data structures of `crates/quipu`.

## Decision

We map discrete trades from the market order book into a continuous knotted ledger. Every time a trade executes in the order book, a new knotted record of that trade's price is structurally tied onto a Quipu cord.

## Consequences

- **Positive:** Translates abstract, ephemeral market liquidity into a permanent, physical visual artifact, providing a unique method for observing the sedimentation of financial activity.
- **Negative:** The ledger can grow unbounded over long simulation runs, requiring visual culling or bounding logic (e.g., maintaining only the last N trades) to prevent UI overload and memory bloat.
