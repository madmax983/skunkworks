# 018. Chimera Sovereignty System

## Status
Proposed

## Context
Chimera organisms exist within a shared spatial environment (the Petri Dish), but they previously lacked a formal mechanism to exert control over that space or interact economically with other organisms. This limited the potential for emergent behaviors related to territory defense, resource monopolization, and trade. Organisms could only interact through direct modification of shared memory (grid cells) or communication (signals), without a concept of "ownership" or "cost" for entering another's domain.

## Decision
Implement a **Sovereignty System** integrated into the Chimera VM (Nova feature set) that introduces the concept of land ownership and taxation.

### Key Components:
1.  **Sovereignty Grid:** A parallel 16x16 grid (`Vec<Vec<Option<usize>>>`) tracking the owner ID of each cell.
2.  **Taxation Logic:** A hook in `vm.step()` (`process_territory`) that automatically deducts energy from visitors in foreign territory and transfers it to the owner's market wallet.
3.  **New OpCodes:**
    *   `Claim(radius)`: Claims a circular area for the current strand.
    *   `Cede(x, y)`: Releases ownership of a specific cell.
    *   `Sovereignty(x, y)`: Queries the owner of a cell.
    *   `Tax(rate)`: Sets the tax rate for the current strand's territory.

## Consequences
*   **Positive:**
    *   **Emergent Conflict:** Organisms must now balance expansion (claiming costs energy) with defense and economic gain (taxes).
    *   **Economic Pressure:** The energy cost of traversing foreign territory creates a natural selection pressure for efficient movement or diplomacy (ceding/alliances).
    *   **Market Integration:** Direct linkage between spatial control and the `Market` system enables "landlord" strategies.
*   **Negative:**
    *   **Performance:** The `process_territory` hook adds a check for every organism on every tick, slightly increasing simulation overhead.
    *   **Complexity:** VM state size increases with the addition of `sovereignty_grid` and `tax_rates`.
