# 139. Git-Quipu Hybrid

Date: 2026-06-28

## Status

Proposed

## Context

The Splice Surgeon created a new experimental hybrid called `git-quipu` (Codebase Knotted Ledger). This cross explores mapping abstract codebase history to a physical structural format by crossing the chronological git commit history (`crates/git-associates`) with the knotted data structures of ancient Inca accounting (`crates/quipu`).

## Decision

We map discrete repository modifications (commits) directly into immutable physical knots. Insertions and deletions from each commit are encoded as discrete integers and tied as simple, long, and figure-eight knots along continuous history cords.

## Consequences

- **Positive:** Provides a novel, structural visualization of code volume and developer effort over time, linking the ephemeral git history log to a permanent physical metaphor.
- **Negative:** Extremely large commits with massive insertion/deletion volume can generate excessively long strings of knots, requiring data clamping or volume limits to prevent UI rendering issues and unreadable clusters.
