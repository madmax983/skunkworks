# 20. Chimera Hive Networking

Date: 2024-05-21

## Status

Accepted

## Context

The `chimera-lang` experiment simulates biological organisms within a constrained virtual environment (the Petri Dish). While the "Nova" expansion introduced complex internal biology (hormones, time travel), organisms were fundamentally isolated within their own VM instance.

To enable **social behavior**, **swarming**, and **distributed simulation**, we need a mechanism for Chimera organisms to communicate with entities outside their local memory space. This communication channel must:

1.  **Be Asynchronous:** Blocking the main VM loop for network I/O would freeze the simulation, which is unacceptable for a real-time system.
2.  **Be Resilient:** In a biological context, communication is often noisy and lossy (e.g., pheromones dissipating, sound waves fading). A strict TCP connection model feels too rigid and "engineered."
3.  **Support Many-to-Many:** Organisms should be able to broadcast or message multiple peers without maintaining heavy connection state.

## Decision

We will implement the **Hive** networking system using **UDP (User Datagram Protocol)**, gated behind the `feature = "hive"` flag.

### 1. Protocol
We choose UDP over TCP because:
- **Fire-and-Forget:** Fits the biological metaphor of "shouting" or "releasing a signal."
- **Low Overhead:** No handshake or connection state maintenance required for the VM.
- **Real-time:** Dropped packets are preferable to latency spikes caused by retransmission.

### 2. Serialization
We will use **JSON** for the wire format.
- **Flexibility:** Allows sending complex Chimera `Value` types (Integers, Strings) easily.
- **Debuggability:** Messages are human-readable, aiding in development and visualization.

### 3. Ephemeral Send Sockets
To avoid thread contention and blocking on the primary listening socket, the `HiveSend` opcode will create a **new, ephemeral UDP socket** for every transmission (`0.0.0.0:0`).
- The listening socket (`HiveBind`) remains dedicated to `HiveRecv`.

### 4. Non-Blocking I/O
All sockets will be set to `nonblocking = true`.
- `HiveRecv` will return `0` (or a specific "Empty" signal) if no data is available, rather than halting execution.

## Consequences

**Positive:**
- **Decoupling:** VMs can run on different machines or processes and still interact.
- **Resilience:** The system tolerates peer failure naturally (no "connection reset" errors).
- **Simplicity:** The VM instruction set only needs 4 opcodes (`HiveBind`, `HiveSend`, `HiveRecv`, `HiveClose`).

**Negative:**
- **Reliability:** Packets *will* be lost. The application layer (Chimera code) must handle this (e.g., via redundancy).
- **Security:** Binding to `0.0.0.0` exposes the VM to the local network. Malicious packets could potentially crash the deserializer.
- **Performance:** JSON serialization is slower than binary formats (like Bincode or Protobuf). Creating a socket per `HiveSend` adds syscall overhead.
