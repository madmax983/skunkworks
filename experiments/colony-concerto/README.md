# Colony Concerto 🐜🎻

**Lineage:** `hive-mind-dependencies` × `schrodingers-beat`

A swarm of builder ants traverses a dependency graph. To "build" a node (dependency), an ant must acquire a lock on it.
Contention for these locks—representing critical build paths—creates rhythmic phasing and dissonance.

## Concept

- **The Score:** The structure of the dependency graph determines the melody. Layer 0 nodes are bass, higher layers are higher pitch.
- **The Performers:** Ants are the threads. They traverse the graph probabilistically based on pheromones.
- **The Instrument:** `Mutex<Node>`. When an ant locks a node, it plays a tone. If it fights for a lock, it plays a dissonance.

## Key Traits

- **Polyrhythmic Contention:** Inherited from `schrodingers-beat`. Threads fighting for resources create the rhythm.
- **Swarm Intelligence:** Inherited from `hive-mind-dependencies`. Ants find paths through the graph.
- **Visualized Concurrency:** TUI shows which nodes are locked (Red/Yellow) and which are idle (White).

## Controls

- `q`: Quit
