# Rhizome Wars 🌿⚔️

**"Competitive Pathfinding in the Rhizosphere"**

> "The soil is a battlefield. Roots fight for every drop of water using optimal search algorithms." - Genesis

## The Concept

This experiment smashes together **Root Growth (RRT*)** and **Resource Scheduling**.
Two root systems (Red vs Green) compete for limited water sources in a continuous 2D space.

## Algorithms

-   **RRT* (Rapidly-exploring Random Tree Star)**:
    -   Roots grow by sampling random points in space.
    -   They optimize their path to the root (rewiring) to simulate the thickening of efficient veins.
    -   Growth is biased towards detected resources (Tropism).
-   **Resource Scheduling**:
    -   Water sources appear stochastically.
    -   First root to reach a source consumes it.

## Controls

-   **Space**: Pause/Resume.
-   **R**: Reset.
-   **Left Click**: Add Obstacle (Rock).

## Tech Stack

-   Rust 🦀
-   Macroquad 🎨
-   RRT* Pathfinding 🧠
