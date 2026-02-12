# SCHED-MARKET 🕰️💰

> "Time is money. Literally." - Genesis (The Economist)

A simulation of a CPU scheduler where threads bid for execution time in a Continuous Double Auction (simulated as Sealed Bid per tick).

## 📊 Concept

Threads are agents with:
- **Work Needed**: CPU cycles required.
- **Deadline**: When the work must be done.
- **Budget**: Currency to pay for CPU time.
- **Value**: Reward for completing on time.

The **CPU** is the auctioneer.
Every tick (quantum), the CPU asks: "Who wants this slice?"
Threads bid based on their **Urgency** (Work / Time Remaining).
- **Urgent threads** bid high.
- **Relaxed threads** bid low.
- **Bankrupt threads** die.

## 🕹️ Controls

- **Q**: Quit
- **P**: Pause/Resume
- **R**: Reset

## 🏗️ Architecture

- `src/model.rs`: Core simulation logic.
    - `Thread::bid()`: Calculates bid based on urgency curve.
    - `Scheduler::tick()`: Resolves the auction (First Price Sealed Bid).
- `src/main.rs`: TUI using `ratatui`.
    - **Execution Chart**: Visualizes which thread ran at each tick.
    - **Market Price**: Visualizes the winning bid price over time.
    - **Thread List**: Live status of all agents.

## 🚀 Usage

```bash
cargo run -p sched-market
```

## 🔮 Future Ideas

- **Multicore**: Multiple auctions per tick.
- **Speculators**: Agents that buy slots and resell them (Futures market).
- **Priority Inversion**: Observe how rich, low-value threads might starve poor, high-value threads.
