# 174. Market-Gray Hybrid

Date: 2026-08-16

## Status

Accepted

## Context

The Splice Surgeon created a new experimental hybrid called `market-gray` (Morphogenetic Financial Liquidity). This cross explores mapping a discrete Continuous Double Auction (CDA) market grid into a continuous Gray-Scott reaction-diffusion substrate by crossing `crates/market-sim` with `crates/gray-scott`.

## Decision

We map the physical "bids" and "asks" from the market grid to act as active biological sources feeding the grid (the "U" chemical) in the morphogenetic substrate. Executed trades drop the intense "kill" chemical (the "V" chemical).

## Consequences

- **Positive:** Generates an emergent "organic footprint" of the market, yielding self-sustaining Turing patterns driven entirely by financial events rather than discrete data points.
- **Negative:** The chemical substrate might reset to an empty state rapidly without continuous financial stimulation due to high kill rates.
