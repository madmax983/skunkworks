# 🔭 Product Spec: GallifreyDB

**Status**: Draft
**Owner**: Vantage
**Priority**: P1

## 1. User Story

> "As a **RAG Developer**, I want to **query my vector store as it existed at a specific point in time**, so that I can **debug hallucinations and reproduce past model behaviors exactly**."

## 2. Context & Gap Analysis

**The Problem:**
Vector databases (Pinecone, Qdrant) are typically mutable. When a document is updated or deleted, the embedding is overwritten. This makes it impossible to answer: "Why did the LLM retrieve this document *yesterday*?"
In regulated industries (Finance, Healthcare), reproducibility is not optional.

**The "So What?":**
If we cannot reproduce a RAG pipeline's output, we cannot trust it. **GallifreyDB** adds the dimension of Time to vector search, making it an immutable ledger of semantic state.

## 3. Success Metrics

*   **Latency**: Nearest Neighbor search with a time filter (`t <= T`) must be within 10% overhead of a standard search (< 50ms p95).
*   **Storage Efficiency**: Differential storage for updates (deltas) to avoid exploding storage costs.
*   **Recall**: 100% recall of the correct document version for the requested timestamp.

## 4. Acceptance Criteria

### Core Functionality
- [ ] **Temporal Indexing**: Every insert/update/delete is a new version. The index effectively stores `(Vector, Metadata, ValidFrom, ValidTo)`.
- [ ] **Time-Travel Query**: API must support `search(vector, timestamp)`.
- [ ] **HNSW Implementation**: Robust implementation of HNSW for base vector index (Reference: Warden's P0 fix).
- [ ] **Snapshotting**: Ability to "fork" the database state for A/B testing retrieval strategies.

### Usability
- [ ] **Git-like Semantics**: Support for "Branches" of knowledge.
- [ ] **SQL-like Filtering**: `SELECT * WHERE score > 0.8 AND category = 'legal' AT TIME '2023-10-27T10:00:00Z'`.

## 5. Out of Scope (Phase 1)

*   🚫 **Distributed Consensus**: Single-node durable storage first. Raft/Paxos later.
*   🚫 **GPU Acceleration**: CPU-bound SIMD optimization is sufficient for v1.
*   🚫 **Multi-Modal**: Text embeddings only. Audio/Video/Image embeddings later.

## 6. Philosophy Alignment
*   **Utility**: Solves the "Auditability" crisis in GenAI.
*   **Simplicity**: One binary. No external dependencies (ZooKeeper, etcd).
