# 🦜 Phonetic Flock

**A Linguistic Simulation of Word Evolution using Flocking Dynamics.**

> "Words are not static; they move, they flock, and when they collide, they change." - The Splice Surgeon

## 🧬 Lineage
- **Parent A**: `experiments/literary-boids` (The Mycelium)
  - *Inheritance*: Flocking physics, Canvas rendering, text seeding.
- **Parent B**: `experiments/phonetic-decay` (The Philologist)
  - *Inheritance*: Sound laws (Grimm's Law, Vowel Shift, etc.), phonological evolution engine.
- **Hybrid Trait**: Words are treated as biological agents (Boids). When they flock together (high density), they influence each other's phonology, triggering sound shifts. Isolated words remain static, while crowded words evolve rapidly into pidgins or new dialects.

## 🕹️ Controls
- **Q**: Quit the simulation.

## 🎵 How it Works
1.  **Seeding**: The world is populated with words from Kafka's *The Metamorphosis*.
2.  **Flocking**: Words follow standard Boid rules (Separation, Alignment, Cohesion).
3.  **Evolution**:
    - When a word has more than 3 neighbors within its view radius, it feels "social pressure".
    - This triggers a random **Sound Law** (e.g., *p* -> *f*, *a* -> *ei*).
4.  **Visualization**:
    - **White**: Original word (unchanged).
    - **Yellow**: Changed (phonetic shift).
    - **Red**: Decay (word got shorter).
    - **Green**: Growth (word got longer).

## 🏗️ Architecture
The system combines the `Vec2` physics engine of `literary-boids` with the `Evolver` state machine of `phonetic-decay`. It runs on `ratatui` for terminal visualization.
