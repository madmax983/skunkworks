# Waggle Dance 🐝💃

**Genetic Cross**: `Bee Waggle Dance` + `Distributed Consensus`

A simulation of quorum sensing in honeybees for decentralized decision making.
This experiment demonstrates how a swarm of simple agents can agree on the best "food source" (consensus value) without a leader, purely through local interactions and positive feedback loops.

## The Concept

The hive needs to choose the best site among several options.
- **Scouts** find sites randomly.
- **Dancers** return to the hive and advertise the site. The duration and intensity of the dance are proportional to the site's quality.
- **Observers** watch dances and are recruited to visit the site. Better sites spawn longer, more intense dances, recruiting more bees.
- **Consensus** emerges when the majority of the hive is foraging at the highest quality site.

## Controls

- **Left Click**: Move the primary food source (Source 0).
- **Right Click**: Increase the quality of Source 0 (makes it more attractive).
- **Space**: Add a new random food source.
- **R**: Reset the simulation.

## Algorithm

1.  **Scout**: Random walk. If `distance(source) < visual_range`, evaluate `quality`.
2.  **Return**: Fly to hive.
3.  **Dance**:
    -   `duration = distance * C1`
    -   `intensity = quality * C2`
    -   Emit signal for `duration` ticks.
4.  **Observe**:
    -   Watch random dance.
    -   `probability_to_recruit = dance.intensity * sensitivity`.
    -   If recruited, fly to `target`.
5.  **Forage**: Fly to `target`. Re-evaluate `quality`. If `quality` drops, abandon (become Scout).

## Stack

-   **Macroquad**: Visualization.
-   **Rand**: Stochastic behavior.
