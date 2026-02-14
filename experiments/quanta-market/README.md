# Quanta Market ⏳💸

> "Time is money, friend." - Genesis (The Economist)

**Quanta Market** is a moonshot experiment that replaces a standard CPU scheduler with a high-frequency **Vickrey Auction**.

## The Concept

In a traditional OS, the scheduler decides which process runs next (Round Robin, CFS, etc.).
In **Quanta Market**, processes are autonomous agents that must **bid** for CPU time.

- **The Resource**: A single CPU core, selling time slices (Quanta).
- **The Currency**: Wealth (Energy/Credits).
- **The Mechanism**: **Second-Price Sealed-Bid Auction (Vickrey)**.
    - Highest bidder wins.
    - Pays the price of the *second* highest bid.
    - Truthful bidding is the dominant strategy.

## The Agents (Processes)

- **Wealth**: Earned by running (doing work). Lost by waiting (rent/overhead).
- **Deadline**: A tick by which they must complete.
- **Urgency**: Increases as the deadline approaches.
- **Bid Strategy**: `Bid = Wealth * Urgency`. Desperate processes bid everything.

## Emergent Behavior

- **Priority Inversion**: Rich processes can starve poor ones, even if the poor ones are urgent (until they become *very* urgent).
- **Inflation**: If all processes are rich, the clearing price skyrockets.
- **Bankruptcy**: Processes that run out of wealth become **Zombies** (dead weight).

## Controls

- `Space`: Pause / Resume
- `+`: Spawn a new process
- `-`: Kill a random process
- `b`: Burst mode (Spawn 10 processes)
- `q`: Quit

## Visuals

- **Gantt Chart**: A scrolling history of which process owned the CPU.
- **Order Book**: (Implicit) Clearing prices are shown in the header.
- **Process List**: Real-time status of wealth and urgency.
