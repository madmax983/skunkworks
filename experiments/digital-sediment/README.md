# Digital Sediment ⚛️

> "The past is not a foreign country; it is a layer of bedrock under our feet, slowly compressing into oil." - Genesis (The Archivist)

## Concept

**Digital Sediment** is a "Format Archaeology" experiment that visualizes a Git repository's history as geological strata. As time passes, older commits "decay" under the pressure of new code. This decay manifests as bit rot, character corruption, and data loss.

This tool allows you to:
1.  **Excavate**: Traverse the history of the repository.
2.  **Observe Decay**: See how files from the past might look if digital storage degraded like physical matter.
3.  **Recover**: Use an archaeological parser (`nom`) to identify and highlight surviving structural artifacts (keywords, types) amidst the rubble.

## Controls

-   `Up` / `Down`: Navigate through time (commit history).
-   `d`: Toggle **Decay Simulation** (corruption based on depth).
-   `r`: Toggle **Recovery Mode** (highlight valid syntax in corrupted text).
-   `q`: Quit.

## Tech Stack

-   **Ratatui**: Terminal User Interface.
-   **Git2**: Repository traversal.
-   **Nom**: Resilient parsing for recovery.
-   **EntropyEngine**: Custom chaos generator for simulating bit rot.

## Hypothesis

Data does not last forever. By simulating the inevitable decay of information, we can better understand the value of redundancy and the fragility of our digital legacy.
