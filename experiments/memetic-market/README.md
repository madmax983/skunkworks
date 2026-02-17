# Memetic Market 📈🧠

> "The Attention Economy is a Bubble Machine." — Genesis (The Economist)

**Memetic Market** is a simulation of how hype cycles form and crash in an attention-based economy.
It models agents (Influencers, Followers, Contrarians) allocating their scarce **Attention** to various **Topics** (Memes).

## 🧬 Concept

This experiment combines **Behavioral Economics** + **User Behavior Modeling**.

*   **Currency**: Attention (Finite resource per agent).
*   **Asset**: Topics (Memes/Technologies) with hidden `Intrinsic Value`.
*   **Price**: Total Attention allocated to a topic.
*   **Emergence**:
    *   **Bubbles**: When Price exceeds Intrinsic Value due to hype.
    *   **Crashes**: When agents withdraw attention en masse.
    *   **Hype Cycle**: Innovation -> Peak -> Trough -> Slope -> Plateau.

## 🤖 Agents

*   **Trend Follower**: Buys if price is rising.
*   **Value Investor**: Buys if price < intrinsic value.
*   **Hype Beast**: Buys if price is high (Momentum).
*   **Contrarian**: Bets against the trend.
*   **Random**: Noise traders.

## 🎮 Visuals

The TUI dashboard shows:
*   **Left**: Ticker of active topics and price changes.
*   **Right**: Chart of the selected topic's Price History vs Intrinsic Value.
*   **Bottom Left**: Top Agents by Cash (Unallocated Attention).

## 🚀 Usage

```bash
cargo run -p memetic-market
```

## Controls

*   `Up` / `Down`: Select Topic.
*   `q`: Quit.

## Experiment Notes

*   Stack: `ratatui` (TUI), `rayon` (Parallel Agents), `rand` (Stochasticity).
*   Parameters: Tuned for fast cycles (~100ms ticks).
