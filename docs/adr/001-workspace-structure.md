# 1. Workspace Structure for Experiments

Date: 2024-05-21

## Status

Accepted

## Context

The `skunkworks` repository serves as a breeding ground for various experimental Rust projects ("moonshots"). These projects, such as `git_rhythm` and `literary-boids`, often have different goals, dependencies, and maturity levels.

Managing these experiments raises structural challenges:
1. **Isolation:** Changes in one experiment should not break another.
2. **Dependency Management:** We want to avoid duplicating `Cargo.lock` files while keeping dependency versions consistent where possible.
3. **Discoverability:** It should be easy to find and run any experiment.

Managing them as completely separate git repositories adds significant overhead for maintenance. Managing them as modules within a single "monolith" crate leads to "dependency hell" where incompatible dependencies (e.g., different backend versions for graphics libraries) conflict.

## Decision

We will structure the repository as a **Cargo Workspace**.

- The root `Cargo.toml` defines the `[workspace]` and lists members.
- Each experiment resides in its own subdirectory under `experiments/` (e.g., `experiments/git_rhythm`).
- Each experiment is a valid Cargo package with its own `Cargo.toml`.

## Consequences

**Positive:**
- **Unified Workflow:** `cargo build` and `cargo test` at the root run for all projects.
- **Dependency Sharing:** A single `Cargo.lock` at the root manages resolution for the entire workspace, reducing disk space and compilation time for shared dependencies (like `anyhow` or `ratatui`).
- **Isolation:** Experiments declare their own specific dependencies. One project using `wgpu` doesn't force `wgpu` on a project that only needs `std`.

**Negative:**
- **Root Manifest Maintenance:** Adding a new experiment requires manually updating the `members` list in the root `Cargo.toml`.
- **Lockfile Contention:** Updating a dependency for one experiment might trigger updates for others, potentially causing friction if breaking changes occur in shared transitive dependencies.
