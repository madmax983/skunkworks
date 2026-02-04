# 🧬 Chimera Market

**Hybrid**: `chimera-lang` × `market-flow`

A TUI simulation where trading algorithms are **Chimera VM** instances that evolve via natural selection.

## 🔬 Concept

The market is populated by 20 "Traders". Each Trader possesses a genome (DNA) of Chimera Assembly instructions.
At every tick:
1.  **Market Physics**: Bids (Green) move up, Asks (Red) move down. Collisions create Trades.
2.  **Perception**: Traders perceive the current price, trend, and their own inventory/balance.
3.  **Cognition**: Each Trader runs its genetic code on a Chimera VM.
4.  **Action**: The VM outputs an action (Buy/Sell) and a price offset.
5.  **Selection**: Successful traders (high balance) replicate and mutate. Bankrupt traders are replaced.

## 🧬 Lineage

-   **Parent A (`chimera-lang`)**: Provided the Virtual Machine architecture, DNA structure, and mutation enzymes.
-   **Parent B (`market-flow`)**: Provided the particle-based limit order book simulation and TUI visualization components.

## ⚗️ Emergent Traits

-   **Algorithmic Speciation**: Distinct trading strategies (Scalpers, Trend Followers, Market Makers) may emerge from the random soup of instructions.
-   **Market Ecology**: The ecosystem balances itself. If everyone buys, price skyrockets, favoring sellers (or short-sellers if implemented).

## 🎮 Controls

-   `q` or `Esc`: Quit
