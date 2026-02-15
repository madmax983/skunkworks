# 33. Extract Git Associates

Date: 2024-05-23
Status: Accepted

## Context

Several experiments in the Skunkworks repository, such as `tectonic-git`, `git-cantata`, and `git-harmony`, require access to the git history of the repository. They need to scan commits, analyze diffs, and track file changes over time.

Previously, this logic was either duplicated across experiments or implemented in a way that was tightly coupled to a specific experiment's needs. This led to:
- Code duplication: multiple implementations of `git2` traversal.
- Inconsistent behavior: different experiments might parse diffs slightly differently.
- Maintenance burden: bug fixes in one experiment's git logic weren't propagated to others.

We need a shared, high-level abstraction for git repository analysis that can be reused across all experiments.

## Decision

We will extract the common git analysis logic into a new shared crate named `git-associates`.

This crate will:
- Wrap the `git2` crate to provide a higher-level, easier-to-use API.
- Expose a `GitModel` struct that serves as the main entry point.
- Provide standardized structs for `Commit`, `CommitStats`, `FileChange`, and `Hunk`.
- Focus on read-only analysis and visualization needs (scanning history, diffing workdir), rather than full git client functionality (committing, pushing).

The API will be designed to return plain Old Rust Structs (PORS) that are easy to serialize and pass to UI or simulation layers, decoupling the git implementation details from the experiment logic.

## Consequences

### Positive
- **Reduced Duplication**: Common git logic is centralized.
- **Consistency**: All experiments will see the same view of the git history.
- **Simplified Experiments**: Experiment code can focus on visualization/simulation rather than `git2` plumbing.
- **Reusability**: New experiments involving git history can be bootstrapped much faster.

### Negative
- **Dependency**: All git-based experiments now depend on `git-associates` (and thus `git2`).
- **Abstraction Overhead**: The wrapper might hide some low-level `git2` features. If an experiment needs very specific git functionality not exposed by `GitModel`, it might need to bypass the wrapper or extend it.
