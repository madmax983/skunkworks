# Fairy Consensus 🍄🧚‍♀️

**"Fairy rings + Consensus protocol visualization"**

## Concept
A visualization of distributed consensus (inspired by Raft/Paxos) mapped to the biological behavior of fungal fairy rings.

- **Nodes** are Spore Clusters.
- **Vote Requests** are expanding Hyphal Rings.
- **Leadership** is signaled by ring generation.
- **Consensus** spreads like an infection or colonization of the substrate.

## How it works
- **Spore Clusters (Nodes):** Exist in one of four states:
    - `Follower` (Gray): Listening for signals.
    - `Candidate` (Yellow): Timed out, seeking leadership.
    - `Leader` (Red): Elected, broadcasting heartbeats (Rings).
    - `Committed` (Green): Accepted a leader's signal.
- **Rings:** Expand outward from Leaders. When they hit a Follower, they reset its election timer and convert it to the Leader's cause (Committed).
- **Election:** If a Follower hears nothing for a random duration (Election Timeout), it becomes a Candidate and spawns a Ring to solicit votes.

## Controls
- `q`: Quit simulation.

## Tech Stack
- **Ratatui**: TUI rendering (Canvas).
- **Rand**: Stochastic behavior.
