# Process Auction 📉📊

**"Agents bidding for CPU cycles in a Double Auction Market."**

This experiment simulates an operating system scheduler where processes must bid for CPU time using an internal currency ("Wealth").

## Concept
- **Processes**: Agents with a finite amount of work to do. They earn a "Universal Basic Income" (UBI) every tick.
- **CPU Cores**: Resources that sell time slices (ticks).
- **Market**: A Continuous Double Auction (CDA) grid where Bids (Processes) and Asks (Cores) meet.
- **Price**: The emergent cost of a CPU cycle, driven by demand (number of ready processes) and supply (number of idle cores).

## Logic
1.  **Bidding**: Processes calculate a bid price based on their wealth and urgency (progress).
2.  **Matching**: The `market-sim` crate handles the collision of Bid and Ask particles.
3.  **Execution**: When a trade occurs, the process pays the price and is assigned to a Core for a fixed time slice.
4.  **Preemption**: If a Core finds a higher bidder (or just a new trade), it *can* preempt the current tenant (ruthless market). *Currently implemented as: Winner takes the core, previous tenant gets booted.*

## Visualization
- **Top Left**: Process Table (PID, State, Wealth, Progress).
- **Top Right**: CPU Core Status.
- **Bottom**: The Market Grid.
    - **Green**: Bids (Buyers).
    - **Red**: Asks (Sellers).
    - **Yellow**: Trades (Execution).

## Controls
- `q`: Quit.

## Dependencies
- `ratatui` (TUI)
- `rayon` (Parallel Agents)
- `market-sim` (Physics-based Market)
