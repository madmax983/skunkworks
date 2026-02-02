# 🔭 Product Spec: GallifreyDB

**Status**: Draft
**Owner**: Vantage
**Priority**: P2

## 1. User Story

> "As a **Data Auditor**, I want to **query the exact state of a record at any specific point in history**, so that I can **reconstruct past events and verify compliance without restoring backups**."

## 2. Context & Gap Analysis

**The Problem:**
Most Key-Value stores (Redis, RocksDB) are mutable: an update destroys the previous value. To see "what happened yesterday," engineers must parse complex transaction logs or restore full database backups, which is slow and error-prone.

**The "So What?":**
In financial and simulation contexts (like `ram-bazaar`), the *history* of a value is often as important as its current state. "How did the price reach $100?" is a different question than "What is the price?".

**Gap Analysis:**
*   **Postgres**: Supports Temporal Tables (SQL:2011), but is heavy and complex to configure.
*   **Git**: Great for code, bad for high-frequency data.
*   **GallifreyDB** needs to fill the niche of a **lightweight, embedded, temporal KV store**.
*   *Critique:* Why doesn't GallifreyDB support `GeoSpatial` indexing yet? If we are tracking agents in a simulation (like `tui-semantic`), we need to query "Who was at (x,y) at time T?".

## 3. Success Metrics

*   **Storage Efficiency**: Historical version overhead should be < 10% for unchanged data (Delta encoding).
*   **Latency**: Point-in-time lookup (`get("key", timestamp)`) should be < 2x the latency of a standard head lookup.
*   **Capacity**: Must handle 100M+ historical versions on a standard SSD.

## 4. Acceptance Criteria

### Core Functionality
- [ ] **Append-Only Storage**: Deletes are just a special "Tombstone" marker. Nothing is ever truly erased from disk.
- [ ] **Time-Travel Query**: `get(key, timestamp)` returns the value valid at that instant.
- [ ] **Range History**: `history(key, start_time, end_time)` returns an iterator of all changes.
- [ ] **Immutability**: Once written, a version ID is permanent.

### Future: GeoSpatial (Phase 2)
- [ ] **Spatial Indexing**: Add ability to query keys by coordinate bounding box *and* time. (e.g., "Find all agents in sector 7 during the incident").

## 5. Out of Scope (Phase 1)

*   🚫 **SQL Support**: No SQL parser. Simple API only.
*   🚫 **Distributed Consensus**: Single-node embedded usage first (like SQLite).
*   🚫 **GeoSpatial**: Explicitly deferred to Phase 2, but architecture must allow for secondary indexes.

## 6. Philosophy Alignment
*   **Disk is Cheap, History is Priceless**: optimizing for storage space should not compromise the ability to audit.
*   **Simple over Smart**: A log-structured merge tree (LSM) is preferred over complex B-Tree variants for this append-heavy workload.
