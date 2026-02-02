# 🔭 Product Spec: Arthropod

**Status**: Draft
**Owner**: Vantage
**Priority**: P3

## 1. User Story

> "As a **Simulation Architect**, I want to **isolate and inspect the decision history of a specific agent** within a massive swarm, so that I can **diagnose emergent bugs and feedback loops**."

## 2. Context & Gap Analysis

**The Problem:**
In agent-based simulations (like `pidgin-mesh` or `thermo-tarmites`), bugs often manifest as "emergent behavior" that only appears when thousands of agents interact. Traditional debuggers (GDB) are useless here because stopping one thread doesn't stop the swarm, and "pausing the world" destroys the temporal dynamics we are trying to observe.

**The "So What?":**
If we can't understand *why* the swarm is collapsing, we can't fix it. We need observability tools that treat "Agents" as first-class citizens, not just threads or memory addresses.

**Gap Analysis:**
*   `semantic-spy` exists but is a passive viewer.
*   `semantic-dvr` records history but lacks granular filtering.
*   **Arthropod** fills the gap: it is the **active debugger and analysis workbench** for Swarm Intelligence.

## 3. Success Metrics

*   **Filter Speed**: Must filter a stream of 10,000 events/sec by Agent ID with < 10ms latency.
*   **Visual Clarity**: The "Causal Graph" must remain readable even with 50+ interacting nodes.
*   **Integration**: Zero-config attachment to any experiment implementing `tui-semantic`.

## 4. Acceptance Criteria

### Core Functionality
- [ ] **Stream Ingestion**: Consumes JSONL output from standard input or a log file (compatible with `tui-semantic`).
- [ ] **Agent Filtering**: Users can select an Agent ID (e.g., via regex or list) and see *only* events related to that entity.
- [ ] **State Diffing**: Highlight exactly which fields changed between tick T and T+1.
- [ ] **Anomaly Detection**: (Optional) Flag agents that violate defined invariants (e.g., "Energy < 0").

### Usability
- [ ] **Timeline View**: A TUI visualization showing the agent's lifespan and key events.
- [ ] **"Why?" Button**: Clicking an event shows the inputs (sensor data) that led to that state change.

## 5. Out of Scope (Phase 1)

*   🚫 **Live Injection**: We cannot modify the simulation state on the fly (Read-Only).
*   🚫 **3D Visualization**: 2D TUI/Canvas only.
*   🚫 **Distributed Tracing**: Single-node simulations only.

## 6. Philosophy Alignment
*   **Observability without Interference**: The tool must not alter the outcome of the simulation it is measuring (Heisenberg Principle).
*   **Drill Down**: Start with the swarm (macro), end with the neuron (micro).
