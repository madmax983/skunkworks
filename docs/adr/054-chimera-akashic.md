# 054. Chimera Akashic System (Persistence)

## Status
Accepted

## Context
The Chimera VM is ephemeral by default. When the process terminates, the state (DNA, Grid, Stack) is lost. While serialization (`.dna`) exists, it is static. Organisms needed a way to store data across execution lifetimes, such as:
1.  **Memories:** Long-term learning or knowledge accumulation.
2.  **Shared State:** Communication between different runs or instances.
3.  **Karma:** A meta-currency that tracks actions across lifetimes.

## Decision
We introduced the **Akashic** feature set, which implements a persistent key-value store for the VM.

Key components:
*   **Akashic Records (`akashic.rs`):** A module that manages a persistent file-based store (typically JSON or Bincode).
*   **OpCodes:**
    *   `AkashicWrite`: Stores a value (Int, String, Junction) associated with a key.
    *   `AkashicRead`: Retrieves a value by key.
    *   `AkashicSave`/`AkashicLoad`: Snapshots the entire VM state as a "Memory" in the record.
    *   `Karma`: Modifies the organism's karmic balance, which influences luck and random events.

## Consequences

### Positive
*   **Persistence:** Enables long-running experiments where organisms can "remember" past lives.
*   **Meta-Progression:** Allows for "Roguelite" mechanics where upgrades or knowledge persist after death.
*   **Data Sharing:** Multiple VMs can potentially share an Akashic record (if synchronized), enabling a collective memory.

### Negative
*   **Side Effects:** The VM is no longer purely functional/deterministic; execution depends on external state.
*   **Performance:** Disk I/O during `AkashicWrite/Read` can stall the simulation.
*   **Complexity:** Managing the lifecycle and integrity of the persistent file adds operational overhead.
