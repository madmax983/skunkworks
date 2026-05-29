# 098. Origami Capacity Bounds Checks

Date: 2026-05-14

## Status
Accepted

## Context
The `generate_miura_grid` and `generate_miura_mesh` functions within `crates/origami` calculated vector capacity allocations using unchecked and partially checked inputs. When supplied with extremely large values (e.g., `usize::MAX - 1`), this led to fatal `capacity overflow` panics, creating a Denial of Service vulnerability if those dimensions were externally provided.

## Decision
Refactored the internal vector capacity calculations to strictly bind the result to safe maximum sizes based on `isize::MAX / std::mem::size_of::<T>()`. The `Vec::with_capacity` calls now use safe, bounded sizes to prevent capacity overflow panics.

## Consequences

### Positive
*   **Security & Stability:** Prevents fatal panics in the origami mesh generation logic, fortifying the system against Denial of Service via malicious or chaotic inputs.
*   **Resilience:** Makes the crate more robust for use in the broader ecosystem (e.g., fuzzing with Havoc).

### Negative
*   **None**
