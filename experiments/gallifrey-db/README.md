# GallifreyDB 🔭

> "What did the database look like last Tuesday?"

**Vantage Spec v0.1.0**

## 1. The "Why?" (Business Value)
In regulated industries (FinTech, Healthcare) and complex debugging scenarios, knowing the *current* state of a record is insufficient. Developers and Auditors need to reconstruct historical states to:
1.  **Audit Changes:** Who changed the interest rate, when, and what was it before?
2.  **Debug Anomalies:** Reproduce a bug that only happens when the user's balance was $0.00, without restoring a terabyte-sized backup.
3.  **Undo/Redo:** Provide "Ctrl+Z" functionality for user data natively.

Implementing this at the application layer (Audit Tables) is brittle, error-prone, and slow. `GallifreyDB` solves this by making *Time* a first-class citizen in the database layer.

## 2. User Story
*   **As a:** Financial Data Auditor
*   **I want to:** Query the value of a Ledger Entry as it existed at a specific ISO-8601 timestamp
*   **So that:** I can verify compliance reports without manual log parsing.

## 3. Acceptance Criteria (MVP)
*   **Immutability:** Existing data is never overwritten. Updates create new versions.
*   **Temporal Query:**
    *   `get(key)` returns the latest value.
    *   `get_at(key, timestamp)` returns the value effective at that timestamp.
*   **API:** simple Rust API and a CLI for demonstration.
*   **Storage:** In-memory (HashMap of Vectors) for MVP.

## 4. Out of Scope (Phase 1)
*   Persistent Storage (Disk I/O).
*   Network/Server.
*   Complex range queries.

## 5. Technical Constraints
*   Must use `chrono` for time handling.
*   Must handle "Key Not Found" vs "Key Not Created Yet" gracefully.
