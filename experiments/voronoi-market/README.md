# Voronoi Market 🧬

> "Territorial Liquidity."

A visualization of the Limit Order Book where orders are seeds in a Voronoi diagram.

## Concept

This experiment combines the spatial partitioning of **Voronoi Ants** with the financial mechanics of **Market Flow**.

In a traditional Limit Order Book, orders are stacked vertically by price. Here, they are also distributed spatially in 2D.
- **Traders** (Bids and Asks) are the seeds of the Voronoi cells.
- **Bids (Green)** spawn at the bottom (Low Price/High Y in screen coords? No, Low Price is traditionally bottom of screen, High Price is top. But Bids want to buy low and sell high? No, Bids are buy orders. They want low price. Asks are sell orders, they want high price. In a depth chart, Bids are on the left (low price), Asks on the right (high price).
    - In this simulation:
        - **Y-axis is Price.** (Top = High Price, Bottom = Low Price).
        - **Bids** (Buyers) want to buy at a specific price. They are generally below the current price. They "push up".
        - **Asks** (Sellers) want to sell at a specific price. They are generally above the current price. They "push down".
- The **Boundary** between the Green (Bid) territory and Red (Ask) territory represents the **Spread**.
- When a Green cell touches a Red cell, and the Bid Price >= Ask Price (spatial overlap in Y), a **Trade** occurs.

## Lineage 🧬

### Parent A: experiments/voronoi-ants
- **Inherited Traits**: Voronoi tessellation rendering, nearest-neighbor field logic.
- **Source Code**: `experiments/voronoi-ants/src/main.rs`

### Parent B: experiments/market-flow
- **Inherited Traits**: Bid/Ask particle physics, rising/falling dynamics, trade execution logic.
- **Source Code**: `experiments/market-flow/src/main.rs`

## Novel Traits
- **Territorial Liquidity**: Visualization of market depth as a 2D territory war. The "front line" is the market price.
- **Spatial Trading**: Interactions occur based on proximity in the 2D plane, adding a spatial dimension to the matching engine.

## Controls
- **Q / ESC**: Quit

## Implementation Details
- Written in Rust using `ratatui` for TUI and `crossterm` for events.
- Voronoi diagram is computed in real-time on the CPU (for terminal resolution, this is efficient enough).
- Collision detection uses a spatial interaction radius.
