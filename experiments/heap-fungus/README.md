# Heap Fungus 🍄🗑️

> "Memory is the soil. Objects are the fruit. Garbage is the feast."

**Heap Fungus** is a visualization of the **Mark-and-Sweep Garbage Collection** algorithm as a biological competition between two fungal species.

## 🧬 Concept

The memory heap is a forest floor.
- **Soil (Free Memory)**: Empty space where fungi can grow.
- **Objects (Allocations)**: Nutrients placed by the Mutator (the program).
- **Roots (Stack)**: The source of life for the symbiotic Marker fungus.

## 🍄 Species

1.  **Myco-Marker (Symbiote)**:
    -   **Role**: The Marker (GC Mark Phase).
    -   **Behavior**: Grows from the Roots along References (Hyphae) to reach Objects.
    -   **Effect**: Coats reachable objects with a protective enzyme (Blue/Cyan). It cannot grow across empty soil.

2.  **Myco-Sweeper (Decomposer)**:
    -   **Role**: The Sweeper (GC Sweep Phase).
    -   **Behavior**: Spores land randomly on the soil. It grows rapidly across empty space and attacks unprotected objects.
    -   **Effect**: Devours any object *not* protected by the Marker fungus. When consumed, the object dissolves into fresh soil (Free Memory). It is repelled by the Marker's enzyme.

## 🎮 Controls

-   `q`: Quit.
-   `a`: **Allocate** (Drop new nutrient objects).
-   `d`: **Drop Roots** (Sever connections, creating garbage).
-   `r`: **Reset** simulation.
-   `SPACE`: **Pause/Resume**.

## 🧪 Simulation

The simulation runs in real-time.
-   **Mutator**: Randomly allocates and modifies references.
-   **GC**: The fungi react to the changes in topology.
    -   If an object becomes unreachable, the Marker fungus retreats (dies back).
    -   The Sweeper fungus then moves in to consume the now-unprotected object.

## 📦 Usage

```bash
cargo run -p heap-fungus
```
