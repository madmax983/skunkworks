# 177. Hyper-Market Hybrid

Date: 2023-10-24

## Status

Proposed

## Context

We need to document the architectural crossbreed between `hyper-system` and `market-sim` as identified in `experiments/hyper-market`. The Splice Surgeon combined continuous 4D geometric projections with discrete double auction physics. We need an ADR to formalize the consequences of this hybrid architecture.

## Decision

We accept the `hyper-market` hybrid crate. The hybrid projects discrete trade volume from `market-sim` as continuous hyper-dimensional torque applied to a 4D vector from `hyper-system`.

## Consequences

This successfully ties a discrete financial model into a higher-dimensional spatial projection, creating a novel morphogenetic visualization of market liquidity.
